import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import org.openqa.selenium.Dimension as Dimension
import org.openqa.selenium.WebElement as WebElement
import org.openqa.selenium.support.ui.Select as Select
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.webui.common.WebUiCommonHelper as WebUiCommonHelper
import com.kms.katalon.core.webui.driver.DriverFactory as DriverFactory
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import internal.GlobalVariable as GlobalVariable

WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)

DriverFactory.getWebDriver().manage().window().setSize(new Dimension(1920, 1080))

WebUI.click(findTestObject('Worktype Definition Screen/Page_ProHance/WORK OUTPUT'))

WebUI.switchToWindowTitle('ProHance Work Output')

WebUI.click(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/div_SIDEBAR MENU'))

WebUI.click(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/a_Administration'))

WebUI.click(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/li_Work Type Definition'))

WebUI.switchToFrame(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/iframe'), 10)

WebUI.click(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/add new link'))

WebUI.setText(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/name text area'), 'new work type')

WebUI.click(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/save button'))

WebUI.click(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/back button'))

WebUI.setText(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/search text area'), 
    'new work type')

String workType = 'new work type'

WebUI.waitForPageLoad(30)

WebUI.waitForElementVisible(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/dynamic_modify_icon', 
        [('worktype') : workType]), 10)

WebUI.click(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/1st modify icon'))

WebUI.click(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/target tab link'))

boolean isChecked =WebUI.verifyElementNotChecked(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/check box1'), 
    10, FailureHandling.STOP_ON_FAILURE)

if (isChecked) {
	println("Checkbox is not checked")
	
} else {
	println("Checkbox is checked")
	
	WebUI.click(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/check box1'))
}

def userattributename = WebUI.getText(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/selected user attribute_in wt definition screen'))

println(userattributename)

WebUI.switchToDefaultContent()

WebUI.click(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/div_SIDEBAR MENU'))

WebUI.click(findTestObject('Wo_settings/Page_ProHance Work Output/li_Work Output Settings'))

WebUI.switchToFrame(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/iframe'), 10)

WebElement dropdown = WebUiCommonHelper.findWebElement(findTestObject('Object Repository/Wo_settings/Page_ProHance Work Output/user attribute options'), 
    10)

Select select = new Select(dropdown)

String selectedText = select.getFirstSelectedOption().getText()

println('Selected Value: ' + selectedText)

assert userattributename == selectedText

WebUI.closeBrowser()



