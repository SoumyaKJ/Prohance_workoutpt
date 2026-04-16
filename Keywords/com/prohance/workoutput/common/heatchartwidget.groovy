package com.prohance.workoutput.common

import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.annotation.Keyword

import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

import internal.GlobalVariable

public class Heatchartwidget {
	
	 @Keyword
	 def addHeatChartWidget()
	  {
	 
	WebUI.click(findTestObject('Object Repository/Heat Chart/Page_ProHance/a_WORK OUTPUT'))
	
	WebUI.switchToWindowTitle('ProHance Work Output')
	
	WebUI.click(findTestObject('Object Repository/Heat Chart/Page_ProHance Work Output/div_Soumya Admin Account_arrow-top'))
	
	WebUI.click(findTestObject('Object Repository/Heat Chart/Page_ProHance Work Output/li_Output Dashboard'))
	
	WebUI.click(findTestObject('Object Repository/Heat Chart/Page_ProHance Work Output/span_Add New Widget'))
	
	WebUI.click(findTestObject('Object Repository/Heat Chart/Page_ProHance Work Output/div_Heat Chart'))
	
	WebUI.click(findTestObject('Object Repository/Heat Chart/Page_ProHance Work Output/div_ADD WIDGET'))
	return true
	 }
}
