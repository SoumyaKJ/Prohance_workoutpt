import com.kms.katalon.core.testobject.TestObject as TestObject
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import org.openqa.selenium.WebElement as WebElement
import com.kms.katalon.core.testobject.ConditionType as ConditionType
import org.openqa.selenium.By as By
import java.util.List as List
import com.kms.katalon.core.webui.driver.DriverFactory as DriverFactory
import org.openqa.selenium.Dimension as Dimension
org.openqa.selenium.Dimension

WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)

DriverFactory.getWebDriver().manage().window().setSize(new Dimension(1920, 1080))

WebUI.click(findTestObject('Worktype Definition Screen/Page_ProHance/WORK OUTPUT'))

WebUI.switchToWindowTitle('ProHance Work Output')

WebUI.click(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/div_SIDEBAR MENU'))

WebUI.click(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/a_Administration'))

WebUI.click(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/li_Work Type Definition'))

WebUI.switchToFrame(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/iframe'), 10)

WebUI.click(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/add new link'))

WebUI.setText(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/name text area'), 'new work type')

WebUI.click(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/save button'))

WebUI.click(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/back button'))

WebUI.setText(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/search text area'), 
    'new work type')

String workType = 'new work type'

WebUI.waitForPageLoad(30)

WebUI.waitForElementVisible(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/dynamic_modify_icon', 
        [('worktype') : workType]), 10)

WebUI.click(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/1st modify icon'))

WebUI.click(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/target tab link'))

WebUI.verifyElementText(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/row1'), 'Handling Time')

TestObject checkbox = findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/check box1')

WebUI.verifyElementPresent(checkbox, 10)

WebUI.verifyElementClickable(checkbox)

WebUI.verifyElementText(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/label1'), 
    'Define Estimated Hours (Handling Time) & Targets specific for this Work Type')

String dynamictext="Target Condition"

String label=WebUI.getText(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/label2'))

assert label.startsWith(dynamictext)

def conditions = []

def actualtargetcondtions = WebUI.findWebElements(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/target dropdowns'), 
    10)

conditions = actualtargetcondtions.collect({ 
        it.getText().trim()
    })

def expectedconditions = ['', 'Greater Than', 'Greater Equal', 'Less Than', 'Lesser Equal', 'Equal', 'Not Equal']

assert conditions == expectedconditions

WebUI.verifyElementText(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/label3'), 
    'Est. Handling Time (Minutes)')

WebUI.verifyElementText(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/label4'), 
    'Effective Date')

WebUI.verifyElementText(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/label5'), 
    'Data Ranges')

WebUI.verifyElementText(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/parameter label'), 
    'Parameter')

WebUI.verifyElementText(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/min label'), 
    'Min. Value')

WebUI.verifyElementText(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/max label'), 
    'Max. Value')

WebUI.verifyElementText(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/default lable'), 
    'Default Value')

WebUI.verifyElementPresent(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/clear button'), 10)

WebUI.verifyElementPresent(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/save button1'), 10)

WebUI.verifyElementPresent(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/button1'), 
    10)

WebUI.verifyElementPresent(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/cancel button'), 
    10)

WebUI.verifyElementPresent(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/next button'), 
    10)

WebUI.verifyElementPresent(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/back button1'), 
    10)

//All row data verification
List<WebElement> rows = WebUI.findWebElements(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/user attribute table rows'), 
    10)

rows.each({ def row ->
        List<WebElement> cols = row.findElements(By.tagName('td'))

        List<String> rowData = cols.collect({ 
                it.getText().trim()
            })

        println(rowData)
    })

//Calender icon verfication
boolean isPresent = WebUI.verifyElementPresent(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/calender link'), 
    10)

assert isPresent

//Selected Date verification
String targetMonthYear = 'July 2018'

String targetDay = '12'

// Open calendar
String currentMonthYear
String currentday

WebUI.click(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/click on calander'))
// Navigate until correct month/year appears
while (true) {
  currentMonthYear = WebUI.getText(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/calader header')).trim()

    println('Current Calendar: ' + currentMonthYear)

  if (currentMonthYear == targetMonthYear)
	   {
        break
   
		 }
	
WebUI.click(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/calendar next button'))

WebUI.delay(1)
}

//select target day

TestObject obj = new TestObject() 
					 
obj.addProperty("xpath", ConditionType.EQUALS, "//html/body/div[6]/div[1]/table/tbody/tr[3]/td[text()=${targetDay}]")
					
currentday = WebUI.getText(obj).trim().toInteger()
	
println('Currentday: ' + currentday)

assert targetMonthYear==currentMonthYear

assert targetDay==currentday

assert "${targetMonthYear} ${targetDay}" == "${currentMonthYear} ${currentday}"

println  " target month:${targetDay} ${targetMonthYear}, selected month :${currentday} ${currentMonthYear}}"

//view history model pop up

WebUI.verifyElementPresent(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/view history model'), 
    10)

WebElement model = WebUI.findWebElement(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/view history model'))

assert model.isDisplayed()

assert model.isEnabled()

TestObject model1 = findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/view history model')

WebUI.verifyElementClickable(model1)

WebUI.closeBrowser()


