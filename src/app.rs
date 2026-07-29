//! 主界面：输入框 + 解析按钮 + 可调列宽三栏树形列表

use std::rc::Rc;

use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::{
    ActiveTheme, button::Button, h_flex, input::{Input, InputState}, label::Label, 
    list::ListItem, tree::{TreeItem, TreeState, tree}, v_flex,
};

use crate::row::{self, Row};

/// 列宽下限（像素）
const MIN_COL_W: f32 = 80.0;

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
        let has_children = i + 1 < n && rows[i + 1].depth > depth;

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

    let mut prefix = h_flex().flex_shrink_0();

    // 祖先延续线
    for k in 0..depth {
        let slot = div()
            .w(px(TREE_SLOT_W))
            .h(px(TREE_ROW_H))
            .flex_shrink_0()
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
        .flex_shrink_0()
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

    let mut prefix = h_flex().flex_shrink_0();

    // 祖先延续线
    for k in 0..depth {
        let slot = div()
            .w(px(TREE_SLOT_W))
            .h(px(TREE_ROW_H))
            .flex_shrink_0()
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
        .flex_shrink_0()
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
    hex_input: Option<Entity<InputState>>,
    rows: Rc<Vec<Row>>,
    tree_lines: Rc<Vec<TreeLineInfo>>,
    tree_state: Entity<TreeState>,
    error: Option<String>,

    /// 三列宽度：[字段名, 数据, 说明]
    col_widths: [f32; 3],
    drag_state: Option<(usize, f32)>,
    
    /// 默认十六进制字符串
    default_hex: String,
    /// 是否已初始化
    initialized: bool,
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
            col_widths: [320.0, 280.0, 450.0],
            drag_state: None,
            default_hex,
            initialized: false,
        }
    }
    
    fn ensure_initialized(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.initialized {
            // 创建 InputState
            let default_value = self.default_hex.clone();
            self.hex_input = Some(cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("输入十六进制报文，可以带空格")
                    .default_value(default_value)
            }));
            
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
        
        let hex_str = hex_input.read(cx).value().to_string();

        let result = (|| -> Result<Vec<Row>, String> {
            let bytes = parse_hex_bytes(&hex_str)?;
            
            // 自动解析协议
            let (parsed_msg, _consumed) = protocol_parser::auto_parse(&bytes, None)
                .map_err(|e| format!("解析失败: {}", e))?;

            // 使用 to_value_tree() 方法转换为 FieldValue 树
            let value_tree = parsed_msg.to_value_tree()
                .map_err(|e| format!("转换失败: {}", e))?;

            let rows = row::build_rows(&value_tree);
            
            Ok(rows)
        })();

        match result {
            Ok(rows) => {
                let tree_lines = compute_tree_lines(&rows);
                let (items, _) = build_tree_items(&rows, 0, 0);
                
                self.rows = Rc::new(rows);
                self.tree_lines = Rc::new(tree_lines);
                self.error = None;
                self.tree_state.update(cx, |state, cx| {
                    state.set_items(items, cx);
                });
            }
            Err(e) => {
                self.error = Some(e);
                self.rows = Rc::new(Vec::new());
                self.tree_lines = Rc::new(Vec::new());
            }
        }
        cx.notify();
    }

    fn begin_col_resize(&mut self, col: usize, start_x: f32, cx: &mut Context<Self>) {
        self.drag_state = Some((col, start_x));
        cx.notify();
    }

    fn update_col_resize(&mut self, current_x: f32, cx: &mut Context<Self>) {
        if let Some((col, ref mut last_x)) = self.drag_state {
            let delta = current_x - *last_x;
            self.col_widths[col] = MIN_COL_W.max(self.col_widths[col] + delta);
            *last_x = current_x;
            cx.notify();
        }
    }

    fn end_col_resize(&mut self, cx: &mut Context<Self>) {
        if self.drag_state.take().is_some() {
            cx.notify();
        }
    }
}

// ── 渲染 ────────────────────────────────────────────

