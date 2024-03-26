#!/bin/bash

# cllm Installation Script (Integrated)

# cllm download URL and file name
CLLM_URL=""
CLLM_FILE="cllm.tar.gz"

# Installation directory
INSTALL_DIR="/usr/local/bin"

# User-defined temporary directory
TMP_DIR="/tmp/cllm"

# Check the operating system
OS=$(uname -s)

# Download and install cllm files based on the operating system
case "$OS" in
    Linux*)
        # cllm download URL and file name for Linux
        CLLM_URL="https://github.com/dev-backpack/cllm/releases/download/v0.1.2/cllm-x86_64-unknown-linux-gnu.tar.gz"

        # Install dependencies for cli-clipboard
        sudo apt install xorg-dev libxcb-composite0-dev
        ;;
    Darwin*)
        # cllm download URL and file name for macOS
        CLLM_URL="https://github.com/dev-backpack/cllm/releases/download/v0.1.2/cllm-x86_64-apple-darwin.tar.gz"
        ;;
    *)
        echo "Unsupported operating system."
        exit 1
        ;;
esac

# Download and install cllm
mkdir -p "$TMP_DIR"
chmod 755 $TMP_DIR
echo "Downloading cllm..."
curl -sfL "$CLLM_URL" -o "$TMP_DIR/$CLLM_FILE"
echo "Extracting cllm files..."
tar xzf "$TMP_DIR/$CLLM_FILE" -C "$TMP_DIR"
sudo chown root "$TMP_DIR/$CLLM_FILE"
sudo mv "$TMP_DIR"/cllm "$INSTALL_DIR"

# Display installation completion message
echo "cllm installation completed!"

# Delete temporary files
rm -rf "$TMP_DIR"
