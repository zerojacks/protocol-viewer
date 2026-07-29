# Protocol Viewer WASM 浏览器支持

本文档说明如何将 Protocol Viewer 配置为可在浏览器中访问的 WebAssembly 应用。

## 前提条件

### 1. 安装 Rust WASM 工具链

```bash
# 添加 WASM 编译目标
rustup target add wasm32-unknown-unknown

# 安装 wasm-bindgen-cli（版本需要与 Cargo.toml 中的 wasm-bindgen 依赖版本匹配）
cargo install wasm-bindgen-cli
```

### 2. 安装 Bun 或 Node.js

推荐使用 Bun（更快）：

```bash
# macOS/Linux
curl -fsSL https://bun.sh/install | bash

# Windows (PowerShell)
powershell -c "irm bun.sh/install.ps1 | iex"
```

或使用 Node.js：
```bash
# 访问 https://nodejs.org/ 下载安装
```

## 项目结构

需要创建以下结构：

```
protocol-viewer/
├── Cargo.toml                    # 主项目配置
├── src/
│   ├── main.rs                   # 桌面版入口
│   └── app.rs                    # 共享的应用逻辑
├── web/                          # WASM Web 版本
│   ├── Cargo.toml                # Web 版本配置
│   ├── src/
│   │   └── lib.rs                # WASM 入口
│   ├── www/                      # 前端资源
│   │   ├── index.html
│   │   ├── package.json
│   │   ├── vite.config.ts
│   │   └── src/
│   │       └── main.ts
│   └── Makefile                  # 构建脚本
└── README.md
```

## 实施步骤

### 步骤 1: 创建 Web 子项目

在 protocol-viewer 目录下创建 `web/` 目录：

```bash
cd d:\ProjackSpace\projectspace\protocol-viewer
mkdir web
cd web
```

### 步骤 2: 创建 web/Cargo.toml

```toml
[package]
name = "protocol-viewer-web"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
# WASM 绑定
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
web-sys = { version = "0.3", features = [
    "Window",
    "Document",
    "HtmlCanvasElement",
    "WebGl2RenderingContext",
] }
console_error_panic_hook = "0.1"

# GPUI 和 gpui-component
gpui = { git = "https://github.com/zed-industries/zed" }
gpui_platform = { git = "https://github.com/zed-industries/zed", features = ["font-kit"] }
gpui-component = { git = "https://github.com/longbridge/gpui-component" }

# 协议解析器
protocol-parser = { path = "../../protocol-parser/parser" }

[profile.release]
opt-level = "z"     # 优化大小
lto = true          # 链接时优化
codegen-units = 1   # 更好的优化
strip = true        # 移除符号
```

### 步骤 3: 创建 web/src/lib.rs

```rust
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
                // 这里引用主项目的 ProtocolViewerApp
                let view = cx.new(|cx| {
                    protocol_viewer::ProtocolViewerApp::new(cx)
                });
                cx.new(|cx| gpui_component::Root::new(view, window, cx))
            })
            .expect("Failed to open window");
        })
        .detach();
    });
}
```

### 步骤 4: 创建前端项目

在 `web/www/` 目录下创建：

**package.json**:
```json
{
  "name": "protocol-viewer-web",
  "private": true,
  "version": "0.1.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "vite build",
    "preview": "vite preview"
  },
  "devDependencies": {
    "vite": "^5.0.0",
    "@vitejs/plugin-basic-ssl": "^1.0.1"
  }
}
```

**index.html**:
```html
<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Protocol Viewer - Web</title>
    <style>
        body {
            margin: 0;
            padding: 0;
            width: 100vw;
            height: 100vh;
            overflow: hidden;
        }
        #app {
            width: 100%;
            height: 100%;
        }
    </style>
</head>
<body>
    <div id="app"></div>
    <script type="module" src="/src/main.ts"></script>
</body>
</html>
```

**src/main.ts**:
```typescript
import init from '../pkg/protocol_viewer_web.js';

async function run() {
    // 初始化 WASM 模块
    await init();
}

run().catch(console.error);
```

**vite.config.ts**:
```typescript
import { defineConfig } from 'vite';
import basicSsl from '@vitejs/plugin-basic-ssl';

export default defineConfig({
    plugins: [basicSsl()],
    server: {
        port: 3000,
        https: true, // GPUI WebGL 需要 HTTPS
    },
    build: {
        target: 'esnext',
    },
});
```

### 步骤 5: 创建 Makefile

在 `web/` 目录下创建 `Makefile`：

