# 🏠 本地部署 Protocol Viewer Web 版本

无需 GitHub，直接在本地构建和运行 WASM 版本。

## 📋 前置条件

### 1. 安装 Rust WASM 工具链

```bash
# 添加 WASM 编译目标（如果网速慢，可能需要等待10-20分钟）
rustup target add wasm32-unknown-unknown

# 安装 wasm-bindgen-cli
cargo install wasm-bindgen-cli
```

**注意**：如果下载很慢，可以配置国内镜像源。

### 2. 安装 Node.js 包管理器

**选项 A: Bun（推荐，更快）**

```bash
# Windows PowerShell
powershell -c "irm bun.sh/install.ps1 | iex"

# macOS/Linux
curl -fsSL https://bun.sh/install | bash
```

**选项 B: Node.js**

访问 https://nodejs.org/ 下载安装 LTS 版本。

## 🚀 本地构建和运行

### 方法 1: 使用 Makefile（推荐）

```bash
# 进入 web 目录
cd web

# 安装前端依赖（首次运行）
make install

# 启动开发服务器
make dev
```

访问 **https://localhost:3000**

### 方法 2: 手动步骤

#### 步骤 1: 构建 WASM

```bash
cd web

# 编译 Rust 到 WASM（debug 模式，快速）
cargo build --target wasm32-unknown-unknown

# 生成 JavaScript 绑定
wasm-bindgen \
  --target web \
  --out-dir www/pkg \
  --debug \
  ../target/wasm32-unknown-unknown/debug/protocol_viewer_web.wasm
```

#### 步骤 2: 安装前端依赖

```bash
cd www

# 使用 Bun
bun install

# 或使用 npm
npm install
```

#### 步骤 3: 启动开发服务器

```bash
# 使用 Bun
bun run dev

# 或使用 npm
npm run dev
```

访问 **https://localhost:3000**

**注意**：浏览器会提示证书不安全（自签名），点击"继续访问"即可。

## 🏭 生产构建

构建优化的版本用于部署：

```bash
cd web

# 使用 Makefile
make build-release

# 或手动执行
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen \
  --target web \
  --out-dir www/pkg \
  --no-typescript \
  ../target/wasm32-unknown-unknown/release/protocol_viewer_web.wasm

cd www
bun run build  # 或 npm run build
```

构建产物在 `web/www/dist/` 目录。

## 📂 本地预览生产构建

```bash
cd web/www

# 使用 Bun
bun run preview

# 或使用 npm
npm run preview

# 或使用 Python
python -m http.server 8080 --directory dist

# 或使用 Node.js http-server
npx http-server dist -p 8080
```

访问 http://localhost:8080

## 🌐 本地静态服务器选项

如果不想使用 Vite，可以用其他静态服务器：

### 选项 1: Python（自带）

```bash
cd web/www/dist
python -m http.server 8080
```

### 选项 2: Node.js http-server

```bash
npx http-server web/www/dist -p 8080
```

### 选项 3: Nginx（高级）

nginx.conf 配置：

```nginx
server {
    listen 8080;
    server_name localhost;
    root /path/to/protocol-viewer/web/www/dist;
    index index.html;
    
    location / {
        try_files $uri $uri/ /index.html;
    }
    
    # 启用 Gzip 压缩
    gzip on;
    gzip_types text/plain text/css application/json application/javascript text/xml application/xml application/xml+rss text/javascript application/wasm;
}
```

## 🐛 故障排除

### 问题 1: `rustup target add` 下载很慢

**解决方法A：配置国内镜像**

Windows PowerShell:
```powershell
$env:RUSTUP_DIST_SERVER="https://mirrors.ustc.edu.cn/rust-static"
$env:RUSTUP_UPDATE_ROOT="https://mirrors.ustc.edu.cn/rust-static/rustup"
rustup target add wasm32-unknown-unknown
```

Linux/macOS:
```bash
export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
rustup target add wasm32-unknown-unknown
```

**解决方法B：离线安装**

1. 从其他机器拷贝 `~/.rustup/toolchains/*/lib/rustlib/wasm32-unknown-unknown/`
2. 或使用 rustup 的离线安装模式

### 问题 2: `cargo build` GPUI 依赖下载失败

GPUI 依赖 GitHub 仓库，如果网络问题，可以：

1. **配置 Git 代理**
   ```bash
   git config --global http.proxy http://127.0.0.1:7890
   git config --global https.proxy https://127.0.0.1:7890
   ```

