import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.util.KeywordUtil

// TODO: Replace with your AUT URL and objects

KeywordUtil.logInfo('AI_Testcase_01 started')

try {
	WebUI.openBrowser('')
	WebUI.navigateToUrl('https://example.com')
	WebUI.waitForPageLoad(30)
	KeywordUtil.markPassed('Navigation succeeded')
} finally {
	WebUI.closeBrowser()
}
