import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import org.openqa.selenium.Dimension

import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.webui.driver.DriverFactory
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI


WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)

DriverFactory.getWebDriver().manage().window().setSize(new Dimension(1920, 1080))

WebUI.switchToDefaultContent()

WebUI.click(findTestObject('Object Repository/Work time category/refresh button_prohance'))

WebUI.waitForElementPresent(findTestObject('Work time category/side bar_admin'), 10)

WebUI.click(findTestObject('Work time category/side bar_admin'))

WebUI.click(findTestObject('Object Repository/worktime user screen/users link'))

WebUI.click(findTestObject('Object Repository/worktime user screen/user role sceen'))

WebUI.switchToFrame(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/iframe'), 10)

def userrolename=WebUI.findWebElements(findTestObject('Object Repository/worktime user screen/user role name'),10)

def name=userrolename.collect{it.getText().trim()}

Map<String, List<String>> dropdownMap = [:]

dropdownMap['User Role'] = name

println dropdownMap

//WebUI.closeBrowser()

return dropdownMap








