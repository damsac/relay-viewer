package dev.damsac.relay_viewer

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.*
import androidx.compose.material3.pulltorefresh.PullToRefreshBox
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import dev.damsac.relay_viewer.rust.AppCore
import dev.damsac.relay_viewer.rust.FfiRelayEvent
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        val dataDir = filesDir.absolutePath
        setContent {
            MaterialTheme {
                EventListScreen(dataDir = dataDir)
            }
        }
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun EventListScreen(dataDir: String) {
    var events by remember { mutableStateOf<List<FfiRelayEvent>>(emptyList()) }
    var isLoading by remember { mutableStateOf(true) }
    var error by remember { mutableStateOf<String?>(null) }
    val scope = rememberCoroutineScope()
    val relayUrl = "wss://nostr.damsac.studio"

    val core = remember {
        try {
            AppCore(relayUrl, dataDir)
        } catch (e: Exception) {
            null
        }
    }
    var coreError by remember {
        mutableStateOf<String?>(
            if (core == null) "Failed to connect. Check your network and try again." else null
        )
    }

    fun refresh() {
        scope.launch {
            isLoading = true
            error = null
            try {
                val appCore = core ?: throw Exception(coreError ?: "Not connected")
                val fetched = withContext(Dispatchers.IO) {
                    appCore.fetchEvents(100u)
                }
                events = fetched
            } catch (e: Exception) {
                error = "Could not load events. Pull to refresh."
            }
            isLoading = false
        }
    }

    LaunchedEffect(Unit) { refresh() }

    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text("Relay Viewer") },
                actions = {
                    if (events.isNotEmpty()) {
                        Text(
                            text = "${events.size} events",
                            style = MaterialTheme.typography.labelMedium,
                            color = MaterialTheme.colorScheme.onSurfaceVariant,
                            modifier = Modifier.padding(end = 16.dp)
                        )
                    }
                }
            )
        }
    ) { padding ->
        PullToRefreshBox(
            isRefreshing = isLoading,
            onRefresh = { refresh() },
            modifier = Modifier
                .fillMaxSize()
                .padding(padding)
                .then(if (isLoading) Modifier.testTag("loading") else Modifier)
        ) {
            if (error != null && events.isEmpty()) {
                Column(
                    modifier = Modifier
                        .fillMaxSize()
                        .testTag("error"),
                    verticalArrangement = Arrangement.Center,
                    horizontalAlignment = Alignment.CenterHorizontally
                ) {
                    Text(
                        error ?: "Something went wrong",
                        color = MaterialTheme.colorScheme.error
                    )
                    Spacer(modifier = Modifier.height(8.dp))
                    Button(onClick = { refresh() }) { Text("Retry") }
                }
            } else {
                LazyColumn(modifier = Modifier.fillMaxSize().testTag("event_list")) {
                    items(events, key = { it.id }) { event ->
                        EventCard(event)
                    }
                }
            }
        }
    }
}

@Composable
fun EventCard(event: FfiRelayEvent) {
    Card(
        modifier = Modifier
            .fillMaxWidth()
            .padding(horizontal = 12.dp, vertical = 4.dp)
            .testTag("event_card")
    ) {
        Column(modifier = Modifier.padding(12.dp)) {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically
            ) {
                KindBadge(kind = event.kind, name = event.kindName)
                Text(
                    text = event.relativeTime,
                    style = MaterialTheme.typography.labelSmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant
                )
            }
            if (event.content.isNotEmpty()) {
                Spacer(modifier = Modifier.height(6.dp))
                Text(
                    text = event.content,
                    style = MaterialTheme.typography.bodyMedium,
                    maxLines = 4,
                    overflow = TextOverflow.Ellipsis
                )
            }
            Spacer(modifier = Modifier.height(4.dp))
            Text(
                text = event.displayName,
                style = MaterialTheme.typography.labelSmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant
            )
        }
    }
}

@Composable
fun KindBadge(kind: UInt, name: String) {
    val color = colorForKind(kind)
    Text(
        text = name,
        fontSize = 11.sp,
        fontWeight = FontWeight.SemiBold,
        color = color,
        modifier = Modifier
            .background(
                color = color.copy(alpha = 0.15f),
                shape = RoundedCornerShape(12.dp)
            )
            .padding(horizontal = 8.dp, vertical = 3.dp)
    )
}

fun colorForKind(kind: UInt): Color {
    return when (kind.toInt()) {
        0 -> Color(0xFF9C27B0)     // Metadata — purple
        1 -> Color(0xFF2196F3)     // Text Note — blue
        3 -> Color(0xFF4CAF50)     // Contact List — green
        4 -> Color(0xFFFF9800)     // DM — orange
        5 -> Color(0xFFF44336)     // Delete — red
        6 -> Color(0xFF00BCD4)     // Repost — cyan
        7 -> Color(0xFFE91E63)     // Reaction — pink
        9735 -> Color(0xFFFFC107)  // Zap — amber
        30023 -> Color(0xFF3F51B5) // Long-form — indigo
        else -> Color(0xFF9E9E9E)  // Unknown — gray
    }
}
