#!/bin/bash

# Protocol Viewer 快速部署脚本
# 使用方法: ./deploy.sh YOUR_GITHUB_USERNAME

set -e  # 遇到错误立即退出

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 打印带颜色的消息
print_info() {
    echo -e "${BLUE}ℹ ${NC}$1"
}

print_success() {
    echo -e "${GREEN}✓ ${NC}$1"
}

print_warning() {
    echo -e "${YELLOW}⚠ ${NC}$1"
}

print_error() {
    echo -e "${RED}✗ ${NC}$1"
}

# 检查参数
if [ -z "$1" ]; then
    print_error "请提供 GitHub 用户名"
    echo "使用方法: ./deploy.sh YOUR_GITHUB_USERNAME"
    echo "示例: ./deploy.sh john-doe"
    exit 1
fi

GITHUB_USERNAME="$1"
REPO_NAME="protocol-viewer"
REPO_URL="https://github.com/${GITHUB_USERNAME}/${REPO_NAME}.git"

echo ""
print_info "Protocol Viewer 部署到 GitHub Pages"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
print_info "GitHub 用户名: ${GITHUB_USERNAME}"
print_info "仓库名称: ${REPO_NAME}"
print_info "远程地址: ${REPO_URL}"
echo ""

# 检查是否已有远程仓库
if git remote get-url origin &> /dev/null; then
    print_warning "已存在远程仓库 origin"
    EXISTING_URL=$(git remote get-url origin)
    print_info "当前 origin: ${EXISTING_URL}"
    
    read -p "是否覆盖？(y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        print_info "取消操作"
        exit 0
    fi
    
    print_info "移除旧的 origin..."
    git remote remove origin
fi

# 添加远程仓库
print_info "添加远程仓库..."
git remote add origin "${REPO_URL}"
print_success "远程仓库已添加"

# 检查当前分支
CURRENT_BRANCH=$(git branch --show-current)
print_info "当前分支: ${CURRENT_BRANCH}"

# 推送代码
print_info "推送代码到 GitHub..."
echo ""
git push -u origin "${CURRENT_BRANCH}"
echo ""
print_success "代码推送成功！"

# 提示后续步骤
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
print_success "代码已推送到 GitHub！"
echo ""
print_info "下一步操作："
echo ""
echo "1. 访问仓库设置页面："
echo "   ${BLUE}https://github.com/${GITHUB_USERNAME}/${REPO_NAME}/settings/pages${NC}"
echo ""
echo "2. 在 'Build and deployment' 部分："
echo "   - Source: 选择 ${GREEN}'GitHub Actions'${NC}"
echo "   - 点击 'Save'"
echo ""
echo "3. 查看部署进度："
echo "   ${BLUE}https://github.com/${GITHUB_USERNAME}/${REPO_NAME}/actions${NC}"
echo ""
echo "4. 等待构建完成（约 5-10 分钟），然后访问："
echo "   ${GREEN}https://${GITHUB_USERNAME}.github.io/${REPO_NAME}/${NC}"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
print_success "部署完成！"
echo ""

# 询问是否打开浏览器
if command -v xdg-open &> /dev/null; then
    OPEN_CMD="xdg-open"
elif command -v open &> /dev/null; then
    OPEN_CMD="open"
elif command -v start &> /dev/null; then
    OPEN_CMD="start"
else
    OPEN_CMD=""
fi

if [ -n "$OPEN_CMD" ]; then
    read -p "是否在浏览器中打开 GitHub 仓库？(Y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Nn]$ ]]; then
        $OPEN_CMD "https://github.com/${GITHUB_USERNAME}/${REPO_NAME}"
        print_success "已打开浏览器"
    fi
fi

echo ""
print_info "详细文档: DEPLOYMENT_GUIDE.md"
print_info "如有问题，请查看 GitHub Actions 日志"
echo ""
