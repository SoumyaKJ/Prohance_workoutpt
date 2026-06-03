import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import org.openqa.selenium.Dimension

import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.webui.driver.DriverFactory
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

WebUI.openBrowser('')

WebUI.maximizeWindow()

DriverFactory.getWebDriver().manage().window().setSize(new Dimension(1920, 1080))

String url = 'https://13.126.98.191:9443/prohance'
String username = 'superadmin'
String password = '1'

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

