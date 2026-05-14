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

WebUI.click(findTestObject('Normalization Screen/Page_ProHance Work Output/2nd radio button'))

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

WebUI.click(findTestObject('Normalization Screen/Page_ProHance Work Output/2nd radio button'))
	
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
// Call the login test case to perform application login
WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)

// Set the browser window size to 1920x1080
DriverFactory.getWebDriver().manage().window().setSize(new Dimension(1920, 1080))

// Click on the WORK OUTPUT link
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance/a_WORK OUTPUT'))

// Switch to the window titled 'ProHance Work Output'
WebUI.switchToWindowTitle('ProHance Work Output')

// Click to open the sidebar menu
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/div_SIDEBAR MENU'))

// Click the Administration option in the sidebar
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/span_Administration'))

// Click the Work Output Normalization list item
WebUI.click(findTestObject('Normalization Screen/Page_ProHance Work Output/li_Work Output Normalization'))

// Switch to the normalization frame
WebUI.switchToFrame(findTestObject('Normalization Screen/Page_ProHance Work Output/frame'), 10)

// Click the modify icon for type3
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/Page_ProHance Work Output/type3_modify_icon'))

// Wait for the page to load after clicking modify
WebUI.waitForPageLoad(10)

// Select the 2nd radio button (WO Category)
WebUI.click(findTestObject('Normalization Screen/Page_ProHance Work Output/2nd radio button'))

// Select the AOS category radio button
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/AOS_category radio button selection'))

// Select the AAFS category radio button
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/AAFS_category radio button selection'))

// WO Category - find all option elements for the 2nd radio button options
def options = WebUI.findWebElements(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/2nd radio button options'),
10)

def activities = options.collect({
it.getText().trim()
}).findAll({
it != 'UNKNOWN'
})

// WO AOS Activities - reselect the 2nd radio button
WebUI.click(findTestObject('Normalization Screen/Page_ProHance Work Output/2nd radio button'))

// Select the AOS activity radio button
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/aos_activity radio button'))

// Wait for AAFS options element to be present before proceeding
WebUI.waitForElementPresent(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/aafs_options'),10)

// Find all AOS option elements
def options1 = WebUI.findWebElements(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/aos_options'),
10)

def activities1 = options1.collect({
it.getText().trim()}).findAll({
it != 'UNKNOWN AOS'

})

print(activities1)

// WO AAFS activities - select the AAFS activity radio button
WebUI.click(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/aafs_activity radio button'))

// Wait for AAFS options element to be present
WebUI.waitForElementPresent(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/aafs_options'),10)

// Find all AAFS option elements
def options2 = WebUI.findWebElements(findTestObject('Object Repository/Normalization Screen/Page_ProHance Work Output/aafs_options'),
10)

def activities2 = options2.collect({
it.getText().trim() }).findAll({
it != 'UNKNOWN AAFS'


})

// Call test case to get WT AOS activities
def wtaosactivities=WebUI.callTestCase(findTestCase('WT AOS_list/aos'), [:], FailureHandling.STOP_ON_FAILURE)

// Assert that AOS activities in WO match WT AOS activities
assert(activities1.sort()==wtaosactivities.sort())

print(' All AOS Activity present in the WO')

// Call test case to get WT AAFS activities
def wtaafsactivities=WebUI.callTestCase(findTestCase('WT AAFS_list/aafs'), [:], FailureHandling.STOP_ON_FAILURE)

// Assert that AAFS activities in WO match WT AAFS activities
assert(activities2.sort()==wtaafsactivities.sort())

print(' All AAFS Activity present in the WO')

// Comparing with Work time categories by calling the WT categories test case
def worktimecategories = WebUI.callTestCase(findTestCase('WT_categories/work type categories_list'), [:], FailureHandling.STOP_ON_FAILURE)

// Assert that WO categories match Work time categories
assert worktimecategories== activities.sort()

print(' All category present in the WO')

// Close the browser
WebUI.closeBrowser()