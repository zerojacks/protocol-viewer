use gpui_kit::*;
use gpui_kit::component::Root;
use protocol_viewer::ProtocolViewerApp;

fn main() {
    gpui_kit::application()
        .with_assets(gpui_kit::assets::Assets)
        .run(|cx| {
            // 必须在使用任何 GPUI Kit 功能之前调用
            gpui_kit::init(cx);
            
            let bounds = Bounds::centered(None, size(px(1400.0), px(900.0)), cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: Some(TitlebarOptions {
                        title: Some("协议解析查看器".into()),
                        appears_transparent: false,
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                |window, cx| {
                    let view = cx.new(|cx| ProtocolViewerApp::new(cx));
                    // 窗口的第一级应该是 Root
                    cx.new(|cx| Root::new(view, window, cx))
                },
            )
            .unwrap();
            
            cx.activate(true);
        });
}
