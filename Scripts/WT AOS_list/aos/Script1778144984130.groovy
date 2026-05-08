import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.webui.keyword.builtin.SwitchToFrameKeyword
import com.kms.katalon.core.webui.keyword.builtin.WaitForElementVisibleKeyword

import org.openqa.selenium.WebElement

WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Work time category/side bar_admin'))

WebUI.click(findTestObject('Work time category/activities'))

WebUI.click(findTestObject('Object Repository/Work time category/AOS'))

WebUI.switchToFrame(findTestObject('Normalization Screen/Page_ProHance Work Output/frame'), 10)

List<WebElement> aos = WebUI.findWebElements(findTestObject('Object Repository/Work time category/aos_names'), 10)

List<String>activityonsystem  = aos.collect { WebElement el ->
	(el?.getText() ?: '').trim()
}
WebUI.click(findTestObject('Object Repository/Work time category/manual declared tab'))

WebUI.waitForElementPresent(findTestObject('Object Repository/Work time category/aos_names'),10)
	
List<WebElement> aos1 = WebUI.findWebElements(findTestObject('Object Repository/Work time category/aos_names'), 10)

List<String> activityonsystem1 = aos1.collect {
	it.getText().trim()
}

activityonsystem.addAll(activityonsystem1)

print "***AOS List***"

activityonsystem.each {aoslist->

WebUI.comment("${aoslist}")}

return activityonsystem
