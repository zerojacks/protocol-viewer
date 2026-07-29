# 改进记录

## 参考 spec-engine-viewer 的优化

基于 `D:\Download\spec-engine-viewer\spec-engine-viewer\src` 的实现，我们对解析结果展示进行了以下改进：

### 1. **优化 Row 数据结构** ✅

#### 改进前
- 字段顺序不一致（field, data, desc, depth）
- 数据和说明分离不清晰

#### 改进后
```rust
#[derive(Clone, Debug)]
pub struct Row {
    pub depth: usize,      // 深度放在首位，更符合逻辑
    pub field: String,     // 字段名
    pub data: String,      // 数据（十六进制）
    pub desc: String,      // 说明（可读值）
}
```

### 2. **重构展示逻辑** ✅

#### 新增工具函数
```rust
// 不带分隔符的十六进制：0022
pub fn hex_no_sep(bytes: &[u8]) -> String

// 带空格分隔的十六进制：00 22
pub fn hex_spaced(bytes: &[u8]) -> String

// 叶子节点值描述
fn describe_leaf(v: &FieldValue) -> String
```

#### 核心算法：flatten
- 使用递归"拍平"算法，将 `FieldValue` 树转换为有序的 `Row` 列表
- 容器类型（Map/List/Node）智能展开
- 叶子节点直接转换为可读文本

```rust
pub fn flatten(value: &FieldValue, depth: usize, out: &mut Vec<Row>)
```

### 3. **改进 Node 处理** ✅

#### 改进前
```rust
FieldValue::Node { name, raw, value } => {
    // 简单递归，没有区分容器和叶子
}
```

#### 改进后
```rust
FieldValue::Node { name, raw, value } => {
    let is_container = matches!(value.as_ref(), FieldValue::Map(_) | FieldValue::List(_));
    out.push(Row {
        depth,
        field: name.clone(),
        data: hex_spaced(raw),  // 数据列显示原始字节
        desc: if is_container {
            String::new()       // 容器节点 desc 留空
        } else {
            describe_leaf(value) // 叶子节点显示解析值
        },
    });
    if is_container {
        flatten(value, depth + 1, out);
    }
}
```

**关键改进**：
- **数据列**显示原始字节（hex_spaced）
- **说明列**显示解析后的可读值（describe_leaf）
- 容器节点的 desc 留空，避免重复信息

### 4. **优化树线渲染** ✅

#### 父节点处理
```rust
if tl.has_children {
    // 只绘制祖先延续线，让 Tree 组件显示展开/折叠图标
    for k in 0..tl.depth {
        // 绘制竖线 │
    }
}
```

#### 叶子节点处理
```rust
else {
    // 完整树线：祖先延续线 + 连接符（├── 或 └──）
    build_tree_prefix(tl, line_color)
}
```

### 5. **数值格式化改进** ✅

#### Float 显示
```rust
FieldValue::Float(f) => {
    if f.fract() == 0.0 {
        format!("{:.0}", f)  // 整数显示：220
    } else {
        format!("{}", f)      // 小数显示：220.5
    }
}
```

#### WithUnit 显示
```rust
FieldValue::WithUnit { value, unit } => {
    format!("{} {}", describe_leaf(value), unit)  // "220 V"
}
```

### 6. **代码组织优化** ✅

#### 模块化设计
- `row.rs`：纯数据转换逻辑，不依赖 UI 框架
- `app.rs`：UI 渲染逻辑
- 数据和视图完全分离，便于测试和复用

#### 注释改进
```rust
//! 这份模型是 UI 树控件渲染和图片导出共用的唯一数据源：两边都只消费
//! `Vec<Row>`，不需要为了展示再重新遍历一次 Value / 再做一次类型转换。
```

### 7. **错误处理优化** ✅

#### 兜底处理
```rust
// Map/List/Node 属于容器，理论上不会走到这个分支（flatten 里会先分流），
// 兜底给空字符串而不是 panic，避免未来新增 FieldValue 变体时这里漏配直接崩溃。
FieldValue::Map(_) | FieldValue::List(_) | FieldValue::Node { .. } => String::new(),
```

## 与参考实现的差异

### 保留的差异
1. **输入方式不同**
   - 参考实现：协议/区域/DI/数据 四个输入框
   - 我们的实现：单一十六进制输入框（自动识别协议）

2. **自动解析**
   - 参考实现：需要手动指定协议
   - 我们的实现：使用 `protocol_parser::auto_parse()` 自动识别

3. **导出功能**
   - 参考实现：支持导出 PNG 图片
   - 我们的实现：暂未实现（可后续添加）

### 采用的改进
1. ✅ 优化的 Row 数据结构
2. ✅ flatten 递归算法
3. ✅ 数据/说明分离展示
4. ✅ 改进的树线渲染
5. ✅ 更好的数值格式化

## 测试验证

```bash
# 编译
cargo build

# 运行
cargo run
```

### 测试用例
```
输入：68 49 00 40 04 11 02 04 02 E8 0A 18 39 36 00 19 00 50 39 36 00 19 00 07 09 37 00 19 00 85 31 39 00 19 00 35 24 45 00 20 00 48 24 45 00 20 00 27 52 46 00 20 00 24 56 46 00 20 00 18 58 46 00 20 00 26 77 46 00 20 00 56 16

预期结果：
- 自动识别为 CSG 本地通信协议
- 树形结构正确展示
- 数据列显示原始字节（带空格）
- 说明列显示解析后的可读值
```

## 下一步优化方向

### 短期（本次未实现）
- [ ] 添加图片导出功能（参考 `export.rs`）
- [ ] 支持自定义列宽保存
- [ ] 添加搜索/过滤功能

### 中期
- [ ] 支持 DLT645 和 CSG1209022 的树形展示
- [ ] 添加主题切换功能
- [ ] 性能优化（虚拟滚动）

### 长期
- [ ] 支持多报文批量解析
- [ ] 报文对比功能
- [ ] 历史记录管理

## 总结

通过参考 `spec-engine-viewer` 的优秀实现，我们成功改进了：

1. **数据结构**：更清晰的 Row 定义
2. **算法**：使用 flatten 递归"拍平"算法
3. **展示**：数据和说明分离，更直观
4. **代码质量**：更好的注释和错误处理

这些改进让代码更易维护、扩展和理解。🎉