```makefile
.PHONY: install dev build-debug build-release clean

# 安装依赖
install:
	@echo "Installing dependencies..."
	rustup target add wasm32-unknown-unknown
	cargo install wasm-bindgen-cli
	cd www && bun install

# 开发服务器
dev: build-debug
	@echo "Starting development server..."
	cd www && bun run dev

# Debug 构建
build-debug:
	@echo "Building WASM (debug)..."
	cargo build --target wasm32-unknown-unknown
	wasm-bindgen \
		--target web \
		--out-dir www/pkg \
		--debug \
		../target/wasm32-unknown-unknown/debug/protocol_viewer_web.wasm

# Release 构建
build-release:
	@echo "Building WASM (release)..."
	cargo build --release --target wasm32-unknown-unknown
	wasm-bindgen \
		--target web \
		--out-dir www/pkg \
		--no-typescript \
		../target/wasm32-unknown-unknown/release/protocol_viewer_web.wasm
	@echo "Building frontend..."
	cd www && bun run build

# 清理
clean:
	cargo clean
	rm -rf www/pkg
	rm -rf www/dist
	rm -rf www/node_modules
```

### 步骤 6: 修改主项目

将 `src/main.rs` 中的应用逻辑抽离到独立模块，使其可以被 WASM 版本复用。

**src/lib.rs** (新建):
```rust
pub mod app;
pub mod row;

pub use app::ProtocolViewerApp;
```

**src/main.rs**:
```rust
use gpui::*;

fn main() {
    gpui_platform::application().run(move |cx| {
        gpui_component::init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(|cx| protocol_viewer::ProtocolViewerApp::new(cx));
                cx.new(|cx| gpui_component::Root::new(view, window, cx))
            })
            .expect("Failed to open window");
        })
        .detach();
    });
}
```

**Cargo.toml** (添加):
```toml
[lib]
name = "protocol_viewer"
path = "src/lib.rs"

[[bin]]
name = "protocol-viewer"
path = "src/main.rs"
```

## 运行

### 开发模式

```bash
cd web
make install  # 首次运行，安装依赖
make dev      # 启动开发服务器
```

访问 https://localhost:3000

### 生产构建

```bash
cd web
make build-release
```

构建产物在 `www/dist/` 目录。

## 部署

### 静态托管

可以将 `www/dist/` 目录部署到任何静态托管服务：

- **GitHub Pages**: 
  ```bash
  # 部署到 gh-pages 分支
  cd www/dist
  git init
  git add -A
  git commit -m 'deploy'
  git push -f git@github.com:username/protocol-viewer.git main:gh-pages
  ```

- **Netlify/Vercel**: 直接拖拽 `www/dist/` 目录上传

- **Nginx**: 
  ```nginx
  server {
      listen 80;
      server_name protocol-viewer.example.com;
      root /var/www/protocol-viewer/dist;
      index index.html;
      
      location / {
          try_files $uri $uri/ /index.html;
      }
  }
  ```

## 注意事项

### 1. HTTPS 要求

GPUI 的 WebGL 渲染需要 HTTPS 环境。开发时 Vite 会自动创建自签名证书。

### 2. 浏览器兼容性

需要支持 WebAssembly 和 WebGL2 的现代浏览器：
- Chrome 57+
- Firefox 52+
- Safari 11+
- Edge 16+

### 3. 文件大小优化

WASM 文件可能较大（10-20MB）。优化方法：

- 使用 `wasm-opt` 进一步压缩：
  ```bash
  wasm-opt -Oz -o optimized.wasm input.wasm
  ```

- 启用 Gzip/Brotli 压缩（服务器配置）

- 使用 CDN 加速

### 4. 依赖限制

某些桌面专用功能在 WASM 中可能不可用：
- 文件系统访问（需要使用 File API）
- 系统剪贴板（需要使用 Clipboard API）
- 进程管理

## 故障排除

### WASM 编译失败

```bash
# 检查工具链版本
rustup show
rustc --version
wasm-bindgen --version

# 重新安装
rustup update
cargo install --force wasm-bindgen-cli
```

### WebGL 上下文错误

检查浏览器是否启用了 WebGL：访问 https://get.webgl.org/

### 内存不足

增加 WASM 内存限制（在 `lib.rs` 中）：

```rust
#[wasm_bindgen(start)]
pub fn main() {
    // 设置更大的内存
    wasm_bindgen::prelude::__wbindgen_memory()
        .grow(1000) // 增加 64MB * 1000
        .expect("Failed to grow memory");
    
    // ... 其他代码
}
```

## 参考资源

- [GPUI Component Web Gallery](https://longbridge.github.io/gpui-component/gallery/)
- [Rust and WebAssembly Book](https://rustwasm.github.io/book/)
- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)
- [Vite Documentation](https://vitejs.dev/)
