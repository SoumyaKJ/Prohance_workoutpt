import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import org.openqa.selenium.Dimension
import org.openqa.selenium.WebElement

import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.webui.driver.DriverFactory
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)

DriverFactory.getWebDriver().manage().window().setSize(new Dimension(1920, 1080))

WebUI.click(findTestObject('Work time category/side bar_admin'))

WebUI.click(findTestObject('Work time category/activities'))

WebUI.click(findTestObject('Object Repository/Work time category/AOS'))

WebUI.switchToFrame(findTestObject('Normalization Screen/Page_ProHance Work Output/frame'), 10)

List<WebElement> aos = WebUI.findWebElements(findTestObject('Object Repository/Work time category/aos_names'), 10)

List<String>activityonsystem  = aos.collect { WebElement el ->
	(el?.getText() ?: '').trim()
}

if(WebUI.waitForElementPresent(findTestObject('Object Repository/Work time category/manual declared tab'),10))
{

WebUI.click(findTestObject('Object Repository/Work time category/manual declared tab'))
}
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
