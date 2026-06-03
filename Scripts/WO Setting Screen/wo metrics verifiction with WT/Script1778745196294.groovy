import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase

import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import org.openqa.selenium.Dimension
import org.openqa.selenium.WebElement

import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testobject.TestObject
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

TestObject checkbox = findTestObject('Object Repository/Wo_settings/Page_ProHance Work Output/AHT check box')

WebElement element = WebUI.findWebElement(checkbox, 10)

if (!(WebUI.verifyElementChecked(checkbox, 5, FailureHandling.OPTIONAL))) {
    WebUI.click(checkbox)

    println('Checkbox was not selected, now clicked')
} else {
    println('Checkbox already selected')
}

WebUI.click(findTestObject('Object Repository/Wo_settings/Page_ProHance Work Output/actual productive hours_option'))

WebUI.click(findTestObject('Object Repository/Wo_settings/Page_ProHance Work Output/product model'))

def stdandcustom = WebUI.findWebElements(findTestObject('Object Repository/Wo_settings/Page_ProHance Work Output/std and custom metric options'), 
    10)

def metrics = stdandcustom.collect({ 
        it.getText().trim()
    })

metrics.each({ 
        println(it)
    })

def wtcustommteric=WebUI.callTestCase(findTestCase('WT_custom_metric/duration type_custom metric'), [:], FailureHandling.STOP_ON_FAILURE)

assert(wtcustommteric.sort()==metrics.sort())

WebUI.closeBrowser()

