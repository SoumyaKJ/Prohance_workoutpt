import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import org.openqa.selenium.Dimension
import org.openqa.selenium.WebElement

import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testobject.ConditionType
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.webui.common.WebUiCommonHelper
import com.kms.katalon.core.webui.driver.DriverFactory
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)

DriverFactory.getWebDriver().manage().window().setSize(new Dimension(1920, 1080))

WebUI.click(findTestObject('Worktype Definition Screen/Page_ProHance/WORK OUTPUT'))

WebUI.switchToWindowTitle('ProHance Work Output')

WebUI.click(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/div_SIDEBAR MENU'))

WebUI.click(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/a_Administration'))

WebUI.click(findTestObject('Wo_settings/Page_ProHance Work Output/li_Work Output Settings'))

WebUI.switchToFrame(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/iframe'), 10)

WebUI.click(findTestObject('Object Repository/Wo_settings/Page_ProHance Work Output/select empty option'))

WebUI.click(findTestObject('Object Repository/Wo_settings/Page_ProHance Work Output/submit button'))

def yesBtn = findTestObject('Object Repository/Wo_settings/Page_ProHance Work Output/confirmation pop up yes')

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

WebUI.waitForPageLoad(10)

WebUI.waitForElementVisible(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/dynamic_modify_icon', 
        [('worktype') : workType]), 10)

WebUI.click(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/1st modify icon'))

WebUI.click(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/user group mapping'))

//WebUI.switchToFrame(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/iframe'), 10)

TestObject modify = findTestObject('Worktype Definition Screen/Page_ProHance Work Output/modify in group mapping screen')

List<WebElement> elements = WebUiCommonHelper.findWebElements(modify, 5)

if (elements.size() > 0)
	 {
    WebUI.comment("Modify element is present")
    
    WebUI.click(modify)
	
	WebUI.delay(1)
  
	boolean isNotChecked = WebUI.verifyElementNotChecked(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/grp mpg 1st check box'),
		10, FailureHandling.STOP_ON_FAILURE)
	
	if (isNotChecked) {
		println('Checkbox is not checked') 
		
	    WebUI.click(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/grp mpg 1st check box'))
			
	} else {
		
		println('Checkbox is checked')
	
	}
	
	//collecting all usersttribute dropdowns
	def attr = WebUI.findWebElements(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/grp ua option'),
		10)
	
	def options = attr.collect({it.getText().trim()}).findAll {it &&!it.toUpperCase().startsWith("DEFAULT")}
	
	def userattributename = WebUI.getText(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/grp selected ua'))
	
	println(userattributename)
	
	def option = WebUI.findWebElements(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/grp ua options'),
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
	
				obj.addProperty('xpath', ConditionType.EQUALS, "//select[@id='ehtUGRuleOption']/option[${i + 1}]")
	
				WebUI.click(obj)
				
				WebUI.delay(1)
				
				def userattribute = WebUI.getText(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/grp selected ua'))
				
				def optionss = WebUI.findWebElements(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/grp ua options'),
					10)
				
				
				def useratr = optionss.collect({it.getText().trim()})
				
			   (dropdownMaps[userattribute]) = useratr
			   
			  
		 }
		
	 
	//Work time Active user attributes
	
	def allactiveWtoption=WebUI.callTestCase(findTestCase('WT_users/WT_Active user attribute list'), [:], FailureHandling.STOP_ON_FAILURE)
	
	//verifying all Active user attributes are present in the work type definition screen> WT user attribute dropdown
	
	
		dropdownMaps = dropdownMaps.sort { it.key }
		
		def dropdwns=dropdownMaps.each { attributeName, dropdownValues ->
		
			if (attributeName?.trim()) {
		
				def cleanedValues = dropdownValues
					.collect { it?.trim() }
					.findAll {it && !it.toUpperCase().startsWith('DEFAULT')
					}
					
					.sort()
		
				println('User Attribute: ' + attributeName)
		
				println('Dropdown Options: ' + cleanedValues)
		
				println('--------------------------------')
			
				} 
				
			}
			
		return dropdwns
//-------------------------------------
		
	 }
	 else
	 {		
		 
		 String parentWindow = WebUI.getWindowIndex()
		 
		 print parentWindow
		 
		 WebUI.click(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/add user group'))

		 WebUI.switchToWindowIndex(2)
		
		 WebUI.click(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/usergroup screen 1st group'))
		
		 WebUI.click(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/add button in user grp mapping'))
		
		WebUI.switchToWindowIndex(parentWindow)
		
		CustomKeywords.'com.prohance.workoutput.common.Selectinguserattributeoption.selectinguserattributeingrplevel'()
			 
	 }
		   
	WebUI.closeBrowser()
		
		
		
