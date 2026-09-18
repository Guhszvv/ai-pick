#!/usr/bin/env bash

set -euo pipefail

REPO="Guhszvv/ai-pick"
INSTALL_DIR="${HOME}/.local/bin"

require() {
  command -v "$1" >/dev/null 2>&1 || {
    echo "error: '$1' is required."
    exit 1
  }
}

require curl
require tar

OS="$(uname -s | tr '[:upper:]' '[:lower:]')"

ARCH="$(uname -m)"
case "$ARCH" in
x86_64) ARCH="x86_64" ;;
aarch64 | arm64) ARCH="aarch64" ;;
*)
  echo "Unsupported architecture: $ARCH"
  exit 1
  ;;
esac

ASSET="ai-pick-${OS}-${ARCH}.tar.gz"

echo "Fetching latest release..."

URL=$(
  curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" |
    grep browser_download_url |
    grep "$ASSET" |
    cut -d '"' -f 4
)

if [ -z "$URL" ]; then
  echo "Could not find asset: $ASSET"
  exit 1
fi

TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

echo "Downloading..."

curl -fsSL "$URL" -o "$TMP/ai-pick.tar.gz"

echo "Installing..."

# Validate the downloaded file is a valid gzip archive
if [[ ! -s "$TMP/ai-pick.tar.gz" ]]; then
  echo "error: downloaded file is empty or missing"
  exit 1
fi

file "$TMP/ai-pick.tar.gz" | grep -q "gzip" || {
  echo "error: downloaded file is not a valid gzip archive"
  exit 1
}

mkdir -p "$INSTALL_DIR"

tar -xzf "$TMP/ai-pick.tar.gz" -C "$TMP"

cp "$TMP/ai-pick" "$INSTALL_DIR/ai-pick"
chmod 755 "$INSTALL_DIR/ai-pick"

echo
echo "✓ ai-pick installed successfully."

case ":$PATH:" in
*":$INSTALL_DIR:"*) ;;
*)
  echo
  echo "Add this to your shell config:"
  echo
  echo "export PATH=\"$INSTALL_DIR:\$PATH\""
  ;;
esac
