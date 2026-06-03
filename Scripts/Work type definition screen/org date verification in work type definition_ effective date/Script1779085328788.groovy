import com.kms.katalon.core.testobject.TestObject as TestObject
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.testobject.ConditionType as ConditionType
import com.kms.katalon.core.webui.driver.DriverFactory as DriverFactory
import org.openqa.selenium.Dimension as Dimension

//user has to pass the org name

def orginstalldate=CustomKeywords.'com.prohance.workoutput.common.Superadminscreens.superadmin'()

//----------------------------------------------------
//WebUI.waitForPageLoad(30) // adding waiting time for headlesss mode

WebUI.click(findTestObject('Worktype Definition Screen/Page_ProHance/wo link'))

WebUI.switchToWindowTitle('ProHance Work Output')

WebUI.click(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/div_SIDEBAR MENU'))

WebUI.click(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/a_Administration'))

WebUI.click(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/li_Work Type Definition'))

WebUI.switchToFrame(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/iframe'),10)

WebUI.click(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/add new link'))

WebUI.setText(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/name text area'), 'new work type2')

WebUI.click(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/save button'))

WebUI.click(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/back button'))

WebUI.setText(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/search text area'), 
    'new work type2')

String workType = 'new work type2'

WebUI.waitForPageLoad(30)

WebUI.waitForElementVisible(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/dynamic_modify_icon', 
        [('worktype') : workType]), 10)

WebUI.click(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/1st modify icon'))

WebUI.click(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/target tab link'))

def orgstartdate = WebUI.findWebElements(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/effective date'), 
    10)

def date = orgstartdate.collect({ 
        it.getAttribute('value').trim()
    })

boolean allSame = date.unique().size() == 1

if (allSame) {
    println("All elements have the same value $date[0]")
} else {
    println("Values are different")
	println date
}

println(date[0])

assert(date[0]== orginstalldate)

print " Work types effective date matching with superdmin screen org installation date"

WebUI.closeBrowser()