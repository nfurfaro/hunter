#!/bin/bash
s
# Define the version
version="1.0.0"

# Download the binary using cURL
curl -LO https://github.com/nfurfaro/hunter/releases/download/$version/hunter

# Make the binary executable
chmod +x hunter

# Move the binary to a directory in your PATH
mv hunter /usr/local/bin

# Confirm installation
if command -v hunter &> /dev/null
then
    echo "Hunter was installed successfully."
else
    echo "Hunter could not be installed."
fi