import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import org.openqa.selenium.Dimension
import org.openqa.selenium.WebElement

import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.webui.driver.DriverFactory
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

//WebUI.switchToDefaultContent()
WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)

DriverFactory.getWebDriver().manage().window().setSize(new Dimension(1920, 1080))

//WebUI.click(findTestObject('Object Repository/Work time category/refresh button_prohance'))

//WebUI.waitForElementPresent(findTestObject('Work time category/side bar_admin'),10)

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
