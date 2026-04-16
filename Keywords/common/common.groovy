package common

import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
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
import com.kms.katalon.core.testobject.ConditionType
import org.openqa.selenium.WebElement
	
	public class common {
	
	class IframeHelper {
	
		static boolean switchToFrameContaining(TestObject target, int timeout = 20) {
	
			List<WebElement> frames = WebUI.findWebElements(
				new TestObject().addProperty("//iframe[contains(@id,'contentFrame')]", ConditionType.EQUALS, "iframe"),
				timeout
			)
	
			println("Total iframes found: " + frames.size())
	
			for (int i = 0; i < frames.size(); i++) {
	
				WebUI.switchToDefaultContent()
	
				WebUI.switchToFrame(frames.get(i), timeout)
	
				if (WebUI.verifyElementPresent(target, 2, com.kms.katalon.core.model.FailureHandling.OPTIONAL)) {
					println("✔ Found element inside iframe index: " + i)
					return true
				}
			}
	
			WebUI.switchToDefaultContent()
			return false
		}
	}

}
