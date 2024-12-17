#!/bin/bash
set -e

PACKAGE="target/debian.deb"

if [ ! -f $PACKAGE ]; then
    echo "Package not found! Run ./scripts/package.sh first."
    exit 1
fi

# Install the package
sudo dpkg -i $PACKAGE

# Start the service
sudo systemctl daemon-reload
sudo systemctl enable rust_web_server
sudo systemctl start rust_web_server

echo "Rust Web Server is now running!"
