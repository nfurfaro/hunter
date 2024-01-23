#!/bin/bash

# Define the version
version="v1.0.0"

# List the available binaries
echo "Please select your binary:"
echo "1) hunter_x86_64-unknown-linux-gnu"
echo "2) hunter_aarch64_apple_darwin"
read -p "Selection: " selection

# Set the binary name based on the selection
case $selection in
    1) binary="hunter_x86_64-unknown-linux-gnu";;
    2) binary="hunter_aarch64_apple_darwin";;
    *) echo "Invalid selection"; exit 1;;
esac

# Download the binary using cURL
if curl -LO https://github.com/nfurfaro/hunter/releases/download/$version/$binary; then
    echo "Binary downloaded successfully."
else
    echo "Failed to download binary."
    exit 1
fi

# Make the binary executable
if chmod +x $binary; then
    echo "Binary made executable."
else
    echo "Failed to make binary executable."
    exit 1
fi