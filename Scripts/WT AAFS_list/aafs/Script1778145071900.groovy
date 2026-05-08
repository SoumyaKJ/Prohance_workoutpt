import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.webui.keyword.builtin.SwitchToFrameKeyword

import org.openqa.selenium.WebElement

WebUI.switchToDefaultContent()
//WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/Work time category/refresh button_prohance'))

WebUI.waitForElementPresent(findTestObject('Work time category/side bar_admin'),10)

WebUI.click(findTestObject('Work time category/side bar_admin'))

WebUI.click(findTestObject('Work time category/activities'))

WebUI.click(findTestObject('Object Repository/Work time category/AAFS'))

WebUI.switchToFrame(findTestObject('Normalization Screen/Page_ProHance Work Output/frame'), 10)

List<WebElement> aafs = WebUI.findWebElements(findTestObject('Object Repository/Work time category/aafs_names'), 10)
	
List<String>activityawayfromsystem  = aafs.collect { WebElement el ->
	(el?.getText() ?: '').trim()
	}
	
print "***AAFS List***"

activityawayfromsystem.each {aafslist->

WebUI.comment("${aafslist}")}

return activityawayfromsystem
