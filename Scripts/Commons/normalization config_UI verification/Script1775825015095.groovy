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
import com.kms.katalon.core.util.KeywordUtil as KeywordUtil
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import org.openqa.selenium.WebElement as WebElement
import com.kms.katalon.core.testobject.ConditionType as ConditionType
import org.openqa.selenium.By as By

WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/Page_ProHance/a_WORK OUTPUT'))

WebUI.switchToWindowTitle('ProHance Work Output')

WebUI.click(findTestObject('Object Repository/Page_ProHance Work Output/div_SIDEBAR MENU'))

WebUI.click(findTestObject('Object Repository/Page_ProHance Work Output/span_Administration'))

WebUI.click(findTestObject('Normalization Screen/Page_ProHance Work Output/li_Work Output Normalization'))

WebUI.switchToFrame(findTestObject('Normalization Screen/Page_ProHance Work Output/iframe'), 10)

WebUI.waitForElementVisible(findTestObject('Normalization Screen/Page_ProHance Work Output/rows'), 50)

def headers = WebUI.findWebElements(findTestObject('Normalization Screen/Page_ProHance Work Output/header_name'), 
    10)

println(headers.size())

//All columns are enable
if (headers.size() == 7) {
    //checking table headers
    headers = WebUI.findWebElements(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/table header'), 
        10).collect({ 
            it.getText().trim()
        })

    println("Actual headers:$headers")

    def Expectedheaders = ['Actions', 'Type', 'Metric Name', 'Divisor', 'Output Multiplier', 'Unit', 'Status']

    if (headers == Expectedheaders) {
        println('Normalization table columns headers are displaying as expected')
    } else {
        println('Normalization table columns headers are not displaying as expected')
    }
    
    if (headers != Expectedheaders) {
        KeywordUtil.markFailedAndStop('Fail')
    }
    
    //Action colum modify icon verifivction
    def modify = WebUI.findWebElements(findTestObject('Normalization Screen/Page_ProHance Work Output/modify_icon'), 10).collect(
        { 
            it.getAttribute('title').trim()
        })

    //collection of default normalization name	
    def normname = WebUI.findWebElements(findTestObject('Normalization Screen/Page_ProHance Work Output/normalization type name'), 
        10).collect({ 
            it.getText().trim()
        })

    println("$modify $normname")

    def actualactions = ['Modify', 'Modify', 'Modify', 'Modify']

    def Actualnormname = ['Type 1', 'Type 2', 'Type 3', 'Type 4']

    if (modify == actualactions) {
        println('modify action is enable')
    } else {
        println('modify action is not enable')
    }
    
    if (modify != actualactions) {
        KeywordUtil.markFailedAndStop('Fail')
    }
    
    if (normname == Actualnormname)
		 {
        println('Normlaiztaion name is same as expected')
    }
	 else {
        println('Normlaiztaion name is not same as expected')
    }
    
    if (normname != Actualnormname) {
        KeywordUtil.markFailedAndStop('Fail' //Modify Action disable	
            //checking table headers
            ) //collection of default normalization name
    }
} else if (headers.size() < 7) {
    headers = WebUI.findWebElements(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/table header'), 
        10).collect({ 
            it.getText().trim()
        })

    println("Actual headers:$headers")

    def Expectedheaders = ['Type', 'Metric Name', 'Divisor', 'Output Multiplier', 'Unit', 'Status']

    if (headers == Expectedheaders) {
        println('Normalization table columns headers are displaying as expected')
    } else {
        println('Normalization table columns headers are not displaying as expected')
    }
    
    if (headers != Expectedheaders) {
        KeywordUtil.markFailedAndStop('Fail')
    }
    
    def normname = WebUI.findWebElements(findTestObject('Normalization Screen/Page_ProHance Work Output/normaname without action column'), 
        10).collect({ 
            it.getText().trim()
        })

    println("$normname")

    def Actualnormname = ['Type 1', 'Type 2', 'Type 3', 'Type 4']

    if (normname == Actualnormname) {
        println('Normlaiztaion name is same as expected')
    } else {
        println('Normlaiztaion name is not same as expected')
    }
    
    if (normname != Actualnormname) {
        KeywordUtil.markFailedAndStop('Fail')
    }
}

def reecordstodisplay = WebUI.getText(findTestObject('Normalization Screen/Page_ProHance Work Output/records to display'), 
    FailureHandling.STOP_ON_FAILURE)

println(reecordstodisplay)

WebUI.setText(findTestObject('Normalization Screen/Page_ProHance Work Output/search field'), 'Type 4')

def searchnormname = WebUI.findWebElements(findTestObject('Normalization Screen/Page_ProHance Work Output/normaname without action column'), 
    10).collect({ 
        it.getText().trim()
    })

println("collected from tablenormalization name:$searchnormname")

WebUI.verifyElementText(findTestObject('Normalization Screen/Page_ProHance Work Output/export label'), '|   Export: ')

def export = WebUI.findWebElements(findTestObject('Normalization Screen/Page_ProHance Work Output/export report'), 10).collect(
    { 
        it.getAttribute('title').trim()
    })

print(export)

def headername=WebUI.findWebElement(findTestObject('Normalization Screen/Page_ProHance Work Output/status header'))

println(headername.getText())


WebUI.closeBrowser()

