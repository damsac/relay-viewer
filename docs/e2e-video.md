---
summary: E2E video recording pipeline — records app running on simulators/emulators, uploads to Blossom
read_when:
  - setting up e2e video recording
  - debugging e2e video CI
  - configuring Blossom upload
  - adding visual regression testing
---

# E2E Video Pipeline

Automatically records a short video of the app running on iOS Simulator and Android Emulator as part of CI. Videos are uploaded to Blossom (decentralized media hosting) and linked in PR comments.

## How It Works

1. **Build**: The Rust core is cross-compiled and native bindings are generated (same as the main CI)
2. **Boot**: A simulator (iOS) or emulator (Android) is started
3. **Install & Launch**: The debug build is installed and launched on the virtual device
4. **Record**: Screen recording captures ~8 seconds of the running app
5. **Upload**: The recording is uploaded to Blossom servers via the `blossom-upload` crate
6. **Comment**: A PR comment is posted with a link to the video (or an artifact fallback)

## Workflows

- `.github/workflows/e2e-video-ios.yml` — iOS Simulator on `macos-15`
- `.github/workflows/e2e-video-android.yml` — Android Emulator on `ubuntu-latest`

Both trigger on PRs that touch `crates/`, platform directories, or `scripts/`. They can also be triggered manually via `workflow_dispatch`.

## Blossom Upload

The `crates/blossom-upload/` crate handles uploading files to Blossom servers. Blossom is a decentralized media hosting protocol (BUD-01/BUD-02) that uses Nostr keys for authentication.

### How It Works

1. Generates a throwaway Nostr keypair (no persistent key needed)
2. Signs a kind-24242 authorization event with SHA-256 hash of the file
3. Tries each configured server in order until one accepts
4. Returns the URL of the uploaded blob

### Usage

```bash
# Upload to default public servers
cargo run -p blossom-upload -- recording.mp4

# Upload to specific servers
cargo run -p blossom-upload -- \
  --server https://blossom.primal.net \
  --server https://blossom.oxtr.dev \
  recording.mp4
```

### Default Servers

- `https://blossom.primal.net`
- `https://cdn.satellite.earth`
- `https://blossom.oxtr.dev`

## Configuration

To customize for your project, update the `FFI_CRATE` env var and template variables in the workflow files:

```yaml
env:
  FFI_CRATE: your-app-ffi  # Must match your FFI crate name
```

The iOS workflow also references `RelayViewer` for the Xcode scheme name and bundle ID prefix. Replace this with your actual project name when scaffolding.

## Fallback

If Blossom upload fails (server down, rate limited, etc.), the video is uploaded as a GitHub Actions artifact with 5-day retention. The PR comment notes the fallback.
