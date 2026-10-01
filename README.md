# Protocol Viewer

Protocol Viewer 是一个用于查看电力通信报文解析结果的工具，提供 **桌面版**和 **浏览器版**。输入十六进制报文后，程序会自动识别协议并将解析结果显示为可交互的树形结构。

- **在线体验：**[https://zerojacks.github.io/protocol-viewer/](https://zerojacks.github.io/protocol-viewer/)
- **GitHub 仓库：**[zerojacks/protocol-viewer](https://github.com/zerojacks/protocol-viewer)

## 功能

- 自动检测并解析支持的协议报文。
- 输入内容变化时即时解析，并以树形结构展示字段层级。
- 展开或折叠包含子字段的节点。
- 点击解析结果中的字段，在输入区定位并选中对应报文字节。
- 拖动分隔条调整“帧域 / 数据 / 说明”三列宽度。
- 位字段显示提取出的 bit 值及其语义说明；选择位字段时可定位到对应报文字节。
- 输入格式错误或报文解析失败时显示错误信息。

## 支持的协议

协议识别、解码和解析树转换由 [`protocol-parser`](https://github.com/zerojacks/protocol-parser) 提供。目前集成的协议包括：

- DL/T 645-2007
- Q/CSG1209022-2019
- Q/CSG1209021-2019（南网本地通信协议）

能否成功解析仍取决于报文是否符合相应协议格式，以及解析器所含的协议定义。

## 使用

### 在线使用

打开 [在线体验页面](https://zerojacks.github.io/protocol-viewer/)，在顶部输入框粘贴十六进制报文。程序会在输入变化时自动解析，无需点击单独的解析按钮。点击树中的字段可在输入内容中选中对应字节。

### 本地运行桌面版

需要安装 [Rust](https://www.rust-lang.org/tools/install)。仓库使用 Rust nightly 工具链；通过 rustup 执行 Cargo 命令时会按项目配置选择该工具链。

在仓库根目录运行：

```sh
cargo run
```

构建桌面程序：

```sh
cargo build --release
```

### 本地运行 Web 版

Web 版使用 Rust/WASM 和 Vite。需要安装 Rust nightly、Bun（或 Node.js/npm），并安装 `wasm-bindgen-cli` **0.2.126**。在仓库根目录执行：

```sh
rustup target add wasm32-unknown-unknown --toolchain nightly
cargo +nightly install wasm-bindgen-cli --version 0.2.126 --locked
```

构建 WASM 并生成浏览器绑定：

```sh
cd web
cargo +nightly build --release --target wasm32-unknown-unknown
wasm-bindgen --target web --out-dir www/pkg --no-typescript target/wasm32-unknown-unknown/release/protocol_viewer_web.wasm
```

安装前端依赖并启动开发服务器：

```sh
cd www
npm install
npm run dev
```

终端会显示本地访问地址，默认端口为 `3000`。使用 Bun 时，可将 `npm install` / `npm run dev` 分别替换为 `bun install` / `bun run dev`。

生成静态生产文件：完成 WASM 生成后，在 `web/www` 目录运行：

```sh
npm run build
```

构建产物位于 `web/www/dist/`。

## 报文输入格式

输入十六进制字节，可使用空格、换行、冒号或连字符分隔，也可连续输入。例如以下几种写法都可识别：

```text
68 0C 00 80 00 01 01 00 01 E8 6B 16
68:0C:00:80:00:01:01:00:01:E8:6B:16
680C00800001010001E86B16
```

输入不能为空；十六进制字符数量必须为偶数，且报文必须符合受支持协议的帧格式。

## GitHub Pages 部署

仓库的 [GitHub Actions 部署工作流](.github/workflows/deploy.yml)会在向 `main` 或 `master` 分支推送时构建并部署 Web 版，也可在 Actions 页面手动触发。

首次部署前，请在仓库 **Settings → Pages** 中将部署来源设为 **GitHub Actions**。部署完成后访问：

[https://zerojacks.github.io/protocol-viewer/](https://zerojacks.github.io/protocol-viewer/)

## 桌面版发行

向仓库推送以 `v` 开头的版本标签（例如 `v1.0.0`），[桌面版发布工作流](.github/workflows/release.yml)会构建并发布 Linux x86_64 的 `.deb` 安装包、Windows x86_64 的 setup 安装程序，以及 macOS Intel 和 Apple Silicon 的 `.dmg` 安装镜像。首次发布前，请确认仓库 Actions 设置允许工作流创建 Release。

在 GitHub 的 Releases 页面下载对应平台的安装包。Linux 可使用系统软件包管理器安装 `.deb`；Windows 运行 setup 安装程序；macOS 打开 `.dmg` 并将应用拖入“应用程序”。当前 macOS 构建未签名或公证，首次打开时系统可能会显示安全确认提示。

## 项目结构

```text
protocol-viewer/
├── src/
│   ├── app.rs       # 桌面与 Web 共用的界面、输入解析及树形交互
│   ├── row.rs       # 将解析器的 FieldValue 树转换为展示行
│   ├── lib.rs       # 共享库导出
│   └── main.rs      # 桌面应用入口
├── examples/        # 报文解析示例
├── web/
│   ├── src/         # WebAssembly 应用入口
│   └── www/         # Vite 前端及静态页面
├── packaging/
│   ├── linux/      # Debian 安装包和桌面入口
│   ├── macos/      # .app 与 DMG 打包脚本
│   └── windows/    # Windows 安装程序配置
└── .github/workflows/
    ├── deploy.yml   # GitHub Pages 自动部署
    └── release.yml  # 桌面版多平台构建与发布
```

## 技术栈

- Rust 与 GPUI Kit：共享 UI 和桌面应用。
- `protocol-parser`：协议自动识别、报文解析和解析树生成。
- WebAssembly、`wasm-bindgen` 与 Vite：浏览器版本及静态资源构建。
