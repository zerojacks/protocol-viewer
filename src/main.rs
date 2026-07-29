use gpui::*;
use gpui_platform::application;
use protocol_viewer::ProtocolViewerApp;

fn main() {
    application().run(|cx: &mut App| {
        // This must be called before using any GPUI Component features
        gpui_component::init(cx);
        
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
                // Wrap in Root as required by gpui-component
                cx.new(|cx| gpui_component::Root::new(view, window, cx))
            },
        )
        .unwrap();
        cx.activate(true);
    });
}
