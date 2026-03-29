use serde::{Deserialize, Serialize};

/// A Nostr relay event with resolved display metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelayEvent {
    pub id: String,
    pub pubkey: String,
    pub kind: u32,
    pub content: String,
    pub created_at: i64,
    /// Resolved profile name or truncated npub.
    pub display_name: String,
    /// Human-readable kind name (e.g. "Text Note", "Reaction").
    pub kind_name: String,
}

/// Map a Nostr event kind number to a human-readable name.
pub fn kind_name(kind: u32) -> String {
    match kind {
        0 => "Metadata".to_string(),
        1 => "Text Note".to_string(),
        3 => "Contact List".to_string(),
        4 => "DM".to_string(),
        5 => "Delete".to_string(),
        6 => "Repost".to_string(),
        7 => "Reaction".to_string(),
        9735 => "Zap".to_string(),
        30023 => "Long-form".to_string(),
        n => format!("Kind {n}"),
    }
}

/// Build a short display name from a hex pubkey by encoding it as npub and
/// truncating to `npub1xxxx...xxxx` (first 10 + last 4 chars).
pub fn truncated_npub(hex: &str) -> String {
    use nostr_sdk::prelude::*;
    match PublicKey::from_hex(hex) {
        Ok(pk) => match pk.to_bech32() {
            Ok(npub) => {
                if npub.len() > 16 {
                    format!("{}...{}", &npub[..10], &npub[npub.len() - 4..])
                } else {
                    npub
                }
            }
            Err(_) => format!("{}...", &hex[..hex.len().min(12)]),
        },
        Err(_) => format!("{}...", &hex[..hex.len().min(12)]),
    }
}

/// Compute a human-readable relative timestamp string (e.g. "2m ago", "3h ago").
pub fn relative_time(unix_secs: i64) -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;
    let delta = now - unix_secs;
    if delta < 0 {
        return "just now".to_string();
    }
    let minutes = delta / 60;
    let hours = minutes / 60;
    let days = hours / 24;
    match () {
        _ if minutes < 1 => "just now".to_string(),
        _ if minutes < 60 => format!("{minutes}m ago"),
        _ if hours < 24 => format!("{hours}h ago"),
        _ if days < 30 => format!("{days}d ago"),
        _ => {
            let months = days / 30;
            if months < 12 {
                format!("{months}mo ago")
            } else {
                format!("{}y ago", days / 365)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kind_name_known() {
        assert_eq!(kind_name(0), "Metadata");
        assert_eq!(kind_name(1), "Text Note");
        assert_eq!(kind_name(7), "Reaction");
        assert_eq!(kind_name(9735), "Zap");
    }

    #[test]
    fn test_kind_name_unknown() {
        assert_eq!(kind_name(42), "Kind 42");
        assert_eq!(kind_name(10002), "Kind 10002");
    }

    #[test]
    fn test_relative_time_just_now() {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        assert_eq!(relative_time(now), "just now");
    }

    #[test]
    fn test_relative_time_minutes() {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        assert_eq!(relative_time(now - 300), "5m ago");
    }

    #[test]
    fn test_relative_time_hours() {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        assert_eq!(relative_time(now - 7200), "2h ago");
    }

    #[test]
    fn test_truncated_npub_invalid() {
        assert_eq!(truncated_npub("deadbeef"), "deadbeef...");
    }
}
