uniffi::setup_scaffolding!();

use relay_viewer_core::{relative_time, Error as CoreError, RelayClient, RelayEvent};
use std::sync::Mutex;
use tokio::runtime::Runtime;

/// FFI error type — maps from core errors to UniFFI-exportable errors.
#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum FfiError {
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Could not connect to relay: {0}")]
    Relay(String),
    #[error("Something went wrong: {0}")]
    Internal(String),
}

impl From<CoreError> for FfiError {
    fn from(e: CoreError) -> Self {
        match e {
            CoreError::NotFound(msg) => FfiError::NotFound(msg),
            CoreError::Relay(msg) => FfiError::Relay(msg),
            other => FfiError::Internal(other.to_string()),
        }
    }
}

/// FFI record for a relay event — includes computed fields for display.
#[derive(uniffi::Record)]
pub struct FfiRelayEvent {
    pub id: String,
    pub pubkey: String,
    pub kind: u32,
    pub content: String,
    pub created_at: i64,
    /// Resolved profile name or truncated npub.
    pub display_name: String,
    /// Human-readable kind name (e.g. "Text Note", "Reaction").
    pub kind_name: String,
    /// Human-readable relative timestamp (e.g. "2m ago", "3h ago").
    pub relative_time: String,
}

impl From<RelayEvent> for FfiRelayEvent {
    fn from(e: RelayEvent) -> Self {
        let rel = relative_time(e.created_at);
        FfiRelayEvent {
            id: e.id,
            pubkey: e.pubkey,
            kind: e.kind,
            content: e.content,
            created_at: e.created_at,
            display_name: e.display_name,
            kind_name: e.kind_name,
            relative_time: rel,
        }
    }
}

/// The main app object exposed to native code via UniFFI.
#[derive(uniffi::Object)]
pub struct AppCore {
    client: Mutex<RelayClient>,
    rt: Runtime,
}

#[uniffi::export]
impl AppCore {
    /// Create a new AppCore. Instant — does NOT connect to relays.
    /// Connection happens lazily on the first `fetch_events` call.
    #[uniffi::constructor]
    pub fn new(relay_url: String, data_dir: String) -> Result<Self, FfiError> {
        let rt = Runtime::new().map_err(|e| FfiError::Internal(e.to_string()))?;
        let client = RelayClient::new(&[relay_url.as_str()], &data_dir)
            .map_err(FfiError::from)?;
        Ok(Self {
            client: Mutex::new(client),
            rt,
        })
    }

    /// Fetch latest events of all kinds from the relay.
    /// Triggers lazy relay connection on first call.
    pub fn fetch_events(&self, limit: u16) -> Result<Vec<FfiRelayEvent>, FfiError> {
        let mut client = self.client.lock().unwrap();
        let events = self
            .rt
            .block_on(client.fetch_events(limit))
            .map_err(FfiError::from)?;
        Ok(events.into_iter().map(FfiRelayEvent::from).collect())
    }

    /// Return cached events from SQLite (no network).
    pub fn cached_events(&self, limit: u32) -> Result<Vec<FfiRelayEvent>, FfiError> {
        let client = self.client.lock().unwrap();
        let events = client.cached_events(limit).map_err(FfiError::from)?;
        Ok(events.into_iter().map(FfiRelayEvent::from).collect())
    }
}
