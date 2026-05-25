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

def options=[]
def userattribute
List<WebElement> rows = WebUI.findWebElements(findTestObject('Object Repository/worktime user screen/status title'),10)

def types = rows.collect { row ->
	row.getAttribute("class")
}
List<WebElement> attributes = WebUI.findWebElements(findTestObject('Object Repository/worktime user screen/user attribute with text box field'),10)
def userattr= attributes.collect { attr ->
	attr.getText().trim()
	
	
}
Map<String, List<String>> dropdownMap = [:]
for (int i = 0; i <rows.size(); i++) 
	{

	if (types[i].contains("pointer inactiveClass") &&"Drop Down".equals(userattr[i].trim()))
		
	{
	
        
		TestObject obj3 = new TestObject()

		obj3.addProperty("xpath",ConditionType.EQUALS,"//table[@id='CommonDataTableId']/tbody/tr[${i+1}]/td[3]/div")

		 userattribute = WebUI.getText(obj3).trim()
		
		attribute.add(userattribute)
		
		TestObject obj2 = new TestObject()

        obj2.addProperty(
        "xpath",
         ConditionType.EQUALS,
         "//table[@id='CommonDataTableId']/tbody/tr[${i + 1}]/td[1]/a/i")

       WebUI.click(obj2)
	   
	   WebUI.delay(1)
	   
	   // get dropdown options
	   List<WebElement> option = WebUI.findWebElements(
		   findTestObject('Object Repository/worktime user screen/dropdown user attribute options'),
		   10)
	   
	   // convert to text list
	   List<String> optionTexts = option.collect {
		   it.getText()?.trim()
		   }
		   
		   WebUI.click(findTestObject('Object Repository/worktime user screen/back button'))
		  
		   //println "User Attribute: ${userattribute}"
		    //println  "Dropdown Options: ${optionTexts}"
		  
		   dropdownMap[userattribute] = optionTexts
		   
				}
		   
	}
	
	
	dropdownMap.each { attributeName, dropdownValues  ->
		
			println("User Attribute: " + attributeName)
		
			println("Dropdown Options: " + dropdownValues )
		
			println("--------------------------------")
	
	}

WebUI.closeBrowser()

return dropdownMap