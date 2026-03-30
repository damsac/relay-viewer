import XCTest

class RelayViewerUITests: XCTestCase {
    let app = XCUIApplication()

    override func setUp() {
        continueAfterFailure = false
        app.launch()
    }

    func testScrollThroughEvents() {
        // Wait for events to load (up to 15 seconds)
        let firstCell = app.cells.firstMatch
        let exists = firstCell.waitForExistence(timeout: 15)

        if exists {
            // Scroll through the list
            for _ in 0..<4 {
                app.swipeUp()
                sleep(1)
            }
            // Scroll back up
            for _ in 0..<2 {
                app.swipeDown()
                sleep(1)
            }
        }

        // Take a screenshot at the end
        let screenshot = app.screenshot()
        let attachment = XCTAttachment(screenshot: screenshot)
        attachment.lifetime = .keepAlways
        add(attachment)
    }
}
