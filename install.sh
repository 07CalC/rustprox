#!/bin/bash
set -e

BINARY_NAME="rustprox"
VERSION="v1.0"
OWNER="07calc"
REPO="rustprox"

OS=$(uname -s)
case "$OS" in
  Linux) PLATFORM="linux" ;;
  Darwin) PLATFORM="macos" ;;
  *) echo "❌ Unsupported OS: $OS"; exit 1 ;;
esac

FILENAME="${BINARY_NAME}-${PLATFORM}"
DOWNLOAD_URL="https://github.com/${OWNER}/${REPO}/releases/download/${VERSION}/${FILENAME}"
echo "⬇️ Downloading $BINARY_NAME for $PLATFORM..."
curl -fsSL "$DOWNLOAD_URL" -o "$BINARY_NAME"

echo "🚀 Installing..."
chmod +x "$BINARY_NAME"
sudo mv "$BINARY_NAME" /usr/local/bin/

echo "✅ Installed $BINARY_NAME to /usr/local/bin!"
