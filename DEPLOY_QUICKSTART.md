# 🚀 快速部署指南

5 分钟将 Protocol Viewer 部署到 GitHub Pages！

## 方法一：使用部署脚本（推荐）

### Windows

```cmd
deploy.bat YOUR_GITHUB_USERNAME
```

### macOS / Linux

```bash
./deploy.sh YOUR_GITHUB_USERNAME
```

将 `YOUR_GITHUB_USERNAME` 替换为你的 GitHub 用户名。

## 方法二：手动部署

### 步骤 1: 创建 GitHub 仓库

1. 访问 https://github.com/new
2. Repository name: `protocol-viewer`
3. Public（必须）
4. **不要**勾选 "Initialize this repository with a README"
5. 点击 "Create repository"

### 步骤 2: 连接远程仓库

```bash
# 替换 YOUR_USERNAME 为你的 GitHub 用户名
git remote add origin https://github.com/YOUR_USERNAME/protocol-viewer.git
```

### 步骤 3: 推送代码

```bash
git push -u origin master
# 如果你的分支是 main
git branch -M main
git push -u origin main
```

### 步骤 4: 启用 GitHub Pages

1. 访问 `https://github.com/YOUR_USERNAME/protocol-viewer/settings/pages`
2. **Build and deployment** 部分
3. Source: 选择 **"GitHub Actions"**
4. 点击 Save

### 步骤 5: 等待部署

1. 访问 `https://github.com/YOUR_USERNAME/protocol-viewer/actions`
2. 看到 "Deploy to GitHub Pages" 工作流
3. 等待绿色勾号 ✅（约 5-10 分钟）
4. 访问 `https://YOUR_USERNAME.github.io/protocol-viewer/`

## 🎉 完成！

你的 Protocol Viewer 现在已经在线上运行了！

## ❓ 常见问题

### Q: 推送失败 "repository not found"

**解决方法**：
1. 确认已在 GitHub 创建仓库
2. 检查用户名和仓库名是否正确
3. 确认有仓库访问权限

### Q: GitHub Actions 构建失败

**查看日志**：
1. 访问 Actions 标签
2. 点击失败的工作流
3. 展开失败的步骤查看详细错误

常见原因：
- wasm-bindgen 版本不匹配
- Rust 编译错误
- 依赖下载超时

### Q: 页面显示 404

**检查**：
1. GitHub Pages 是否已启用
2. Source 是否设置为 "GitHub Actions"
3. 工作流是否成功完成
4. 等待 2-3 分钟让 CDN 更新

### Q: 如何更新部署？

只需推送新代码：

```bash
git add .
git commit -m "你的修改说明"
git push
```

GitHub Actions 会自动重新部署。

## 📚 更多文档

- 详细部署指南：[DEPLOYMENT_GUIDE.md](DEPLOYMENT_GUIDE.md)
- WASM 配置说明：[WASM_SETUP.md](WASM_SETUP.md)
- Web 版本文档：[web/README.md](web/README.md)

## 🆘 需要帮助？

1. 查看 [GitHub Actions 日志](https://github.com/YOUR_USERNAME/protocol-viewer/actions)
2. 查看 [DEPLOYMENT_GUIDE.md](DEPLOYMENT_GUIDE.md) 的故障排除部分
3. 检查浏览器控制台的错误信息

## 🌐 分享你的部署

部署成功后，你可以：

1. **添加到 README 徽章**
   ```markdown
   ![Deploy Status](https://github.com/YOUR_USERNAME/protocol-viewer/actions/workflows/deploy.yml/badge.svg)
   ```

2. **分享链接**
   ```
   https://YOUR_USERNAME.github.io/protocol-viewer/
   ```

3. **自定义域名**（可选）
   - 在 GitHub Pages 设置中添加自定义域名
   - 配置 DNS CNAME 记录

祝你部署顺利！🎊
