import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.testobject.ConditionType as ConditionType
import java.util.Map
import java.util.HashMap

import com.kms.katalon.core.webui.driver.DriverFactory
import org.openqa.selenium.Dimension

WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)

DriverFactory.getWebDriver().manage().window().setSize(new Dimension(1920,1080))

WebUI.click(findTestObject('Worktype Definition Screen/Page_ProHance/a_WORK OUTPUT'))

WebUI.switchToWindowTitle('ProHance Work Output')

WebUI.click(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/div_SIDEBAR MENU'))

WebUI.click(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/a_Administration'))

WebUI.click(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/li_Work Type Definition'))

WebUI.switchToFrame(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/iframe'), 10)

def header = WebUI.findWebElements(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/table header'),10)

def page=WebUI.findWebElements(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/pagination'),10)

def numberofpage=page.size()

//def worktypes = []

def Category = []

def wtandcategory= []

println(numberofpage)

for (int i = 1; i <=numberofpage; i++)
	 
	{
		def rows = WebUI.findWebElements(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/worktypes_rows'),10)
			
		for (int j = 1; j <=rows.size(); j++)
				 {
	
					 if (header.size() == 6)
						  {
							
							print("$j")			 
							TestObject obj = new TestObject()
					 
							obj.addProperty("xpath", ConditionType.EQUALS, "//table[@id='CommonDataTableId']/tbody/tr[${j}]/td[2]")
							
							def worktype=WebUI.getText(obj).trim()
							
							//worktypes.add(worktype)
							
							TestObject obj1 = new TestObject()
							
							obj1.addProperty("xpath", ConditionType.EQUALS, "//*[@id='CommonDataTableId']/tbody/tr[${j}]/td[4]/div")
						   
							def Categoryname = WebUI.getText(obj1).trim()
							
							//Category.add(Categoryname)
							
							wtandcategory.add("$worktype-> $Categoryname")

							}
									 
						 else if (header.size() < 6) 
							 
							{
							  TestObject obj2 = new TestObject()
					 
							  obj2.addProperty("xpath", ConditionType.EQUALS, "//table[@id='CommonDataTableId']/tbody/tr[${j}]/td[2]")
							  
							  def worktype=WebUI.getText(obj2).trim()
							  
							   //worktypes.add(worktype)
							   
							   TestObject obj3 = new TestObject()
							   
							   obj3.addProperty("xpath", ConditionType.EQUALS, "//*[@id='CommonDataTableId']/tbody/tr[${j}]/td[3]/div")
							  
							  def Categoryname = WebUI.getText(obj3).trim()
							
							  //Category.add(Categoryname)
						
							 	
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
			
			
			
			def map = [:].withDefault { [] }

           wtandcategory.each {
            def (worktype, category) = it.split("->")*.trim()
            map[category] << worktype
}

 map.each{println it}

WebUI.closeBrowser()