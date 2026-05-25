import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import java.time.Duration as Duration
import org.openqa.selenium.By as By
import org.openqa.selenium.Dimension as Dimension
import org.openqa.selenium.WebElement as WebElement
import org.openqa.selenium.interactions.Actions as Actions
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testobject.ConditionType as ConditionType
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webui.common.WebUiCommonHelper as WebUiCommonHelper
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

List<String> alluserattribute = WebUI.callTestCase(findTestCase('WT_users/all text box user attribute list'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)

DriverFactory.getWebDriver().manage().window().setSize(new Dimension(1920, 1080))

WebUI.waitForElementPresent(findTestObject('Work time category/side bar_admin'), 10)

WebUI.click(findTestObject('Work time category/side bar_admin'))

WebUI.click(findTestObject('Object Repository/worktime user screen/users link'))

WebUI.click(findTestObject('Object Repository/worktime user screen/users'))

WebUI.switchToFrame(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/iframe'), 10)

WebUI.click(findTestObject('Object Repository/worktime user screen/report columns'))

//List<WebElement> items =WebUI.findWebElements(findTestObject('Object Repository/worktime user screen/available columns'),10)
TestObject obj = new TestObject()

obj.addProperty('xpath', ConditionType.EQUALS, '//ul[@id=\'sortable1\']/li')

List<WebElement> items = WebUI.findWebElements(obj, 10)

items.each({ 
        println(it.getText().trim())
    })

List<WebElement> filteredItems = items.findAll({ def item ->
        String text = item.getText().trim()

        alluserattribute.contains(text)
    })

filteredItems.each({ 
        println(it.getText().trim())
    })

TestObject targetObj = findTestObject('Object Repository/worktime user screen/added columns')

WebElement target = WebUiCommonHelper.findWebElement(targetObj, 10)

WebElement container = DriverFactory.getWebDriver().findElement(By.xpath('//ul[@id=\'sortable1\']'))

for (WebElement item : filteredItems) {
    WebUI.executeJavaScript('arguments[0].scrollIntoView({block:\'center\'});', Arrays.asList(item))

    WebUI.executeJavaScript('window.scrollBy(0, -200);', null)

    WebUI.delay(1)

    Actions actions = new Actions(DriverFactory.getWebDriver())

    int tx = Math.max(1, ((target.getSize().getWidth() / 2) as int))

    int ty = Math.max(1, ((target.getSize().getHeight() / 2) as int))

    actions.moveToElement(item).pause(Duration.ofMillis(300)).moveToElement(item, 10, 10).pause(Duration.ofMillis(300)).clickAndHold().pause(
        Duration.ofMillis(500)).moveToElement(target, tx, ty).pause(Duration.ofMillis(500)).release().perform()

    WebUI.delay(1)
}

WebUI.click(findTestObject('Object Repository/worktime user screen/fetch'))

WebUI.click(findTestObject('Object Repository/worktime user screen/excleimport'))

def textvalue = CustomKeywords.'com.prohance.workoutput.common.excelhaeader.readExcelheaderFromDownloads'()

def dropdownvalues = WebUI.callTestCase(findTestCase('WT_users/all dropdown user attribute list'), [:], FailureHandling.STOP_ON_FAILURE)

def userroleattr=WebUI.callTestCase(findTestCase('WT_users/user role attribute names'), [:], FailureHandling.STOP_ON_FAILURE)

def allatttribute = textvalue + dropdownvalues+userroleattr

def allatttributewithoptions = allatttribute
			.sort { it.key}
			.each{ def key, def value ->
			def cleanedValues = value
            .collect { it?.trim() }
            .findAll {it &&!it.toUpperCase().startsWith('Unknown Tenure')
            }
			.unique()
            .sort()
			println("User Attribute : ${key}")
			
			def filteredValues = cleanedValues.findAll {!it.equalsIgnoreCase("Unknown Tenure")}

			println("Dropdown Options : ${filteredValues}")

			println('--------------------------------')}

		
WebUI.closeBrowser()
return allatttributewithoptions



