//! 主界面：输入框 + 解析按钮 + 可调列宽三栏树形列表

use std::rc::Rc;

use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use gpui_kit::component::{
    h_flex, v_flex,
    input::{Textarea, TextareaState, InputEvent},
    list::ListItem,
    tree::{tree, TreeState, TreeItem},
    ActiveTheme,
};

use crate::row::{self, Row};

/// 列宽百分比：[字段名, 数据, 说明]
const COL_WIDTH_PERCENT: [f32; 3] = [0.30, 0.25, 0.45];  // 30%, 25%, 45%
/// 列宽下限（像素）
const MIN_COL_W: f32 = 80.0;
const RESIZE_HANDLE_W: f32 = 4.0;

// ── 树线常量 ────────────────────────────────────────

/// 每级深度的槽宽
const TREE_SLOT_W: f32 = 24.0;
/// 线宽
const TREE_LINE_W: f32 = 1.0;
/// 行高
const TREE_ROW_H: f32 = 28.0;
/// 垂直连接线重叠像素
const LINE_OVERLAP: f32 = 2.0;

// ── 树线信息 ────────────────────────────────────────

#[derive(Clone, Debug)]
struct TreeLineInfo {
    depth: usize,
    /// 祖先每一级是否还有后续兄弟
    ancestor_continues: Vec<bool>,
    /// 当前行是否为最后一个子节点
    is_last_child: bool,
    /// 当前行是否有子节点
    has_children: bool,
}

fn compute_tree_lines(rows: &[Row]) -> Vec<TreeLineInfo> {
    let n = rows.len();
    let mut out = Vec::with_capacity(n);

    for i in 0..n {
        let depth = rows[i].depth;
        let has_children = rows[i].has_children;

        let mut ancestor_continues = Vec::with_capacity(depth);
        for d in 0..depth {
            let mut has_later = false;
            for j in (i + 1)..n {
                if rows[j].depth < d {
                    break;
                }
                if rows[j].depth == d {
                    has_later = true;
                    break;
                }
            }
            ancestor_continues.push(has_later);
        }

        let is_last_child = {
            let mut found_sibling = false;
            for j in (i + 1)..n {
                if rows[j].depth < depth {
                    break;
                }
                if rows[j].depth == depth {
                    found_sibling = true;
                    break;
                }
            }
            !found_sibling
        };

        out.push(TreeLineInfo {
            depth,
            ancestor_continues,
            is_last_child,
            has_children,
        });
    }
    out
}

// ── 工具函数 ────────────────────────────────────────

fn parse_hex_bytes(input: &str) -> Result<Vec<u8>, String> {
    let cleaned: String = input
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .replace(['-', ':'], "");
    
    if cleaned.is_empty() {
        return Err("数据内容不能为空".into());
    }
    if cleaned.len() % 2 != 0 {
        return Err("十六进制字符串长度必须是偶数".into());
    }
    
    let mut out = Vec::with_capacity(cleaned.len() / 2);
    let bytes = cleaned.as_bytes();
    for chunk in bytes.chunks(2) {
        let s = std::str::from_utf8(chunk).map_err(|_| "非法字符".to_string())?;
        let b = u8::from_str_radix(s, 16).map_err(|_| format!("非法十六进制: {s}"))?;
        out.push(b);
    }
    Ok(out)
}

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

fn debug_log(message: String) {
    #[cfg(not(target_family = "wasm"))]
    println!("[protocol-viewer] {message}");
    #[cfg(target_family = "wasm")]
    log::info!("{message}");
}

fn build_tree_items(rows: &[Row], start: usize, depth: usize) -> (Vec<TreeItem>, usize) {
    let mut items = Vec::new();
    let mut i = start;
    
    while i < rows.len() && rows[i].depth == depth {
        let row_idx = i;
        i += 1;
        
        let (children, next_i) = if i < rows.len() && rows[i].depth == depth + 1 {
            build_tree_items(rows, i, depth + 1)
        } else {
            (Vec::new(), i)
        };
        i = next_i;
        
        let mut item = TreeItem::new(format!("row-{row_idx}"), rows[row_idx].field.clone());
        if !children.is_empty() {
            item = item.children(children).expanded(true);
        }
        items.push(item);
    }
    (items, i)
}

