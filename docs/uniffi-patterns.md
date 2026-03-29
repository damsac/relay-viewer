---
summary: UniFFI FFI layer patterns — Ffi* wrapper types, error mapping, Object facade
read_when:
  - adding new types to the FFI boundary
  - debugging binding generation
  - understanding the Ffi* wrapper pattern
---

# UniFFI Patterns

## The Ffi* Wrapper Pattern

Don't expose core types directly through UniFFI. Instead, create `Ffi*` wrapper types in `crates/ffi/` that map between Rust's type system and what UniFFI can express.

Why: UniFFI has limitations (no generics, limited enum support, no lifetimes). The wrapper layer absorbs these constraints so the core crate stays idiomatic Rust.

## Example

```rust
// crates/core/src/models.rs — pure Rust, no UniFFI concerns
pub struct Trail {
    pub id: Uuid,
    pub name: String,
    pub points: Vec<TrackPoint>,
}

// crates/ffi/src/lib.rs — UniFFI-friendly wrappers
#[derive(uniffi::Record)]
pub struct FfiTrail {
    pub id: String,       // Uuid → String for FFI
    pub name: String,
    pub points: Vec<FfiTrackPoint>,
}

impl From<Trail> for FfiTrail { ... }
impl TryFrom<FfiTrail> for Trail { ... }
```

## Error Mapping

Define an `FfiError` enum in the ffi crate that maps from your core error type:

```rust
#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum FfiError {
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<CoreError> for FfiError { ... }
```

## The Object Facade

Expose a single `#[uniffi::Object]` that wraps your core state. Native code interacts with this object — it's the entry point for all operations.

```rust
#[derive(uniffi::Object)]
pub struct AppCore {
    store: Mutex<Store>,
}

#[uniffi::export]
impl AppCore {
    #[uniffi::constructor]
    pub fn new(data_dir: String) -> Result<Self, FfiError> { ... }
    pub fn create_item(&self, input: FfiCreateInput) -> Result<FfiItem, FfiError> { ... }
}
```

## UniFFI Version

RMP uses UniFFI 0.31. Pin the exact version in both `crates/ffi/Cargo.toml` and `crates/uniffi-bindgen/Cargo.toml`.
