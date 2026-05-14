import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase

import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import org.openqa.selenium.Dimension as Dimension
import com.kms.katalon.core.model.FailureHandling as FailureHandling
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
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import internal.GlobalVariable as GlobalVariable

WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)

DriverFactory.getWebDriver().manage().window().setSize(new Dimension(1920, 1080))

WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance/a_WORK OUTPUT'))

WebUI.switchToWindowTitle('ProHance Work Output')

WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/div_SIDEBAR MENU'))

WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/span_Administration'))

WebUI.click(findTestObject('Normalization Screen/Page_ProHance Work Output/li_Work Output Normalization'))

WebUI.switchToFrame(findTestObject('Normalization Screen/Page_ProHance Work Output/frame'), 10)

WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/type3_modify_icon'))

WebUI.waitForPageLoad(10)

WebUI.click(findTestObject('Normalization Screen/Page_ProHance Work Output/3rd radio button'))

WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/AOS_category radio button selection'))

WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/AAFS_category radio button selection'))

// WO Category
def options = WebUI.findWebElements(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/2nd radio button options'), 
    10)

def activities = options.collect({ 
        it.getText().trim()
    }).findAll({ 
        it != 'UNKNOWN'
    })
	


// WO AOS Activities

WebUI.click(findTestObject('Normalization Screen/Page_ProHance Work Output/3rd radio button'))
	
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/aos_activity radio button'))

WebUI.waitForElementPresent(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/aafs_options'),10)

def options1 = WebUI.findWebElements(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/aos_options'), 
    10)

def activities1 = options1.collect({ 
        it.getText().trim()}).findAll({ 
        it != 'UNKNOWN AOS'
    
    })

print(activities1)


// WO AAFS activities 
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/aafs_activity radio button'))

WebUI.waitForElementPresent(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/aafs_options'),10)

def options2 = WebUI.findWebElements(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/aafs_options'), 
    10)

def activities2 = options2.collect({ 
        it.getText().trim() }).findAll({ 
        it != 'UNKNOWN AAFS'
    
   
    })

//AOS activity
		
def wtaosactivities=WebUI.callTestCase(findTestCase('WT AOS_list/aos'), [:], FailureHandling.STOP_ON_FAILURE)

assert(activities1.sort()==wtaosactivities.sort())

print(' All AOS Activity present in the WO')

//AAFS activity

def wtaafsactivities=WebUI.callTestCase(findTestCase('WT AAFS_list/aafs'), [:], FailureHandling.STOP_ON_FAILURE)

assert(activities2.sort()==wtaafsactivities.sort())

print(' All AAFS Activity present in the WO')

// Comparing with Work time categories

def worktimecategories = WebUI.callTestCase(findTestCase('WT_categories/work type categories_list'), [:], FailureHandling.STOP_ON_FAILURE)
	
assert worktimecategories== activities.sort()
	
print(' All category present in the WO')


WebUI.closeBrowser()

/*
// Call the applogin test case to perform application login
WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)

// Set browser window size to 1920x1080
DriverFactory.getWebDriver().manage().window().setSize(new Dimension(1920, 1080))

// Click the WORK OUTPUT link
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance/a_WORK OUTPUT'))

// Switch to the ProHance Work Output window
WebUI.switchToWindowTitle('ProHance Work Output')

// Click the SIDEBAR MENU
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/div_SIDEBAR MENU'))

// Click the Administration span
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/span_Administration'))

// Click the Work Output Normalization menu item
WebUI.click(findTestObject('Normalization Screen/Page_ProHance Work Output/li_Work Output Normalization'))

// Switch to the Work Output Settings iframe
WebUI.switchToFrame(findTestObject('Normalization Screen/Page_ProHance Work Output/frame'), 10)

// Click the type3 modify icon
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/type3_modify_icon'))

// Wait for the page to load completely
WebUI.waitForPageLoad(10)

// Click the 3rd radio button
WebUI.click(findTestObject('Normalization Screen/Page_ProHance Work Output/3rd radio button'))

// Select AOS category radio button
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/AOS_category radio button selection'))

// Select AAFS category radio button
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/AAFS_category radio button selection'))

def options = WebUI.findWebElements(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/2nd radio button options'),
10)

def activities = options.collect({
it.getText().trim()
}).findAll({
it != 'UNKNOWN'
})

// Click the 3rd radio button for AOS activities
WebUI.click(findTestObject('Normalization Screen/Page_ProHance Work Output/3rd radio button'))

// Click the AOS activity radio button
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/aos_activity radio button'))

// Wait for AAFS options element to be present
WebUI.waitForElementPresent(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/aafs_options'),10)

// Find AOS options elements
def options1 = WebUI.findWebElements(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/aos_options'),
10)

def activities1 = options1.collect({
it.getText().trim()}).findAll({
it != 'UNKNOWN AOS'

})

print(activities1)

// Click the AAFS activity radio button
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/aafs_activity radio button'))

// Wait for AAFS options element to be present
WebUI.waitForElementPresent(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/aafs_options'),10)

// Find AAFS options elements
def options2 = WebUI.findWebElements(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/aafs_options'),
10)

def activities2 = options2.collect({
it.getText().trim() }).findAll({
it != 'UNKNOWN AAFS'


})

// Call test case to get Work Time AOS activities
def wtaosactivities=WebUI.callTestCase(findTestCase('WT AOS_list/aos'), [:], FailureHandling.STOP_ON_FAILURE)

assert(activities1.sort()==wtaosactivities.sort())

print(' All AOS Activity present in the WO')

// Call test case to get Work Time AAFS activities
def wtaafsactivities=WebUI.callTestCase(findTestCase('WT AAFS_list/aafs'), [:], FailureHandling.STOP_ON_FAILURE)

assert(activities2.sort()==wtaafsactivities.sort())

print(' All AAFS Activity present in the WO')

// Call test case to get Work Time categories
def worktimecategories = WebUI.callTestCase(findTestCase('WT_categories/work type categories_list'), [:], FailureHandling.STOP_ON_FAILURE)

assert worktimecategories== activities.sort()

print(' All category present in the WO')

// Close the browser
WebUI.closeBrowser()*/