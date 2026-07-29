use wasm_bindgen::prelude::*;
use gpui::*;

#[wasm_bindgen(start)]
pub fn main() {
    // 设置 panic hook，在浏览器控制台显示错误
    console_error_panic_hook::set_once();

    // 初始化 GPUI
    gpui_platform::application().run(move |cx| {
        gpui_component::init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                // 使用主项目的 ProtocolViewerApp
                let view = cx.new(|cx| protocol_viewer::ProtocolViewerApp::new(cx));
                cx.new(|cx| gpui_component::Root::new(view, window, cx))
            })
            .expect("Failed to open window");
        })
        .detach();
    });
}
