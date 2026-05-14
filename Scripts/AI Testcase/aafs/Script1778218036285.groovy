import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.webui.keyword.builtin.SwitchToFrameKeyword

import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.webui.keyword.WebUI
import com.kms.katalon.core.webui.common.WebUiCommonHelper
import org.openqa.selenium.WebElement

/**
 * Test Objects required (create/update these in Object Repository):
 *  - Object Repository/Admin/btn_Refresh
 *  - Object Repository/Admin/sidebar_Admin
 *  - Object Repository/Admin/menu_Activities
 *  - Object Repository/Admin/opt_AAFS
 *  - Object Repository/Admin/frame_WorkOutput
 *  - Object Repository/Admin/AAFS/lbl_ActivityNameItems   (locator should match ALL activity name elements)
 */

// 1. Switch to default content.
WebUI.switchToDefaultContent()

// 2. Click the refresh button.
WebUI.click(findTestObject('Object Repository/Admin/btn_Refresh'))

// 3. Wait for the Admin sidebar to be present.
WebUI.waitForElementPresent(findTestObject('Object Repository/Admin/sidebar_Admin'), 30)

// 4. Click the Admin sidebar menu.
WebUI.click(findTestObject('Object Repository/Admin/sidebar_Admin'))

// 5. Click the Activities menu.
WebUI.click(findTestObject('Object Repository/Admin/menu_Activities'))

// 6. Click the AAFS option.
WebUI.click(findTestObject('Object Repository/Admin/opt_AAFS'))

// 7. Switch to the Work Output frame.
WebUI.switchToFrame(findTestObject('Object Repository/Admin/frame_WorkOutput'), 30)

// 8-10. Capture activity names -> List<String>, trim, ignore blank/null.
List<WebElement> activityElements = WebUiCommonHelper.findWebElements(
		findTestObject('Object Repository/Admin/AAFS/lbl_ActivityNameItems'),
		30
)

List<String> activityNames = activityElements
		.collect { WebElement el -> el?.getText() }
		.findAll { String t -> t != null && t.trim().length() > 0 }
		.collect { String t -> t.trim() }

// 11. Print all captured AAFS activity names.
WebUI.comment("AAFS activity names captured: ${activityNames.size()}")
activityNames.eachWithIndex { String name, int idx ->
	WebUI.comment(String.format('[%02d] %s', idx + 1, name))
}

// 12. Return the final list.
return activityNames

Summarize
/*
*  Capture and return trimmed, non-empty activity names from the AAFS section after navigating the admin UI.
*
*  1. Switch to the page's default content/frame to ensure starting from top-level DOM.
*  2. Click the Refresh button to reload or refresh the current view.
*  3. Wait up to 30 seconds for the Admin sidebar element to be present.
*  4. Click the Admin sidebar to expand or focus the admin navigation.
*  5. Click the Activities menu item to navigate to activities.
*  6. Click the AAFS option to select the specific activities subset.
*  7. Switch into the Work Output iframe (waiting up to 30 seconds) to operate inside that frame's DOM.
*  8. Locate all activity name elements matching the provided test object locator within the frame and collect them as a list of WebElement objects.
*  9. Map each WebElement to its text, filter out null or purely whitespace strings, and trim remaining strings to produce a List<String> of activity names.
* 10. Log the count of captured activity names and then log each name with a two-digit index prefix.
* 11. Return the final List<String> of trimmed, non-empty activity names.
*
*/