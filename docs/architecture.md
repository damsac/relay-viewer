---
summary: RMP architecture — Rust core, UniFFI bindings, native iOS/Android shells
read_when:
  - starting a new RMP app
  - understanding the cross-platform pattern
  - onboarding to an existing RMP project
---

# RMP Architecture

## The Pattern

RMP apps follow a strict layered architecture:

```
┌─────────────┐  ┌──────────────────┐
│  iOS (Swift) │  │ Android (Kotlin) │   ← Thin UI shells
└──────┬───────┘  └────────┬─────────┘
       │    UniFFI bindings │
       └────────┬───────────┘
         ┌──────┴──────┐
         │  Rust Core  │                  ← ALL business logic
         └─────────────┘
```

**Rust Core** (`crates/core/`): All business logic, data models, state management, persistence. Pure Rust — no platform-specific code. This is where tests live for domain logic.

**FFI Layer** (`crates/ffi/`): UniFFI scaffolding. Defines `Ffi*` wrapper types that bridge between Rust's type system and what UniFFI can export. Thin — no logic, just type conversion.

**Native Shells** (`ios/`, `android/`): SwiftUI and Kotlin/Compose apps that render state from Rust and dispatch actions back. No business logic. The native layer handles: UI rendering, platform APIs (GPS, camera, notifications), and lifecycle management.

## State Flow

Unidirectional, inspired by the Ferrostar pattern:

1. Native UI dispatches an **Action** (user tap, GPS update, timer tick)
2. Action crosses the FFI boundary into Rust
3. Rust core processes the action, updates internal state
4. Rust returns new **State** snapshot to native
5. Native renders the new state

State never flows from native → Rust → native in a cycle. The Rust core is the single source of truth.

## Why This Pattern

- **One codebase for logic**: Write business logic once in Rust, test it thoroughly, share it across platforms.
- **Native UI**: SwiftUI and Compose feel native because they ARE native. No cross-platform UI toolkit.
- **Agent-friendly**: Agents work in one language (Rust) for logic, with thin platform adapters. The architecture invariants (invariants.toml) enforce this boundary.
- **Testable**: Core logic is pure Rust with no platform deps — tests run on any platform, fast, in CI.
