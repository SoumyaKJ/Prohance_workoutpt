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
import org.openqa.selenium.WebElement as WebElement

WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/Category/Page_ProHance/a_WORK OUTPUT'))

WebUI.switchToWindowTitle('ProHance Work Output')

WebUI.click(findTestObject('Object Repository/Category/Page_ProHance Work Output/i_Soumya Admin Account_fa fa-chevron-right _d36e5e'))

WebUI.click(findTestObject('Object Repository/Category/Page_ProHance Work Output/span_Administration'))

WebUI.click(findTestObject('Wo_seetings/Page_ProHance Work Output/li_Work Output Settings'))

WebUI.switchToFrame(findTestObject('Wo_seetings/Page_ProHance Work Output/iframe_contentFrame'), 10)

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

def stdandcustom = WebUI.findWebElements(findTestObject('Object Repository/Wo_seetings/Page_ProHance Work Output/std and custom metric options'), 
    10)

def metrics = stdandcustom.collect({ 
        it.getText().trim()
    })

metrics.each({ 
        println(it)
    })

def wtcustommteric=WebUI.callTestCase(findTestCase('WT_custom_metric/all_custom metric'), [:], FailureHandling.STOP_ON_FAILURE)

assert(wtcustommteric.sort()==metrics.sort())

WebUI.closeBrowser()

