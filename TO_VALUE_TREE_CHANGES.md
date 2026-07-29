# to_value_tree 支持多协议修改总结

## 修改目标

让 protocol-viewer 支持三个协议的 `to_value_tree` 功能，而不仅仅是 CsgLocalComm。

## 修改内容

### 1. DLT645-2007 协议 (`dlt645-2007`)

#### 修改文件：`dlt645-2007/src/report.rs`

**修改前**：
- `render_message_as_value(msg: &Message, raw_bytes: &[u8])` 需要外部传入原始字节

**修改后**：
- `render_message_as_value(msg: &Message)` 不再需要外部原始字节
- 内部调用 `msg.frame.encode(false)` 从 Message 对象重建原始字节
- 根节点名称从 `"报文"` 改为 `"DL/T 645-2007 报文"`

#### 修改文件：`dlt645-2007/src/engine.rs`

**修改前**：
```rust
pub fn to_value_tree(&self, raw_bytes: &[u8]) -> Result<FieldValue> {
    crate::report::render_message_as_value(self, raw_bytes)
}
```

**修改后**：
```rust
pub fn to_value_tree(&self) -> Result<FieldValue> {
    crate::report::render_message_as_value(self)
}
```

---

### 2. CSG1209022 协议 (`csg1209022`)

#### 修改文件：`csg1209022/src/report.rs`

**修改前**：
- `render_message_as_value(msg: &Message, raw_bytes: &[u8])` 需要外部传入原始字节

**修改后**：
- `render_message_as_value(msg: &Message)` 不再需要外部原始字节
- 内部调用 `msg.frame.encode()` 从 Message 对象重建原始字节
- 根节点名称从 `"报文"` 改为 `"Q/CSG1209022-2019 报文"`

#### 修改文件：`csg1209022/src/engine.rs`

**修改前**：
```rust
pub fn to_value_tree(&self, raw_bytes: &[u8]) -> Result<FieldValue> {
    crate::report::render_message_as_value(self, raw_bytes)
}
```

**修改后**：
```rust
pub fn to_value_tree(&self) -> Result<FieldValue> {
    crate::report::render_message_as_value(self)
}
```

---

### 3. CsgLocalComm 协议 (`csg-local-comm`)

**无修改**：
- 该协议已经实现了 `render_message_as_value(msg: &Message)` 不需要原始字节的版本
- 根节点名称为 `"Q/CSG1209021-2019 报文"`

---

### 4. 统一接口 (`protocol-parser/parser`)

#### 修改文件：`parser/src/lib.rs`

**修改前**：
```rust
pub fn to_value_tree(&self) -> Result<FieldValue, ParseError> {
    match self {
        ParsedMessage::Dlt645(_msg) => {
            Err(ParseError::NotImplemented("DLT645 to_value_tree 需要原始字节".to_string()))
        }
        ParsedMessage::Csg1209022(_msg) => {
            Err(ParseError::NotImplemented("CSG1209022 to_value_tree 需要原始字节".to_string()))
        }
        ParsedMessage::CsgLocalComm(msg) => {
            csg_local_comm::report::render_message_as_value(msg)
                .map_err(|e| ParseError::RenderError(format!("{:?}", e)))
        }
    }
}
```

**修改后**：
```rust
pub fn to_value_tree(&self) -> Result<FieldValue, ParseError> {
    match self {
        ParsedMessage::Dlt645(msg) => {
            msg.to_value_tree()
                .map_err(|e| ParseError::RenderError(format!("{:?}", e)))
        }
        ParsedMessage::Csg1209022(msg) => {
            msg.to_value_tree()
                .map_err(|e| ParseError::RenderError(format!("{:?}", e)))
        }
        ParsedMessage::CsgLocalComm(msg) => {
            csg_local_comm::report::render_message_as_value(msg)
                .map_err(|e| ParseError::RenderError(format!("{:?}", e)))
        }
    }
}
```

---

## 设计优点

1. **统一接口**：所有三个协议的 `to_value_tree()` 现在都不需要外部传入原始字节
2. **自包含**：每个 Message 对象可以独立重建自己的原始字节表示
3. **简化调用**：protocol-viewer 的调用代码更简洁：
   ```rust
   let value_tree = parsed_msg.to_value_tree()?; // 不需要传 raw_bytes
   ```
4. **清晰的根节点名称**：
   - `"DL/T 645-2007 报文"`
   - `"Q/CSG1209022-2019 报文"`
   - `"Q/CSG1209021-2019 报文"`

---

## 测试结果

✅ 所有协议的 protocol-parser 编译成功  
✅ protocol-viewer 编译成功  
✅ 三个协议现在都支持 `to_value_tree()` 功能

---

## 后续可能的改进

1. 如果原始字节的重建（encode）有性能开销，可以考虑在 `Message` 结构体中缓存原始字节
2. 可以为 `Message` 添加 `Debug` trait 输出，方便调试
3. 可以统一三个协议的错误类型，避免格式化为字符串
