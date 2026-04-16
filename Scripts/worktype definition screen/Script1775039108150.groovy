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
import org.openqa.selenium.WebElement

import static org.assertj.core.api.Assertions.registerCustomDateFormat

import org.openqa.selenium.By as By
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.testobject.ConditionType

//WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/Page_ProHance/a_WORK OUTPUT'))

WebUI.switchToWindowTitle('ProHance Work Output')

WebUI.click(findTestObject('Object Repository/Page_ProHance Work Output/div_SIDEBAR MENU'))

WebUI.click(findTestObject('Object Repository/Page_ProHance Work Output/span_Administration'))

WebUI.click(findTestObject('Object Repository/Page_ProHance Work Output/li_Work Type Definition'))

WebUI.switchToFrame(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/iframe'), 0)

WebUI.waitForElementVisible(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/worktypes_rows'), 50)

def headers = WebUI.findWebElements(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/table header'),
	10).collect({ it.getText().trim()})
println(headers.size())
def Activeworktypes= []
/*
// Find all table rows (excluding header)
def rows = WebUI.findWebElements(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/worktypes_rows'), 
    10)

// Optional: find header row separately


println("Headers: $headers")



// Iterate through each row
rows.each{ def row ->
        List<WebElement> cols = row.findElements(By.tagName('td'))

        cols.each({ def col ->
                print(col.getText().trim() + ' , ')
            })

        println() // new line for next row
            
}
*/
if (headers.size() == 6) 
	{
       def usergroup = WebUI.findWebElements(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/Work type User Group'), 10)
       List<WebElement> usergroups = usergroup
	   def groups = usergroups.collect{it.getText()}
	  
	   def worktyprows = WebUI.findWebElements(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/worktype full row'),10)
	   int rowcount = worktyprows.size()
	   def allWorkTypes = []
	   for (int i = 0; i<rowcount; i++)
		 
		    {   
				int j=i+1
				TestObject dynamicObject2 = new TestObject()
				
				 int numberofusergroups = Integer.parseInt(groups[i])
				//worktype status
				dynamicObject2.addProperty("xpath",ConditionType.EQUALS,"//*[@id='CommonDataTableId']/tbody/tr[${j}]/td[6]/span")
				def worktypestate = WebUI.getAttribute(dynamicObject2, 'title')?.trim()
				
				if(worktypestate=='Active'&& numberofusergroups > 0)
				{
						
					//Active worktypes
								TestObject dynamicObject = new TestObject()
								dynamicObject.addProperty("xpath",ConditionType.EQUALS,"//*[@id='CommonDataTableId']/tbody/tr[${j}]/td[2]/div")
				    
					//groups >0
								TestObject dynamicObject1 = new TestObject()
								dynamicObject1.addProperty("xpath",ConditionType.EQUALS,"//*[@id='CommonDataTableId']/tbody/tr[${j}]/td[5]")
					
								def worktypes = WebUI.getText(dynamicObject).trim()
								def activeworktypeusergroup = WebUI.getText(dynamicObject1).trim()
					
				   //collecting all Active worktypes	
								Activeworktypes.add("$worktypes\n")
								allWorkTypes.add("$worktypes, $activeworktypeusergroup, $worktypestate\n")
						
				}
			j++
			
			}
			
			print(Activeworktypes)
			return Activeworktypes
	}
else if(headers.size()<6)
	{	 
		def usergroup = WebUI.findWebElements(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/usergroups'), 10)
		List<WebElement> usergroups = usergroup
		def groups = usergroups.collect{it.getText()}
		  
	    def worktyprows = WebUI.findWebElements(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/worktype full row'),10)
	    int rowcount = worktyprows.size()
		println(rowcount)
	    def allWorkTypes = []
		//int numberofusergroups=0;
	    for (int i = 0; i<rowcount; i++)
		 
		    {   
				int j=i+1
				TestObject dynamicObject2 = new TestObject()
				
			    int numberofusergroups = Integer.parseInt(groups[i])
				//worktype status
				dynamicObject2.addProperty("xpath",ConditionType.EQUALS,"//*[@id='CommonDataTableId']/tbody/tr[${j}]/td[5]/i")
				def worktypestate = WebUI.getAttribute(dynamicObject2, 'title')?.trim()
				
				if(worktypestate=='Active'&& numberofusergroups > 0)
				{
						
					//Active worktypes
								TestObject dynamicObject = new TestObject()
								dynamicObject.addProperty("xpath",ConditionType.EQUALS,"//*[@id='CommonDataTableId']/tbody/tr[${j}]/td[1]/div")
				    
					//groups >0
								TestObject dynamicObject1 = new TestObject()
								dynamicObject1.addProperty("xpath",ConditionType.EQUALS,"//*[@id='CommonDataTableId']/tbody/tr[${j}]/td[4]")
					
								def worktypes = WebUI.getText(dynamicObject).trim()
								def activeworktypeusergroup = WebUI.getText(dynamicObject1).trim()
					
				   //collecting all Active worktypes	
								Activeworktypes.add("$worktypes\n")
								allWorkTypes.add("${worktypes} ${activeworktypeusergroup} ${worktypestate}\n")
								
				}
			j++
			
			}
			
			print(Activeworktypes)
			return Activeworktypes
	}
				
			
WebUI.closeBrowser()
	

