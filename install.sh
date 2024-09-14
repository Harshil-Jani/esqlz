#!/bin/bash

# Define variables
VERSION="v1.0.0"
BINARY_NAME="esqlz"
GITHUB_URL="https://github.com/Harshil-Jani/esqlz/releases/download/$VERSION/$BINARY_NAME"

# Download the binary
echo "Downloading $BINARY_NAME..."
curl -sSL "$GITHUB_URL" -o "$BINARY_NAME"

# Make the binary executable
chmod +x "$BINARY_NAME"

# Move the binary to /usr/local/bin for global usage
echo "Installing $BINARY_NAME to /usr/local/bin..."
sudo mv "$BINARY_NAME" /usr/local/bin/

# Verify the installation
if command -v $BINARY_NAME >/dev/null 2>&1; then
    echo "$BINARY_NAME has been installed successfully!"
else
    echo "Installation failed. Please try again. If the issue persists, please open an issue at https://github.com/Harshil-Jani/esqlz/issues"
fi
