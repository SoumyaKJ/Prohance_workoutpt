import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import org.openqa.selenium.By
import org.openqa.selenium.JavascriptExecutor
import org.openqa.selenium.WebElement

import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.webui.driver.DriverFactory
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
 

WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/Category/Page_ProHance/a_WORK OUTPUT'))

WebUI.switchToWindowTitle('ProHance Work Output')

WebUI.click(findTestObject('Object Repository/Category/Page_ProHance Work Output/i_Soumya Admin Account_fa fa-chevron-right _d36e5e'))

WebUI.click(findTestObject('Object Repository/Category/Page_ProHance Work Output/span_Administration'))

WebUI.click(findTestObject('Wo_seetings/Page_ProHance Work Output/li_Work Output Settings'))

WebUI.verifyElementText(findTestObject('Object Repository/Wo_seetings/Page_ProHance Work Output/td_Define Work Type Estimated Hours (Handling Ti'), 
    'Define Work Type Estimated Hours (Handling Time) on')

WebUI.verifyElementText(findTestObject('Object Repository/Wo_seetings/Page_ProHance Work Output/td_Data Collection - Label (ProHance Mate)_'), 
    'Data Collection - Label (ProHance Mate)*')

//Data collection field type and number of character verification
WebUI.switchToFrame(findTestObject('Wo_seetings/Page_ProHance Work Output/iframe_contentFrame'), 10)

WebUI.click(findTestObject('Wo_seetings/Page_ProHance Work Output/li_Work Output Settings'))

def generaltab = WebUI.getText(findTestObject('Object Repository/Wo_seetings/Page_ProHance Work Output/general tab'))

def actual = 'General'

assert actual == generaltab

def value = WebUI.getAttribute(findTestObject('Object Repository/Wo_seetings/Page_ProHance Work Output/data label'), 'class')

def Actualvalue = 'textbox'

assert value == Actualvalue

def numberofchar = WebUI.getText(findTestObject('Object Repository/Wo_seetings/Page_ProHance Work Output/datacollection label_character count'))

def actualchar = '#25'

assert numberofchar == actualchar

//Category field type and number of character verification
def value1 = WebUI.getAttribute(findTestObject('Object Repository/Wo_seetings/Page_ProHance Work Output/category label'), 
    'class')

def Actualvalue1 = 'textbox'

assert value1 == Actualvalue1

def numberofchar1 = WebUI.getText(findTestObject('Object Repository/Wo_seetings/Page_ProHance Work Output/category label char'))

def actualchar1 = '#25'

assert numberofchar1 == actualchar1

def AHT_label = WebUI.getText(findTestObject('Object Repository/Wo_seetings/Page_ProHance Work Output/Enable auto computation of AHT_label'))

def AHT_actuallabel = 'Enable auto computation of \'ACT\''

assert AHT_label == AHT_actuallabel

TestObject checkbox = findTestObject('Object Repository/Wo_seetings/Page_ProHance Work Output/AHT check box')

WebElement element = WebUI.findWebElement(checkbox, 10)

if (!(WebUI.verifyElementChecked(checkbox, 5, FailureHandling.OPTIONAL))) {
    WebUI.click(checkbox)

    println('Checkbox was not selected, now clicked')
} else {
    println('Checkbox already selected')
}

WebUI.click(findTestObject('Object Repository/Wo_seetings/Page_ProHance Work Output/actual productive hours_option'))

WebUI.click(findTestObject('Object Repository/Wo_seetings/Page_ProHance Work Output/product model'))

def text = WebUI.getText(findTestObject('Wo_seetings/Page_ProHance Work Output/Select categories for Actual Productive Hours'))

def actaulatext = 'Select categories for \'Actual Productive Hours\''

assert text == actaulatext

WebUI.click(findTestObject('Object Repository/Wo_seetings/Page_ProHance Work Output/AHT_dropdown'))

WebUI.click(findTestObject('Object Repository/Wo_seetings/Page_ProHance Work Output/actual productive hours_option'))

def productoptions = WebUI.findWebElements(findTestObject('Object Repository/Wo_seetings/Page_ProHance Work Output/product_options'), 
    10)

def type = WebUI.getAttribute(findTestObject('Wo_seetings/Page_ProHance Work Output/product_options'), 'type')

if (type == 'checkbox') {
	println('options provided with check box')
}

def inputs = WebUI.findWebElements(
    findTestObject('Object Repository/Wo_seetings/Page_ProHance Work Output/product_options'),
    10
)

def driver = DriverFactory.getWebDriver()

List<WebElement> labels = driver.findElements(By.xpath("//table[@id='BI_table']//label"))
 def options= labels.collect { element1 ->

    String text1 = ((JavascriptExecutor) driver)
            .executeScript("return arguments[0].textContent;", element1)

    return text1?.trim()
}.findAll { it }

println(options)

def wtratingscaleoptions=WebUI.callTestCase(findTestCase('WT rating scale/rating scale options'), [:], FailureHandling.STOP_ON_FAILURE)
 
assert(options==wtratingscaleoptions)

WebUI.closeBrowser()



