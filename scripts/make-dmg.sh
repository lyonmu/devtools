#!/bin/bash
set -euo pipefail

APP_NAME="DevTools"
VOLUME_NAME="DevTools"
BUNDLE_DIR="target/release/bundle/osx"
APP_PATH="${BUNDLE_DIR}/${APP_NAME}.app"
DMG_PATH="${BUNDLE_DIR}/${APP_NAME}.dmg"

cleanup() {
    if [[ -n "${STAGING_DIR:-}" && -d "${STAGING_DIR}" ]]; then
        rm -rf "${STAGING_DIR}"
    fi
}
trap cleanup EXIT

echo "Building ${APP_NAME}.app bundle..."

if [[ "$(uname)" != "Darwin" ]]; then
    echo "Error: macOS DMG packaging requires macOS (hdiutil is macOS-only)."
    exit 1
fi

for TOOL in cargo hdiutil ditto; do
    if ! command -v "${TOOL}" >/dev/null 2>&1; then
        echo "Error: Required tool '${TOOL}' not found."
        exit 1
    fi
done

if ! cargo bundle --version >/dev/null 2>&1; then
    echo "Error: cargo-bundle is not installed. Run: cargo install cargo-bundle"
    exit 1
fi

if [[ ! -f "icons/icon.icns" ]]; then
    echo "icons/icon.icns not found; generating it from chip.png..."
    ./scripts/make-icons.sh
fi

cargo bundle --release

if [[ ! -d "${APP_PATH}" ]]; then
    echo "Error: Expected app bundle not found: ${APP_PATH}"
    exit 1
fi

STAGING_DIR="$(mktemp -d /tmp/devtools-dmg-stage.XXXXXX)"
ditto "${APP_PATH}" "${STAGING_DIR}/${APP_NAME}.app"
ln -s /Applications "${STAGING_DIR}/Applications"

mkdir -p "${BUNDLE_DIR}"
echo "Creating ${DMG_PATH} with drag-to-Applications install layout..."
hdiutil create \
    -volname "${VOLUME_NAME}" \
    -srcfolder "${STAGING_DIR}" \
    -ov \
    -format UDZO \
    "${DMG_PATH}"

echo "Success! Generated ${DMG_PATH}"
echo "Open it and drag ${APP_NAME}.app onto the Applications shortcut to install."
