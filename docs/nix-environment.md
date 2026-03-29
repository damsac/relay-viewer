---
summary: Nix dev shell — Rust toolchain, Android SDK/NDK, cargo-ndk, just, xcodegen
read_when:
  - modifying flake.nix
  - debugging Nix dev shell issues
  - adding dependencies to the dev environment
---

# Nix Environment

## Overview

RMP uses a Nix flake for reproducible dev environments. `nix develop` gives you everything needed to build for all platforms.

## What's Included

- **Rust**: stable toolchain via rust-overlay, with cross-compile targets for iOS (aarch64-device, aarch64-sim) and Android (aarch64, armv7, x86_64)
- **Android SDK**: cmdline-tools, platform-tools, build-tools 34+35, platforms 34+35, NDK 28
- **Tools**: cargo-ndk, just, jdk17, xcodegen (macOS only)
- **Shell hook**: sets ANDROID_HOME, ANDROID_NDK_HOME, JAVA_HOME, generates android/local.properties

## macOS vs Linux

On macOS: full iOS + Android development. XcodeGen is included. You need Xcode installed separately.
On Linux: Rust + Android development only. No iOS builds (requires Xcode/macOS).

## Customizing

The flake is parameterized. The project name is a variable at the top — change it when scaffolding a new app. Add extra Nix packages to `buildInputs` as needed.
