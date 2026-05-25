import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import org.openqa.selenium.Dimension

import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.webui.driver.DriverFactory
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)

DriverFactory.getWebDriver().manage().window().setSize(new Dimension(1920, 1080))

WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance/a_WORK OUTPUT'))

WebUI.switchToWindowTitle('ProHance Work Output')

WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/div_SIDEBAR MENU'))

WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/span_Administration'))

WebUI.click(findTestObject('Normalization Screen/Page_ProHance Work Output/li_Work Output Normalization'))

WebUI.switchToFrame(findTestObject('Normalization Screen/Page_ProHance Work Output/frame'), 10)

WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/type_modify_icon'))

WebUI.switchToDefaultContent()

WebUI.verifyElementText(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/td_Metric Name'), 
    'Metric Name*')

WebUI.verifyElementText(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/td_Output Multiplier Value'), 
    'Output Multiplier Value*')

WebUI.verifyElementText(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/td_Is Active'), 
    'Is Active*')

WebUI.verifyElementText(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/span_Units Label'), 
    'Units Label')

WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/td_Divisor'))

WebUI.verifyElementText(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/span_Yes'), 
    ' Yes')

WebUI.verifyElementText(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/label_No'), 
    ' No')

WebUI.switchToFrame(findTestObject('Normalization Screen/Page_ProHance Work Output/frame'), 10)

def actualText = WebUI.findWebElements(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Actual Productive Hours_label'),10)

def actual = actualText.collect {
	it.getText().trim()
	}

actual.addAll('Include Activity Away From System')


def expectedText=WebUI.callTestCase(findTestCase('WT rating scale/rating scale options'), [:], FailureHandling.STOP_ON_FAILURE)

assert (actual==expectedText)

WebUI.closeBrowser()


