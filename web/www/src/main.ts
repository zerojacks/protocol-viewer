import init from '../pkg/protocol_viewer_web.js';

async function run() {
    try {
        // 初始化 WASM 模块
        await init();
        
        // 隐藏加载指示器
        const loading = document.getElementById('loading');
        if (loading) {
            loading.style.display = 'none';
        }
        
        console.log('Protocol Viewer initialized successfully');
    } catch (error) {
        console.error('Failed to initialize Protocol Viewer:', error);
        
        const loading = document.getElementById('loading');
        if (loading) {
            loading.innerHTML = `
                <div style="color: #e74c3c; text-align: center;">
                    <h2>加载失败</h2>
                    <p style="margin-top: 10px;">${error}</p>
                    <p style="margin-top: 10px; font-size: 12px;">请刷新页面重试</p>
                </div>
            `;
        }
    }
}

run();