2. **使用国内镜像（如果有）**
   
   修改 `web/Cargo.toml`，将 GitHub 链接替换为镜像源（如果可用）。

3. **离线构建**
   
   在有网络的机器上先构建一次，然后拷贝 `~/.cargo` 和 `target/` 目录。

### 问题 3: wasm-bindgen 版本不匹配

错误信息：
```
it looks like the Rust project used to create this wasm file was linked against
version `X.Y.Z` of wasm-bindgen
```

**解决方法**：

```bash
# 检查项目使用的版本
grep wasm-bindgen web/Cargo.toml

# 安装匹配的版本
cargo install wasm-bindgen-cli --version X.Y.Z
```

### 问题 4: 浏览器显示 "Failed to fetch"

**原因**：GPUI 需要 HTTPS 环境（WebGL 要求）

**解决方法**：
1. 使用 Vite 开发服务器（自动提供 HTTPS）
2. 或配置 Nginx/Apache 的 HTTPS
3. 浏览器忽略证书警告（开发环境）

### 问题 5: 页面空白，控制台报 WASM 错误

**检查步骤**：

1. 打开浏览器开发者工具（F12）
2. 查看 Console 标签的错误信息
3. 确认 WebAssembly 和 WebGL2 支持：
   - 访问 about:支持 (Firefox) 或 chrome://gpu/ (Chrome)
   - 访问 https://get.webgl.org/webgl2/

### 问题 6: WASM 文件太大

WASM 文件可能 10-20MB，优化方法：

```bash
# 安装 wasm-opt
cargo install wasm-opt

# 优化 WASM 文件
wasm-opt -Oz -o optimized.wasm input.wasm
```

或在 `web/Cargo.toml` 中添加：

```toml
[profile.release]
opt-level = "z"     # 优化文件大小
lto = true          # 链接时优化
codegen-units = 1   # 更好的优化（更慢的编译）
strip = true        # 移除调试符号
```

## 📊 性能对比

| 模式 | WASM 大小 | 编译时间 | 加载速度 |
|------|-----------|----------|----------|
| Debug | ~50MB | 2-5分钟 | 慢 |
| Release | ~15MB | 10-20分钟 | 中等 |
| Release + wasm-opt | ~10MB | 15-25分钟 | 快 |

## 🔧 开发工作流

### 快速迭代（修改 Rust 代码）

```bash
# Terminal 1: 监听 Rust 代码变化并自动编译
cd web
cargo watch -x 'build --target wasm32-unknown-unknown' -s 'wasm-bindgen --target web --out-dir www/pkg --debug ../target/wasm32-unknown-unknown/debug/protocol_viewer_web.wasm'

# Terminal 2: 前端开发服务器（热更新）
cd www
bun run dev
```

### 修改前端代码

Vite 会自动热更新，只需修改 `web/www/src/main.ts` 或 `index.html`。

## 📝 目录结构

```
web/
├── Cargo.toml          # Rust WASM 项目配置
├── src/
│   └── lib.rs          # WASM 入口
├── www/                # 前端资源
│   ├── index.html
│   ├── package.json
│   ├── vite.config.ts
│   ├── src/
│   │   └── main.ts     # TypeScript 入口
│   └── pkg/            # 生成的 WASM 绑定（不提交到 Git）
└── Makefile            # 构建脚本
```

## 🎯 下一步

本地测试成功后，可以：

1. **部署到 GitHub Pages**
   - 参考 [DEPLOYMENT_GUIDE.md](DEPLOYMENT_GUIDE.md)
   - 使用 `deploy.sh` 或 `deploy.bat` 脚本

2. **部署到其他平台**
   - Netlify
   - Vercel
   - Cloudflare Pages
   - 自己的服务器

3. **优化性能**
   - 启用 Gzip/Brotli 压缩
   - 配置 CDN
   - 使用 wasm-opt 优化
   - 添加 Service Worker 缓存

## 💡 提示

- **第一次编译很慢**：GPUI 依赖较大，首次编译可能需要 10-20 分钟
- **增量编译很快**：后续修改只编译变化的部分，通常几秒到几十秒
- **debug 模式快**：开发时用 debug 模式，文件大但编译快
- **release 用于部署**：部署时用 release 模式，文件小加载快

## 🆘 需要帮助？

1. 查看详细构建日志：`cargo build --target wasm32-unknown-unknown -vv`
2. 检查 Rust 版本：`rustc --version`（需要 1.70+）
3. 检查 wasm-bindgen 版本：`wasm-bindgen --version`
4. 浏览器控制台查看错误信息

祝你本地测试顺利！🚀
