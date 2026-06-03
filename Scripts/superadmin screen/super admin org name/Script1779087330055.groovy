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

def organizationname = WebUI.findWebElements(findTestObject('Object Repository/work time/orgname_in_superadmin screen'),10)

def rows = WebUI.findWebElements(findTestObject('Object Repository/work time/installation date column'),10)

def orginstalldate = ""

def currentorg

for (int j = 0; j < rows.size(); j++) {

    String orgname = organizationname[j].getText().trim()

    if (orgname.equalsIgnoreCase("JAMOCHA TECH")) {

        TestObject obj = new TestObject()

        obj.addProperty(
                "xpath",
                ConditionType.EQUALS,
                "//table[@id='CommonDataTableId']/tbody/tr[${j + 1}]/td[2]")
		//*[@id="CommonDataTableId"]/tbody/tr[2]/td[2]/span

        orginstalldate = WebUI.getText(obj).trim()
		
		currentorg = new TestObject()

        currentorg.addProperty(
                "xpath",
                ConditionType.EQUALS,
                "//table[@id='CommonDataTableId']/tbody/tr[${j + 1}]/td[2]/span")

        println "${orgname} -> ${orginstalldate}"
		
		

        break
    }

}
WebUI.click(currentorg)

//WebUI.closeBrowser()
