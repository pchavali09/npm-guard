#!/bin/bash

if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
  echo "❌ Windows is not supported in V1. Please use WSL or contribute a PowerShell adapter!"
  exit 1
fi

BINARY_NAME="npm-guard"
INSTALL_DIR="$HOME/.npm-guard/bin"

# --- 1. ROBUST SHELL DETECTION ---
# Check the user's configured login shell, not just the running process
CURRENT_SHELL=$(basename "$SHELL")

if [ "$CURRENT_SHELL" = "zsh" ]; then
    PROFILE_FILE="$HOME/.zshrc"
elif [ "$CURRENT_SHELL" = "bash" ]; then
    PROFILE_FILE="$HOME/.bashrc"
else
    # Fallback: Try to guess based on file existence
    if [ -f "$HOME/.zshrc" ]; then
        PROFILE_FILE="$HOME/.zshrc"
    elif [ -f "$HOME/.bashrc" ]; then
        PROFILE_FILE="$HOME/.bashrc"
    else
        echo "⚠️  Could not detect shell config. Please add alias manually."
        exit 1
    fi
fi

echo "🏗️  Installing $BINARY_NAME..."

# ... (Rest of the script: Cargo build, copy binary, etc.) ...
# Copy the compilation and shim logic from the previous version here
# Or I can provide the full file if you prefer.

echo "⚙️  Compiling from source (Rust)..."
cargo build --release --quiet

mkdir -p "$INSTALL_DIR"
cp "target/release/$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"

SHIM_ENTRY="
# --- npm-guard shim ---
npm() {
    $INSTALL_DIR/$BINARY_NAME \"\$@\"
}
"

if grep -q "npm-guard shim" "$PROFILE_FILE"; then
    echo "✅ Shim already exists in $PROFILE_FILE"
else
    echo "$SHIM_ENTRY" >> "$PROFILE_FILE"
    echo "✅ Added shim to $PROFILE_FILE"
fi

echo "🚀 Installation complete!"
echo "👉 Run 'source $PROFILE_FILE' or restart your terminal."