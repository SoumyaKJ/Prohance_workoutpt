import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.webui.keyword.builtin.SwitchToFrameKeyword as SwitchToFrameKeyword
import com.kms.katalon.core.webui.common.WebUiCommonHelper as WebUiCommonHelper
import org.openqa.selenium.WebElement as WebElement
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import internal.GlobalVariable as GlobalVariable
import com.kms.katalon.core.testobject.ConditionType

WebUI.switchToDefaultContent()

WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)

//WebUI.click(findTestObject('Object Repository/Work time category/refresh button_prohance'))
WebUI.waitForElementPresent(findTestObject('Work time category/side bar_admin'), 10)

WebUI.click(findTestObject('Work time category/side bar_admin'))

WebUI.click(findTestObject('Object Repository/worktime user screen/users link'))

WebUI.click(findTestObject('Object Repository/worktime user screen/users'))

//WebUI.waitForElementVisible(findTestObject('worktime user screen/more action link'), 10)
WebUI.switchToFrame(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/iframe'), 10)

WebUI.click(findTestObject('Object Repository/worktime user screen/more action link'))

WebUI.click(findTestObject('Object Repository/worktime user screen/user attribute link'))

def attribute = []

List<WebElement> rows = WebUI.findWebElements(findTestObject('Object Repository/worktime user screen/status title'),10)

def types = rows.collect { row ->
	row.getAttribute("class")
}

println(types)

for (int i = 0; i < types.size(); i++) 
	{

	if (types[i].contains("pointer inactiveClass")) {

		TestObject obj3 = new TestObject()

		obj3.addProperty(
			"xpath",
			ConditionType.EQUALS,
			"//table[@id='CommonDataTableId']/tbody/tr[${i + 1}]/td[3]/div"
		)

		def userattribute = WebUI.getText(obj3).trim()

		attribute.add(userattribute)
	}
}

attribute.addAll(['', 'User Role', 'Designation', 'Tenure Range'])

attribute.each { println(it)}

WebUI.closeBrowser()

return attribute