package dev.damsac.relay_viewer

import androidx.compose.ui.test.*
import androidx.compose.ui.test.junit4.createAndroidComposeRule
import org.junit.Rule
import org.junit.Test

class RelayViewerE2eTest {
    @get:Rule
    val compose = createAndroidComposeRule<MainActivity>()

    @Test
    fun scrollThroughEvents() {
        // Wait for at least one event card to appear (up to 30 seconds)
        compose.waitUntil(timeoutMillis = 30_000) {
            compose.onAllNodesWithTag("event_card").fetchSemanticsNodes().isNotEmpty()
        }

        // Small settle delay for rendering
        Thread.sleep(1_000)

        // Scroll through the list
        val list = compose.onNodeWithTag("event_list")
        repeat(4) {
            list.performScrollToIndex(it * 5)
            Thread.sleep(1_000)
        }

        // Scroll back up
        list.performScrollToIndex(0)
        Thread.sleep(2_000)
    }
}
