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


String actualText = WebUI.getText(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Actual Productive Hours_label'))

actualText = actualText.replaceAll("\\s+", " ").trim()

String expectedText = "Actual Productive Hours Business Impact Categories Productive Neutral Non Productive Include Active Away From System"

assert actualText.equalsIgnoreCase(expectedText)

WebUI.closeBrowser()


/*

// Call the login test case to perform application login
WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)

// Set the browser window size to 1920x1080
DriverFactory.getWebDriver().manage().window().setSize(new Dimension(1920, 1080))

// Click on the WORK OUTPUT link
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance/a_WORK OUTPUT'))

// Switch to the window titled 'ProHance Work Output'
WebUI.switchToWindowTitle('ProHance Work Output')

// Click the sidebar menu
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/div_SIDEBAR MENU'))

// Click the Administration span in the sidebar
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/span_Administration'))

// Click the Work Output Normalization list item
WebUI.click(findTestObject('Normalization Screen/Page_ProHance Work Output/li_Work Output Normalization'))

// Switch to the content frame for Work Output Settings
WebUI.switchToFrame(findTestObject('Normalization Screen/Page_ProHance Work Output/frame'), 10)

// Click the modify icon for the type
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/type_modify_icon'))

// Switch back to the default content from the frame
WebUI.switchToDefaultContent()

// Verify the Metric Name label text
WebUI.verifyElementText(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/td_Metric Name'),
'Metric Name*')

// Verify the Output Multiplier Value label text
WebUI.verifyElementText(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/td_Output Multiplier Value'),
'Output Multiplier Value*')

// Verify the Is Active label text
WebUI.verifyElementText(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/td_Is Active'),
'Is Active*')

// Verify the Units Label text
WebUI.verifyElementText(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/span_Units Label'),
'Units Label')

// Click the Divisor column/label
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/td_Divisor'))

// Verify the 'Yes' label text for divisor
WebUI.verifyElementText(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/span_Yes'),
' Yes')

// Verify the 'No' label text for divisor
WebUI.verifyElementText(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/label_No'),
' No')

// Switch again to the content frame for Work Output Settings
WebUI.switchToFrame(findTestObject('Normalization Screen/Page_ProHance Work Output/frame'), 10)

// Retrieve the actual text of the Actual Productive Hours label
String actualText = WebUI.getText(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Actual Productive Hours_label'))

actualText = actualText.replaceAll("\\s+", " ").trim()

// Define the expected text for comparison
String expectedText = "Actual Productive Hours Business Impact Categories Productive Neutral Non Productive Include Active Away From System"

// Assert that the actual text matches the expected text
assert actualText == expectedText

// Close the browser
WebUI.closeBrowser()*/