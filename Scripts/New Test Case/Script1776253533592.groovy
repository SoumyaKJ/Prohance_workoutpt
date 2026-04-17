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

WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/Page_ProHance Work Output/Page_ProHance/a_WORK OUTPUT'))

WebUI.switchToWindowTitle('ProHance Work Output')

WebUI.click(findTestObject('Page_ProHance Work Output/div_SIDEBAR MENU'))

WebUI.click(findTestObject('Object Repository/Page_ProHance Work Output/Page_ProHance Work Output/span_Administration'))

WebUI.click(findTestObject('Object Repository/Page_ProHance Work Output/Page_ProHance Work Output/li_Work Type Definition'))

WebUI.rightClick(findTestObject('Object Repository/Page_ProHance Work Output/Page_ProHance Work Output/div_Development Metrics'))

WebUI.verifyElementText(findTestObject('Object Repository/Page_ProHance Work Output/Page_ProHance Work Output/div_Development Metrics'), 
    'Development Metrics')

WebUI.rightClick(findTestObject('Object Repository/Page_ProHance Work Output/Page_ProHance Work Output/div_dipankar_process'))

WebUI.verifyElementText(findTestObject('Object Repository/Page_ProHance Work Output/Page_ProHance Work Output/div_dipankar_process'), 
    '#dipankar_process')

WebUI.click(findTestObject('Object Repository/Page_ProHance Work Output/Page_ProHance Work Output/td_Development Metrics'))

WebUI.click(findTestObject('Object Repository/Page_ProHance Work Output/Page_ProHance Work Output/div_Soumya Admin Account_arrow-top'))

WebUI.click(findTestObject('Object Repository/Page_ProHance Work Output/Page_ProHance Work Output/li_Work Type Category'))

WebUI.setText(findTestObject('Object Repository/Page_ProHance Work Output/Page_ProHance Work Output/input_Search_form-control input-sm'), 
    'Development Metrics')

WebUI.click(findTestObject('Object Repository/Page_ProHance Work Output/Page_ProHance Work Output/a_32'))

WebUI.click(findTestObject('Object Repository/Page_ProHance Work Output/Page_ProHance Work Output/div_configtable tbody tr td     padding 0px_2d422c'))

WebUI.setText(findTestObject('Object Repository/Page_ProHance Work Output/Page_ProHance Work Output/input_Search_form-control input-sm'), 
    '#dipankar_process')

WebUI.rightClick(findTestObject('Object Repository/Page_ProHance Work Output/Page_ProHance Work Output/div_dipankar_process (1)'))

WebUI.verifyElementPresent(findTestObject('Object Repository/Page_ProHance Work Output/Page_ProHance Work Output/div_dipankar_process (1)'), 
    0)

