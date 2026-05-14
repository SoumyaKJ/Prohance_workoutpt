import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import org.openqa.selenium.Dimension

import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.webui.driver.DriverFactory
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

/*WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)

DriverFactory.getWebDriver().manage().window().setSize(new Dimension(1920, 1080))

WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance/a_WORK OUTPUT'))

WebUI.switchToWindowTitle('ProHance Work Output')

WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/div_SIDEBAR MENU'))

WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/span_Administration'))

WebUI.click(findTestObject('Normalization Screen/Page_ProHance Work Output/li_Work Output Normalization'))

WebUI.switchToFrame(findTestObject('Normalization Screen/Page_ProHance Work Output/frame'), 10)

WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/type_modify_icon'))

WebUI.waitForPageLoad(10)

def breadcrumb=WebUI.getText(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/breadcrumb'))

print breadcrumb

def Actualtext="Administration Work Output Work Output Normalization Work Output Normalization Details [Type 1] "

assert breadcrumb==Actualtext

WebUI.closeBrowser()
*/

// Call the applogin test case
WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)

DriverFactory.getWebDriver().manage().window().setSize(new Dimension(1920, 1080))

// Click the WORK OUTPUT link
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance/a_WORK OUTPUT'))

// Switch to the ProHance Work Output window
WebUI.switchToWindowTitle('ProHance Work Output')

// Click the sidebar menu
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/div_SIDEBAR MENU'))

// Click the Administration span
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/span_Administration'))

// Click the Work Output Normalization list item
WebUI.click(findTestObject('Normalization Screen/Page_ProHance Work Output/li_Work Output Normalization'))

// Switch to the Work Output Settings iframe
WebUI.switchToFrame(findTestObject('Normalization Screen/Page_ProHance Work Output/frame'), 10)

// Click the modify icon for the type
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/type_modify_icon'))

// Wait for the page to load
WebUI.waitForPageLoad(10)

def breadcrumb=WebUI.getText(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/breadcrumb'))

print breadcrumb

def Actualtext="Administration Work Output Work Output Normalization Work Output Normalization Details [Type 1] "

assert breadcrumb==Actualtext

// Close the browser
WebUI.closeBrowser()