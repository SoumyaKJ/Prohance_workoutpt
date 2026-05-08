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

WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/type_modify_icon'))

WebUI.waitForPageLoad(10)

def saveBtn = findTestObject('Normalization Screen/Page_ProHance Work Output/save button')

WebUI.waitForElementClickable(saveBtn, 10)

TestObject btn = findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/type_modify_icon')

WebUI.waitForElementClickable(btn, 10)

def maxcharlength = WebUI.getAttribute(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/metric name field'), 
    'maxlength')

def maxlength = maxcharlength.toInteger()

def file = new File(RunConfiguration.getProjectDir() + '/Data Files/normalization metric name.xlsx')
	
def workbook = new XSSFWorkbook(file)
	
def sheet = workbook.getSheetAt(0)
	
WebUI.waitForElementClickable(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/type_modify_icon'), 10)

WebUI.waitForElementVisible(findTestObject('Normalization Screen/Page_ProHance Work Output/metric name field'), 20)

sheet.iterator().eachWithIndex({ def row, def index ->
	
if (index == 0)
		 
        WebUI.setText(findTestObject('Normalization Screen/Page_ProHance Work Output/metric name field'), row.getCell(0).stringCellValue)
		
		println "Row index: $index"
		
		println "Cell value: ${row.getCell(0)}"
		
		def entertext = WebUI.getAttribute( findTestObject('Normalization Screen/Page_ProHance Work Output/metric name field'),"value")
		
		println "Entered text: " + entertext
		
		def maxchar= entertext.length()
		
		assert maxchar == maxlength
		
		print "maximum length is 100\n"
		
		WebUI.click(saveBtn)
		
		def successmsg=WebUI.getText(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/save_sucessfull message'))
		
		print successmsg
		
		WebUI.click(btn)
	})
	
WebUI.closeBrowser()