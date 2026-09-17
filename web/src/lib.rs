use wasm_bindgen::prelude::*;
use gpui::{prelude::*, *};
use std::borrow::Cow;

// 嵌入中文字体 - Noto Sans SC
const CHINESE_FONT_DATA: &[u8] = include_bytes!("../fonts/NotoSansSC-Regular.ttf");

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    // 设置 panic hook，在浏览器控制台显示错误
    console_error_panic_hook::set_once();

    // 初始化日志到浏览器控制台
    console_log::init_with_level(log::Level::Info).ok();
    
    // 初始化 tracing for WASM
    #[cfg(target_family = "wasm")]
    tracing_wasm::set_as_global_default();

    log::info!("开始初始化 GPUI WASM...");

    // 初始化 WASM 平台
    #[cfg(target_family = "wasm")]
    gpui_platform::web_init();
    
    log::info!("GPUI 平台初始化完成");
    
    // 获取单线程 Web Application
    #[cfg(target_family = "wasm")]
    let app = {
        let app = gpui_platform::single_threaded_web();
        
        // 临时修复：故意泄漏 Rc<AppCell> 以保持应用程序存活
        struct WasmApplication(std::rc::Rc<AppCell>);
        let wasm_app = unsafe { std::mem::transmute::<Application, WasmApplication>(app) };
        std::mem::forget(wasm_app.0.clone());
        unsafe { std::mem::transmute::<WasmApplication, Application>(wasm_app) }
    };
    
    #[cfg(not(target_family = "wasm"))]
    let app = gpui_platform::application();

    log::info!("Application 创建完成");

    app.run(|cx: &mut App| {
        log::info!("进入 application run 回调");
        
        gpui_component::init(cx);
        
        log::info!("gpui_component 初始化完成");
        
        // 加载中文字体
        log::info!("加载中文字体...");
        let chinese_font = Cow::Borrowed(CHINESE_FONT_DATA);
        cx.text_system()
            .add_fonts(vec![chinese_font])
            .expect("Failed to load fonts");
        log::info!("✓ 中文字体加载成功");
        
        // 【关键】设置默认字体家族
        cx.global_mut::<gpui_component::theme::Theme>().font_family = "Noto Sans SC".into();
        log::info!("✓ 设置默认字体家族为 Noto Sans SC");
        
        log::info!("准备打开窗口");

        let window = cx.open_window(WindowOptions::default(), |window, cx| {
            log::info!("窗口创建回调被调用");
            
            // 使用 ProtocolViewerApp
            let view = cx.new(|cx| {
                log::info!("创建 ProtocolViewerApp");
                protocol_viewer::ProtocolViewerApp::new(cx)
            });
            
            log::info!("ProtocolViewerApp 创建完成");
            
            // 直接使用 Root 包装
            cx.new(|cx| {
                log::info!("创建 Root");
                gpui_component::Root::new(view, window, cx)
            })
        })
        .expect("Failed to open window");
        
        log::info!("窗口打开成功");
        
        // 尝试主动触发窗口重绘
        let _ = window.update(cx, |_, _, cx| {
            log::info!("触发窗口重绘");
            cx.notify();
        });
        
        cx.activate(true);
        
        log::info!("Application 激活完成");
    });
    
    log::info!("run() 函数执行完成");

    Ok(())
}
