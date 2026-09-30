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
        FieldValue::WithUnit { value, unit } => {
            // 内层是容器时不拼"值 + 单位"：容器本身没有叶子文本，拼出来会多一个前导空格。
            // 这种情况是"用 `WithUnit` 给一行挂说明文本、子项另放"的写法
            // （见 csg-local-comm 的 DI 行），说明列就取 `unit`。
            if is_container(value) {
                unit.clone()
            } else {
                format!("{} {}", describe_leaf(value), unit)
            }
        }
        // 位字段落在"数据"列时显示的必须是这段 bit 的取值（单 bit = 0/1，区间 = 提取出的数值），
        // 不能退回 bit_byte 的整字节十六进制，也不能用语义描述顶替——语义描述属于"说明"列，
        // 由 flatten 通过 value 参数单独取出。
        FieldValue::Bit { .. } => v.bit_text().unwrap_or_default(),
        FieldValue::Pn(n) => format!("Pn{}", n),
        FieldValue::Skip => String::new(),
        // Map/List/Node 属于容器，理论上不会走到这个分支（flatten 里会先分流），
        // 兜底给空字符串而不是 panic，避免未来新增 FieldValue 变体时这里漏配直接崩溃。
        FieldValue::Map(_) | FieldValue::List(_) | FieldValue::Node { .. } => String::new(),
        FieldValue::Invalid { reason } => format!("{}", reason),
    }
}

/// 位字段的"值"只能是它自己的取值，永远不是容器——即使 `Bit.value` 里挂着语义描述，
/// 也不需要再往下钻一层子节点。
fn is_container(value: &FieldValue) -> bool {
    match value {
        FieldValue::Map(_) | FieldValue::List(_) | FieldValue::Node { .. } => true,
        // `WithUnit` 内层是容器时要继续下钻：这一行是"说明文本挂在 unit 上、子项放在
        // 内层"的写法（见 csg-local-comm 的 DI 行）。
        FieldValue::WithUnit { value: inner, .. } => is_container(inner),
        _ => false,
    }
}

/// 位字段在原始报文字节流里的**绝对**字节下标，完全由 `Bit.bit_byte` 决定。
///
/// `block_raw` / `block_start` 是**离它最近的那个带原始字节的祖先**（帧层是控制域C 这种
/// 单字节节点，DI 里是"待升级电表地址列表"这种多字节位图节点）以及这个祖先在报文里的
/// 起始字节下标。中间隔着的 `List`/纯包装 `Node` 的 `raw` 是空的，所以不能只看直接父节点。
///
/// `bit_byte` 是"这一段 bit 所在的那个原始字节"，本来就是定位用的，于是：
/// - `bit_start / 8 != 0`（spec-engine `parse_bitfield` 里 `bit_start` 是**整块相对**的）：
///   块内字节下标就是这个值，直接信；
/// - `bit_start / 8 == 0`（帧层 `bit_node()`、以及 spec-engine `parse_bitmask`——它们的
///   `bit_start` 都是**相对这一个字节**的位下标）：这个值给不出块内位置，于是在块自己的
///   字节流里按**值**找 `bit_byte` 那一字节，它在块里必然存在。
///
/// 相同字节值在块内重复出现时取最靠前的一个（最坏是块内高亮错位，不会跑到块外）；
/// 块里找不到（比如祖先的 raw 与这段内容不一致）时才退回块的起点。
fn bit_byte_offset_in_block(value: &FieldValue, block_raw: &[u8], block_start: usize) -> usize {
    let Some(byte_offset) = value.bit_source_byte_offset() else {
        return block_start; // 没有 bit_byte：无从定位
    };
    if byte_offset != 0 {
        return block_start + byte_offset;
    }
    let Some(source_byte) = value.bit_source_byte().and_then(|b| b.first().copied()) else {
        return block_start;
    };
    block_start
        + block_raw
            .iter()
            .position(|b| *b == source_byte)
            .unwrap_or(0)
}

/// 位字段行在原始报文字节流里占用的字节数。
///
/// 位字段自己的 `raw` 是空的（原始字节在 `Bit.bit_byte` 里），位置必须**从 `bit_byte`
/// 推**：一个位字段规格不会跨字节，所以确定位置之后正好占 1 个字节。没有 `bit_byte`
/// 就无从定位，返回 0（这一行没有高亮范围，而不是错误地去指向别的地方）。
fn bit_span(value: &FieldValue) -> usize {
    match value.bit_source_byte_offset() {
        Some(_) => 1,
        None => 0,
    }
}

