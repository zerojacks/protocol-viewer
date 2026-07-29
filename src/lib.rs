//! Protocol Viewer 共享库
//! 
//! 此模块导出应用逻辑，可被桌面版和 WASM 版本共同使用

pub mod app;
pub mod row;

pub use app::ProtocolViewerApp;
