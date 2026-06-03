package com.prohance.workoutput.common

import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import com.kms.katalon.core.testobject.ConditionType

import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject

import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.checkpoint.Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testcase.TestCase
import com.kms.katalon.core.testdata.TestData
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows

import internal.GlobalVariable

public class Selectinguserattributeoption {
	
	@Keyword
	def selectinguserattributeingrplevel() {
	
	WebUI.switchToFrame(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/iframe'), 10)
		
	def modify=findTestObject('Worktype Definition Screen/Page_ProHance Work Output/modify in group mapping screen')
	
	WebUI.click(modify)
	
	boolean isNotChecked = WebUI.verifyElementNotChecked(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/grp mpg 1st check box'),
		10, FailureHandling.STOP_ON_FAILURE)
	
	if (isNotChecked) {
		println('Checkbox is not checked') 
		
	    WebUI.click(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/grp mpg 1st check box'))
			
	} else {
		
		println('Checkbox is checked')
	
		}
	
	WebUI.delay(1)
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
					.findAll {it && !it.toUpperCase().startsWith("DEFAULT")
					}
					
					.sort()
		
				println('User Attribute: ' + attributeName)
		
				println('Dropdown Options: ' + cleanedValues)
		
				println('--------------------------------')
			
				} 
				assert(options.sort()==allactiveWtoption.sort())
				
				println "All user Attribute and user attribute options are matching with work time"
			}
		return dropdwns
}
}
