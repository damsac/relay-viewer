# relay-viewer

Nostr relay event visualizer for [wss://nostr.damsac.studio](wss://nostr.damsac.studio).

Shows a live feed of every event kind on the damsac relay with color-coded kind badges, content previews, truncated npub authors, and relative timestamps.

Built with [RMP](https://github.com/damsac/rmp-build) (Rust Multi-Platform): Rust core with UniFFI bindings, native SwiftUI and Kotlin/Compose UI shells.

## Event kinds displayed

| Kind | Name |
|------|------|
| 0 | Metadata |
| 1 | Text Note |
| 3 | Contact List |
| 4 | DM |
| 5 | Delete |
| 6 | Repost |
| 7 | Reaction |
| 9735 | Zap |
| 30023 | Long-form |

## Development

```bash
just test            # Run Rust tests
just clippy          # Lint
just fmt             # Format check
just ios-build       # Build iOS (macOS only)
just android-build   # Build Android
```
