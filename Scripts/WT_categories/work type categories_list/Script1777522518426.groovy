import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import org.openqa.selenium.Dimension
import org.openqa.selenium.WebElement

import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.webui.driver.DriverFactory
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI



WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)

//WebUI.click(findTestObject('Object Repository/Work time category/refresh button_prohance'))

//WebUI.waitForElementPresent(findTestObject('Work time category/side bar_admin'),10)

DriverFactory.getWebDriver().manage().window().setSize(new Dimension(1920, 1080))

WebUI.click(findTestObject('Work time category/side bar_admin'))

WebUI.click(findTestObject('Work time category/activities'))

WebUI.click(findTestObject('Work time category/categories link'))

WebUI.switchToFrame(findTestObject('Normalization Screen/Page_ProHance Work Output/frame'), 10)

//Catgories List

List<WebElement> headerElements = WebUI.findWebElements(findTestObject('Work time category/table header'), 10)

List<String> headerNames = headerElements.collect { WebElement el ->
	(el?.getText() ?: '').trim()
}

headerNames.each { name ->
	WebUI.comment("Header: ${name}")
}

List<WebElement> categoryname = WebUI.findWebElements(findTestObject('Object Repository/Work time category/catagories name'), 10)

List<String>category  = categoryname.collect { WebElement el1 ->
	(el1?.getText() ?: '').trim()
}
category.each {categories-> WebUI.comment("category:${categories}")}

return category
