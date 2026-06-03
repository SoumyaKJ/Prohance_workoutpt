import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import org.openqa.selenium.Dimension as Dimension
import org.openqa.selenium.WebElement as WebElement
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webui.common.WebUiCommonHelper as WebUiCommonHelper
import com.kms.katalon.core.webui.driver.DriverFactory as DriverFactory
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.testobject.ConditionType as ConditionType
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import internal.GlobalVariable as GlobalVariable

//user has to pass the org name

def orginstalldate=CustomKeywords.'com.prohance.workoutput.common.Superadminscreens.superadmin'()

//--------------------------------------------------------------------------------------------------------
//workoutput
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

WebUI.click(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/user group mapping'))

//WebUI.switchToFrame(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/iframe'), 10)
TestObject modify = findTestObject('Worktype Definition Screen/Page_ProHance Work Output/modify in group mapping screen')

List<WebElement> elements = WebUiCommonHelper.findWebElements(modify, 5)

if (elements.size() > 0) {
    WebUI.comment('Modify element is present')

    WebUI.click(modify)

    WebUI.delay(2)

    boolean isNotChecked = WebUI.verifyElementNotChecked(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/grp mpg 1st check box'), 
        10, FailureHandling.STOP_ON_FAILURE)

    if (isNotChecked) {
        println('Checkbox is not checked')

        WebUI.click(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/grp mpg 1st check box'))
    } else {
        println('Checkbox is checked')
    }
    
    def orgstartdate = WebUI.findWebElements(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/effective date'), 
        10)

    def date = orgstartdate.collect({ 
            it.getAttribute('value').trim()
        })

    boolean allSame = date.unique().size() == 1

    if (allSame) {
        println("All elements have the same value $date[0]")
    } else {
        println('Values are different')

        println(date)
    }
    
    println(date[0])

    assert (date[0]) == orginstalldate // if groups are not added >adding group and checking org date	
	print " org installation date matching with superdmin screen"
} else {
    String parentWindow = WebUI.getWindowIndex()

    print(parentWindow)

    WebUI.click(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/add user group'))

    WebUI.switchToWindowIndex(2)

    WebUI.click(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/usergroup screen 1st group'))

    WebUI.click(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/add button in user grp mapping'))

    WebUI.switchToWindowIndex(parentWindow)

    WebUI.switchToFrame(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/iframe'), 10)

    def orgstartdate = WebUI.findWebElements(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/effective date'), 
        10)

    def date = orgstartdate.collect({ 
            it.getAttribute('value').trim()
        })

    boolean allSame = date.unique().size() == 1

    if (allSame) {
        println("All elements have the same value $date[0]")
    } else {
        println('Values are different')

        println(date)
    }
    
    println(date[0])

    assert (date[0]) == orginstalldate
	print " org installation date matching with superdmin screen"
}

WebUI.closeBrowser()