// ── 树线渲染 ────────────────────────────────────────

/// 叶子节点：完整树线
fn build_tree_prefix(tl: &TreeLineInfo, line_color: Hsla) -> Div {
    let depth = tl.depth;
    let half_h = TREE_ROW_H / 2.0;
    let half_w = TREE_SLOT_W / 2.0;
    let lw = TREE_LINE_W;
    let hw = lw / 2.0;

    let mut prefix = h_flex().flex_shrink_1();

    // 祖先延续线
    for k in 0..depth {
        let slot = div()
            .w(px(TREE_SLOT_W))
            .h(px(TREE_ROW_H))
            .flex_shrink_1()
            .relative();

        if tl.ancestor_continues.get(k).copied().unwrap_or(false) {
            prefix = prefix.child(slot.child(
                div()
                    .absolute()
                    .left(px(half_w - hw))
                    .top(px(0.0))
                    .w(px(lw))
                    .h(px(TREE_ROW_H + LINE_OVERLAP))
                    .bg(line_color),
            ));
        } else {
            prefix = prefix.child(slot);
        }
    }

    // 连接器
    let slot = div()
        .w(px(TREE_SLOT_W))
        .h(px(TREE_ROW_H))
        .flex_shrink_1()
        .relative();

    if tl.is_last_child {
        // └──
        prefix = prefix.child(
            slot.child(
                div()
                    .absolute()
                    .left(px(half_w - hw))
                    .top(px(0.0))
                    .w(px(lw))
                    .h(px(half_h + LINE_OVERLAP / 2.0))
                    .bg(line_color),
            )
            .child(
                div()
                    .absolute()
                    .left(px(half_w))
                    .top(px(half_h - hw))
                    .w(px(half_w))
                    .h(px(lw))
                    .bg(line_color),
            ),
        );
    } else {
        // ├──
        prefix = prefix.child(
            slot.child(
                div()
                    .absolute()
                    .left(px(half_w - hw))
                    .top(px(0.0))
                    .w(px(lw))
                    .h(px(TREE_ROW_H + LINE_OVERLAP))
                    .bg(line_color),
            )
            .child(
                div()
                    .absolute()
                    .left(px(half_w))
                    .top(px(half_h - hw))
                    .w(px(half_w))
                    .h(px(lw))
                    .bg(line_color),
            ),
        );
    }

    prefix
}

/// 父节点：祖先延续线 + 连接线（Tree 组件会添加展开/折叠图标）
fn build_parent_prefix(tl: &TreeLineInfo, line_color: Hsla) -> Div {
    let depth = tl.depth;
    let half_h = TREE_ROW_H / 2.0;
    let half_w = TREE_SLOT_W / 2.0;
    let lw = TREE_LINE_W;
    let hw = lw / 2.0;

    let mut prefix = h_flex().flex_shrink_1();

    // 祖先延续线
    for k in 0..depth {
        let slot = div()
            .w(px(TREE_SLOT_W))
            .h(px(TREE_ROW_H))
            .flex_shrink_1()
            .relative();

        if tl.ancestor_continues.get(k).copied().unwrap_or(false) {
            prefix = prefix.child(slot.child(
                div()
                    .absolute()
                    .left(px(half_w - hw))
                    .top(px(0.0))
                    .w(px(lw))
                    .h(px(TREE_ROW_H + LINE_OVERLAP))
                    .bg(line_color),
            ));
        } else {
            prefix = prefix.child(slot);
        }
    }

    // 连接符 + 空间留给 Tree 组件的图标
    let slot = div()
        .w(px(TREE_SLOT_W))
        .h(px(TREE_ROW_H))
        .flex_shrink_1()
        .relative();

    if tl.is_last_child {
        // └── (上半段竖线 + 横线)
        prefix = prefix.child(
            slot.child(
                div()
                    .absolute()
                    .left(px(half_w - hw))
                    .top(px(0.0))
                    .w(px(lw))
                    .h(px(half_h + LINE_OVERLAP / 2.0))
                    .bg(line_color),
            )
            .child(
                div()
                    .absolute()
                    .left(px(half_w))
                    .top(px(half_h - hw))
                    .w(px(half_w))
                    .h(px(lw))
                    .bg(line_color),
            ),
        );
    } else {
        // ├── (全高竖线 + 横线)
        prefix = prefix.child(
            slot.child(
                div()
                    .absolute()
                    .left(px(half_w - hw))
                    .top(px(0.0))
                    .w(px(lw))
                    .h(px(TREE_ROW_H + LINE_OVERLAP))
                    .bg(line_color),
            )
            .child(
                div()
                    .absolute()
                    .left(px(half_w))
                    .top(px(half_h - hw))
                    .w(px(half_w))
                    .h(px(lw))
                    .bg(line_color),
            ),
        );
    }

    prefix
}

