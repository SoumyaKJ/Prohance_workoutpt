import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase

import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

def wouserattribute=WebUI.callTestCase(findTestCase('Work type definition screen/collecting user attribute options in work type definition screen'), 
    [:], FailureHandling.STOP_ON_FAILURE)

def wtuserattribute=WebUI.callTestCase(findTestCase('WT_users/collecting user attribute and options in user screen'), [:], FailureHandling.STOP_ON_FAILURE)

				
// ---------------- NORMALIZE FUNCTION ----------------

def normalizeMap = { map ->

    map.collectEntries { key, value ->

        def cleanedValues = value
            .collect { it?.trim() }
            .findAll {
                it &&!it.equalsIgnoreCase('Unknown Tenure') &&
                !it.toUpperCase().startsWith('DEFAULT')
            }
            .unique()
            .sort()

        [(key?.trim()) : cleanedValues]
    }
    .sort { it.key }
}


// ---------------- NORMALIZE BOTH MAPS ----------------

def expectedMap = normalizeMap(wouserattribute)

def actualMap = normalizeMap(wtuserattribute)


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

