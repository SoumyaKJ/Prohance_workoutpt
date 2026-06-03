import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase

import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import org.openqa.selenium.Dimension
import org.openqa.selenium.WebElement

import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.webui.driver.DriverFactory
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)

DriverFactory.getWebDriver().manage().window().setSize(new Dimension(1920, 1080))

WebUI.click(findTestObject('Object Repository/Category/Page_ProHance/a_WORK OUTPUT'))

WebUI.switchToWindowTitle('ProHance Work Output')

WebUI.click(findTestObject('Object Repository/Category/Page_ProHance Work Output/i_Soumya Admin Account_fa fa-chevron-right _d36e5e'))

WebUI.click(findTestObject('Object Repository/Category/Page_ProHance Work Output/span_Administration'))

WebUI.click(findTestObject('Wo_settings/Page_ProHance Work Output/li_Work Output Settings'))

WebUI.switchToFrame(findTestObject('Wo_settings/Page_ProHance Work Output/iframe_contentFrame'), 10)


WebUI.verifyElementText(findTestObject('Object Repository/Wo_settings/Page_ProHance Work Output/data approval label'),
	'Enable data approval process')

TestObject checkbox = findTestObject('Object Repository/Wo_settings/Page_ProHance Work Output/data approval check box')

WebElement element = WebUI.findWebElement(checkbox, 10)

if (!WebUI.verifyElementChecked(checkbox, 5, FailureHandling.OPTIONAL)) {
	
	WebUI.click(checkbox)

	println("Checkbox was not selected, now clicked")

} else {

	println("Checkbox already selected")
}

def sourcecheckbox = WebUI.findWebElements(findTestObject('Object Repository/Wo_settings/Page_ProHance Work Output/approval check box'),10)

def actualValues = sourcecheckbox.collect { it.getText().trim()}

def expectedvalue=['Timer', 'Clicker', 'Data Forms', 'Web Form', 'Excel Import']

assert(actualValues==expectedvalue)

WebUI.closeBrowser()






