#!/bin/bash
set -e

echo "Generating macOS .icns icon from chip.png..."

# Check if chip.png exists
if [ ! -f "chip.png" ]; then
    echo "Error: chip.png not found in project root!"
    exit 1
fi

# Create icons directory
mkdir -p icons

# Create iconset directory
ICONSET_DIR="icons/icon.iconset"
rm -rf "$ICONSET_DIR"
mkdir -p "$ICONSET_DIR"

# Generate different sizes using sips
SIZES=(16 32 64 128 256 512 1024)
for SIZE in "${SIZES[@]}"; do
    sips -z "$SIZE" "$SIZE" chip.png --out "$ICONSET_DIR/icon_${SIZE}x${SIZE}.png" 2>/dev/null || true
    # Also create @2x versions for retina
    if [ "$SIZE" -le 512 ]; then
        RETINA_SIZE=$((SIZE * 2))
        sips -z "$RETINA_SIZE" "$RETINA_SIZE" chip.png --out "$ICONSET_DIR/icon_${SIZE}x${SIZE}@2x.png" 2>/dev/null || true
    fi
done

# Convert to .icns
iconutil -c icns "$ICONSET_DIR" -o "icons/icon.icns"

# Cleanup
rm -rf "$ICONSET_DIR"

echo "Success! Generated icons/icon.icns"
