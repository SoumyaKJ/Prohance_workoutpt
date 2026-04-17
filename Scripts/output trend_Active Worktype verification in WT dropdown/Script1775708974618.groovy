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
import com.kms.katalon.core.webui.keyword.builtin.CloseBrowserKeyword as CloseBrowserKeyword
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

//Active work type checking in outout trend widgets
WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/Output Trend Report/Page_ProHance/a_WORK OUTPUT'))

WebUI.switchToWindowTitle('ProHance Work Output')

WebUI.click(findTestObject('Object Repository/Output Trend Report/Page_ProHance Work Output/span_Add New Widget'))

WebUI.click(findTestObject('Object Repository/Output Trend Report/Page_ProHance Work Output/div_Output Trend'))

WebUI.click(findTestObject('Object Repository/Output Trend Report/Page_ProHance Work Output/div_ADD WIDGET'))

def text = WebUI.getText(findTestObject('Object Repository/Output Trend Report/Page_ProHance Work Output/span_Business Rating Avg. AHT For_filter_3'))

println(text)

if (text == 'Output Trend') {
    WebUI.click(findTestObject('Output Trend Report/Page_ProHance Work Output/output trend filter'))

    WebUI.click(findTestObject('Output Trend Report/Page_ProHance Work Output/All groups label'))

    WebUI.click(findTestObject('Output Trend Report/Page_ProHance Work Output/metric selection'))

    WebUI.click(findTestObject('Output Trend Report/Page_ProHance Work Output/all category'))

    WebUI.waitForPageLoad(2)

    def totalworktypes = WebUI.findWebElements(findTestObject('Output Trend Report/Page_ProHance Work Output/all worktypes'), 
        10).collect({ 
            it.getText().trim()
        }).findAll({ 
            it != 'All Work Types' // exclude "All Work Types"
        })

    println(totalworktypes.size())

    WebUI.waitForElementVisible(findTestObject('Output Trend Report/Page_ProHance Work Output/fetch'), 10)

    WebUI.click(findTestObject('Output Trend Report/Page_ProHance Work Output/fetch'))

    //WebUI.callTestCase(findTestCase('worktype definition screen'), [:], FailureHandling.STOP_ON_FAILURE)

    def w = WebUI.callTestCase(findTestCase('worktype definition screen all mapped wt with active'), [:], FailureHandling.STOP_ON_FAILURE)

    //def wStrings = w.collect { it.toString().trim() }
    //def totalStrings = totalworktypes.collect { it.toString().trim() }
    def wStrings = CustomKeywords.'com.prohance.workoutput.common.Sample.cleanList'(w)

    def totalStrings = CustomKeywords.'com.prohance.workoutput.common.Sample.cleanList'(totalworktypes)

	print(wStrings)
	
	print(totalStrings)
	
	
    if (wStrings == totalStrings) {
        println('Outputput Widgtes:All Active worktypes present in the worktype dropdowns')
    } else {
        println('Outputput Widgtes:All Active worktypes not present in the worktype dropdowns')
    }
}

WebUI.closeBrowser()

