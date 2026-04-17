import com.kms.katalon.core.testobject.TestObject

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
import com.kms.katalon.core.testobject.ConditionType as ConditionType
import org.openqa.selenium.By as By
import java.util.List as List
import org.openqa.selenium.WebElement

WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Worktype Definition Screen/Page_ProHance/a_WORK OUTPUT'))

WebUI.switchToWindowTitle('ProHance Work Output')

WebUI.click(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/div_SIDEBAR MENU'))

WebUI.click(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/a_Administration'))

WebUI.click(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/li_Work Type Definition'))

WebUI.switchToFrame(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/iframe'), 10)

def header = WebUI.findWebElements(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/table header'),10)

def page=WebUI.findWebElements(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/pagination'),10)

def numberofpage=page.size()

def value

def worktypes = []

println(numberofpage)

for (int i = 1; i <=numberofpage; i++)
	 
	{    
		 WebUI.waitForElementVisible(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/worktypes_rows'), 10)

		 List<WebElement> rows = WebUI.findWebElements(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/worktypes_rows'),10)
					  
			for (int j = 1; j <rows.size(); j++) 
				 {
	
					 
					 TestObject obj = new TestObject() 
					 
					 obj.addProperty("xpath", ConditionType.EQUALS, "//*[@id='CommonDataTableId']/tbody/tr[${j}]/td[5]")
					
					 def numberofgroup = WebUI.getText(obj).trim().toInteger()
					 
					 
					 TestObject obj1 = new TestObject()
					 
					 obj1.addProperty("xpath", ConditionType.EQUALS, "//*[@id='CommonDataTableId']/tbody/tr[${j}]/td[6]/span/i")
					
					 def status = WebUI.getAttribute(obj1, "title").trim()
				
				if (numberofgroup > 0 && status.equals("Click to inactivate"))
					 
							 {
									
						 
									 if (header.size() == 6) {
										 
										 TestObject obj3 = new TestObject()
					 
										 obj3.addProperty("xpath", ConditionType.EQUALS, "//table[@id='CommonDataTableId']/tbody/tr[${j}]/td[2]")
										 def worktype=WebUI.getText(obj3).trim()
										 worktypes.add(worktype)
										// print(worktypes)
										 
										 
									 }
									 else if (header.size() < 6) {
										  TestObject obj4 = new TestObject()
					 
										 obj4.addProperty("xpath", ConditionType.EQUALS, "//table[@id='CommonDataTableId']/tbody/tr[${j}]/td[2]")
										 def worktype=WebUI.getText(obj4).trim()
										 worktypes.add(worktype)
										 //print(worktypes)
									 }
						
					 }
					 
				
					 
			}
			
			
		
			TestObject nextBtn =findTestObject('Worktype Definition Screen/Page_ProHance Work Output/pagination_next button')
			
			String classValue = WebUI.getAttribute(nextBtn, "class")
			
			if (classValue.contains("disabled"))
				 {
					 break
				 }
				 WebUI.click(nextBtn)
				
				 WebUI.delay(1)
				
			}
	 
			worktypes.each{println it}

WebUI.closeBrowser()

