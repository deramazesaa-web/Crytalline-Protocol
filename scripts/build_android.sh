#!/bin/bash
set -e

# Define project name based on Cargo.toml
PROJECT_NAME="crystalline_protocol"

# Create the output directory structure for Android Studio
mkdir -p target/android/jniLibs/arm64-v8a
mkdir -p target/android/jniLibs/armeabi-v7a
mkdir -p target/android/jniLibs/x86_64

echo "--- [CRYSTALLINE PROTOCOL] Starting Android Build Sequence ---"

# 1. Build for ARM64 (Modern Android Phones)
echo "Building for aarch64 (arm64-v8a)..."
cargo ndk -t aarch64-linux-android build --release --lib
cp target/aarch64-linux-android/release/lib$PROJECT_NAME.so target/android/jniLibs/arm64-v8a/

# 2. Build for ARMv7 (Older Android Phones)
echo "Building for armv7 (armeabi-v7a)..."
cargo ndk -t armv7-linux-androideabi build --release --lib
cp target/armv7-linux-androideabi/release/lib$PROJECT_NAME.so target/android/jniLibs/armeabi-v7a/

# 3. Build for x86_64 (Android Emulator)
echo "Building for x86_64 (emulator)..."
cargo ndk -t x86_64-linux-android build --release --lib
cp target/x86_64-linux-android/release/lib$PROJECT_NAME.so target/android/jniLibs/x86_64/

echo "--- [SUCCESS] Android Libraries Generated ---"
echo "Location: target/android/jniLibs/"
ls -R target/android/jniLibs/
