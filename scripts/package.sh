#!/bin/bash
set -e

# Step 1: Build Rust project
echo "Building project..."
cargo build --release

# Step 2: Create Debian directory structure
echo "Setting up package directory..."
PACKAGE_DIR=target/debian
mkdir -p ${PACKAGE_DIR}/DEBIAN
mkdir -p ${PACKAGE_DIR}/usr/local/bin
mkdir -p ${PACKAGE_DIR}/etc/systemd/system

# Step 3: Copy files
cp debian/DEBIAN/control ${PACKAGE_DIR}/DEBIAN/control
cp debian/usr/etc/systemd/system/rust_web_server.service ${PACKAGE_DIR}/etc/systemd/system/
cp target/release/rust_web_server ${PACKAGE_DIR}/usr/local/bin/

# Step 4: Build the .deb package
echo "Building .deb package..."
dpkg-deb --build ${PACKAGE_DIR}

echo "Package created: target/debian.deb"
