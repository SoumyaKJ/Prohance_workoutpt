/*import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
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

String url = findTestData('Data Files/login').getValue('URL', 1)
String username = findTestData('Data Files/login').getValue('USERNAME', 1)
String password = findTestData('Data Files/login').getValue('PASSWORD', 1)

WebUI.navigateToUrl(url)

WebUI.setText(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance/input_Username_tlogin'), username)

WebUI.setText(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance/input_Password_tpwdsaved'), password)

WebUI.click(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance/input_Captcha Text_btn-login loginbtn'))

def terminatePopup = findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance/input_Would you like to terminate the other_21bc89')

def cookieAlert = findTestObject('Object Repository/Page_ProHance/button_OK')

if (WebUI.waitForElementVisible(terminatePopup, 2, FailureHandling.OPTIONAL)) {
	WebUI.click(terminatePopup)
}

if (WebUI.waitForElementVisible(cookieAlert, 2, FailureHandling.OPTIONAL)) {
	WebUI.click(cookieAlert)
}*/
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testdata.TestData
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

TestData data = findTestData('Data Files/login')

int rowCount = data.getRowNumbers()

print rowCount

for (int row = 1; row <= rowCount; row++) {

	String url = data.getValue('URL', row)
	String username = data.getValue('USERNAME', row)
	String password = data.getValue('PASSWORD', row)

	WebUI.openBrowser('')
	WebUI.maximizeWindow()

	WebUI.navigateToUrl(url)

	WebUI.setText(
		findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance/input_Username_tlogin'),
		username
	)

	WebUI.setText(
		findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance/input_Password_tpwdsaved'),
		password
	)

	WebUI.click(
		findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance/input_Captcha Text_btn-login loginbtn')
	)

	def terminatePopup = findTestObject(
		'Object Repository/Worktype Definition Screen/Page_ProHance/input_Would you like to terminate the other_21bc89'
	)

	def cookieAlert = findTestObject(
		'Object Repository/Page_ProHance/button_OK'
	)

	if (WebUI.waitForElementVisible(terminatePopup, 2, FailureHandling.OPTIONAL)) {
		WebUI.click(terminatePopup)
	}

	if (WebUI.waitForElementVisible(cookieAlert, 2, FailureHandling.OPTIONAL)) {
		WebUI.click(cookieAlert)
	}

	WebUI.closeBrowser()
}
