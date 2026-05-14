import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.webui.keyword.builtin.SwitchToFrameKeyword

import org.openqa.selenium.WebElement

WebUI.switchToDefaultContent()

WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.waitForElementPresent(findTestObject('Work time category/side bar_admin'),10)

WebUI.click(findTestObject('Work time category/side bar_admin'))

//WebUI.click(findTestObject('Object Repository/work time/organization link'))

WebUI.click(findTestObject('Object Repository/work time/rating scale link'))

WebUI.switchToFrame(findTestObject('Wo_seetings/Page_ProHance Work Output/iframe_contentFrame'), 10)

WebUI.click(findTestObject('Object Repository/work time/business impact label'))

def ratingscale=WebUI.findWebElements(findTestObject('Object Repository/work time/rating scale optons'),10)

def options = ratingscale.collect {
    it.getAttribute('value')
}
options.addAll('Include Activity Away From System')

println options

WebUI.closeBrowser()

return options