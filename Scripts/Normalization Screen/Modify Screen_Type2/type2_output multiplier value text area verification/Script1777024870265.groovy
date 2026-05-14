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
import java.io.FileInputStream as FileInputStream
import com.kms.katalon.core.configuration.RunConfiguration as RunConfiguration
import org.apache.poi.xssf.usermodel.XSSFWorkbook as XSSFWorkbook

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

def maxcharlength = WebUI.getAttribute(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Output Multiplier Value_field'),'maxlength')

def maxlength = maxcharlength.toInteger()

def saveBtn = findTestObject('Normalization Screen/Page_ProHance Work Output/save button')

WebUI.waitForElementClickable(saveBtn, 10)

TestObject btn = findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/type_modify_icon')

WebUI.waitForElementClickable(btn, 10)

//Characters verification

WebUI.setText(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Output Multiplier Value_field'),'Output Multiplier Value')

WebUI.click(saveBtn)

WebUI.waitForAlert(5)

def alert= WebUI.findWebElement(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/alert'))

def alertmsg=alert.getText();

def expectedtext="Output Multiplier Value should be numeric and shall be greater than 0, accepts up to 2 decimal places."

assert alertmsg==expectedtext

WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/alertaccept'))

//Special character verification

WebUI.setText(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Output Multiplier Value_field'),'!@#$%%^^^&*)(**^$$')

WebUI.click(saveBtn)

WebUI.waitForAlert(5)

def alertmsg1=alert.getText();

assert alertmsg==expectedtext

WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/alertaccept'))

//Decimal value verification with max value

WebUI.setText(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Output Multiplier Value_field'),'1245123')

WebUI.click(saveBtn)

def numberalert = WebUI.findWebElement(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/alert'))

WebUI.waitForElementVisible(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/alert'),10)

WebUI.delay(1)

def alertmsg3 = numberalert.getText()

def Expectedmsg ="Output Multiplier Value should not be more than 99999.99"

assert alertmsg3==Expectedmsg

WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/alertaccept'))

// Max 8 Number verification

WebUI.setText(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Output Multiplier Value_field'),'12451.23')

WebUI.click(saveBtn)

// input taken from data files

def file = new File(RunConfiguration.getProjectDir() + '/Data Files/Output Multiplier Value.xlsx')
	
def workbook = new XSSFWorkbook(file)
	
def sheet = workbook.getSheetAt(0)

WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/type_modify_icon'))

WebUI.waitForPageLoad(10)

WebUI.waitForElementVisible(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Output Multiplier Value_field'), 20)

sheet.iterator().eachWithIndex{ def row, def index ->
	
if (index == 0) 
        
	 return
	 	
	    WebUI.waitForElementVisible(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Output Multiplier Value_field'), 20)
		
		def value = row.getCell(0)?.toString()?.trim()
		
		if (value?.contains("E")) {
			value = new BigDecimal(value).toPlainString()
		}
		
        WebUI.setText(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Output Multiplier Value_field'),value)
		
		println "Row index: $index"
		
		println "Cell value: ${row.getCell(0)}"
		
		def entertext = WebUI.getAttribute(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Output Multiplier Value_field'),"value")
		
		println "Entered text: " + entertext
		
		def maxchar= entertext.length()
		
		assert maxchar == maxlength
		
		print "maximum length is 8\n"
		
		WebUI.click(saveBtn)
		
		def successmsg=WebUI.getText(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/save_sucessfull message'))
		
		print successmsg
		
		WebUI.click(btn)
	}
	WebUI.closeBrowser()
	
	/*
// Call the login test case
WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)

// Set browser window size to 1920x1080
DriverFactory.getWebDriver().manage().window().setSize(new Dimension(1920, 1080))

// Click WORK OUTPUT link
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance/a_WORK OUTPUT'))

// Switch to window titled 'ProHance Work Output'
WebUI.switchToWindowTitle('ProHance Work Output')

// Click SIDEBAR MENU
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/div_SIDEBAR MENU'))

// Click Administration span
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/span_Administration'))

// Click Work Output Normalization list item
WebUI.click(findTestObject('Normalization Screen/Page_ProHance Work Output/li_Work Output Normalization'))

// Switch to the content frame with timeout 10 seconds
WebUI.switchToFrame(findTestObject('Normalization Screen/Page_ProHance Work Output/frame'), 10)

// Click the type2 modify icon
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/type2_modify_icon'))

// Wait for the page to load
WebUI.waitForPageLoad(10)

def maxcharlength = WebUI.getAttribute(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Output Multiplier Value_field'),'maxlength')

def maxlength = maxcharlength.toInteger()

def saveBtn = findTestObject('Normalization Screen/Page_ProHance Work Output/save button')

// Wait until the save button is clickable
WebUI.waitForElementClickable(saveBtn, 10)

TestObject btn = findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/type_modify_icon')

// Wait until the modify icon button is clickable
WebUI.waitForElementClickable(btn, 10)

// Set non-numeric characters into the Output Multiplier Value field
WebUI.setText(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Output Multiplier Value_field'),'Output Multiplier Value')

// Click the save button
WebUI.click(saveBtn)

// Wait for an alert to appear
WebUI.waitForAlert(5)

def alert= WebUI.findWebElement(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/alert'))

def alertmsg=alert.getText();

def expectedtext="Output Multiplier Value should be numeric and shall be greater than 0, accepts up to 2 decimal places."

assert alertmsg==expectedtext

// Click to accept the alert
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/alertaccept'))

// Set special characters into the Output Multiplier Value field
WebUI.setText(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Output Multiplier Value_field'),'!@#$%%^^^&*)(**^$$')

// Click the save button
WebUI.click(saveBtn)

// Wait for an alert to appear
WebUI.waitForAlert(5)

def alertmsg1=alert.getText();

assert alertmsg==expectedtext

// Click to accept the alert
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/alertaccept'))

// Set a large integer into the Output Multiplier Value field
WebUI.setText(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Output Multiplier Value_field'),'1245123')

// Click the save button
WebUI.click(saveBtn)

def numberalert = WebUI.findWebElement(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/alert'))

// Wait for the numeric-alert element to be visible
WebUI.waitForElementVisible(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/alert'),10)

// Small delay to ensure alert text is available
WebUI.delay(1)

def alertmsg3 = numberalert.getText()

def Expectedmsg ="Output Multiplier Value should not be more than 99999.99"

assert alertmsg3==Expectedmsg

// Click to accept the alert
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/alertaccept'))

// Set a valid decimal within limits into the Output Multiplier Value field
WebUI.setText(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Output Multiplier Value_field'),'12451.23')

// Click the save button
WebUI.click(saveBtn)

// Load the Excel file from Data Files
def file = new File(RunConfiguration.getProjectDir() + '/Data Files/Output Multiplier Value.xlsx')

def workbook = new XSSFWorkbook(file)

def sheet = workbook.getSheetAt(0)

// Click the type modify icon to reopen editing
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/type_modify_icon'))

// Wait for the page to load
WebUI.waitForPageLoad(10)

// Wait until the Output Multiplier Value field is visible
WebUI.waitForElementVisible(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Output Multiplier Value_field'), 20)

sheet.iterator().eachWithIndex{ def row, def index ->

if (index == 0)

return

// Ensure the field is visible before interacting
WebUI.waitForElementVisible(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Output Multiplier Value_field'), 20)

def value = row.getCell(0)?.toString()?.trim()

if (value?.contains("E")) {
value = new BigDecimal(value).toPlainString()
}

// Set the value from Excel into the Output Multiplier Value field
WebUI.setText(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Output Multiplier Value_field'),value)

println "Row index: $index"

println "Cell value: ${row.getCell(0)}"

def entertext = WebUI.getAttribute(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Output Multiplier Value_field'),"value")

println "Entered text: " + entertext

def maxchar= entertext.length()

assert maxchar == maxlength

print "maximum length is 8\n"

// Click the save button
WebUI.click(saveBtn)

def successmsg=WebUI.getText(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/save_sucessfull message'))

print successmsg

// Click the modify icon to proceed to next row
WebUI.click(btn)
}
WebUI.closeBrowser()


*/