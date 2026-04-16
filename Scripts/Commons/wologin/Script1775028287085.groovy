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

WebUI.openBrowser('')

WebUI.maximizeWindow()

WebUI.navigateToUrl(GlobalVariable.URL)

WebUI.setText(findTestObject('Object Repository/democlass/Page_ProHance/input_Username_tlogin'), GlobalVariable.USERNAME)

WebUI.setText(findTestObject('Object Repository/democlass/Page_ProHance/input_Password_tpwdsaved'), GlobalVariable.PASSWORD)

WebUI.setText(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance/input_Username_tlogin'), 'soumya')

WebUI.setText(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance/input_Password_tpwdsaved'), '1')

WebUI.click(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance/input_Captcha Text_btn-login loginbtn'))

def terminatePopup = findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance/input_Would you like to terminate the other_21bc89')

def cookieAlert = findTestObject('Object Repository/Page_ProHance/button_OK')

if (WebUI.waitForElementVisible(terminatePopup, 2, FailureHandling.OPTIONAL)) {
	WebUI.click(terminatePopup)
}

if (WebUI.waitForElementVisible(cookieAlert, 2, FailureHandling.OPTIONAL)) {
	WebUI.click(cookieAlert)
}
