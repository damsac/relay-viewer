# relay-viewer

RMP (Rust Multi-Platform) app. Rust core -> UniFFI -> native Swift/Kotlin.

Nostr relay event visualizer for wss://nostr.damsac.studio. Shows a live feed of all event kinds with color-coded badges, content previews, and relative timestamps.

## First thing: run agent-brief
```bash
./scripts/agent-brief
```

## Commands (justfile is the surface)
```
just test            # Run all tests
just ios-build       # Build iOS (xcframework + xcodegen)
just android-build   # Build Android (cargo-ndk + gradle)
just pre-merge       # Full CI check locally
```

## Architecture
- `crates/core/`: ALL business logic. Pure Rust. No platform deps.
- `crates/ffi/`: UniFFI bindings. Thin wrappers around core types.
- `ios/`: SwiftUI app. Thin UI shell. Renders state from Rust.
- `android/`: Kotlin/Compose app. Thin UI shell. Renders state from Rust.
- State flows unidirectionally: Native -> Action -> Rust -> State -> Native renders.

## Key files
- `justfile`: command surface (start here)
- `scripts/`: build tooling (don't modify without reading docs/)
- `invariants/invariants.toml`: architecture rules (machine-checkable)
- `docs/`: read_when frontmatter tells you which docs matter for your task
