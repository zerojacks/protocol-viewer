# 🎯 开始使用 Protocol Viewer

## 你现在的状态

✅ **所有配置文件已完成**  
✅ **代码已提交到 Git（4 个 commits）**  
✅ **准备就绪，可以部署**

## 🚀 接下来做什么？

### 选项 A: 快速测试桌面版（推荐先做）⭐

验证功能是否正常：

```bash
cargo run
```

5分钟内看到效果，无需任何额外配置！

---

### 选项 B: 部署到 GitHub Pages（在线访问）

#### 步骤 1: 创建 GitHub 仓库

1. 访问 https://github.com/new
2. Repository name: `protocol-viewer`
3. 选择 **Public**
4. **不要**勾选 "Initialize with README"
5. 点击 "Create repository"

#### 步骤 2: 连接并推送

**Windows**:
```cmd
deploy.bat YOUR_GITHUB_USERNAME
```

**macOS/Linux**:
```bash
./deploy.sh YOUR_GITHUB_USERNAME
```

或手动执行：
```bash
git remote add origin https://github.com/YOUR_USERNAME/protocol-viewer.git
git push -u origin master
```

#### 步骤 3: 启用 GitHub Pages

1. 访问 `https://github.com/YOUR_USERNAME/protocol-viewer/settings/pages`
2. Source 选择 **"GitHub Actions"**
3. 点击 Save

#### 步骤 4: 等待部署

- 访问 `https://github.com/YOUR_USERNAME/protocol-viewer/actions`
- 等待绿色 ✅（约 5-10 分钟）
- 访问 `https://YOUR_USERNAME.github.io/protocol-viewer/`

---

### 选项 C: 本地运行 Web 版

如果想在本地测试 WASM 版本：

```bash
cd web

# 首次运行（安装依赖）
make install

# 启动开发服务器
make dev
```

访问 https://localhost:3000

**注意**：
- 需要先安装 WASM 工具链（参考 [LOCAL_DEPLOYMENT.md](LOCAL_DEPLOYMENT.md)）
- 如果工具链安装有困难，建议直接用选项 B（GitHub Actions 自动编译）

---

## 📚 文档指引

根据你的需求选择文档：

| 文档 | 适用场景 |
|------|----------|
| **START_HERE.md** （本文件） | 快速开始，不知道从哪里开始 |
| **QUICK_LOCAL_TEST.md** | 想快速测试，不想配置复杂环境 |
| **DEPLOY_QUICKSTART.md** | 5分钟部署到 GitHub Pages |
| **DEPLOYMENT_GUIDE.md** | 完整部署指南 + 故障排除 |
| **LOCAL_DEPLOYMENT.md** | 本地构建 WASM 详细说明 |
| **WASM_SETUP.md** | WASM 技术详解 |
| **README.md** | 项目功能和使用说明 |

## 🎯 推荐流程（新手）

```
1. 测试桌面版
   cargo run
   ↓
2. 推送到 GitHub
   deploy.bat YOUR_USERNAME
   ↓
3. 启用 GitHub Pages
   Settings → Pages → GitHub Actions
   ↓
4. 访问在线版本
   https://YOUR_USERNAME.github.io/protocol-viewer/
```

**总耗时**：15-20 分钟

## 🎯 推荐流程（开发者）

```
1. 测试桌面版
   cargo run
   ↓
2. 安装 WASM 工具链
   rustup target add wasm32-unknown-unknown
   cargo install wasm-bindgen-cli
   ↓
3. 本地测试 Web 版
   cd web && make dev
   ↓
4. 部署到 GitHub Pages
   deploy.bat YOUR_USERNAME
```

**总耗时**：30-60 分钟（包含工具安装）

## 🎯 推荐流程（仅部署）

```
1. 推送到 GitHub
   deploy.bat YOUR_USERNAME
   ↓
2. 启用 GitHub Pages
   Settings → Pages → GitHub Actions
   ↓
3. 完成！
   访问在线版本
```

**总耗时**：10 分钟

## 💡 小提示

### 想快速看效果？
→ 运行 `cargo run`，立即看到桌面版

### 想分享给别人？
→ 部署到 GitHub Pages，分享链接

### 想本地调试 Web 版？
→ 参考 [LOCAL_DEPLOYMENT.md](LOCAL_DEPLOYMENT.md)

### 想了解技术细节？
→ 参考 [WASM_SETUP.md](WASM_SETUP.md)

### 遇到问题？
→ 参考 [DEPLOYMENT_GUIDE.md](DEPLOYMENT_GUIDE.md) 的故障排除部分

## ✅ 快速检查清单

部署前确认：

- [ ] Git 仓库已初始化
- [ ] 所有文件已提交（git status 显示干净）
- [ ] 有 GitHub 账号
- [ ] 准备好仓库名称（建议：protocol-viewer）

部署后确认：

- [ ] GitHub Pages 已启用
- [ ] GitHub Actions 工作流成功运行
- [ ] 可以访问在线地址
- [ ] 页面正常加载和解析报文

## 🎉 完成后你将拥有

- ✅ 原生桌面应用（Windows/macOS/Linux）
- ✅ 在线 Web 应用（浏览器访问）
- ✅ 自动 CI/CD（推送代码自动部署）
- ✅ 完整的技术文档
- ✅ 可分享的在线链接

## 🚀 现在开始！

选择你的路径，开始使用 Protocol Viewer！

**最简单的方式**：
```bash
# 1. 测试功能
cargo run

# 2. 部署在线版
deploy.bat YOUR_GITHUB_USERNAME
```

祝你使用愉快！🎊
