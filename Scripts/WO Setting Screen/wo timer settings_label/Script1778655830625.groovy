import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import org.openqa.selenium.WebElement as WebElement

WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/Category/Page_ProHance/a_WORK OUTPUT'))

WebUI.switchToWindowTitle('ProHance Work Output')

WebUI.click(findTestObject('Object Repository/Category/Page_ProHance Work Output/i_Soumya Admin Account_fa fa-chevron-right _d36e5e'))

WebUI.click(findTestObject('Object Repository/Category/Page_ProHance Work Output/span_Administration'))

WebUI.click(findTestObject('Wo_seetings/Page_ProHance Work Output/li_Work Output Settings'))

WebUI.switchToFrame(findTestObject('Wo_seetings/Page_ProHance Work Output/iframe_contentFrame'), 10)

WebUI.verifyElementText(findTestObject('Object Repository/Wo_seetings/Page_ProHance Work Output/wo_timer'),
	'WO Timer Settings')

WebUI.verifyElementText(findTestObject('Object Repository/Wo_seetings/Page_ProHance Work Output/timer_label1'),
	'Display/Edit Output Value')

def type1 = WebUI.getAttribute(findTestObject('Object Repository/Wo_seetings/Page_ProHance Work Output/timer_label1_checkbox'), 'type')

if (type1 == 'checkbox')
	{
	println ('options provided with check box')
	}
WebUI.verifyElementText(findTestObject('Object Repository/Wo_seetings/Page_ProHance Work Output/wo_timer_label2'),
		'Edit Timer Reference')
	
def type2= WebUI.getAttribute(findTestObject('Object Repository/Wo_seetings/Page_ProHance Work Output/wo timer_label2_check box'), 'type')
	
if (type2 == 'checkbox')
		{
		println ('options provided with check box')
		}
WebUI.verifyElementText(findTestObject('Object Repository/Wo_seetings/Page_ProHance Work Output/wo_timer_label3'),
			'Display Reset, Pause, Start/Stop buttons')
		
def type3= WebUI.getAttribute(findTestObject('Object Repository/Wo_seetings/Page_ProHance Work Output/wo_timer_label3_check box'), 'type')
		
if (type3 == 'checkbox')
	{
		println ('options provided with check box')
		}

WebUI.verifyElementText(findTestObject('Object Repository/Wo_seetings/Page_ProHance Work Output/wo_timer_label4'),
			'Timer duration to be reported in*')

def unit = WebUI.findWebElements(findTestObject('Object Repository/Wo_seetings/Page_ProHance Work Output/timer unit dropdown option'),
	10)

def actualValues = unit.collect { it.getText().trim()}

println(actualValues)

assert actualValues == ['Hours', 'Minutes', 'Seconds']

WebUI.closeBrowser()