/// 一个值在原始报文字节流里占用的字节数（Row 的 `raw_range` 用的是**字节下标**，
/// 和 `app.rs` 里 `hex_byte_ranges` 切出来的字节数组一一对应）。
fn raw_span(value: &FieldValue) -> usize {
    match value {
        FieldValue::Node { raw, value, .. } => {
            if matches!(value.as_ref(), FieldValue::Bit { .. }) {
                bit_span(value)
            } else {
                raw.len()
            }
        }
        FieldValue::Bit { .. } => bit_span(value),
        FieldValue::Map(entries) => entries.iter().map(|(_, value)| raw_span(value)).sum(),
        FieldValue::List(items) => items.iter().map(raw_span).sum(),
        _ => 0,
    }
}

/// 递归拍平：把 FieldValue 树按 YAML 声明的嵌套层级展开成一份有序的 Row 列表。
///
/// - `FieldValue::Node{name, raw, value}`：产生一行（field=name, data=hex(raw)），
///   如果 value 本身还是容器（Map/List/Node），说明需要继续，行的 desc 留空，
///   然后以 depth+1 递归展开子节点；否则 desc 用 describe_leaf 直接给出最终值。
/// - **位字段**（`value` 是 `FieldValue::Bit`）：`raw` 按约定是空的，
///   - "数据"列 = `bit_text()`，也就是这一段 bit 的取值（单 bit = 0/1），**不是**所在字节；
///   - "说明"列 = `Bit.value` 里的语义描述（没有语义描述时退回 bit 取值，避免整行空着）；
///   - 位置 = `Bit.bit_byte` 在**离它最近的那个带原始字节的祖先**（`block_raw`/`block_start`）
///     里的下标，所以多字节位图块（如 DI "待升级电表地址列表" 256 字节）里的每个 bit
///     也能落到各自所在的字节，而不是都挤在块的第一字节上。
/// - `FieldValue::Map(entries)`：
///   - **单条目 Map**：这是包装层，直接跳过，展开其唯一的值（不增加深度）
///   - **多条目 Map**：表示多个字段，正常展开所有条目
/// - `FieldValue::List(items)`：本身不产生行，把内部条目按 **depth+1** 递归摊开
///
/// `block_raw`/`block_start` 是"当前所在的原始字节块"：带有非空 `raw` 的节点会把自己
/// 的 raw 和起始下标传下去，只有 `List`/纯包装节点会继续沿用祖先的块。
fn flatten_at(
    value: &FieldValue,
    depth: usize,
    raw_offset: usize,
    block_raw: &[u8],
    block_start: usize,
    out: &mut Vec<Row>,
) {
    match value {
        FieldValue::Node { name, raw, value } => {
            let has_children = is_container(value.as_ref());

            // 位字段的"数据"列文本（单 bit = 0/1，区间 = 提取出的数值），非位字段为空
            let bit_text = value.as_ref().bit_text();

            // "说明"列的文本。容器节点默认留空（说明留给孩子），但有两种例外：
            // - 位字段：说明列给 `Bit.value` 里的语义描述，没有就用 bit 取值兜底；
            // - `WithUnit{内层是容器}`：这是"说明文本挂在这一行、子项放在内层"的写法
            //   （csg-local-comm 的 DI 行），说明列取 `unit`。
            let desc_text = match (bit_text.as_ref(), value.as_ref()) {
                (Some(bit_text), FieldValue::Bit { value: semantic, .. }) => semantic
                    .as_deref()
                    .map(inline_text)
                    .unwrap_or_else(|| bit_text.clone()),
                (Some(bit_text), _) => bit_text.clone(),
                (None, FieldValue::WithUnit { value: inner, unit }) if is_container(inner) => {
                    unit.clone()
                }
                (None, other) if !has_children => describe_leaf(other),
                (None, _) => String::new(),
            };

            let (data, byte_range) = if let Some(bit_text) = bit_text {
                // 高亮范围：位字段自己的 raw 是空的，位置从 Bit.bit_byte 在所在字节块里推
                let start = bit_byte_offset_in_block(value.as_ref(), block_raw, block_start);
                (bit_text, (start, bit_span(value.as_ref())))
            } else {
                (hex_spaced(raw), (raw_offset, raw.len()))
            };

            out.push(Row {
                depth,
                field: name.clone(),
                data,
                desc: desc_text,
                has_children,
                raw_range: byte_range.0..byte_range.0 + byte_range.1,
            });

            // 子节点：带 raw 的节点就是子节点所在的字节块，起始下标取父节点这一行的起点
            // （包装节点的 raw 是空的，继续沿用祖先的块）
            if has_children {
                let (child_block, child_block_start) = if raw.is_empty() {
                    (block_raw, block_start)
                } else {
                    (raw.as_slice(), byte_range.0)
                };
                flatten_at(value, depth + 1, child_block_start, child_block, child_block_start, out);
            }
        }
        FieldValue::Map(entries) => {
            if entries.len() == 1 {
                // 单条目 Map：这是包装层，直接跳过，展开其值（不增加深度）
                flatten_at(&entries[0].1, depth, raw_offset, block_raw, block_start, out);
            } else {
                // 多条目 Map：表示多个字段，正常展开所有条目
                let mut child_offset = raw_offset;
                for (_, v) in entries {
                    flatten_at(v, depth, child_offset, block_raw, block_start, out);
                    child_offset += raw_span(v);
                }
            }
        }
        // `WithUnit{内层是容器}`：说明文本已经挂在父行上了，这里只继续展开内层子项，
        // 不产生额外的行、也不增加深度。
        FieldValue::WithUnit { value: inner, .. } if is_container(inner) => {
            flatten_at(inner, depth, raw_offset, block_raw, block_start, out);
        }
        FieldValue::List(items) => {
            // List 本身不产生行，展开所有项时深度 +1
            // 因为 List 的项是其父 Node 的子节点
            let mut child_offset = raw_offset;
            for item in items {
                flatten_at(item, depth, child_offset, block_raw, block_start, out);  // 父 Node 已经增加深度
                child_offset += raw_span(item);
            }
        }
        // 顶层直接传入一个非 Node 的裸值（理论上 parse_di 总是包一层 Node，
        // 这里兜底处理调用方直接传 parse_field 结果的情况）。
        other => out.push(Row {
            depth,
            field: String::new(),
            // 裸的 Bit 也是位字段：数据列给 bit 取值
            data: if other.bit_text().is_some() {
                other.bit_text().unwrap_or_default()
            } else {
                describe_leaf(other)
            },
            desc: describe_leaf(other),
            has_children: false,
            raw_range: raw_offset..raw_offset + bit_span(other),
        }),
    }
}

