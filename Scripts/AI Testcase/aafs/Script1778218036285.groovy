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
