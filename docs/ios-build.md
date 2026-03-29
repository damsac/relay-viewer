---
summary: iOS cross-compilation — xcframework creation, Nix env fixup, UniFFI Swift bindings
read_when:
  - debugging iOS build failures
  - modifying the iOS cross-compile pipeline
  - understanding how Swift bindings are generated
---

# iOS Build

## Overview

`scripts/ios-build` cross-compiles the FFI crate for iOS, generates Swift bindings via UniFFI, and packages everything into an XCFramework.

## Targets

- `aarch64-apple-ios` — physical devices (iPhone, iPad)
- `aarch64-apple-ios-sim` — Apple Silicon simulator

## Nix Environment Fixup

When running inside a Nix dev shell on macOS, several environment variables interfere with iOS cross-compilation. The build script detects Nix and unsets: `SDKROOT`, `NIX_CFLAGS_COMPILE`, `NIX_LDFLAGS`, `MACOSX_DEPLOYMENT_TARGET`. It re-points `CC` to `/usr/bin/clang` (the system Xcode toolchain, not Nix's).

This is necessary because Nix's clang targets macOS, not iOS. The Rust compiler needs the real Xcode SDK for iOS targets.

## Build Pipeline

1. Cross-compile `crates/ffi` for both iOS targets
2. Build host library (for UniFFI metadata extraction)
3. Run `uniffi-bindgen generate` → `ios/Bindings/` (Swift + C header + modulemap)
4. Create XCFramework via `xcodebuild -create-xcframework`

## XcodeGen

The iOS project is generated from `ios/project.yml` using XcodeGen. The generated `.xcodeproj` is gitignored. Run `xcodegen generate` after `ios-build` to get a buildable Xcode project.

## Committed Bindings

Swift bindings in `ios/Bindings/` are committed to the repo. CI verifies they're up-to-date by running the build and checking `git diff`. This means iOS developers can build without having Rust installed — they just need the pre-built XCFramework.
