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
import java.util.Map as Map

WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/Category/Page_ProHance/a_WORK OUTPUT'))

WebUI.switchToWindowTitle('ProHance Work Output')

WebUI.click(findTestObject('Object Repository/Category/Page_ProHance Work Output/i_Soumya Admin Account_fa fa-chevron-right _d36e5e'))

WebUI.click(findTestObject('Object Repository/Category/Page_ProHance Work Output/span_Administration'))

WebUI.click(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/li_Work Type Definition'))

WebUI.switchToFrame(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/iframe'), 10)

WebUI.waitForElementVisible(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/worktypes_rows'), 10)

def pagination = WebUI.findWebElements(findTestObject('Normalization Screen/Page_ProHance Work Output/pagination'), 10)

def header = WebUI.findWebElements(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/table header'),10)

def numberofpage = pagination.size()

println(numberofpage)

def Data = []

if (numberofpage > 3) {
	
    for (int i = 1; i <= numberofpage; i++) 
		{
        def rows = WebUI.findWebElements(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/worktypes_rows'),10)
		
		for (int j = 1; j <=rows.size(); j++)
			{
				def totalnumberofrows=rows.size()
				
				print("$j")
				
				print("$totalnumberofrows")
				
				if (header.size() == 6) {
				
				//category from Wt defintino table
						
				TestObject obj = new TestObject()
				
				obj.addProperty("xpath", ConditionType.EQUALS, "//*[@id='CommonDataTableId']/tbody/tr[${j}]/td[4]/div")
			   
				def Category = WebUI.getText(obj).trim()
				
				//worktype from Wt defintino table
				TestObject obj1 = new TestObject()
				
				obj1.addProperty("xpath", ConditionType.EQUALS, "//*[@id='CommonDataTableId']/tbody/tr[${j}]/td[2]/div")
			   
				def worktype = WebUI.getText(obj1).trim()
				
				print("$worktype,$Category")
					  
	 //category screen
		 WebUI.waitForElementVisible(findTestObject('Object Repository/Page_ProHance Work Output/div_SIDEBAR MENU'), 10)
		 
		 WebUI.switchToDefaultContent()
		
		 WebUI.click(findTestObject('Object Repository/Page_ProHance Work Output/div_SIDEBAR MENU'))
		 
		 WebUI.click(findTestObject('Object Repository/Page_ProHance Work Output/Page_ProHance Work Output/li_Work Type Category'))
		 
		 WebUI.waitForElementVisible(findTestObject('Object Repository/Page_ProHance Work Output/Page_ProHance Work Output/li_Work Type Category'), 10)
		 
		 //seraching category in category screen
		 
         TestObject searchBox = findTestObject('Object Repository/Page_ProHance Work Output/Page_ProHance Work Output/input_Search_form-control input-sm')
			
		 WebUI.waitForElementVisible(searchBox, 10)
		 
         WebUI.clearText(searchBox)
		 
         WebUI.setText(searchBox, Category)
  
		 WebUI.click(findTestObject('Object Repository/Page_ProHance Work Output/Page_ProHance Work Output/a_32'))
					  
		 TestObject worktypeserach=findTestObject('Object Repository/Page_ProHance Work Output/Page_ProHance Work Output/input_Search_form-control input-sm')					
		 
		 WebUI.waitForElementVisible(worktypeserach, 10)
		 
         WebUI.clearText(worktypeserach)
		
         WebUI.setText(worktypeserach, worktype)
		 
		 WebUI.waitForElementVisible(findTestObject('Object Repository/Page_ProHance Work Output/Page_ProHance Work Output/category_wt'), 10)
		 
		 WebUI.switchToFrame(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/iframe'), 10)
		 
		 def categoryworktype=  WebUI.findWebElement(findTestObject('Object Repository/Page_ProHance Work Output/Page_ProHance Work Output/category_wt'))
		 
		 def worktypename=categoryworktype.getText().trim()
		 
		 print(worktypename)
					
					if (worktypename == worktype)
					{
						print("worktype present under mapped category")
					}
					else
						{
						print("worktype present under mapped category")
					}
                     
				}
				
				else if(header.size()< 6) 
					{ 
							TestObject obj = new TestObject()
				
				obj.addProperty("xpath", ConditionType.EQUALS, "//*[@id='CommonDataTableId']/tbody/tr[${j}]/td[3]/div")
			   
				def Category = WebUI.getText(obj).trim()
				
				TestObject obj1 = new TestObject()
				
				obj1.addProperty("xpath", ConditionType.EQUALS, "//*[@id='CommonDataTableId']/tbody/tr[${j}]/td[1]/div")
			   
				def worktype = WebUI.getText(obj1).trim()
				
				print("$worktype,$Category")
					  
	 //category screen
		 WebUI.waitForElementVisible(findTestObject('Object Repository/Page_ProHance Work Output/div_SIDEBAR MENU'), 10)
		 
		 WebUI.switchToDefaultContent()
		
		 WebUI.click(findTestObject('Object Repository/Page_ProHance Work Output/div_SIDEBAR MENU'))
		 
		 WebUI.click(findTestObject('Object Repository/Page_ProHance Work Output/Page_ProHance Work Output/li_Work Type Category'))
		 
		 WebUI.waitForElementVisible(findTestObject('Object Repository/Page_ProHance Work Output/Page_ProHance Work Output/li_Work Type Category'), 10)
		 
		 //seraching category in category screen
		 
         TestObject searchBox = findTestObject('Object Repository/Page_ProHance Work Output/Page_ProHance Work Output/input_Search_form-control input-sm')
			
		 WebUI.waitForElementVisible(searchBox, 10)
		 
         WebUI.clearText(searchBox)
		 
         WebUI.setText(searchBox, Category)
  
		 WebUI.click(findTestObject('Object Repository/Page_ProHance Work Output/Page_ProHance Work Output/a_32'))
					  
		 TestObject worktypeserach=findTestObject('Object Repository/Page_ProHance Work Output/Page_ProHance Work Output/input_Search_form-control input-sm')					
		 
		 WebUI.waitForElementVisible(worktypeserach, 10)
		 
         WebUI.clearText(worktypeserach)
		
         WebUI.setText(worktypeserach, worktype)
		 
		 WebUI.waitForElementVisible(findTestObject('Object Repository/Page_ProHance Work Output/Page_ProHance Work Output/category_wt'), 10)
		 
		 WebUI.switchToFrame(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/iframe'), 10)
		 
		 def categoryworktype=  WebUI.findWebElement(findTestObject('Object Repository/Page_ProHance Work Output/Page_ProHance Work Output/category_wt'))
		 
		 def worktypename=categoryworktype.getText().trim()
		 
		 print(worktypename)
					
					if (worktypename == worktype)
					{
						print("worktype present under mapped category")
					}
					else
						{
						print("worktype present under mapped category")
					}
                     
				}
				
	WebUI.switchToDefaultContent()
	
	WebUI.click(findTestObject('Object Repository/Page_ProHance Work Output/Page_ProHance Work Output/refresh'))
				
	WebUI.waitForElementVisible(findTestObject('Object Repository/Page_ProHance Work Output/div_SIDEBAR MENU'), 10)
	
	WebUI.click(findTestObject('Object Repository/Page_ProHance Work Output/div_SIDEBAR MENU'))
	
	WebUI.click(findTestObject('Object Repository/Category/Page_ProHance Work Output/span_Administration'))
	
	WebUI.click(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/li_Work Type Definition'))
				
	WebUI.switchToFrame(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/iframe'), 10)
		
	if(rows.size()==j)	
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

 }
		
	
}    

WebUI.closeBrowser()

