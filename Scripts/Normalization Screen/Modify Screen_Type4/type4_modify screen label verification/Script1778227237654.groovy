import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import org.openqa.selenium.By as By
import org.openqa.selenium.Dimension as Dimension
import org.openqa.selenium.WebElement as WebElement
import com.kms.katalon.core.model.FailureHandling as FailureHandling
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

WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance/a_WORK OUTPUT'))

WebUI.switchToWindowTitle('ProHance Work Output')

WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/div_SIDEBAR MENU'))

WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/span_Administration'))

WebUI.click(findTestObject('Normalization Screen/Page_ProHance Work Output/li_Work Output Normalization'))

WebUI.switchToFrame(findTestObject('Normalization Screen/Page_ProHance Work Output/frame'), 10)

WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/type4_modify_button'))

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

String actualText = WebUI.getText(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/divisor_name'))

actualText = actualText.replaceAll("\\s+", " ").trim()

String expectedText = "Divisor*"

assert actualText == expectedText

WebUI.closeBrowser()



