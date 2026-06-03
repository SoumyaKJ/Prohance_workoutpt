import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import org.openqa.selenium.By
import org.openqa.selenium.Dimension
import org.openqa.selenium.WebElement
import org.openqa.selenium.WebDriver
import org.openqa.selenium.WebElement
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.webui.driver.DriverFactory
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)

DriverFactory.getWebDriver().manage().window().setSize(new Dimension(1920,1080))

WebUI.click(findTestObject('Object Repository/Page_ProHance/a_WORK OUTPUT'))

WebUI.switchToWindowTitle('ProHance Work Output')

WebUI.click(findTestObject('Object Repository/Page_ProHance Work Output/div_SIDEBAR MENU'))

WebUI.click(findTestObject('Object Repository/Page_ProHance Work Output/span_Administration'))

WebUI.click(findTestObject('Normalization Screen/Page_ProHance Work Output/li_Work Output Normalization'))

WebUI.switchToFrame(findTestObject('Normalization Screen/Page_ProHance Work Output/iframe'), 10)

WebUI.waitForElementVisible(findTestObject('Normalization Screen/Page_ProHance Work Output/rows'), 50)

def headers = WebUI.findWebElements(findTestObject('Normalization Screen/Page_ProHance Work Output/header_name'), 
    10)

println(headers.size())

//All columns are enable
if (headers.size() == 7) {
    //checking table headers
    headers = WebUI.findWebElements(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/table header'), 
        10).collect({ 
            it.getText().trim()
        })

    println("Actual headers:$headers")
	
	List<WebElement> rows = driver.findElements(
    By.xpath("//table[@id='CommonDataTableId']/tbody/tr")
)

rows.each { row ->
    println(row.getText())
//--------------------------------------------	
	List<String> metricNames = driver.findElements(
		By.xpath("//table[@id='CommonDataTableId']/tbody/tr/td[3]")
	).collect { it.text.trim() }
	
	println metricNames
}
	
	
	
	
	
	
}