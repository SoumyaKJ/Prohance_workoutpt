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


// Call the applogin test case to perform application login
WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)

// Set browser window size to 1920x1080
DriverFactory.getWebDriver().manage().window().setSize(new Dimension(1920, 1080))

// Click the WORK OUTPUT link on the page
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance/a_WORK OUTPUT'))

// Switch to the window titled 'ProHance Work Output'
WebUI.switchToWindowTitle('ProHance Work Output')

// Click the sidebar menu to expand it
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/div_SIDEBAR MENU'))

// Click the Administration span in the sidebar
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/span_Administration'))

// Click the Work Output Normalization list item
WebUI.click(findTestObject('Normalization Screen/Page_ProHance Work Output/li_Work Output Normalization'))

// Switch to the Work Output Settings iframe
WebUI.switchToFrame(findTestObject('Normalization Screen/Page_ProHance Work Output/frame'), 10)

// Click the modify icon for the type
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/type_modify_icon'))

// Wait for the page to load completely
WebUI.waitForPageLoad(10)

// Get the breadcrumb text from the page
def breadcrumb=WebUI.getText(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/breadcrumb'))

// Print the breadcrumb value to the console
print breadcrumb

// Define the expected breadcrumb text
def Actualtext="Administration Work Output Work Output Normalization Work Output Normalization Details [Type 1] "

// Assert that the actual breadcrumb matches the expected text
assert breadcrumb==Actualtext

// Close the browser
WebUI.closeBrowser()