// ── 主结构体 ────────────────────────────────────────

pub struct ProtocolViewerApp {
    hex_input: Option<Entity<TextareaState>>,
    rows: Rc<Vec<Row>>,
    tree_lines: Rc<Vec<TreeLineInfo>>,
    tree_state: Entity<TreeState>,
    error: Option<String>,

    /// 前两列占比：[字段名, 数据]；说明列始终占用剩余宽度。
    col_ratios: [f32; 2],
    drag_state: Option<usize>,
    
    /// 默认十六进制字符串
    default_hex: String,
    /// 是否已初始化
    initialized: bool,
    /// 容器宽度（用于计算百分比）
    container_width: f32,
    /// 上次解析的内容（用于检测变化）
    last_parsed_content: String,
}

impl ProtocolViewerApp {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let default_hex = "68 49 00 40 04 11 02 04 02 E8 0A 18 39 36 00 19 00 50 39 36 00 19 00 07 09 37 00 19 00 85 31 39 00 19 00 35 24 45 00 20 00 48 24 45 00 20 00 27 52 46 00 20 00 24 56 46 00 20 00 18 58 46 00 20 00 26 77 46 00 20 00 56 16".to_string();
        
        let tree_state = cx.new(|cx| TreeState::new(cx));

        Self {
            hex_input: None,
            rows: Rc::new(Vec::new()),
            tree_lines: Rc::new(Vec::new()),
            tree_state,
            error: None,
            col_ratios: [COL_WIDTH_PERCENT[0], COL_WIDTH_PERCENT[1]],
            drag_state: None,
            default_hex,
            initialized: false,
            container_width: 1200.0,
            last_parsed_content: String::new(),
        }
    }
    
    fn ensure_initialized(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.initialized {
            let default_value = self.default_hex.clone();
            
            let hex_input = cx.new(|cx| {
                TextareaState::new(window, cx)
                    .placeholder("输入十六进制报文（支持空格和换行）")
                    .default_value(default_value)
                    .auto_grow(3, 10)  // 最小3行，最大10行
            });
            
            // 订阅输入变化事件
            cx.subscribe(&hex_input, |this, _state, event: &InputEvent, cx| {
                if matches!(event, InputEvent::Change) {
                    this.parse(cx);
                }
            }).detach();
            
            self.hex_input = Some(hex_input);
            self.initialized = true;
            
            // 初始解析
            self.parse(cx);
        }
    }

    fn parse(&mut self, cx: &mut Context<Self>) {
        let hex_input = match &self.hex_input {
            Some(input) => input,
            None => return,
        };
        
        let hex_str = hex_input.read(cx).value();

        let result = (|| -> Result<Vec<Row>, String> {
            let bytes = parse_hex_bytes(&hex_str)?;
            
            // 自动解析协议
            let (parsed_msg, _consumed) = protocol_parser::auto_parse(&bytes, None)
                .map_err(|e| format!("解析失败: {}", e))?;

            // 使用 to_value_tree() 方法转换为 FieldValue 树
            let value_tree = parsed_msg.to_value_tree()
                .map_err(|e| format!("转换失败: {}", e))?;

            let mut rows = row::build_rows(&value_tree);
            let byte_ranges = hex_byte_ranges(&hex_str);
            for row in &mut rows {
                let start = row.raw_range.start.min(byte_ranges.len());
                let end = row.raw_range.end.min(byte_ranges.len());
                row.raw_range = if start < end {
                    byte_ranges[start].start..byte_ranges[end - 1].end
                } else {
                    start..start
                };
            }
            debug_log(format!(
                "parse: input_bytes={}, text_bytes={}, rows={}, mapped_ranges={}",
                bytes.len(),
                hex_str.len(),
                rows.len(),
                rows.iter().filter(|row| !row.raw_range.is_empty()).count()
            ));
            for (index, row) in rows.iter().take(30).enumerate() {
                let text = hex_str
                    .get(row.raw_range.clone())
                    .unwrap_or("<invalid-range>");
                debug_log(format!(
                    "row[{index}] field={:?} depth={} range={:?} text={:?}",
                    row.field, row.depth, row.raw_range, text
                ));
            }
            
            Ok(rows)
        })();

        match result {
            Ok(rows) => {
                let tree_lines = compute_tree_lines(&rows);
                let (items, _) = build_tree_items(&rows, 0, 0);
                
                self.rows = Rc::new(rows);
                self.tree_lines = Rc::new(tree_lines);
                self.error = None;
                self.last_parsed_content = hex_str.to_string();
                
                self.tree_state.update(cx, |state, cx| {
                    state.set_items(items, cx);
                });
            }
            Err(e) => {
                self.error = Some(e);
                self.rows = Rc::new(Vec::new());
                self.tree_lines = Rc::new(Vec::new());
                self.last_parsed_content = hex_str.to_string();
                
                // 清空树状态
                self.tree_state.update(cx, |state, cx| {
                    state.set_items(Vec::new(), cx);
                });
            }
        }
        cx.notify();
    }

    fn select_row(&mut self, row_index: usize, window: &mut Window, cx: &mut Context<Self>) {
        let Some(row) = self.rows.get(row_index) else {
            return;
        };
        let range = row.raw_range.clone();
        debug_log(format!(
            "select: row={} field={:?} range={:?}",
            row_index, row.field, range
        ));
        if let Some(input) = self.hex_input.clone() {
            input.update(cx, |state, cx| {
                state.set_selected_range(range, cx);
                state.focus(window, cx);
                debug_log(format!("select: actual_range={:?}", state.selected_range()));
            });
        } else {
            debug_log("select: hex_input is None".to_string());
        }
        cx.notify();
    }

    fn begin_col_resize(&mut self, col: usize, cx: &mut Context<Self>) {
        self.drag_state = Some(col);
        cx.notify();
    }

    fn update_col_resize(&mut self, current_x: f32, cx: &mut Context<Self>) {
        if let Some(col) = self.drag_state {
            let width = if col == 0 {
                current_x
            } else {
                current_x - self.calculate_col_width(0) - RESIZE_HANDLE_W
            };
            self.set_col_width(col, width, cx);
        }
    }

    fn end_col_resize(&mut self, cx: &mut Context<Self>) {
        if self.drag_state.take().is_some() {
            cx.notify();
        }
    }

    fn set_col_width(&mut self, col: usize, width: f32, cx: &mut Context<Self>) {
        if self.container_width <= 0.0 {
            return;
        }

        let current_field = self.calculate_col_width(0);
        let current_data = self.calculate_col_width(1);
        if col == 0 {
            let max_field = current_field + (current_data - MIN_COL_W);
            let field = width.clamp(MIN_COL_W, max_field);
            let data = (current_field + current_data - field).max(MIN_COL_W);
            self.col_ratios[0] = field / self.container_width;
            self.col_ratios[1] = data / self.container_width;
        } else {
            let reserved = MIN_COL_W + RESIZE_HANDLE_W * 2.0 + 8.0;
            let max_data = (self.container_width - current_field - reserved).max(MIN_COL_W);
            let data = width.clamp(MIN_COL_W, max_data);
            self.col_ratios[1] = data / self.container_width;
        }
        cx.notify();
    }

    fn calculate_col_width(&self, col: usize) -> f32 {
        if col < 2 {
            (self.container_width * self.col_ratios[col]).max(MIN_COL_W)
        } else {
            let remaining = self.container_width
                - self.calculate_col_width(0)
                - self.calculate_col_width(1)
                - RESIZE_HANDLE_W * 2.0;
            remaining.max(MIN_COL_W)
        }
    }
}

