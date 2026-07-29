@echo off
REM Protocol Viewer 快速部署脚本 (Windows)
REM 使用方法: deploy.bat YOUR_GITHUB_USERNAME

setlocal enabledelayedexpansion

if "%~1"=="" (
    echo [错误] 请提供 GitHub 用户名
    echo.
    echo 使用方法: deploy.bat YOUR_GITHUB_USERNAME
    echo 示例: deploy.bat john-doe
    exit /b 1
)

set GITHUB_USERNAME=%~1
set REPO_NAME=protocol-viewer
set REPO_URL=https://github.com/%GITHUB_USERNAME%/%REPO_NAME%.git

echo.
echo ═══════════════════════════════════════════
echo Protocol Viewer 部署到 GitHub Pages
echo ═══════════════════════════════════════════
echo.
echo GitHub 用户名: %GITHUB_USERNAME%
echo 仓库名称: %REPO_NAME%
echo 远程地址: %REPO_URL%
echo.

REM 检查是否已有远程仓库
git remote get-url origin >nul 2>&1
if %errorlevel% equ 0 (
    echo [警告] 已存在远程仓库 origin
    for /f "delims=" %%i in ('git remote get-url origin') do set EXISTING_URL=%%i
    echo 当前 origin: !EXISTING_URL!
    echo.
    
    set /p CONFIRM="是否覆盖？(y/N): "
    if /i not "!CONFIRM!"=="y" (
        echo [信息] 取消操作
        exit /b 0
    )
    
    echo [信息] 移除旧的 origin...
    git remote remove origin
    echo [完成] 远程仓库已移除
)

REM 添加远程仓库
echo [信息] 添加远程仓库...
git remote add origin "%REPO_URL%"
if %errorlevel% neq 0 (
    echo [错误] 添加远程仓库失败
    exit /b 1
)
echo [完成] 远程仓库已添加
echo.

REM 检查当前分支
for /f "delims=" %%i in ('git branch --show-current') do set CURRENT_BRANCH=%%i
echo [信息] 当前分支: %CURRENT_BRANCH%
echo.

REM 推送代码
echo [信息] 推送代码到 GitHub...
echo.
git push -u origin %CURRENT_BRANCH%
if %errorlevel% neq 0 (
    echo.
    echo [错误] 推送失败
    echo.
    echo 可能的原因:
    echo 1. 仓库不存在 - 请先在 GitHub 创建仓库
    echo 2. 权限不足 - 请检查 GitHub 凭据
    echo 3. 分支名不匹配 - 检查远程仓库的默认分支
    echo.
    exit /b 1
)

echo.
echo ═══════════════════════════════════════════
echo [完成] 代码已推送到 GitHub！
echo ═══════════════════════════════════════════
echo.
echo 下一步操作:
echo.
echo 1. 访问仓库设置页面:
echo    https://github.com/%GITHUB_USERNAME%/%REPO_NAME%/settings/pages
echo.
echo 2. 在 'Build and deployment' 部分:
echo    - Source: 选择 'GitHub Actions'
echo    - 点击 'Save'
echo.
echo 3. 查看部署进度:
echo    https://github.com/%GITHUB_USERNAME%/%REPO_NAME%/actions
echo.
echo 4. 等待构建完成(约 5-10 分钟),然后访问:
echo    https://%GITHUB_USERNAME%.github.io/%REPO_NAME%/
echo.
echo ═══════════════════════════════════════════
echo [完成] 部署准备完成！
echo.
echo 详细文档: DEPLOYMENT_GUIDE.md
echo 如有问题,请查看 GitHub Actions 日志
echo.

REM 询问是否打开浏览器
set /p OPEN_BROWSER="是否在浏览器中打开 GitHub 仓库？(Y/n): "
if /i not "!OPEN_BROWSER!"=="n" (
    start https://github.com/%GITHUB_USERNAME%/%REPO_NAME%
    echo [完成] 已打开浏览器
)

echo.
pause
