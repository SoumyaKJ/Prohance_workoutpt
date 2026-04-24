import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.testobject.ConditionType as ConditionType
import com.kms.katalon.core.webui.driver.DriverFactory
import org.openqa.selenium.Dimension

WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)

DriverFactory.getWebDriver().manage().window().setSize(new Dimension(1920, 1080))

WebUI.click(findTestObject('Worktype Definition Screen/Page_ProHance/a_WORK OUTPUT'))

WebUI.switchToWindowTitle('ProHance Work Output')

WebUI.click(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/div_SIDEBAR MENU'))

WebUI.click(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/a_Administration'))

WebUI.click(findTestObject('Object Repository/Page_ProHance Work Output/Page_ProHance Work Output/li_Work Type Category'))

WebUI.switchToFrame(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/iframe'), 10)

def row = WebUI.findWebElements(findTestObject('Object Repository/Category/Page_ProHance Work Output/row'), 10)

def categorylist = []

def worktypename=[]

print(row.size())

def page=WebUI.findWebElements(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/pagination'),10)

def numberofpage=page.size()

for (int k = 1; k <=numberofpage; k++)
	
   {

	   for (int i = 1; i <= row.size(); i++) 
		   {
		
			   WebUI.waitForElementVisible(findTestObject('Object Repository/Category/Page_ProHance Work Output/row'), 10)

			   TestObject obj = new TestObject()

			   obj.addProperty('xpath', ConditionType.EQUALS, "//table[@id='CommonDataTableId']/tbody/tr[$i]/td[2]")

			   def Category = WebUI.getText(obj).trim()
	
	
			   TestObject obj3 = new TestObject()
	
			   obj3.addProperty('xpath', ConditionType.EQUALS, "//table[@id='CommonDataTableId']/tbody/tr[$i]/td[4]/a")
	
			   WebUI.click(obj3)

			   WebUI.waitForElementVisible(findTestObject('Object Repository/Page_ProHance Work Output/Page_ProHance Work Output/category_wt'), 10)
	
			   def rows = WebUI.findWebElements(findTestObject('Object Repository/Category/Page_ProHance Work Output/row'), 10)
			   
			   def pages=WebUI.findWebElements(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/pagination'),10)
			  
			    def numberofpages=pages.size()
				
 for ( int p = 1; p <=numberofpages; p++) 
	 {
		 
		 rows = WebUI.findWebElements(findTestObject('Object Repository/Category/Page_ProHance Work Output/row'), 10)
		
		  for (int j = 1; j <= rows.size(); j++)
				 
				   {
		               print("$j\n")
					   
					   print(rows.size())
					   
					   TestObject obj1 = new TestObject()

					   obj1.addProperty('xpath', ConditionType.EQUALS, "//table[@id='CommonDataTableId']/tbody/tr[$j]/td[1]/div")
			 
					   def worktype = WebUI.getText(obj1).trim()
			 
					   categorylist.add("$Category->$worktype")
			
				   }
				   
					 TestObject nextBtn =findTestObject('Worktype Definition Screen/Page_ProHance Work Output/pagination_next button')
			
					 String classValue = WebUI.getAttribute(nextBtn, "class")
	
					 if (classValue.contains("disabled"))
								   {
									   break
								   }
								   WebUI.click(nextBtn)
								   
								   WebUI.waitForElementVisible(findTestObject('Object Repository/Category/Page_ProHance Work Output/row'), 10)
								   
								   WebUI.delay(1)
								   
							 }
		   
					  WebUI.click(findTestObject('Category/Page_ProHance Work Output/back button'))
		   		}
			
    if(numberofpage==k)
		   {
		   
			TestObject nextBtn =findTestObject('Worktype Definition Screen/Page_ProHance Work Output/pagination_next button')
		
			String classValue = WebUI.getAttribute(nextBtn, "class")
		
			if (classValue.contains("disabled"))
				{
					break
				}
				WebUI.click(nextBtn)
		
				WebUI.delay(1)
		   }
   }	   		
   
	def map = [:].withDefault { [] }
	
			   categorylist.each {
				def (worktype, Category) = it.split("->")*.trim()
				map[worktype] << Category
			   }
		 
 map.each{println it}
   
WebUI.closeBrowser()

