#!/usr/bin/env sh
# Package a built .app into a drag-and-drop DMG.
set -eu

APP_VERSION="${APP_VERSION:-0.1.0}"
DARWIN_ARCH="${DARWIN_ARCH:-x86_64}"
SCRIPT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
ROOT_DIR="$(CDPATH= cd -- "$SCRIPT_DIR/../.." && pwd)"
APP_BUNDLE="$ROOT_DIR/target/release/Protocol Viewer.app"
OUTPUT_DIR="$ROOT_DIR/dist"
OUTPUT_FILE="$OUTPUT_DIR/protocol-viewer-macos-$DARWIN_ARCH-$APP_VERSION.dmg"
STAGING_DIR="$ROOT_DIR/target/dmg-staging"

if [ ! -d "$APP_BUNDLE" ]; then
  echo "未找到 $APP_BUNDLE；请先运行 build-app-bundle.sh。" >&2
  exit 1
fi

rm -rf "$STAGING_DIR"
mkdir -p "$STAGING_DIR" "$OUTPUT_DIR"
cp -R "$APP_BUNDLE" "$STAGING_DIR/"
ln -s /Applications "$STAGING_DIR/Applications"
hdiutil create -volname "Protocol Viewer" -srcfolder "$STAGING_DIR" \
  -ov -format UDZO "$OUTPUT_FILE"
echo "已生成 $OUTPUT_FILE"
