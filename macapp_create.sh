#!/usr/bin/env bash
set -e

# ========= Rust App Create =========
# ========= My App Setting =========
APP_NAME="FlyRust"
BUNDLE_ID="com.richard.flyrust"
VERSION="1.0.0"
ICON_FILE="icon.icns"

# ========= System Default =========
BUILD_DIR="dist"
APP_DIR="$BUILD_DIR/$APP_NAME.app"

CONTENTS="$APP_DIR/Contents"
MACOS="$CONTENTS/MacOS"
RESOURCES="$CONTENTS/Resources"

# ========= Prepare folders =========
rm -rf "$APP_DIR"
mkdir -p "$MACOS" "$RESOURCES"

# ========= Info.plist =========
cat > "$CONTENTS/Info.plist" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN"
 "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleName</key>
  <string>$APP_NAME</string>

  <key>CFBundleDisplayName</key>
  <string>$APP_NAME</string>

  <key>CFBundleIdentifier</key>
  <string>$BUNDLE_ID</string>

  <key>CFBundleVersion</key>
  <string>$VERSION</string>

  <key>CFBundleShortVersionString</key>
  <string>$VERSION</string>

  <key>CFBundleExecutable</key>
  <string>$APP_NAME</string>

  <key>CFBundlePackageType</key>
  <string>APPL</string>

  <key>CFBundleIconFile</key>
  <string>${ICON_FILE%.icns}</string>

  <key>LSMinimumSystemVersion</key>
  <string>10.13</string>

  <key>NSHighResolutionCapable</key>
  <true/>
</dict>
</plist>
EOF

echo "========= Done ========="
echo "✅ Done → $APP_DIR"
