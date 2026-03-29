use rusqlite::Connection;
use rusqlite_migration::{Migrations, M};

use crate::error::Error;
use crate::models::{kind_name, truncated_npub, RelayEvent};

pub struct Store {
    conn: Connection,
}

impl Store {
    pub fn new(data_dir: &str) -> Result<Self, Error> {
        let db_path = format!("{}/relay_viewer.db", data_dir);
        let mut conn = Connection::open(&db_path)?;

        let migrations = Migrations::new(vec![M::up(
            "CREATE TABLE IF NOT EXISTS events (
                id TEXT PRIMARY KEY,
                pubkey TEXT NOT NULL,
                kind INTEGER NOT NULL,
                content TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                display_name TEXT NOT NULL DEFAULT ''
            );
            CREATE INDEX IF NOT EXISTS idx_events_created_at ON events(created_at DESC);
            CREATE INDEX IF NOT EXISTS idx_events_kind ON events(kind);",
        )]);
        migrations.to_latest(&mut conn)?;

        Ok(Self { conn })
    }

    pub fn upsert_event(&self, event: &RelayEvent) -> Result<(), Error> {
        self.conn.execute(
            "INSERT OR REPLACE INTO events (id, pubkey, kind, content, created_at, display_name) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![
                event.id,
                event.pubkey,
                event.kind,
                event.content,
                event.created_at,
                event.display_name
            ],
        )?;
        Ok(())
    }

    pub fn list_events(&self, limit: u32) -> Result<Vec<RelayEvent>, Error> {
        let mut stmt = self.conn.prepare(
            "SELECT id, pubkey, kind, content, created_at, display_name \
             FROM events ORDER BY created_at DESC LIMIT ?1",
        )?;
        let events = stmt
            .query_map(rusqlite::params![limit], |row| {
                let pubkey: String = row.get(1)?;
                let kind: u32 = row.get(2)?;
                let display_name: String = row.get(5)?;
                let display = if display_name.is_empty() {
                    truncated_npub(&pubkey)
                } else {
                    display_name
                };
                Ok(RelayEvent {
                    id: row.get(0)?,
                    pubkey,
                    kind,
                    content: row.get(3)?,
                    created_at: row.get(4)?,
                    display_name: display,
                    kind_name: kind_name(kind),
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(events)
    }

    pub fn event_count(&self) -> Result<u32, Error> {
        let count: u32 = self
            .conn
            .query_row("SELECT COUNT(*) FROM events", [], |row| row.get(0))?;
        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_event(id: &str, pubkey: &str, kind: u32, content: &str, created_at: i64) -> RelayEvent {
        RelayEvent {
            id: id.into(),
            pubkey: pubkey.into(),
            kind,
            content: content.into(),
            created_at,
            display_name: format!("{pubkey}_display"),
            kind_name: kind_name(kind),
        }
    }

    #[test]
    fn test_upsert_and_list() {
        let dir = tempfile::tempdir().unwrap();
        let store = Store::new(dir.path().to_str().unwrap()).unwrap();

        let event = test_event("abc123", "deadbeef", 1, "Hello Nostr!", 1700000000);
        store.upsert_event(&event).unwrap();

        let events = store.list_events(50).unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].content, "Hello Nostr!");
        assert_eq!(events[0].kind, 1);
        assert_eq!(events[0].kind_name, "Text Note");
    }

    #[test]
    fn test_idempotent_insert() {
        let dir = tempfile::tempdir().unwrap();
        let store = Store::new(dir.path().to_str().unwrap()).unwrap();

        let event = test_event("abc123", "deadbeef", 1, "Hello Nostr!", 1700000000);
        store.upsert_event(&event).unwrap();
        store.upsert_event(&event).unwrap();

        assert_eq!(store.event_count().unwrap(), 1);
    }

    #[test]
    fn test_ordering() {
        let dir = tempfile::tempdir().unwrap();
        let store = Store::new(dir.path().to_str().unwrap()).unwrap();

        store
            .upsert_event(&test_event("older", "pk", 1, "old", 1000))
            .unwrap();
        store
            .upsert_event(&test_event("newer", "pk", 7, "new", 2000))
            .unwrap();

        let events = store.list_events(50).unwrap();
        assert_eq!(events[0].id, "newer");
        assert_eq!(events[0].kind_name, "Reaction");
        assert_eq!(events[1].id, "older");
        assert_eq!(events[1].kind_name, "Text Note");
    }

    #[test]
    fn test_multiple_kinds() {
        let dir = tempfile::tempdir().unwrap();
        let store = Store::new(dir.path().to_str().unwrap()).unwrap();

        store
            .upsert_event(&test_event("e1", "pk", 0, "{}", 1000))
            .unwrap();
        store
            .upsert_event(&test_event("e2", "pk", 1, "hello", 2000))
            .unwrap();
        store
            .upsert_event(&test_event("e3", "pk", 7, "+", 3000))
            .unwrap();
        store
            .upsert_event(&test_event("e4", "pk", 9735, "zap", 4000))
            .unwrap();

        let events = store.list_events(50).unwrap();
        assert_eq!(events.len(), 4);
        assert_eq!(events[0].kind_name, "Zap");
        assert_eq!(events[1].kind_name, "Reaction");
        assert_eq!(events[2].kind_name, "Text Note");
        assert_eq!(events[3].kind_name, "Metadata");
    }
}
