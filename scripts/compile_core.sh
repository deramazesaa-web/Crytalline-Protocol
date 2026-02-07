#!/bin/bash
set -e

PROJECT_NAME="crystalline_protocol"

echo "--- [CRYSTALLINE CORE] Starting Multiplatform Build ---"

# --- TARGET: ANDROID ---
echo "Target: Android (aarch64, armv7, x86_64)..."
cargo ndk -t aarch64-linux-android -t armv7-linux-androideabi -t x86_64-linux-android -o ./target/android/jniLibs build --release

# --- TARGET: GENERIC LINUX (For Desktop/Servers) ---
echo "Target: Generic Linux x64..."
cargo build --release --lib
mkdir -p target/linux/
cp target/release/lib$PROJECT_NAME.so target/linux/ 2>/dev/null || cp target/release/lib$PROJECT_NAME.a target/linux/

# --- TARGET: FUTURE (Placeholder for iOS/Embedded) ---
# echo "Target: iOS/Embedded..."
# To be implemented with respective toolchains

echo "--- [SUCCESS] All targets compiled to ./target/ ---"