impl Render for ProtocolViewerApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // 确保已初始化
        self.ensure_initialized(window, cx);
        
        let hex_input = self.hex_input.as_ref().expect("hex_input should be initialized");
        
        let rows = self.rows.clone();
        let tree_lines = self.tree_lines.clone();
        let cw = self.col_widths;

        let line_color = cx.theme().border;
        let separator_color = cx.theme().border;

        v_flex()
            .size_full()
            .gap_2()
            .p_4()
            .bg(cx.theme().background)
            // 输入框 + 按钮
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .child(Label::new("报文:"))
                    .child(Input::new(hex_input).flex_1())
                    .child(
                        Button::new("parse")
                            .label("解析")
                            .on_click(cx.listener(|this, _, _, cx| this.parse(cx))),
                    ),
            )
            // 错误信息
            .when_some(self.error.clone(), |this, err| {
                this.child(
                    div()
                        .p_2()
                        .bg(cx.theme().status_bar)
                        .text_color(cx.theme().foreground)
                        .rounded_md()
                        .child(err),
                )
            })
            // 表头
            .child(
                h_flex()
                    .items_stretch()
                    .gap_0()
                    .h(px(32.))
                    .bg(cx.theme().background)
                    .border_1()
                    .border_color(cx.theme().border)
                    // 字段名
                    .child(
                        div()
                            .w(px(cw[0]))
                            .flex_shrink_0()
                            .px_3()
                            .flex()
                            .items_center()
                            .overflow_hidden()
                            .font_weight(FontWeight::SEMIBOLD)
                            .child(div().truncate().child("字段名")),
                    )
                    // 分隔条 0
                    .child(
                        div()
                            .id(("resize-handle", 0usize))
                            .w(px(4.))
                            .h_full()
                            .flex_shrink_0()
                            .bg(separator_color)
                            .cursor(CursorStyle::ResizeLeftRight)
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, event: &MouseDownEvent, _, cx| {
                                    let start_x: f32 = event.position.x.into();
                                    this.begin_col_resize(0, start_x, cx);
                                }),
                            ),
                    )
                    // 数据
                    .child(
                        div()
                            .w(px(cw[1]))
                            .flex_shrink_0()
                            .px_3()
                            .flex()
                            .items_center()
                            .overflow_hidden()
                            .font_weight(FontWeight::SEMIBOLD)
                            .child(div().truncate().child("数据")),
                    )
                    // 分隔条 1
                    .child(
                        div()
                            .id(("resize-handle", 1usize))
                            .w(px(4.))
                            .h_full()
                            .flex_shrink_0()
                            .bg(separator_color)
                            .cursor(CursorStyle::ResizeLeftRight)
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, event: &MouseDownEvent, _, cx| {
                                    let start_x: f32 = event.position.x.into();
                                    this.begin_col_resize(1, start_x, cx);
                                }),
                            ),
                    )
                    // 说明
                    .child(
                        div()
                            .w(px(cw[2]))
                            .flex_shrink_0()
                            .px_3()
                            .flex()
                            .items_center()
                            .overflow_hidden()
                            .font_weight(FontWeight::SEMIBOLD)
                            .child(div().truncate().child("说明")),
                    ),
            )
            // 树形列表
            .child(
                div()
                    .flex_1()
                    .overflow_hidden()
                    .border_1()
                    .border_color(cx.theme().border)
                    .bg(cx.theme().background)
                    .child(tree(
                        &self.tree_state,
                        move |ix, entry, selected, _window, _cx| {
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
                                h_flex().flex_shrink_0()
                            };

                            ListItem::new(ix).selected(selected).child(
                                h_flex()
                                    .overflow_hidden()
                                    // 第一列：前缀 + 字段名
                                    .child(
                                        div()
                                            .w(px(cw[0]))
                                            .flex_shrink_0()
                                            .overflow_hidden()
                                            .child(
                                                h_flex()
                                                    .items_center()
                                                    .child(prefix)
                                                    .child(
                                                        div()
                                                            .truncate()
                                                            .child(entry.item().label.clone()),
                                                    ),
                                            ),
                                    )
                                    // 第二列：数据
                                    .child(
                                        div()
                                            .w(px(cw[1]))
                                            .flex_shrink_0()
                                            .px_2()
                                            .overflow_hidden()
                                            .child(div().truncate().child(data)),
                                    )
                                    // 第三列：说明
                                    .child(
                                        div()
                                            .flex_1()
                                            .min_w(px(0.))
                                            .px_2()
                                            .overflow_hidden()
                                            .child(div().truncate().child(desc)),
                                    ),
                            )
                        },
                    )),
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
