# Protocol Viewer Web

Protocol Viewer 的 WebAssembly 浏览器版本。

## 🌐 在线演示

**Live Demo**: https://YOUR_GITHUB_USERNAME.github.io/protocol-viewer/

## 📋 前置要求

1. **Rust 工具链**
   ```bash
   rustup target add wasm32-unknown-unknown
   cargo install wasm-bindgen-cli
   ```

2. **Node.js 包管理器**（选择其一）
   - **Bun**（推荐，更快）
     ```bash
     # Windows
     powershell -c "irm bun.sh/install.ps1 | iex"
     
     # macOS/Linux
     curl -fsSL https://bun.sh/install | bash
     ```
   - **Node.js**: https://nodejs.org/

## 🚀 快速开始

### 开发模式

```bash
# 1. 安装依赖（首次运行）
make install

# 2. 启动开发服务器
make dev
```

访问 https://localhost:3000（自签名证书，浏览器会提示不安全，点击"继续访问"即可）

### 生产构建

```bash
make build-release
```

构建产物在 `www/dist/` 目录。

## 📦 部署

### GitHub Pages

1. **启用 GitHub Pages**
   - 进入仓库 Settings → Pages
   - Source 选择 "GitHub Actions"

2. **推送代码**
   ```bash
   git add .
   git commit -m "Add WASM web version"
   git push origin main
   ```

3. **自动部署**
   - GitHub Actions 会自动构建并部署
   - 访问 `https://YOUR_USERNAME.github.io/protocol-viewer/`

### 其他静态托管

构建后将 `www/dist/` 目录上传到：
- **Netlify**: 拖拽上传或连接 Git
- **Vercel**: 连接 Git 仓库自动部署
- **Cloudflare Pages**: 连接 Git 或上传文件

## 🔧 故障排除

### WASM 编译失败

```bash
# 检查工具链
rustup show
rustc --version

# 重新安装
rustup update
cargo install --force wasm-bindgen-cli
```

### WebGL 错误

检查浏览器是否支持 WebGL2：https://get.webgl.org/webgl2/

### 内存不足

在 `src/lib.rs` 中增加 WASM 内存限制。

## 📖 更多文档

查看项目根目录的 `WASM_SETUP.md` 获取详细配置说明。

## 🌐 浏览器兼容性

- ✅ Chrome 57+
- ✅ Firefox 52+
- ✅ Safari 11+
- ✅ Edge 16+

## 📄 许可证

与主项目相同
