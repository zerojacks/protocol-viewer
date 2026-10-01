#!/usr/bin/env sh
# Build a standard macOS .app bundle from the release binary.
set -eu

APP_VERSION="${APP_VERSION:-0.1.0}"
TARGET_TRIPLE="${TARGET_TRIPLE:-}"
PROFILE="${1:-release}"
SCRIPT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
ROOT_DIR="$(CDPATH= cd -- "$SCRIPT_DIR/../.." && pwd)"
PROFILE_DIR="$PROFILE"
BINARY="$ROOT_DIR/target/${TARGET_TRIPLE:+$TARGET_TRIPLE/}$PROFILE_DIR/protocol-viewer"
APP_BUNDLE="$ROOT_DIR/target/$PROFILE_DIR/Protocol Viewer.app"

if [ ! -f "$BINARY" ]; then
  echo "未找到 $BINARY；请先构建对应平台的 protocol-viewer。" >&2
  exit 1
fi

rm -rf "$APP_BUNDLE"
mkdir -p "$APP_BUNDLE/Contents/MacOS" "$APP_BUNDLE/Contents/Resources"
install -m 0755 "$BINARY" "$APP_BUNDLE/Contents/MacOS/protocol-viewer"
sed "s/__APP_VERSION__/$APP_VERSION/g" \
  "$SCRIPT_DIR/Info.plist.in" > "$APP_BUNDLE/Contents/Info.plist"

# macOS 使用 .icns；仅在仓库提供图标时将其打入应用包。
if [ -n "${MACOS_ICON_PATH:-}" ] && [ -f "$MACOS_ICON_PATH" ]; then
  install -m 0644 "$MACOS_ICON_PATH" "$APP_BUNDLE/Contents/Resources/app.icns"
fi

/usr/libexec/PlistBuddy -c 'Print :CFBundleExecutable' "$APP_BUNDLE/Contents/Info.plist" >/dev/null
echo "已生成 $APP_BUNDLE"
