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
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import org.openqa.selenium.WebElement as WebElement
import com.kms.katalon.core.testobject.ConditionType as ConditionType
import org.openqa.selenium.By as By
import java.util.Map as Map

WebUI.callTestCase(findTestCase('Commons/applogin'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/Category/Page_ProHance/a_WORK OUTPUT'))

WebUI.switchToWindowTitle('ProHance Work Output')

WebUI.click(findTestObject('Object Repository/Category/Page_ProHance Work Output/i_Soumya Admin Account_fa fa-chevron-right _d36e5e'))

WebUI.click(findTestObject('Object Repository/Category/Page_ProHance Work Output/span_Administration'))

WebUI.click(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/li_Work Type Definition'))

WebUI.switchToFrame(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/iframe'), 10)

WebUI.waitForElementVisible(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/worktypes_rows'), 10)

def pagination = WebUI.findWebElements(findTestObject('Normalization Screen/Page_ProHance Work Output/pagination'), 10)

def numberofpage = pagination.size()

println(numberofpage)

def Data = []

if (numberofpage > 3) {
    for (int i = 1; i <= numberofpage; i++) {
        def rws = WebUI.findWebElements(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/worktypes_rows'), 
            10)

        rws.each({ def r ->
                List<WebElement> cols = r.findElements(By.tagName('td'))

                if (cols.size() == 6) {
                    // ensure both columns exist
                    def worktype = (cols[1]).getText().trim()

                    def category = (cols[3]).getText().trim()

                    Data.add("$category" // ensure both columns exist
                        )
                } else if (cols.size() < 6) {
                    def worktype = (cols[0]).getText().trim()

                    def category = (cols[2]).getText().trim()

                    Data.add("$category")
                }
            })

        if (numberofpage == i) {
            break
        }
        
        TestObject pageBtn = new TestObject()

        pageBtn.addProperty('xpath', ConditionType.EQUALS, "//ul[@class='pagination']/li[@class='paginate_button '][$i]/a")

        WebUI.click(pageBtn)

        WebUI.waitForElementPresent(findTestObject('Worktype Definition Screen/Page_ProHance Work Output/worktypes_rows'), 
            10)
    }
    
    println("$Data")

    HashSet<String> set = new HashSet()

    for (String item : Data) {
        set.add(item)
    }
    
    set.each({ 
            println(it)
        })
}

WebUI.closeBrowser()

