.PHONY: install dev build-debug build-release clean

# 检测包管理器
PKG_MANAGER := $(shell command -v bun 2> /dev/null)
ifndef PKG_MANAGER
    PKG_MANAGER := $(shell command -v npm 2> /dev/null)
endif

# 安装依赖
install:
	@echo "Installing dependencies..."
	@rustup target add wasm32-unknown-unknown || true
	@cargo install wasm-bindgen-cli || true
	@cd www && $(PKG_MANAGER) install

# 开发服务器
dev: build-debug
	@echo "Starting development server at https://localhost:3000"
	@cd www && $(PKG_MANAGER) run dev

# Debug 构建
build-debug:
	@echo "Building WASM (debug)..."
	@cargo build --target wasm32-unknown-unknown
	@wasm-bindgen \
		--target web \
		--out-dir www/pkg \
		--debug \
		target/wasm32-unknown-unknown/debug/protocol_viewer_web.wasm

# Release 构建
build-release:
	@echo "Building WASM (release)..."
	@cargo build --release --target wasm32-unknown-unknown
	@wasm-bindgen \
		--target web \
		--out-dir www/pkg \
		--no-typescript \
		target/wasm32-unknown-unknown/release/protocol_viewer_web.wasm
	@echo "Building frontend..."
	@cd www && $(PKG_MANAGER) run build
	@echo "Build complete! Output in www/dist/"

# 清理
clean:
	@cargo clean
	@rm -rf www/pkg
	@rm -rf www/dist
	@rm -rf www/node_modules
