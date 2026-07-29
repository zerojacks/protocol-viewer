# ⚡ 快速本地测试（无需 WASM 编译）

如果 WASM 编译工具链安装有困难，可以先测试桌面版本。

## 🖥️ 测试桌面版（最简单）

直接运行桌面版，无需任何 Web 工具：

```bash
cd d:\ProjackSpace\projectspace\protocol-viewer
cargo run
```

这会启动原生桌面应用，可以测试所有协议解析功能。

## 🌐 本地 Web 版测试（推荐顺序）

### 选项 1: 使用预编译的 WASM（最快）

如果有其他机器已经编译好 WASM，可以直接拷贝：

1. 从已编译的机器拷贝 `web/www/pkg/` 目录
2. 在本机安装前端依赖：
   ```bash
   cd web/www
   npm install  # 或 bun install
   ```
3. 启动开发服务器：
   ```bash
   npm run dev  # 或 bun run dev
   ```

### 选项 2: 在线版本（无需编译）

等待 GitHub Actions 自动部署后，直接访问在线版本：

```
https://YOUR_USERNAME.github.io/protocol-viewer/
```

优点：
- ✅ 无需安装任何工具
- ✅ 无需编译
- ✅ 任何设备都能访问

### 选项 3: Docker 容器编译（隔离环境）

如果本地环境有问题，可以用 Docker：

```dockerfile
# Dockerfile.wasm
FROM rust:1.70

# 安装 WASM 工具链
RUN rustup target add wasm32-unknown-unknown && \
    cargo install wasm-bindgen-cli

# 安装 Node.js
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash - && \
    apt-get install -y nodejs

WORKDIR /app
COPY . .

# 编译 WASM
RUN cd web && \
    cargo build --release --target wasm32-unknown-unknown && \
    wasm-bindgen \
      --target web \
      --out-dir www/pkg \
      ../target/wasm32-unknown-unknown/release/protocol_viewer_web.wasm

# 构建前端
RUN cd web/www && npm install && npm run build

# 启动静态服务器
CMD cd web/www/dist && python3 -m http.server 8080
```

使用：
```bash
docker build -f Dockerfile.wasm -t protocol-viewer-web .
docker run -p 8080:8080 protocol-viewer-web
```

## 🚀 推荐的测试顺序

1. **先测试桌面版**（5分钟）
   ```bash
   cargo run
   ```
   验证核心功能是否正常。

2. **推送到 GitHub**（5分钟）
   ```bash
   git remote add origin https://github.com/YOUR_USERNAME/protocol-viewer.git
   git push -u origin master
   ```

3. **等待自动部署**（5-10分钟）
   - GitHub Actions 会自动编译 WASM
   - 无需本地编译工具链
   - 查看进度：`https://github.com/YOUR_USERNAME/protocol-viewer/actions`

4. **访问在线版本**
   ```
   https://YOUR_USERNAME.github.io/protocol-viewer/
   ```

## 📊 方案对比

| 方案 | 优点 | 缺点 | 适用场景 |
|------|------|------|----------|
| 桌面版 | 最快，无需配置 | 仅限本机 | 快速验证功能 |
| GitHub Pages | 无需本地编译，全球可访问 | 需要 GitHub 账号 | 正式部署 |
| 本地 WASM | 完全控制，可调试 | 工具链复杂 | 开发调试 |
| Docker | 环境隔离，可复现 | 需要 Docker | CI/CD 环境 |

## 🎯 最简单的完整流程

```bash
# 1. 测试桌面版（验证功能）
cargo run

# 2. 创建 GitHub 仓库
# 访问 https://github.com/new，创建 protocol-viewer

# 3. 推送代码
git remote add origin https://github.com/YOUR_USERNAME/protocol-viewer.git
git push -u origin master

# 4. 启用 GitHub Pages
# 访问仓库 Settings → Pages
# Source: 选择 "GitHub Actions"

# 5. 等待部署完成
# 访问 Actions 标签查看进度

# 6. 访问在线版本
# https://YOUR_USERNAME.github.io/protocol-viewer/
```

**总耗时**：约 15-20 分钟（大部分时间是等待 GitHub Actions 构建）

## 💡 提示

### 如果不着急使用 Web 版
- 先用桌面版开发和测试
- 等有需要时再配置 WASM 环境
- 或直接使用 GitHub Actions 自动编译

### 如果需要本地调试 Web 版
- 参考 [LOCAL_DEPLOYMENT.md](LOCAL_DEPLOYMENT.md)
- 配置国内镜像加速下载
- 或在网络好的时候一次性配置完成

### 如果要分享给其他人
- GitHub Pages 是最佳选择
- 用户无需安装任何东西
- 直接分享链接即可

## 🆘 遇到问题？

1. **桌面版无法编译**
   - 检查 Rust 版本：`rustc --version`（需要 1.70+）
   - 更新 Rust：`rustup update`
   - 清理重新构建：`cargo clean && cargo build`

2. **Git 推送失败**
   - 检查远程仓库是否存在
   - 检查 GitHub 凭据是否正确
   - 使用 SSH 代替 HTTPS：`git remote set-url origin git@github.com:USER/REPO.git`

3. **GitHub Actions 构建失败**
   - 查看 Actions 标签的详细日志
   - 常见原因：依赖下载超时（重新运行即可）
   - 查看 [DEPLOYMENT_GUIDE.md](DEPLOYMENT_GUIDE.md) 的故障排除部分

祝你测试顺利！🎉
