import com.kms.katalon.core.testobject.TestObject as TestObject
import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import org.openqa.selenium.WebElement as WebElement
import com.kms.katalon.core.testobject.ConditionType as ConditionType
import org.junit.After as After
import org.openqa.selenium.By as By
import java.util.List as List
import com.kms.katalon.core.webui.driver.DriverFactory as DriverFactory
import org.openqa.selenium.Dimension as Dimension

org.openqa.selenium.Dimension

WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)

DriverFactory.getWebDriver().manage().window().setSize(new Dimension(1920, 1080))

WebUI.click(findTestObject('Worktype Definition Screen/Page_ProHance/WORK OUTPUT'))

WebUI.switchToWindowTitle('ProHance Work Output')

WebUI.click(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/div_SIDEBAR MENU'))

//WebUI.click(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/a_Administration'))
WebUI.click(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/wo console'))

WebUI.switchToFrame(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/iframe'), 10)

WebUI.click(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/wo raw link'))

WebUI.click(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/config dropdown'))

WebUI.click(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/fetch'))

WebUI.click(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/refresh button'))

WebUI.waitForElementVisible(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/1st attachment'),60, FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/refresh button'))

WebUI.click(findTestObject('Object Repository/Worktype Definition Screen/Page_ProHance Work Output/1st attachment'))

def configdata =CustomKeywords.'com.prohance.workoutput.common.Readzipfile.readExcel'()

def wtdefinitiondata=WebUI.callTestCase(findTestCase('Work type definition screen/collecting all active WT with only default EHT'), [:], FailureHandling.STOP_ON_FAILURE)

assert configdata==wtdefinitiondata

