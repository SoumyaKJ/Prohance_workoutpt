import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

WebUI.openBrowser('')

WebUI.maximizeWindow()
String url = 'http://10.10.10.31:3434/prohance'
String username = 'superadmin'
String password = 'superadmin@1'

WebUI.navigateToUrl(url)

WebUI.setText(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance/input_Username_tlogin'),username)

WebUI.setText(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance/input_Password_tpwdsaved'),password)

WebUI.click(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance/input_Captcha Text_btn-login loginbtn'))


def terminatePopup = findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance/input_Would you like to terminate the other_21bc89')

def cookieAlert = findTestObject('Object Repository/Page_ProHance/button_OK')

if (WebUI.waitForElementVisible(terminatePopup, 2, FailureHandling.OPTIONAL)) {
    WebUI.click(terminatePopup)
}

if (WebUI.waitForElementVisible(cookieAlert, 2, FailureHandling.OPTIONAL)) {
    WebUI.click(cookieAlert)
}

