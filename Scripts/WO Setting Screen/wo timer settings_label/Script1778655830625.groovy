import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import org.openqa.selenium.WebElement
import org.openqa.selenium.support.ui.Select

import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.webui.common.WebUiCommonHelper
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/Category/Page_ProHance/a_WORK OUTPUT'))

WebUI.switchToWindowTitle('ProHance Work Output')

WebUI.click(findTestObject('Object Repository/Category/Page_ProHance Work Output/i_Soumya Admin Account_fa fa-chevron-right _d36e5e'))

WebUI.click(findTestObject('Object Repository/Category/Page_ProHance Work Output/span_Administration'))

WebUI.click(findTestObject('Wo_seetings/Page_ProHance Work Output/li_Work Output Settings'))

WebUI.switchToFrame(findTestObject('Wo_seetings/Page_ProHance Work Output/iframe_contentFrame'), 10)

WebUI.verifyElementText(findTestObject('Object Repository/Wo_seetings/Page_ProHance Work Output/wo_timer'),
	'WO Timer Settings')

WebUI.verifyElementText(findTestObject('Object Repository/Wo_seetings/Page_ProHance Work Output/timer_label1'),
	'Display/Edit Output Value')

def type1 = WebUI.getAttribute(findTestObject('Object Repository/Wo_seetings/Page_ProHance Work Output/timer_label1_checkbox'), 'type')

if (type1 == 'checkbox')
	{
	println ('options provided with check box')
	}
WebUI.verifyElementText(findTestObject('Object Repository/Wo_seetings/Page_ProHance Work Output/wo_timer_label2'),
		'Edit Timer Reference')
	
def type2= WebUI.getAttribute(findTestObject('Object Repository/Wo_seetings/Page_ProHance Work Output/wo timer_label2_check box'), 'type')
	
if (type2 == 'checkbox')
		{
		println ('options provided with check box')
		}
WebUI.verifyElementText(findTestObject('Object Repository/Wo_seetings/Page_ProHance Work Output/wo_timer_label3'),
			'Display Reset, Pause, Start/Stop buttons')
		
def type3= WebUI.getAttribute(findTestObject('Object Repository/Wo_seetings/Page_ProHance Work Output/wo_timer_label3_check box'), 'type')
		
if (type3 == 'checkbox')
	{
		println ('options provided with check box')
		}

WebUI.verifyElementText(findTestObject('Object Repository/Wo_seetings/Page_ProHance Work Output/wo_timer_label4'),
			'Timer duration to be reported in*')

def unit = WebUI.findWebElements(findTestObject('Object Repository/Wo_seetings/Page_ProHance Work Output/timer unit dropdown option'),
	10)

def actualValues = unit.collect { it.getText().trim()}

println(actualValues)

assert actualValues == ['Hours', 'Minutes', 'Seconds']

WebUI.closeBrowser()
