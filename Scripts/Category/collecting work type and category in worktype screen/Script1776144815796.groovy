import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import com.kms.katalon.core.webui.driver.DriverFactory
import org.openqa.selenium.Dimension
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
import org.openqa.selenium.By as By
import org.openqa.selenium.WebElement as WebElement
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.testobject.ConditionType
import org.openqa.selenium.WebElement

WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)

DriverFactory.getWebDriver().manage().window().setSize(new Dimension(1920, 1080))

WebUI.click(findTestObject('Object Repository/Page_ProHance/a_WORK OUTPUT'))

WebUI.switchToWindowTitle('ProHance Work Output')

WebUI.click(findTestObject('Object Repository/Page_ProHance Work Output/div_SIDEBAR MENU'))

WebUI.click(findTestObject('Object Repository/Page_ProHance Work Output/span_Administration'))

WebUI.click(findTestObject('Object Repository/Page_ProHance Work Output/li_Work Type Definition'))

WebUI.switchToFrame(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/iframe'), 0)

//WebUI.click(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/max records to display'))
WebUI.waitForElementVisible(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/worktypes_rows'), 50)

def headers = WebUI.findWebElements(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/table header'), 
    10).collect({ it.getText().trim() })

println("Headers: $headers")

def rows = WebUI.findWebElements(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/worktypes_rows'), 10)

def page=WebUI.findWebElements(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/pagination'),10)

def numberofpage=page.size()

println(numberofpage)

def Data = []

for (int i=1; i <=numberofpage; i++)
{
			
     def rws = WebUI.findWebElements(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/worktypes_rows'),10)

     rws.each{ def row ->
		 
        List<WebElement> cols = row.findElements(By.tagName('td'))

		

        if (cols.size() == 6) 
			{
          
            def worktype = (cols[1]).getText().trim()

            def category = (cols[3]).getText().trim()

            Data.add("$worktype->$category")
           } 
		else if (cols.size() < 6) 
			{
            def worktype = (cols[0]).getText().trim()

            def category = (cols[2]).getText().trim()

            Data.add("$worktype->$category")
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
		WebUI.waitForPageLoad(10)
		  }
Data.each { println it }

WebUI.closeBrowser()

