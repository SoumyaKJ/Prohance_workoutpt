package com.prohance.workoutput.common
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import org.openqa.selenium.Dimension

import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testobject.ConditionType
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.webui.driver.DriverFactory
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

public class Superadminscreens {
	
	@Keyword
	def superadmin() {

	WebUI.callTestCase(findTestCase('Commons/superadmin login'), [:], FailureHandling.STOP_ON_FAILURE)
	
	DriverFactory.getWebDriver().manage().window().setSize(new Dimension(1920, 1080))
	
	WebUI.waitForPageLoad(30)
	
	WebUI.switchToFrame(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/iframe'), 30)
	
	def organizationname = WebUI.findWebElements(findTestObject('Object Repository/work time/orgname_in_superadmin screen'),10)
	
	def rows = WebUI.findWebElements(findTestObject('Object Repository/work time/installation date column'),10)
	
	def orginstalldate = ""
	
	def currentorg
	
	for (int j = 0; j < rows.size(); j++) {
	
		String orgname = organizationname[j].getText().trim()
	
		if (orgname.equalsIgnoreCase("JAMOCHA TECH")) {
	
			TestObject obj = new TestObject()
	
			obj.addProperty(
					"xpath",
					ConditionType.EQUALS,
					"//table[@id='CommonDataTableId']/tbody/tr[${j + 1}]/td[6]")
	
			orginstalldate = WebUI.getText(obj).trim()
			
			currentorg = new TestObject()
	
			currentorg.addProperty(
					"xpath",
					ConditionType.EQUALS,
					"//table[@id='CommonDataTableId']/tbody/tr[${j + 1}]/td[2]/span")
	
			println "${orgname} -> ${orginstalldate}"
			
			
			break
		}
	}

	WebUI.click(currentorg)
	
	return orginstalldate
}}