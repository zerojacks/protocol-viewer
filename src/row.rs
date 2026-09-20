//! 将解析结果的 FieldValue 树转换为平铺的 Row 列表，用于树形展示。
//! 
//! 这份模型是 UI 树控件渲染和图片导出共用的唯一数据源：两边都只消费
//! `Vec<Row>`，不需要为了展示再重新遍历一次 Value / 再做一次类型转换。

use protocol_parser::FieldValue;
use std::ops::Range;

/// 一行三栏数据：字段名（按 depth 缩进）、数据（原始字节的十六进制）、说明（解析后的可读值）。
#[derive(Clone, Debug)]
pub struct Row {
    /// 深度（从 0 开始）
    pub depth: usize,
    /// 字段名称
    pub field: String,
    /// 数据值（十六进制）
    pub data: String,
    /// 说明描述（可读值）
    pub desc: String,
    /// 是否存在子节点，由扁平化阶段一次性计算。
    pub has_children: bool,
    /// 节点在去掉分隔符后的原始字节流中的范围。
    pub raw_range: Range<usize>,
}

/// 字节数组 -> 不带分隔符的大写十六进制字符串，如 [0x00,0x22] -> "0022"
pub fn hex_no_sep(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02X}", b)).collect()
}

/// 字节数组 -> 带空格分隔的大写十六进制字符串，如 [0x00,0x22] -> "00 22"
pub fn hex_spaced(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join(" ")
}

/// 描述一个"叶子"取值（不下钻的最终展示文本），例如 "220.0 V"、"尖峰谷分时电价"。
/// 容器类型（Map/List/Node）不在这里处理，交给 flatten 递归下钻。
fn describe_leaf(v: &FieldValue) -> String {
    match v {
        FieldValue::Int(i) => i.to_string(),
        FieldValue::Float(f) => {
            // 和 decimal 精度对齐的简单展示；如需保留固定小数位，可在这里按需调整。
            if f.fract() == 0.0 {
                format!("{:.0}", f)
            } else {
                format!("{}", f)
            }
        }
        FieldValue::Str(s) => s.clone(),
        FieldValue::Bytes(b) => hex_no_sep(b),
        FieldValue::WithUnit { value, unit } => format!("{} {}", describe_leaf(value), unit),
        FieldValue::Bit {
            bit_value, value, ..
        } => {
            if let Some(v) = value {
                describe_leaf(v)
            } else {
                bit_value.to_string()
            }
        }
        FieldValue::Pn(n) => format!("Pn{}", n),
        FieldValue::Skip => String::new(),
        // Map/List/Node 属于容器，理论上不会走到这个分支（flatten 里会先分流），
        // 兜底给空字符串而不是 panic，避免未来新增 FieldValue 变体时这里漏配直接崩溃。
        FieldValue::Map(_) | FieldValue::List(_) | FieldValue::Node { .. } => String::new(),
        FieldValue::Invalid { reason } => format!("{}", reason),
    }
}

/// 递归拍平：把 FieldValue 树按 YAML 声明的嵌套层级展开成一份有序的 Row 列表。
///
/// - `FieldValue::Node{name, raw, value}`：产生一行（field=name, data=hex(raw)），
///   如果 value 本身还是容器（Map/List/Node），说明需要继续，行的 desc 留空，
///   然后以 depth+1 递归展开子节点；否则 desc 用 describe_leaf 直接给出最终值。
/// - `FieldValue::Map(entries)`：
///   - **单条目 Map**：这是包装层，直接跳过，展开其唯一的值（不增加深度）
///   - **多条目 Map**：表示多个字段，正常展开所有条目
/// - `FieldValue::List(items)`：本身不产生行，把内部条目按 **depth+1** 递归摊开
fn raw_span(value: &FieldValue) -> usize {
    match value {
        FieldValue::Node { raw, .. } => raw.len(),
        FieldValue::Map(entries) => entries.iter().map(|(_, value)| raw_span(value)).sum(),
        FieldValue::List(items) => items.iter().map(raw_span).sum(),
        _ => 0,
    }
}

fn flatten_at(value: &FieldValue, depth: usize, raw_offset: usize, out: &mut Vec<Row>) {
    match value {
        FieldValue::Node { name, raw, value } => {
            // 检查是否为容器类型（包括嵌套的 Node）
            let is_container = matches!(
                value.as_ref(), 
                FieldValue::Map(_) | FieldValue::List(_) | FieldValue::Node { .. }
            );
            
            out.push(Row {
                depth,
                field: name.clone(),
                data: hex_spaced(raw),
                desc: if is_container {
                    String::new()
                } else {
                    describe_leaf(value)
                },
                has_children: is_container,
                raw_range: raw_offset..raw_offset + raw.len(),
            });
            
            if is_container {
                flatten_at(value, depth + 1, raw_offset, out);
            }
        }
        FieldValue::Map(entries) => {
            if entries.len() == 1 {
                // 单条目 Map：包装层，直接跳过，展开其值（不增加深度）
                flatten_at(&entries[0].1, depth, raw_offset, out);
            } else {
                // 多条目 Map：表示多个字段，正常展开所有条目
                let mut child_offset = raw_offset;
                for (_, v) in entries {
                    flatten_at(v, depth, child_offset, out);
                    child_offset += raw_span(v);
                }
            }
        }
        FieldValue::List(items) => {
            // List 本身不产生行，展开所有项时深度 +1
            // 因为 List 的项是其父 Node 的子节点
            let mut child_offset = raw_offset;
            for item in items {
                flatten_at(item, depth, child_offset, out);  // 父 Node 已经增加深度
                child_offset += raw_span(item);
            }
        }
        // 顶层直接传入一个非 Node 的裸值（理论上 parse_di 总是包一层 Node，
        // 这里兜底处理调用方直接传 parse_field 结果的情况）。
        other => out.push(Row {
            depth,
            field: String::new(),
            data: String::new(),
            desc: describe_leaf(other),
            has_children: false,
            raw_range: raw_offset..raw_offset,
        }),
    }
}

/// 直接构建 Row 列表，不添加根节点
pub fn build_rows(value: &FieldValue) -> Vec<Row> {
    let mut rows = Vec::new();
    // 直接从深度 0 开始展开
    flatten_at(value, 0, 0, &mut rows);
    for index in 0..rows.len().saturating_sub(1) {
        rows[index].has_children = rows[index].has_children
            || rows[index + 1].depth > rows[index].depth;
    }
    rows
}
