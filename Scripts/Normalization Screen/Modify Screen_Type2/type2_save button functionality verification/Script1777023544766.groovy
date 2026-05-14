import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import org.openqa.selenium.WebDriver as WebDriver
import org.openqa.selenium.chrome.ChromeDriver as ChromeDriver
import org.openqa.selenium.Alert as Alert
import org.openqa.selenium.Dimension as Dimension
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
import org.openqa.selenium.NoAlertPresentException

import com.kms.katalon.core.webui.driver.DriverFactory

WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)

DriverFactory.getWebDriver().manage().window().setSize(new Dimension(1920, 1080))

WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance/a_WORK OUTPUT'))

WebUI.switchToWindowTitle('ProHance Work Output')

WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/div_SIDEBAR MENU'))

WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/span_Administration'))

WebUI.click(findTestObject('Normalization Screen/Page_ProHance Work Output/li_Work Output Normalization'))

WebUI.switchToFrame(findTestObject('Normalization Screen/Page_ProHance Work Output/frame'), 10)

WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/type2_modify_icon'))

WebUI.waitForPageLoad(10)

WebUI.clearText(findTestObject('Normalization Screen/Page_ProHance Work Output/unit text field'))

WebUI.clearText(findTestObject('Normalization Screen/Page_ProHance Work Output/Output Multiplier Value_field'))

WebUI.clearText(findTestObject('Normalization Screen/Page_ProHance Work Output/metric name field'))

WebUI.click(findTestObject('Normalization Screen/Page_ProHance Work Output/save button'))

WebUI.waitForAlert(5)
//Metric Name
def alert=WebUI.findWebElement(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/alert'))

def actualAlertText=alert.getText()

println(actualAlertText)

def Expected = 'Name is required information.'

assert actualAlertText == Expected

WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/alertaccept'))

WebUI.setText(findTestObject('Normalization Screen/Page_ProHance Work Output/metric name field'), 'type1')

WebUI.click(findTestObject('Normalization Screen/Page_ProHance Work Output/save button'))

//Units Label

WebUI.waitForAlert(5)

def alert1=WebUI.findWebElement(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/alert'))

def actualAlertText1 = alert1.getText()

println(actualAlertText1)

def Expected1 = 'Metric Unit is required information'

assert actualAlertText1 == Expected1

WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/alertaccept'))

WebUI.setText(findTestObject('Normalization Screen/Page_ProHance Work Output/unit text field'), '%')

WebUI.click(findTestObject('Normalization Screen/Page_ProHance Work Output/save button'))

//Output Multiplier Value*

WebUI.waitForAlert(5)

def alert2=WebUI.findWebElement(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/alert'))

def actualAlertText2 = alert2.getText()

println(actualAlertText2)

def Expected2 = 'Output Multiplier Value is required information'

assert actualAlertText2 == Expected2

WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/alertaccept'))

WebUI.setText(findTestObject('Normalization Screen/Page_ProHance Work Output/Output Multiplier Value_field'), '100')

WebUI.click(findTestObject('Normalization Screen/Page_ProHance Work Output/save button'))

//sucessful message verification
def Expected3 ="Work Output Normalization: type1 modified successfully"

def actualmsg=WebUI.findWebElement(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/save_sucessfull message'))

def successfulmsg=actualmsg.getText()

assert Expected3==successfulmsg

WebUI.closeBrowser()
/*
// Call the login test case to perform application login
WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)

// Set browser window size to 1920x1080
DriverFactory.getWebDriver().manage().window().setSize(new Dimension(1920, 1080))

// Click the WORK OUTPUT link
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance/a_WORK OUTPUT'))

// Switch to the window titled 'ProHance Work Output'
WebUI.switchToWindowTitle('ProHance Work Output')

// Click the sidebar menu
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/div_SIDEBAR MENU'))

// Click the Administration span
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/span_Administration'))

// Click the Work Output Normalization list item
WebUI.click(findTestObject('Normalization Screen/Page_ProHance Work Output/li_Work Output Normalization'))

// Switch to the content frame with a timeout of 10 seconds
WebUI.switchToFrame(findTestObject('Normalization Screen/Page_ProHance Work Output/frame'), 10)

// Click the modify icon for type2
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/type2_modify_icon'))

// Wait for the page to load for up to 10 seconds
WebUI.waitForPageLoad(10)

// Clear the unit text field
WebUI.clearText(findTestObject('Normalization Screen/Page_ProHance Work Output/unit text field'))

// Clear the Output Multiplier Value field
WebUI.clearText(findTestObject('Normalization Screen/Page_ProHance Work Output/Output Multiplier Value_field'))

// Clear the metric name field
WebUI.clearText(findTestObject('Normalization Screen/Page_ProHance Work Output/metric name field'))

// Click the save button
WebUI.click(findTestObject('Normalization Screen/Page_ProHance Work Output/save button'))

// Wait for an alert to be present for up to 5 seconds
WebUI.waitForAlert(5)

// Find the alert web element
def alert=WebUI.findWebElement(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/alert'))

def actualAlertText=alert.getText()

println(actualAlertText)

def Expected = 'Name is required information.'

assert actualAlertText == Expected

// Click the alert accept button
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/alertaccept'))

// Set the metric name field to 'type1'
WebUI.setText(findTestObject('Normalization Screen/Page_ProHance Work Output/metric name field'), 'type1')

// Click the save button
WebUI.click(findTestObject('Normalization Screen/Page_ProHance Work Output/save button'))

// Wait for an alert to be present for up to 5 seconds
WebUI.waitForAlert(5)

// Find the alert web element
def alert1=WebUI.findWebElement(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/alert'))

def actualAlertText1 = alert1.getText()

println(actualAlertText1)

def Expected1 = 'Metric Unit is required information'

assert actualAlertText1 == Expected1

// Click the alert accept button
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/alertaccept'))

// Set the unit text field to '%'
WebUI.setText(findTestObject('Normalization Screen/Page_ProHance Work Output/unit text field'), '%')

// Click the save button
WebUI.click(findTestObject('Normalization Screen/Page_ProHance Work Output/save button'))

// Wait for an alert to be present for up to 5 seconds
WebUI.waitForAlert(5)

// Find the alert web element
def alert2=WebUI.findWebElement(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/alert'))

def actualAlertText2 = alert2.getText()

println(actualAlertText2)

def Expected2 = 'Output Multiplier Value is required information'

assert actualAlertText2 == Expected2

// Click the alert accept button
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/alertaccept'))

// Set the Output Multiplier Value field to '100'
WebUI.setText(findTestObject('Normalization Screen/Page_ProHance Work Output/Output Multiplier Value_field'), '100')

// Click the save button
WebUI.click(findTestObject('Normalization Screen/Page_ProHance Work Output/save button'))

def Expected3 ="Work Output Normalization: type1 modified successfully"

// Find the success message web element
def actualmsg=WebUI.findWebElement(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/save_sucessfull message'))

def successfulmsg=actualmsg.getText()

assert Expected3==successfulmsg

// Close the browser
WebUI.closeBrowser()*/