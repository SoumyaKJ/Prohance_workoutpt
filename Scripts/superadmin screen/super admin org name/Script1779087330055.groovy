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
import com.kms.katalon.core.testobject.ConditionType
import com.kms.katalon.core.testobject.TestObject as TestObject

WebUI.callTestCase(findTestCase('Commons/superadmin login'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.switchToFrame(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/iframe'), 10)

def orgname=WebUI.getText(findTestObject('Object Repository/work time/orgname_in_superadmin screen'))

def rows=WebUI.findWebElements(findTestObject('Object Repository/work time/installation date column'),10)

def orginstalldate

for (int j = 1; j <=rows.size(); j++)
{
	if(orgname == "ProHance Technology")
	{

		
		TestObject obj = new TestObject()
		
		obj.addProperty("xpath", ConditionType.EQUALS, "//table[@id='CommonDataTableId']/tbody/tr[${j}]/td[6]")
	   
		 orginstalldate = WebUI.getText(obj).trim()	
		
		
	}
}

println "$orgname -> $orginstalldate"

WebUI.closeBrowser()

return orginstalldate