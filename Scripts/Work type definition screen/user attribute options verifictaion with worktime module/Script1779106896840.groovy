import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import org.openqa.selenium.Dimension as Dimension
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testobject.ConditionType as ConditionType
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webui.driver.DriverFactory as DriverFactory
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
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

WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)

DriverFactory.getWebDriver().manage().window().setSize(new Dimension(1920, 1080))

WebUI.click(findTestObject('Worktype Definition Screen/Page_ProHance/a_WORK OUTPUT'))

WebUI.switchToWindowTitle('ProHance Work Output')

WebUI.click(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/div_SIDEBAR MENU'))

WebUI.click(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/a_Administration'))

WebUI.click(findTestObject('Wo_seetings/Page_ProHance Work Output/li_Work Output Settings'))

WebUI.switchToFrame(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/iframe'), 10)

WebUI.click(findTestObject('Object Repository/Wo_seetings/Page_ProHance Work Output/select empty option'))

WebUI.click(findTestObject('Object Repository/Wo_seetings/Page_ProHance Work Output/submit button'))

def yesBtn = findTestObject('Object Repository/Wo_seetings/Page_ProHance Work Output/confirmation pop up yes')

if (WebUI.waitForElementVisible(yesBtn, 2, FailureHandling.OPTIONAL)) {
    WebUI.click(yesBtn)
}

WebUI.delay(1)

WebUI.switchToDefaultContent()

WebUI.waitForElementClickable(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/div_SIDEBAR MENU'), 10)

WebUI.click(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/div_SIDEBAR MENU'))

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

boolean isChecked = WebUI.verifyElementNotChecked(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/check box1'), 
    10, FailureHandling.STOP_ON_FAILURE)

if (isChecked) {
    println('Checkbox is checked' //WebUI.click(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/check box1'))
        )
} else {
    println('Checkbox is NOT checked')

    WebUI.click(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/check box1'))
}

WebUI.delay(1)

//collecting all usersttribute dropdowns
def attr = WebUI.findWebElements(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/user attribute dropdown in Wt definition screen'), 
    10)

def options = attr.collect({ 
        it.getText().trim()
    })

def userattributename = WebUI.getText(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/selected user attribute_in wt definition screen'))

println(userattributename)

def option = WebUI.findWebElements(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/only user attribute options in WT def'), 
    10)

def useratrr = option.collect({ 
        it.getText().trim()
    })

//-----------------------------------------------------------

//work type definition screen values
Map<String, List> dropdownMaps = [:]

for (int i = 0; i<attr.size(); i++)
	 {
		
		
            TestObject obj = new TestObject()

            obj.addProperty('xpath', ConditionType.EQUALS, "//select[@id='ehtLocalRuleOption']/option[${i + 1}]")

            WebUI.click(obj)
			
			WebUI.delay(1)
			
			def userattribute = WebUI.getText(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/selected user attribute_in wt definition screen'))
			
		    def optionss = WebUI.findWebElements(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/only user attribute options in WT def'),10)
			
			def useratr = optionss.collect({it.getText().trim()})
			
		   (dropdownMaps[userattribute]) = useratr
	 }
       
	
//Work time Active user attributes

//def allactiveWtoption=WebUI.callTestCase(findTestCase('WT_users/WT_Active user attribute list'), [:], FailureHandling.STOP_ON_FAILURE)

//verifying all Active user attributes are present in the work type definition screen> WT user attribute dropdown
//assert(options.sort()==allactiveWtoption.sort())
	dropdownMaps = dropdownMaps.sort { it.key }
	
	def dropdwns=dropdownMaps.each { attributeName, dropdownValues ->
	
		if (attributeName?.trim()) {
	
			def cleanedValues = dropdownValues
				.collect { it?.trim() }
				.findAll {
					it &&
					!it.toUpperCase().startsWith('DEFAULT')
				}
				
				.sort()
	
			println('User Attribute: ' + attributeName)
	
			println('Dropdown Options: ' + cleanedValues)
	
			println('--------------------------------')
		}
	}
	
	WebUI.closeBrowser()
	
	return dropdwns




