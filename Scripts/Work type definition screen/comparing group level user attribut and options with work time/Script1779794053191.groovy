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

def wouserattributes = WebUI.callTestCase(findTestCase('Work type definition screen/collecting user attributes option in wt definition team mapping screen'), 
    [:], FailureHandling.STOP_ON_FAILURE)

def wtuserattributes = WebUI.callTestCase(findTestCase('WT_users/collecting user attribute and options in user screen'), [:], FailureHandling.STOP_ON_FAILURE)

print wouserattributes

print wtuserattributes

//assert wouserattributes == wtuserattributes


// ---------------- NORMALIZE FUNCTION ----------------

def normalizeMap = { map ->

    map.collectEntries { key, value ->

        def cleanKey = key?.trim()

        def cleanedValues = (value ?: [])
            .collect { it?.trim() }
            .findAll {
                it &&
                !it.equalsIgnoreCase('Unknown Tenure') &&
                !it.toUpperCase().startsWith('DEFAULT')
            }
            .unique()
            .sort()

        if (cleanKey && cleanedValues) {
            [(cleanKey): cleanedValues]
        } else {
            null
        }
    }
    .findAll { it != null }
    .sort { it.key }
}

// ---------------- NORMALIZE BOTH MAPS ----------------

def expectedMap = normalizeMap(wtuserattributes)

def actualMap = normalizeMap(wouserattributes)


// ---------------- PRINT MAPS ----------------

println("=========== EXPECTED MAP ===========")

expectedMap.each { key, value ->

    println("User Attribute : ${key}")

    println("Dropdown Options : ${value}")

    println('--------------------------------')
}

println("=========== ACTUAL MAP ===========")

actualMap.each { key, value ->

    println("User Attribute : ${key}")

    println("Dropdown Options : ${value}")

    println('--------------------------------')
}


// ---------------- COMPARE ----------------

boolean mismatchFound = false

expectedMap.each { key, expectedValues ->

    def actualValues = actualMap[key]

    if (expectedValues != actualValues) {

        mismatchFound = true

        println(" Mismatch found for Attribute : ${key}")

        println("Expected : ${expectedValues}")

        println("Actual   : ${actualValues}")

        println("===================================")
    }
}


// ---------------- CHECK EXTRA ATTRIBUTES ----------------

actualMap.each { key, value ->

    if (!expectedMap.containsKey(key)) {

        mismatchFound = true

        println("❌ Extra attribute found in actual map : ${key}")
    }
}


// ---------------- FINAL ASSERT ----------------

assert !mismatchFound : "Map comparison failed. Differences found."

assert expectedMap ==actualMap

println(" Actual and Expected, Both maps are matching successfully.")
				
WebUI.closeBrowser()