/// 把"说明"列里可能出现的嵌套值拍成一行文本。
///
/// 只用于 `Bit.value` 这个位置：它是解析出来的语义值，可能是 `Int`（没配 enum_map 的
/// 位域）、`Str`（配了 enum_map 的）、也可能是 `WithUnit`，但不会是容器。
/// 和 `describe_leaf` 的区别是这里对 `Bit`/容器不做兜底，调用方保证只在语义值上用它。
fn inline_text(value: &FieldValue) -> String {
    match value {
        FieldValue::Map(_) | FieldValue::List(_) | FieldValue::Node { .. } => String::new(),
        other => describe_leaf(other),
    }
}

/// 直接构建 Row 列表，不添加根节点
pub fn build_rows(value: &FieldValue) -> Vec<Row> {
    let mut rows = Vec::new();
    // 直接从深度 0 开始展开；顶层还没有字节块，块起点按 0 传
    flatten_at(value, 0, 0, &[], 0, &mut rows);
    for index in 0..rows.len().saturating_sub(1) {
        rows[index].has_children = rows[index].has_children
            || rows[index + 1].depth > rows[index].depth;
    }
    rows
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 位字段子节点：[数据]列必须显示这一段bit的取值（单bit=0/1），
    /// 不能显示成所在字节（0xC4），也不能空白。
    fn bit_field(name: &str, bit_start: usize, bit_end: usize, bit_value: u64, desc: &str) -> FieldValue {
        FieldValue::Node {
            name: name.to_string(),
            // 位字段的 raw 按约定留空，原始字节在 bit_byte 里
            raw: Vec::new(),
            value: Box::new(FieldValue::Bit {
                bit_start,
                bit_end,
                bit_value,
                bit_byte: vec![0xC4],
                value: Some(Box::new(FieldValue::Str(desc.to_string()))),
            }),
        }
    }

    fn control_field_node() -> FieldValue {
        // 控制域C = 0xC4：D7=1 D6=1 D5=0 D4=0 D3~D0=4
        FieldValue::Node {
            name: "控制域C".to_string(),
            raw: vec![0xC4],
            value: Box::new(FieldValue::List(vec![
                bit_field("D7传输方向位DIR", 7, 7, 1, "终端发出的上行报文"),
                bit_field("D6启动标志位PRM", 6, 6, 1, "来自启动站"),
                bit_field(
                    "D5帧计数位FCB(下行)/要求访问位ACD(上行)",
                    5,
                    5,
                    0,
                    "终端无告警数据等待访问",
                ),
                bit_field("D4帧计数有效位FCV(下行)/保留(上行)", 4, 4, 0, "保留位"),
                bit_field("D3~D0功能码", 3, 0, 4, "来自启动站:用户数据（发送/无回答）"),
            ])),
        }
    }

    #[test]
    fn bit_field_rows_show_bit_value_not_source_byte() {
        let rows = build_rows(&control_field_node());

        assert_eq!(rows.len(), 6, "控制域C + 5个位字段子项");
        assert_eq!(rows[0].field, "控制域C");
        assert_eq!(rows[0].data, "C4", "父节点仍然显示整字节");

        // 子项：数据列 = 这一段bit的取值
        assert_eq!(rows[1].data, "1");
        assert_eq!(rows[2].data, "1");
        assert_eq!(rows[3].data, "0");
        assert_eq!(rows[4].data, "0");
        assert_eq!(rows[5].data, "4", "D3~D0 显示提取出来的数值4，不是整字节C4");

        // 说明列仍然是语义描述，没有被bit取值顶掉
        assert_eq!(rows[1].desc, "终端发出的上行报文");
        assert_eq!(rows[5].desc, "来自启动站:用户数据（发送/无回答）");

        // 每个位字段子项都指向它所在的那一个字节（控制域C 那 1 个字节），
        // 用 bit_byte 在父节点自己的字节流里定位：父节点 raw 就是 0xC4 本身，位置 0，
        // 加上父节点在报文里的起始下标 0 之外没有再偏移（这里父节点是顶层）。
        for row in &rows[1..] {
            assert_eq!(
                row.raw_range,
                0..1,
                "位字段子项应该指向所在的那一个字节，供UI高亮"
            );
        }
    }

    /// 位字段子项不额外占用字节：它们和父节点共用一个字节的偏移，
    /// 父节点后面的兄弟行必须紧跟着父节点继续排，不能被 5 个 bit 子项顶到后面去。
    #[test]
    fn bit_field_rows_do_not_shift_following_siblings() {
        let tree = FieldValue::List(vec![
            FieldValue::Node {
                name: "起始符".to_string(),
                raw: vec![0x68],
                value: Box::new(FieldValue::Str("起始符".to_string())),
            },
            control_field_node(),
            FieldValue::Node {
                name: "下一字段".to_string(),
                raw: vec![0xAA],
                value: Box::new(FieldValue::Str("下一字段".to_string())),
            },
        ]);
        let rows = build_rows(&tree);

        let control = rows.iter().find(|r| r.field == "控制域C").unwrap();
        let next = rows.iter().find(|r| r.field == "下一字段").unwrap();
        assert_eq!(control.raw_range, 1..2);
        assert_eq!(
            next.raw_range,
            2..3,
            "控制域只占1个字节，5个bit子项不应该把后面的字段顶开"
        );
    }

    /// 报文的十六进制文本经过 app.rs 的 hex_byte_ranges 映射后，点选位字段子项
    /// 应该正好选中它所在的那一个字节（而不是选错位置或者选中整条报文）。
    #[test]
    fn bit_field_row_maps_to_its_own_byte_in_hex_text() {
        let hex_text = "68 16 00 16 00 68 89 00";
        let whole_tree = FieldValue::List(vec![
            FieldValue::Node {
                name: "起始符".to_string(),
                raw: vec![0x68],
                value: Box::new(FieldValue::Str("起始符".to_string())),
            },
            FieldValue::Node {
                name: "长度".to_string(),
                raw: vec![0x16, 0x00, 0x16, 0x00],
                value: Box::new(FieldValue::Str("长度=22".to_string())),
            },
            FieldValue::Node {
                name: "起始符".to_string(),
                raw: vec![0x68],
                value: Box::new(FieldValue::Str("起始符".to_string())),
            },
            control_field_node(),
        ]);
        let rows = build_rows(&whole_tree);

        let byte_ranges = hex_byte_ranges(hex_text);
        let control_row = rows.iter().find(|r| r.field == "控制域C").unwrap();
        let start = control_row.raw_range.start;
        let end = control_row.raw_range.end;
        let text = &hex_text[byte_ranges[start].start..byte_ranges[end - 1].end];
        assert_eq!(text, "89");

        // 位字段子项点选后，映射出来的文本同样应该正好是控制域那一个字节
        for row in rows.iter().filter(|r| r.field.starts_with('D') && r.depth == 1) {
            let start = row.raw_range.start;
            let end = row.raw_range.end;
            let text = &hex_text[byte_ranges[start].start..byte_ranges[end - 1].end];
            assert_eq!(text, "89", "{} 应该指向控制域字节", row.field);
        }
    }

    /// 位字段落在父数据块的第几个字节，必须由 `Bit.bit_byte`/`bit_start` 推出来，
    /// 不能假设它一定在父节点的第一个字节里（DI 里的 bitmask/bitfield 就是多字节块）。
    #[test]
    fn bit_field_row_uses_bit_byte_to_locate_its_byte() {
        /// 一个字节里的一个 bit。`bit_start` 是**该字节内部**的位下标（帧层和
        /// spec-engine bitmask 都是这个基准），位置靠 `bit_byte` 在父块里定位。
        fn bit_in_byte(name: &str, bit_start: usize, byte: u8, bit_value: u64) -> FieldValue {
            FieldValue::Node {
                name: name.to_string(),
                raw: Vec::new(),
                value: Box::new(FieldValue::Bit {
                    bit_start,
                    bit_end: bit_start,
                    bit_value,
                    bit_byte: vec![byte],
                    value: Some(Box::new(FieldValue::Str(name.to_string()))),
                }),
            }
        }

        // 3 字节的位图块：0x02 0x00 0x80。
        // 第二个 bit 用的是 `bit_byte=0x80`、`bit_start=7`（bitmask 里 bit_start 相对
        // 它自己那个字节），靠"在父块里按值找 0x80"落到第 2 个字节。
        let block = FieldValue::Node {
            name: "待升级电表地址列表".to_string(),
            raw: vec![0x02, 0x00, 0x80],
            value: Box::new(FieldValue::List(vec![
                bit_in_byte("测量点2", 1, 0x02, 1),
                bit_in_byte("测量点17", 7, 0x80, 1),
            ])),
        };
        let rows = build_rows(&block);

        assert_eq!(rows[0].raw_range, 0..3, "父节点占3个字节");
        // 第一个 bit 在第0字节里的 bit1 → 指向第0字节
        assert_eq!(rows[1].data, "1");
        assert_eq!(rows[1].raw_range, 0..1, "第0字节里的bit应该指向第0字节");
        // 第二个 bit 在第2字节里的 bit7 → 指向第2字节，而不是父节点的第0字节
        assert_eq!(rows[2].data, "1");
        assert_eq!(
            rows[2].raw_range,
            2..3,
            "bit_byte 落在父数据块的最后一个字节，高亮范围也要落在那里"
        );
    }

    /// 位字段的"说明"列即使没有语义描述（spec-engine 未配 enum_map 时是 `Int`），
    /// 也不能空着；此时退回 bit 取值。
    #[test]
    fn bit_field_row_falls_back_to_bit_value_when_no_semantic_text() {
        let node = FieldValue::Node {
            name: "BIT(3-0)_功能码".to_string(),
            raw: Vec::new(),
            value: Box::new(FieldValue::Bit {
                bit_start: 3,
                bit_end: 0,
                bit_value: 9,
                bit_byte: vec![0x89],
                value: Some(Box::new(FieldValue::Int(9))),
            }),
        };
        let rows = build_rows(&node);
        assert_eq!(rows[0].data, "9", "数据列 = bit 取值");
        assert_eq!(rows[0].desc, "9", "没有语义描述时说明列退回 bit 取值");

        // 完全没有 value 时同理
        let bare = FieldValue::Node {
            name: "BIT(7)_DIR".to_string(),
            raw: Vec::new(),
            value: Box::new(FieldValue::Bit {
                bit_start: 7,
                bit_end: 7,
                bit_value: 1,
                bit_byte: vec![0x89],
                value: None,
            }),
        };
        let rows = build_rows(&bare);
        assert_eq!(rows[0].data, "1");
        assert_eq!(rows[0].desc, "1");
    }

    /// 一行既要有"说明"文本、又要有子项时用的是 `WithUnit` 包装（csg-local-comm 的 DI 行：
    /// 说明列放查表得到的 DI 名称，子项放 DI 的 4 个语义字节）。
    /// 这条测试锁住展示层对这种形状的处理：说明列取 `unit` 文本、`has_children` 仍然为真、
    /// 子项的范围落在父行那 4 个字节里、父行后面的兄弟行不被顶偏。
    #[test]
    fn node_with_label_and_children_keeps_description_and_children() {
        let bytes_of = |values: &[u8]| -> Vec<FieldValue> {
            values
                .iter()
                .enumerate()
                .map(|(i, b)| FieldValue::Node {
                    name: format!("子项{}", i + 1),
                    raw: vec![*b],
                    value: Box::new(FieldValue::Str(format!("{:02X}", b))),
                })
                .collect()
        };

        let di_row = FieldValue::Node {
            name: "DI".to_string(),
            raw: vec![0x02, 0x04, 0x02, 0xE8],
            value: Box::new(FieldValue::WithUnit {
                value: Box::new(FieldValue::List(bytes_of(&[0x02, 0x04, 0x02, 0xE8]))),
                unit: "DI=020402E8 (添加从节点)".to_string(),
            }),
        };
        let tree = FieldValue::List(vec![
            FieldValue::Node {
                name: "SEQ".to_string(),
                raw: vec![0x11],
                value: Box::new(FieldValue::Str("17".to_string())),
            },
            di_row,
            FieldValue::Node {
                name: "数据内容".to_string(),
                raw: vec![0x0A],
                value: Box::new(FieldValue::Str("从节点数量10".to_string())),
            },
        ]);

        let rows = build_rows(&tree);
        let di = rows.iter().find(|r| r.field == "DI").unwrap();
        assert_eq!(di.data, "02 04 02 E8", "数据列是整个 DI 的 4 个字节");
        assert_eq!(di.desc, "DI=020402E8 (添加从节点)", "说明列是 DI 名称");
        assert!(di.has_children, "DI 仍然有 4 个子项可展开");
        assert_eq!(di.raw_range, 1..5);

        // 4 个子项：各占 DI 里的一个字节，顺序就是报文字节顺序
        let children: Vec<&Row> = rows
            .iter()
            .filter(|r| r.depth == 1 && r.field.starts_with("子项"))
            .collect();
        assert_eq!(children.len(), 4);
        for (index, child) in children.iter().enumerate() {
            assert_eq!(child.raw_range, 1 + index..2 + index);
        }

        // 父行后面的兄弟行紧跟着 4 个字节，没有被顶偏
        let next = rows.iter().find(|r| r.field == "数据内容").unwrap();
        assert_eq!(next.raw_range, 5..6);
    }

    /// 直接复用 app.rs 里的十六进制文本→字节字符范围映射（同一段逻辑，避免测试和实现漂移）。
    fn hex_byte_ranges(input: &str) -> Vec<std::ops::Range<usize>> {
        let nibbles: Vec<(usize, usize)> = input
            .char_indices()
            .filter(|(_, ch)| ch.is_ascii_hexdigit())
            .map(|(start, ch)| (start, start + ch.len_utf8()))
            .collect();

        nibbles
            .chunks_exact(2)
            .map(|pair| pair[0].0..pair[1].1)
            .collect()
    }
}
