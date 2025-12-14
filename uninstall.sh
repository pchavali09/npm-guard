#!/bin/bash

BINARY_NAME="npm-guard"
INSTALL_DIR="$HOME/.npm-guard"

# 1. Detect Shell (Same logic as installer)
CURRENT_SHELL=$(basename "$SHELL")
if [ "$CURRENT_SHELL" = "zsh" ]; then
    PROFILE_FILE="$HOME/.zshrc"
elif [ "$CURRENT_SHELL" = "bash" ]; then
    PROFILE_FILE="$HOME/.bashrc"
else
    # Fallback
    if [ -f "$HOME/.zshrc" ]; then PROFILE_FILE="$HOME/.zshrc"; 
    else PROFILE_FILE="$HOME/.bashrc"; fi
fi

echo "🗑️  Uninstalling $BINARY_NAME..."

# 2. Remove Binary and Dir
rm -rf "$INSTALL_DIR"
echo "✅ Removed binary files."

# 3. Clean the Shell Config (Sed is tricky across Mac/Linux, this is safe)
# We create a backup first
cp "$PROFILE_FILE" "$PROFILE_FILE.bak"
echo "ℹ️  Backup created at $PROFILE_FILE.bak"

# Remove the shim block. 
# Note: This simple grep/sed assumes the user hasn't modified the shim manually.
# A safer way for MVPs is asking the user to remove the lines, but let's try auto-removal.

# We will just tell the user to check their file to be safe from regex disasters.
echo "⚠️  ACTION REQUIRED: Please open $PROFILE_FILE and remove the 'npm-guard shim' block manually."
echo "   (Automatic config editing is risky and we value your config safety)."

echo "👋 Uninstalled successfully."