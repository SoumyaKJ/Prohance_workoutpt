import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import org.openqa.selenium.Dimension

import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.webui.driver.DriverFactory
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)

DriverFactory.getWebDriver().manage().window().setSize(new Dimension(1920, 1080))

WebUI.click(findTestObject('Object Repository/Category/Page_ProHance/a_WORK OUTPUT'))

WebUI.switchToWindowTitle('ProHance Work Output')

WebUI.click(findTestObject('Object Repository/Category/Page_ProHance Work Output/i_Soumya Admin Account_fa fa-chevron-right _d36e5e'))

WebUI.click(findTestObject('Object Repository/Category/Page_ProHance Work Output/span_Administration'))

WebUI.click(findTestObject('Wo_settings/Page_ProHance Work Output/li_Work Output Settings'))

WebUI.switchToFrame(findTestObject('Wo_settings/Page_ProHance Work Output/iframe_contentFrame'), 10)

def userattribute = WebUI.findWebElements(findTestObject('Object Repository/Wo_settings/Page_ProHance Work Output/user attribute options'),10)

def attributes=[]

def attribute=userattribute.collect { it.getText().trim()}
	
	attribute.each{print(it)}
	
def actual = attribute[0].split(/\r?\n/)


def wtuseratributelist=WebUI.callTestCase(findTestCase('WT_users/WT_Active user attribute list'), [:], FailureHandling.STOP_ON_FAILURE)

assert wtuseratributelist.sort()==actual.sort()

WebUI.closeBrowser()