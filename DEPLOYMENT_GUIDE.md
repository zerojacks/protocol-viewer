# 📦 Protocol Viewer 部署到 GitHub Pages 指南

本指南将帮助你将 Protocol Viewer 部署到 GitHub Pages，使其可以通过浏览器访问。

## ✅ 已完成的配置

所有 WASM 配置文件已创建完成：

```
✅ protocol-viewer/
├── ✅ .github/workflows/deploy.yml  # GitHub Actions 自动部署
├── ✅ web/
│   ├── ✅ Cargo.toml                # WASM 项目配置
│   ├── ✅ Makefile                  # 构建脚本
│   ├── ✅ src/lib.rs                # WASM 入口
│   └── ✅ www/
│       ├── ✅ index.html
│       ├── ✅ package.json
│       ├── ✅ vite.config.ts
│       └── ✅ src/main.ts
├── ✅ src/lib.rs                    # 共享库
└── ✅ Cargo.toml                    # 支持 [lib] + [[bin]]
```

## 🚀 部署步骤

### 步骤 1: 创建 GitHub 仓库

1. **访问 GitHub**
   - 登录 https://github.com
   - 点击右上角 "+" → "New repository"

2. **配置仓库**
   - Repository name: `protocol-viewer`
   - Description: `电力协议解析查看器 - 支持 DL/T645-2007、CSG1209022、CSG本地通信协议`
   - Public（必须是 Public 才能使用免费 GitHub Pages）
   - **不要**勾选 "Initialize this repository with a README"
   - 点击 "Create repository"

### 步骤 2: 连接本地仓库到 GitHub

在 `protocol-viewer` 目录执行：

```bash
# 添加远程仓库（替换 YOUR_USERNAME）
git remote add origin https://github.com/YOUR_USERNAME/protocol-viewer.git

# 或使用 SSH（如果配置了 SSH key）
git remote add origin git@github.com:YOUR_USERNAME/protocol-viewer.git
```

### 步骤 3: 提交并推送代码

```bash
# 添加所有文件
git add .

# 提交
git commit -m "Initial commit: Add WASM web version with GitHub Pages support"

# 推送到 GitHub（首次推送）
git push -u origin master

# 如果你的默认分支是 main 而不是 master
git branch -M main
git push -u origin main
```

### 步骤 4: 配置 GitHub Pages

1. **进入仓库设置**
   - 访问 `https://github.com/YOUR_USERNAME/protocol-viewer`
   - 点击 "Settings"

2. **启用 GitHub Pages**
   - 左侧菜单找到 "Pages"
   - Source 选择 **"GitHub Actions"**（重要！）
   - 点击 "Save"

### 步骤 5: 等待自动部署

1. **查看构建进度**
   - 点击仓库顶部的 "Actions" 标签
   - 会看到 "Deploy to GitHub Pages" 工作流正在运行
   - 首次构建约需 5-10 分钟（下载依赖较慢）

2. **部署完成**
   - 工作流状态变为 ✅ 绿色对勾
   - 访问 `https://YOUR_USERNAME.github.io/protocol-viewer/`

## 🎯 访问地址

部署成功后，访问：

```
https://YOUR_USERNAME.github.io/protocol-viewer/
```

例如，如果你的 GitHub 用户名是 `john-doe`，则访问：
```
https://john-doe.github.io/protocol-viewer/
```

## 🔧 本地测试（可选）

在部署前可以先在本地测试：

```bash
# 1. 安装依赖（首次运行）
cd web
make install

# 2. 启动开发服务器
make dev
```

访问 https://localhost:3000（自签名证书，浏览器会警告，点击"继续访问"）

## 📝 更新部署

以后每次修改代码后，只需：

```bash
git add .
git commit -m "描述你的修改"
git push
```

GitHub Actions 会自动重新构建和部署。

## ⚠️ 注意事项

### 1. 分支名称

工作流配置为监听 `main` 和 `master` 分支。如果你的分支名不同，需要修改 `.github/workflows/deploy.yml`：

```yaml
on:
  push:
    branches: [ your-branch-name ]  # 修改这里
```

### 2. 仓库名称

如果仓库名不是 `protocol-viewer`，需要修改 `web/www/vite.config.ts`：

```typescript
export default defineConfig({
    base: '/your-repo-name/',  // 修改这里
    // ...
});
```

### 3. 构建时间

- 首次构建：5-10 分钟（需要下载 GPUI 等依赖）
- 后续构建：2-5 分钟（有缓存）

### 4. 浏览器兼容性

需要支持以下特性的现代浏览器：
- WebAssembly
- WebGL 2.0
- ES2020+

兼容：Chrome 57+、Firefox 52+、Safari 11+、Edge 16+

## 🐛 故障排除

### 构建失败：wasm-bindgen 版本不匹配

如果看到类似错误：
```
it looks like the Rust project used to create this wasm file was linked against
version `X.Y.Z` of wasm-bindgen
```

解决方法：在 GitHub Actions 中锁定版本。修改 `.github/workflows/deploy.yml`：

```yaml
- name: Install wasm-bindgen-cli
  run: cargo install wasm-bindgen-cli --version X.Y.Z  # 指定版本
```

### 页面 404

1. 确认 GitHub Pages 已启用且 Source 设置为 "GitHub Actions"
2. 检查工作流是否成功运行（Actions 标签）
3. 等待几分钟让 CDN 缓存更新

### WASM 加载失败

1. 检查浏览器控制台错误信息
2. 确认浏览器支持 WebAssembly 和 WebGL2
3. 尝试清除浏览器缓存

## 📊 监控部署

### 查看工作流日志

1. 访问 `https://github.com/YOUR_USERNAME/protocol-viewer/actions`
2. 点击最近的工作流运行
3. 展开各步骤查看详细日志

### 构建状态徽章

在 README.md 中添加徽章：

```markdown
![Deploy](https://github.com/YOUR_USERNAME/protocol-viewer/actions/workflows/deploy.yml/badge.svg)
```

## 🎉 完成！

现在你的 Protocol Viewer 已成功部署到 GitHub Pages，全世界的用户都可以通过浏览器访问！

---

## 💡 下一步

1. **自定义域名**（可选）
   - 在 GitHub Pages 设置中配置自定义域名
   - 添加 CNAME 文件

2. **添加分析**（可选）
   - Google Analytics
   - Plausible
   - Umami

3. **性能优化**
   - 启用 CDN
   - 启用 Gzip/Brotli 压缩
   - 使用 wasm-opt 优化

4. **SEO 优化**
   - 添加 meta 标签
   - 创建 sitemap.xml
   - 添加 robots.txt

如有问题，请查看：
- 详细配置：`WASM_SETUP.md`
- 快速开始：`web/README.md`
- GitHub Actions 文档：https://docs.github.com/actions