// ── 渲染 ────────────────────────────────────────────

impl Render for ProtocolViewerApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // 确保已初始化
        self.ensure_initialized(window, cx);
        
        // 更新容器宽度（响应窗口大小变化）
        let bounds = window.bounds();
        self.container_width = bounds.size.width.into();
        
        // 使用内置主题
        let theme = cx.theme();
        let colors = &theme.colors;
        let hex_input = self.hex_input.as_ref().expect("hex_input should be initialized");
        let rows = self.rows.clone();
        let tree_lines = self.tree_lines.clone();
        let this = cx.entity();
        
        // 计算实际列宽
        let cw = [
            self.calculate_col_width(0),
            self.calculate_col_width(1),
            self.calculate_col_width(2),
        ];

        let line_color = colors.border;
        let muted_foreground_color = colors.muted_foreground;
        let primary_color = colors.primary;

        v_flex()
            .w_full()
            .h_full()
            .gap_0()
            .bg(colors.background)
            // 顶部输入区域（多行输入，自动增长）
            .child(
                div()
                    .w_full()
                    .px_4()
                    .py_2()
                    .flex()
                    .items_start()
                    .bg(colors.title_bar)
                    .border_b_1()
                    .border_color(colors.border)
                    .child(Textarea::new(hex_input).w_full()),
            )
            // 错误信息（如果有）- 紧贴输入框
            .when_some(self.error.clone(), |this, err| {
                this.child(
                    div()
                        .w_full()
                        .px_4()
                        .py_2()
                        .bg(colors.status_bar)
                        .text_color(colors.foreground)
                        .border_b_1()
                        .border_color(colors.border)
                        .child(err),
                )
            })
            // 表头
            .child(
                h_flex()
                    .w_full()
                    .items_stretch()
                    .gap_0()
                    .h(px(36.))
                    .bg(colors.title_bar)
                    .border_b_1()
                    .border_color(colors.border)
                    // 字段名
                    .child(
                        div()
                            .w(px(cw[0]))
                            .flex_shrink(0.)
                            .px_3()
                            .flex()
                            .items_center()
                            .overflow_hidden()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(colors.foreground)
                            .child(div().truncate().child("帧域")),
                    )
                    // 分隔条 0
                    .child(
                        div()
                            .id(("resize-handle", 0usize))
                            .w(px(RESIZE_HANDLE_W))
                            .h_full()
                            .flex_shrink(0.)
                            .bg(colors.border)
                            .cursor(CursorStyle::ResizeLeftRight)
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _: &MouseDownEvent, _, cx| {
                                    this.begin_col_resize(0, cx);
                                }),
                            ),
                    )
                    // 数据
                    .child(
                        div()
                            .w(px(cw[1]))
                            .flex_shrink(0.)
                            .px_3()
                            .flex()
                            .items_center()
                            .overflow_hidden()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(colors.foreground)
                            .child(div().truncate().child("数据")),
                    )
                    // 分隔条 1
                    .child(
                        div()
                            .id(("resize-handle", 1usize))
                            .w(px(RESIZE_HANDLE_W))
                            .h_full()
                            .flex_shrink(0.)
                            .bg(colors.border)
                            .cursor(CursorStyle::ResizeLeftRight)
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _: &MouseDownEvent, _, cx| {
                                    this.begin_col_resize(1, cx);
                                }),
                            ),
                    )
                    // 说明
                    .child(
                        div()
                            .w(px(cw[2]))
                            .flex_shrink(0.)
                            .px_3()
                            .flex()
                            .items_center()
                            .overflow_hidden()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(colors.foreground)
                            .child(div().truncate().child("说明")),
                    ),
            )
            // 树形列表
            .child(
                div()
                    .flex_1()
                    .overflow_hidden()
                    .bg(colors.background)
                    .child(tree(&self.tree_state, move |ix, entry, selected, _window, _cx| {
                        let idx: usize = entry
                            .item()
                            .id
                            .trim_start_matches("row-")
                            .parse()
                            .unwrap_or(usize::MAX);
                        
                        let (data, desc) = rows
                            .get(idx)
                            .map(|r| (r.data.clone(), r.desc.clone()))
                            .unwrap_or_default();

                        // ── 构建前缀 ──
                        let prefix = if let Some(tl) = tree_lines.get(idx) {
                            if tl.has_children {
                                // 父节点：祖先延续线 + 连接符（Tree 组件会添加展开/折叠图标）
                                build_parent_prefix(tl, line_color)
                            } else {
                                // 叶子节点：完整树线
                                build_tree_prefix(tl, line_color)
                            }
                        } else {
                            h_flex().flex_shrink_1()
                        };

                        ListItem::new(ix)
                            .h(px(TREE_ROW_H))
                            .selected(selected)
                            .child(
                                h_flex()
                                    .h(px(TREE_ROW_H))
                                    .items_center()
                                    .overflow_x_hidden()
                                    .on_mouse_down(MouseButton::Left, {
                                        let this = this.clone();
                                        move |_, window, app| {
                                            debug_log(format!("click: row={idx}"));
                                            let this = this.clone();
                                            window.defer(app, move |window, app| {
                                                this.update(app, |this, cx| {
                                                    this.select_row(idx, window, cx);
                                                });
                                            });
                                        }
                                    })
                                    // 第一列：前缀 + 字段名
                                    .child(
                                        div()
                                            .w(px(cw[0]))
                                            .flex_shrink(0.)
                                            .overflow_x_hidden()
                                            .child(
                                                h_flex()
                                                    .items_center()
                                                    .child(prefix)
                                                    .child(
                                                        div()
                                                            .truncate()
                                                                .when(
                                                                        tree_lines
                                                                                .get(idx)
                                                                                .is_some_and(|line| line.has_children),
                                                                    |this| this.font_weight(FontWeight::SEMIBOLD),
                                                                )
                                                            .child(entry.item().label.clone()),
                                                    ),
                                            ),
                                    )
                                    // 第二列：数据
                                    .child(
                                        div()
                                            .w(px(cw[1]))
                                            .flex_shrink(0.)
                                            .px_2()
                                            .overflow_x_hidden()
                                            .child(
                                                div()
                                                    .truncate()
                                                            .text_color(muted_foreground_color)
                                                    .child(data),
                                            ),
                                    )
                                    // 第三列：说明
                                    .child(
                                        div()
                                            .flex_1()
                                            .min_w(px(0.))
                                            .px_2()
                                            .overflow_x_hidden()
                                            .child(
                                                div()
                                                    .truncate()
                                                    .text_color(primary_color)
                                                    .child(desc),
                                            ),
                                    ),
                            )
                    })),
            )
            .on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, _, cx| {
                if this.drag_state.is_some() {
                    let x: f32 = event.position.x.into();
                    this.update_col_resize(x, cx);
                }
            }))
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| {
                    this.end_col_resize(cx);
                }),
            )
    }
}
