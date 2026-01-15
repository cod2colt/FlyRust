#!/bin/zsh
# macOS APP Build and Package
# Copy and overwrite Contests/Resources/
#
# Usage:
#   ./macapp_build.sh <AppName>
#
# Example:
#   ./macapp_build.sh FlyRust
echo "macOS APP Build and Package"

set -e

APP_NAME="$1"

if [[ -z "$APP_NAME" ]]; then
  echo "Usage: $0 <AppName>"
  exit 1
fi

APP_BUNDLE="${APP_NAME}.app"

echo "Copy Resources..."
BUILD_DIR="dist"
rm -rf "${BUILD_DIR}/${APP_BUNDLE}/Contents/Resources/assets/"
cp -Rf Contents/Resources/assets/. "${BUILD_DIR}/${APP_BUNDLE}/Contents/Resources/assets"

echo "Build release binary..."
cargo build --release

echo "Copy binary to app bundle..."
cp "target/release/${APP_NAME}" "${BUILD_DIR}/${APP_BUNDLE}/Contents/MacOS/${APP_NAME}"

echo "Done! 🎉"
