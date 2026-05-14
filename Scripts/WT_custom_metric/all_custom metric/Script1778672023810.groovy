import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.webui.keyword.builtin.SwitchToFrameKeyword
import com.kms.katalon.core.testobject.ConditionType
import org.openqa.selenium.WebElement

//WebUI.switchToDefaultContent()

WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)
//WebUI.click(findTestObject('Object Repository/Work time category/refresh button_prohance'))

WebUI.waitForElementPresent(findTestObject('Work time category/side bar_admin'),10)

WebUI.click(findTestObject('Work time category/side bar_admin'))

WebUI.click(findTestObject('Work time category/activities'))

WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/WT_custom metric link'))

WebUI.switchToFrame(findTestObject('Normalization Screen/Page_ProHance Work Output/frame'), 10)

def type=WebUI.findWebElements(findTestObject('Object Repository/work time_custom metric/type_column'),10)

print(type.size())

def customMetricName = []

def typetext=[]

String Text

for (int i=1; i<=type.size(); i++)
		{
			
			TestObject typeObj = new TestObject()
			
				typeObj.addProperty("xpath",ConditionType.EQUALS,"//*[@id='CommonDataTableId']/tbody/tr[${i}]/td[4]")
			
				def typeText= WebUI.getText(typeObj).trim()
				
				TestObject obj1 = new TestObject()
				
				obj1.addProperty("xpath", ConditionType.EQUALS, "//*[@id='CommonDataTableId']/tbody/tr[${i}]/td[6]/div/i")
			   
				 def status = WebUI.getAttribute(obj1, "title").trim()
				
				if (typeText == "Duration" && status.equals("Active"))
					 {
						 TestObject obj2 = new TestObject() 
					 
						 obj2.addProperty("xpath", ConditionType.EQUALS, "//*[@id='CommonDataTableId']/tbody/tr[${i}]/td[2]/div")
	 
						 def custommetric = WebUI.getText(obj2).trim()
			
						 customMetricName.add(custommetric)
					 }
					
		}
		
println customMetricName.size()
		
customMetricName.addAll(['Actual Logged Hours','Expected Logged Hours','Actual Productive Hours','Expected Productive Hours'])

customMetricName.each{println(it)}

WebUI.closeBrowser()

return customMetricName