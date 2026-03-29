---
summary: Android cross-compilation — cargo-ndk, UniFFI Kotlin bindings, Gradle integration
read_when:
  - debugging Android build failures
  - modifying the Android cross-compile pipeline
  - understanding how Kotlin bindings are generated
---

# Android Build

## Overview

`scripts/android-build` cross-compiles the FFI crate for Android via `cargo-ndk`, generates Kotlin bindings via UniFFI, and places the shared libraries where Gradle expects them.

## Targets

- `aarch64-linux-android` (arm64-v8a) — most modern devices
- `armv7-linux-androideabi` (armeabi-v7a) — older 32-bit devices
- `x86_64-linux-android` (x86_64) — emulators

## Build Pipeline

1. Cross-compile via `cargo ndk -t arm64-v8a -t armeabi-v7a -t x86_64 build -p <ffi-crate>`
2. Run `uniffi-bindgen generate` → Kotlin bindings
3. Copy `.so` files to `android/app/src/main/jniLibs/<arch>/`

## Gradle Integration

The Android app uses JNA (Java Native Access) to load the shared library. The `app/build.gradle.kts` includes an `ensureUniffiGenerated` task that fails the build if Kotlin bindings are missing — this prevents building with stale bindings.

## NDK Version

RMP templates use NDK 28. This is configured in the Nix flake and should match what's specified in `android/app/build.gradle.kts`.
