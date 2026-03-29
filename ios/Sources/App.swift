import SwiftUI

@main
struct RelayViewerApp: App {
    var body: some Scene {
        WindowGroup {
            EventListView()
        }
    }
}

struct EventListView: View {
    @State private var events: [FfiRelayEvent] = []
    @State private var isLoading = false
    @State private var errorMessage: String?
    @State private var core: AppCore?

    private let relayUrl = "wss://nostr.damsac.studio"

    var body: some View {
        NavigationStack {
            Group {
                if isLoading && events.isEmpty {
                    ProgressView("Connecting to relay...")
                } else if let error = errorMessage, events.isEmpty {
                    VStack(spacing: 12) {
                        Image(systemName: "wifi.exclamationmark")
                            .font(.system(size: 40))
                            .foregroundStyle(.secondary)
                        Text(error)
                            .foregroundStyle(.secondary)
                            .multilineTextAlignment(.center)
                        Button("Retry") { Task { await loadEvents() } }
                    }
                    .padding()
                } else {
                    List(events, id: \.id) { event in
                        EventRow(event: event)
                    }
                    .refreshable { await loadEvents() }
                }
            }
            .navigationTitle("Relay Viewer")
            .toolbar {
                ToolbarItem(placement: .topBarTrailing) {
                    if !events.isEmpty {
                        Text("\(events.count) events")
                            .font(.caption)
                            .foregroundStyle(.secondary)
                    }
                }
            }
            .task { await loadEvents() }
        }
    }

    private func getOrCreateCore() throws -> AppCore {
        if let existing = core {
            return existing
        }
        let dataDir = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask)[0].path()
        let newCore = try AppCore(relayUrl: relayUrl, dataDir: dataDir)
        core = newCore
        return newCore
    }

    func loadEvents() async {
        isLoading = true
        errorMessage = nil
        do {
            let appCore = try getOrCreateCore()
            let fetched = try appCore.fetchEvents(limit: 100)
            events = fetched
        } catch {
            if core == nil {
                errorMessage = "Failed to connect. Check your network and try again."
            } else {
                errorMessage = "Could not load events. Pull to refresh."
            }
        }
        isLoading = false
    }
}

struct EventRow: View {
    let event: FfiRelayEvent

    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            HStack(spacing: 8) {
                KindBadge(kind: event.kind, name: event.kindName)
                Spacer()
                Text(event.relativeTime)
                    .font(.caption)
                    .foregroundStyle(.tertiary)
            }
            if !event.content.isEmpty {
                Text(event.content)
                    .font(.body)
                    .lineLimit(4)
                    .foregroundStyle(.primary)
            }
            Text(event.displayName)
                .font(.caption)
                .foregroundStyle(.secondary)
        }
        .padding(.vertical, 4)
    }
}

struct KindBadge: View {
    let kind: UInt32
    let name: String

    var body: some View {
        Text(name)
            .font(.caption2)
            .fontWeight(.semibold)
            .padding(.horizontal, 8)
            .padding(.vertical, 3)
            .background(colorForKind(kind).opacity(0.15))
            .foregroundStyle(colorForKind(kind))
            .clipShape(Capsule())
    }

    func colorForKind(_ kind: UInt32) -> Color {
        switch kind {
        case 0: return .purple       // Metadata
        case 1: return .blue         // Text Note
        case 3: return .green        // Contact List
        case 4: return .orange       // DM
        case 5: return .red          // Delete
        case 6: return .cyan         // Repost
        case 7: return .pink         // Reaction
        case 9735: return .yellow    // Zap
        case 30023: return .indigo   // Long-form
        default: return .gray
        }
    }
}
