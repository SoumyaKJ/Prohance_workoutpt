import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import org.openqa.selenium.Dimension
import org.openqa.selenium.WebElement

import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testobject.ConditionType
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.webui.driver.DriverFactory
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)

DriverFactory.getWebDriver().manage().window().setSize(new Dimension(1920, 1080))
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

def userattributelist=[]

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


attribute.addAll([ 'User Role', 'Designation', 'Tenure Range'])

attribute.each { println(it)}

WebUI.closeBrowser()

return attribute
