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
curl -LO https://github.com/nfurfaro/hunter/releases/download/$version/$binary

# Make the binary executable
chmod +x $binary

# Move the binary to a directory in your PATH
# mv $binary /usr/local/bin/hunter

# Confirm installation
if command -v hunter &> /dev/null
then
    echo "Hunter was installed successfully."
else
    echo "Hunter could not be installed."
fi