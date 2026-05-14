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

def maxcharlength = WebUI.getAttribute(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/unit text field'),'maxlength')

def maxlength = maxcharlength.toInteger()

def file = new File(RunConfiguration.getProjectDir() + '/Data Files/normalization metric name.xlsx')
	
def workbook = new XSSFWorkbook(file)
	
def sheet = workbook.getSheetAt(0)
	
def saveBtn = findTestObject('Normalization Screen/Page_ProHance Work Output/save button')
	
WebUI.waitForElementClickable(saveBtn, 10)
	
TestObject btn = findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/type_modify_icon')
	
WebUI.waitForElementClickable(btn, 10)
	
WebUI.waitForElementClickable(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/type_modify_icon'), 10)

WebUI.waitForElementVisible(findTestObject('Normalization Screen/Page_ProHance Work Output/metric name field'), 20)

sheet.iterator().eachWithIndex({ def row, def index ->
	
if (index == 0) 
        
	 return
		 
        WebUI.setText(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/unit text field'),row.getCell(0).stringCellValue)
		
		println "Row index: $index"
		
		println "Cell value: ${row.getCell(0)}"
		
		def entertext = WebUI.getAttribute( findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/unit text field'),"value")
		
		println "Entered text: " + entertext
		
		def maxchar= entertext.length()
		
		assert maxchar == maxlength
		
		print "maximum length is 20\n"
		
		WebUI.click(saveBtn)
		
		def successmsg=WebUI.getText(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/save_sucessfull message'))
		
		print successmsg
		
		WebUI.click(btn)
	})
	
WebUI.closeBrowser()
*/
// Call the login test case to authenticate into the application
WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)
// Set browser window size to 1920x1080
DriverFactory.getWebDriver().manage().window().setSize(new Dimension(1920, 1080))
// Click on the WORK OUTPUT link
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance/a_WORK OUTPUT'))
// Switch to the ProHance Work Output window
WebUI.switchToWindowTitle('ProHance Work Output')
// Click the sidebar menu
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/div_SIDEBAR MENU'))
// Click the Administration span
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/span_Administration'))
// Click the Work Output Normalization list item
WebUI.click(findTestObject('Normalization Screen/Page_ProHance Work Output/li_Work Output Normalization'))
// Switch to the content frame for Work Output Normalization
WebUI.switchToFrame(findTestObject('Normalization Screen/Page_ProHance Work Output/frame'), 10)
// Click the modify icon for the type to enable editing
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/type_modify_icon'))
// Wait for the page to finish loading
WebUI.waitForPageLoad(10)
// Retrieve the maxlength attribute from the unit text field
def maxcharlength = WebUI.getAttribute(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/unit text field'),'maxlength')
// Convert maxlength to integer
def maxlength = maxcharlength.toInteger()
// Create a File object for the Excel file containing metric names
def file = new File(RunConfiguration.getProjectDir() + '/Data Files/normalization metric name.xlsx')
// Open the workbook from the file
def workbook = new XSSFWorkbook(file)
// Get the first sheet from the workbook
def sheet = workbook.getSheetAt(0)
// Locate the save button test object
def saveBtn = findTestObject('Normalization Screen/Page_ProHance Work Output/save button')
// Wait until the save button is clickable
WebUI.waitForElementClickable(saveBtn, 10)
// Locate the modify icon test object and assign to btn
TestObject btn = findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/type_modify_icon')
// Wait until the modify icon is clickable
WebUI.waitForElementClickable(btn, 10)
// Wait again for the modify icon to be clickable as an additional check
WebUI.waitForElementClickable(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/type_modify_icon'), 10)
// Wait until the metric name field is visible
WebUI.waitForElementVisible(findTestObject('Normalization Screen/Page_ProHance Work Output/metric name field'), 20)
// Iterate through each row in the sheet skipping the header row
sheet.iterator().eachWithIndex({ def row, def index ->
    if (index == 0)
        return
    // Set the unit text field with the value from the first cell of the current row
    WebUI.setText(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/unit text field'),row.getCell(0).stringCellValue)
    // Print the current row index to the console
    println "Row index: $index"
    // Print the current cell value to the console
    println "Cell value: ${row.getCell(0)}"
    // Retrieve the entered text from the unit text field
    def entertext = WebUI.getAttribute( findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/unit text field'),"value")
    // Print the entered text to the console
    println "Entered text: " + entertext
    // Calculate length of the entered text
    def maxchar= entertext.length()
    // Assert that the entered text length equals the maxlength attribute
    assert maxchar == maxlength
    // Print a message indicating the maximum length validation
    print "maximum length is 20\n"
    // Click the save button to persist changes
    WebUI.click(saveBtn)
    // Retrieve and print the success message after save
    def successmsg=WebUI.getText(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/save_sucessfull message'))
    print successmsg
    // Click the modify icon again to prepare for next iteration
    WebUI.click(btn)
})
// Close the browser once processing is complete
WebUI.closeBrowser()