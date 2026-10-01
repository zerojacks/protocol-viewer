#!/usr/bin/env sh
set -eu

VERSION="${1:?用法: build-deb.sh <version>}"
ARCH="${DEB_ARCH:-amd64}"
SCRIPT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
ROOT_DIR="$(CDPATH= cd -- "$SCRIPT_DIR/../.." && pwd)"
PACKAGE_ROOT="$ROOT_DIR/target/deb/protocol-viewer"
OUTPUT_DIR="$ROOT_DIR/dist"
OUTPUT_FILE="$OUTPUT_DIR/protocol-viewer-linux-x86_64-$VERSION.deb"
BINARY="$ROOT_DIR/target/release/protocol-viewer"

if [ ! -f "$BINARY" ]; then
  echo "未找到 $BINARY；请先构建 release 二进制。" >&2
  exit 1
fi

rm -rf "$PACKAGE_ROOT"
mkdir -p \
  "$PACKAGE_ROOT/DEBIAN" \
  "$PACKAGE_ROOT/usr/bin" \
  "$PACKAGE_ROOT/usr/share/applications"

install -m 0755 "$BINARY" "$PACKAGE_ROOT/usr/bin/protocol-viewer"
install -m 0644 "$SCRIPT_DIR/protocol-viewer.desktop" \
  "$PACKAGE_ROOT/usr/share/applications/protocol-viewer.desktop"

cat > "$PACKAGE_ROOT/DEBIAN/control" <<EOF
Package: protocol-viewer
Version: $VERSION
Architecture: $ARCH
Maintainer: zerojacks <zerojacks@users.noreply.github.com>
Section: utils
Priority: optional
Depends: libfontconfig1, libfreetype6, libudev1, libx11-6, libxcb1, libxcb-render0, libxcb-shape0, libxcb-xfixes0, libxcb-icccm4, libxcb-keysyms1, libxcb-randr0, libxcb-util1, libxkbcommon0, libxkbcommon-x11-0, libwayland-client0, libvulkan1
Description: Visual viewer for parsed power communication protocols
 Desktop application for inspecting protocol frames and parsed fields.
EOF

mkdir -p "$OUTPUT_DIR"
dpkg-deb --root-owner-group --build "$PACKAGE_ROOT" "$OUTPUT_FILE"
echo "已生成 $OUTPUT_FILE"
