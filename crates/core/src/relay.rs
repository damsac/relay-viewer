use nostr_sdk::prelude::*;
use std::collections::HashMap;
use std::time::Duration;

use crate::error::Error;
use crate::models::{kind_name, truncated_npub, RelayEvent};
use crate::store::Store;

pub struct RelayClient {
    client: Client,
    store: Store,
}

fn event_to_relay_event(event: &Event, display_name: &str) -> RelayEvent {
    let kind_num = event.kind.as_u16() as u32;
    RelayEvent {
        id: event.id.to_hex(),
        pubkey: event.pubkey.to_hex(),
        kind: kind_num,
        content: event.content.to_string(),
        created_at: event.created_at.as_u64() as i64,
        display_name: display_name.to_string(),
        kind_name: kind_name(kind_num),
    }
}

impl RelayClient {
    /// Create a new relay client connected to our damsac relay.
    pub async fn new(relay_url: &str, data_dir: &str) -> Result<Self, Error> {
        let store = Store::new(data_dir)?;
        let client = Client::default();

        if let Err(e) = client.add_relay(relay_url).await {
            log::warn!("failed to add relay {}: {}", relay_url, e);
        }

        client.connect().await;
        Ok(Self { client, store })
    }

    /// Resolve display names for a set of pubkeys by fetching kind 0 metadata.
    async fn resolve_display_names(&self, pubkeys: &[PublicKey]) -> HashMap<String, String> {
        let mut names: HashMap<String, String> = HashMap::new();
        if pubkeys.is_empty() {
            return names;
        }

        let filter = Filter::new()
            .authors(pubkeys.iter().copied())
            .kind(Kind::Metadata)
            .limit(pubkeys.len());

        if let Ok(events) = self
            .client
            .fetch_events(filter, Duration::from_secs(5))
            .await
        {
            let mut best: HashMap<String, &Event> = HashMap::new();
            for event in events.iter() {
                let hex = event.pubkey.to_hex();
                let is_newer = best
                    .get(&hex)
                    .is_none_or(|prev| event.created_at > prev.created_at);
                if is_newer {
                    best.insert(hex, event);
                }
            }
            for (hex, event) in &best {
                if let Ok(meta) = Metadata::try_from(*event) {
                    let name = meta
                        .display_name
                        .or(meta.name)
                        .unwrap_or_else(|| truncated_npub(hex));
                    names.insert(hex.clone(), name);
                }
            }
        }

        for pk in pubkeys {
            let hex = pk.to_hex();
            names
                .entry(hex.clone())
                .or_insert_with(|| truncated_npub(&hex));
        }

        names
    }

    /// Fetch recent events of ALL kinds from the relay.
    pub async fn fetch_events(&self, limit: u16) -> Result<Vec<RelayEvent>, Error> {
        // No kind filter — fetch all event kinds
        let filter = Filter::new().limit(limit as usize);

        let events = self
            .client
            .fetch_events(filter, Duration::from_secs(10))
            .await
            .map_err(|e| Error::Relay(e.to_string()))?;

        // Collect unique pubkeys and resolve display names.
        let pubkeys: Vec<PublicKey> = events
            .iter()
            .map(|e| e.pubkey)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        let names = self.resolve_display_names(&pubkeys).await;

        let mut relay_events = Vec::new();
        for event in events.iter() {
            let hex = event.pubkey.to_hex();
            let display = names
                .get(&hex)
                .cloned()
                .unwrap_or_else(|| truncated_npub(&hex));
            let relay_event = event_to_relay_event(event, &display);
            self.store.upsert_event(&relay_event)?;
            relay_events.push(relay_event);
        }

        Ok(relay_events)
    }

    /// Return cached events from SQLite.
    pub fn cached_events(&self, limit: u32) -> Result<Vec<RelayEvent>, Error> {
        self.store.list_events(limit)
    }

    pub async fn disconnect(&self) {
        self.client.disconnect().await;
    }
}
