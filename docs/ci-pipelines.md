---
summary: GitHub Actions CI — Rust tests, iOS full pipeline, Android build
read_when:
  - modifying CI workflows
  - debugging CI failures
  - adding new CI checks
---

# CI Pipelines

## Workflows

### rust.yml
Triggers on push to main + PRs. Two jobs:
- **test**: `cargo test --workspace`
- **lint**: `cargo fmt --all --check` + `cargo clippy --workspace -- -D warnings`

### ios.yml
Triggers on push to main + PRs. Single job on macos-15:
1. Install Rust stable with iOS targets
2. Install xcodegen + xcbeautify via Homebrew
3. Run `scripts/ios-build --release`
4. Verify committed bindings are up-to-date (`git diff --exit-code ios/Bindings/`)
5. Generate Xcode project (`xcodegen generate`)
6. Resolve SPM dependencies
7. Build for iOS Simulator
8. Run tests on simulator
9. Upload XCFramework artifact (5-day retention)

### android.yml
Triggers on push to main + PRs. Single job on ubuntu-latest:
1. Install Rust stable with Android targets
2. Install cargo-ndk
3. Set up JDK 17
4. Run `scripts/android-build`
5. Run `cd android && ./gradlew assembleDebug`
6. Upload APK artifact (5-day retention)

## Adding Checks

New checks go in the appropriate workflow. If a check applies to Rust code only, add it to rust.yml. If it requires platform tooling, add it to the platform-specific workflow.
