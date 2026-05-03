#!/bin/bash
set -e

echo "=== taskbar macOS Setup ==="
echo ""

# Check Xcode Command Line Tools
if ! xcode-select -p &>/dev/null; then
  echo "Installing Xcode Command Line Tools..."
  xcode-select --install
  echo "Please restart this script after Xcode installation completes."
  exit 1
fi
echo "Xcode CLI tools: OK"

# Check Rust
if ! command -v cargo &>/dev/null; then
  echo "Installing Rust..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  source "$HOME/.cargo/env"
fi
echo "Rust: $(cargo --version)"

# Check Node.js
if ! command -v node &>/dev/null; then
  echo "Installing Node.js via Homebrew..."
  if ! command -v brew &>/dev/null; then
    echo "Installing Homebrew first..."
    /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
    eval "$(/opt/homebrew/bin/brew shellenv)" 2>/dev/null || eval "$(/usr/local/bin/brew shellenv)" 2>/dev/null || true
  fi
  brew install node@22
  brew link node@22 2>/dev/null || true
fi
echo "Node: $(node --version)"
echo "npm: $(npm --version)"

# Navigate to project
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR"

# Install npm dependencies
echo ""
echo "Installing npm dependencies..."
npm install

# Build DMG
echo ""
echo "Building taskbar.dmg (this takes a few minutes)..."
npm run tauri build

# Find the DMG
DMG_PATH=$(find src-tauri/target/release/bundle/dmg -name "*.dmg" 2>/dev/null | head -1)

if [ -n "$DMG_PATH" ]; then
  echo ""
  echo "=== Build successful! ==="
  echo "DMG: $DMG_PATH"
  echo ""
  echo "Open it with:"
  echo "  open \"$(dirname "$DMG_PATH")\""
else
  echo ""
  echo "=== Build completed but no DMG found ==="
  echo "Check src-tauri/target/release/bundle/ for other bundle formats."
fi