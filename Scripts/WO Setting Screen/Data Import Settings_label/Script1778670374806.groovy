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

def expectedlabel=WebUI.getText(findTestObject('Object Repository/Wo_settings/Page_ProHance Work Output/data approval setting label2'))

def actallabel="Allow users to upload"

assert expectedlabel.startsWith(actallabel)

def label = WebUI.findWebElements(findTestObject('Object Repository/Wo_settings/Page_ProHance Work Output/data approval_check box option label'),10)

def actualValues = label.collect { it.getText().trim()}

println(actualValues)

assert actualValues == ['Excel Import','Web Form']

WebUI.verifyElementText(findTestObject('Object Repository/Wo_settings/Page_ProHance Work Output/data import setting label1'),
	'Allow users to upload WO data when no Work Time data has been captured from')


def type = WebUI.getAttribute(findTestObject('Object Repository/Wo_settings/Page_ProHance Work Output/data approval_check box verification'),'type')

if (type == 'checkbox')
	{
	println("options provided with check box")
}

WebUI.closeBrowser()




