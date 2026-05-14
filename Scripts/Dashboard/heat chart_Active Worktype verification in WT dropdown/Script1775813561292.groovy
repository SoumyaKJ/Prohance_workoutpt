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
import com.kms.katalon.core.webui.driver.DriverFactory
import org.openqa.selenium.Dimension

WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)

DriverFactory.getWebDriver().manage().window().setSize(new Dimension(1920,1080))

CustomKeywords.'com.prohance.workoutput.common.Heatchartwidget.addHeatChartWidget'()

def text = WebUI.getText(findTestObject('Heat Chart/Page_ProHance Work Output/heat chart label'))

print(text)

if (text == 'Heat Chart')
	 {
		 WebUI.click(findTestObject('Heat Chart/Page_ProHance Work Output/filter'))

		 WebUI.click(findTestObject('Output Trend Report/Page_ProHance Work Output/All groups label'))

		 WebUI.click(findTestObject('Output Trend Report/Page_ProHance Work Output/metric selection'))

		 WebUI.click(findTestObject('Output Trend Report/Page_ProHance Work Output/all category'))
		 
		 def totalworktypes = WebUI.findWebElements(findTestObject('Output Trend Report/Page_ProHance Work Output/all worktypes'), 
        10).collect({  it.getText().trim()}).findAll({ it != 'All Work Types'}) // exclude "All Work Types"
        
		println(totalworktypes.size())
		
	    WebUI.waitForElementVisible(findTestObject('Output Trend Report/Page_ProHance Work Output/fetch'), 10)

		WebUI.click(findTestObject('Output Trend Report/Page_ProHance Work Output/fetch'))

		def worktype= WebUI.callTestCase(findTestCase('Work type definition screen/worktype definition screen all mapped wt with active'), [:], FailureHandling.STOP_ON_FAILURE)

  if (worktype == totalworktypes)
		 {
			 println('Outputput Widgtes:All Active worktypes present in the worktype dropdowns')
		 } 
	else 
		{
        println('Outputput Widgtes:All Active worktypes not present in the worktype dropdowns')
    }
}

WebUI.closeBrowser()

