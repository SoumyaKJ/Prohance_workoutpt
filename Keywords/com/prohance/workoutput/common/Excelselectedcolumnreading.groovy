package com.prohance.workoutput.common
import org.apache.poi.ss.usermodel.*


import com.kms.katalon.core.annotation.Keyword
public class Excelselectedcolumnreading {

	@Keyword
	def readExcelheaderFromDownloads() {

// -------------------- FIND LATEST FILE --------------------
		String downloadDir = System.getProperty("user.home") + "\\Downloads"
		File folder = new File(downloadDir)
		
		assert folder.exists() : "Downloads folder not found"
		
		// wait for file to fully download
		File file = null
		long lastSize = -1
		int stableCount = 0
		
		for (int i = 0; i < 60; i++) { // wait up to ~60 sec
		
			File[] all = folder.listFiles()
		
			file = all.findAll { f ->
            def name = f.name.toLowerCase()

			f.isFile() &&
			name.startsWith("users") &&
			(name.endsWith(".xlsx") || name.endsWith(".xls")) &&
			!name.endsWith(".crdownload") &&
			!name.endsWith(".tmp") &&
			f.length() > 0
}.sort { a, b -> b.lastModified() <=> a.lastModified() }
			 .find { it != null }
		
			if (file != null) {
		
				long sizeNow = file.length()
		
				if (sizeNow == lastSize) {
					stableCount++
				} else {
					stableCount = 0
				}
		
				lastSize = sizeNow
		
				if (stableCount >= 3) { // stable for ~3 sec
					break
				}
			}
		
			Thread.sleep(1000)
		}
		
		assert file != null : "No completed download found"
		
		println("Download completed file: " + file.getAbsolutePath())
// -------------------- WAIT UNTIL STABLE --------------------

//long lastSize = -1
int stable = 0

for (int i = 0; i < 30; i++) {

    long sizeNow = file.length()

    if (sizeNow > 0 && sizeNow == lastSize) {
        stable++
        if (stable >= 3) break
    } else {
        stable = 0
    }

    lastSize = sizeNow
    Thread.sleep(1000)
}

assert file.length() > 0 : "File not fully downloaded"

// -------------------- OPEN EXCEL --------------------

Workbook workbook = null

try {
    workbook = WorkbookFactory.create(file)
    Sheet sheet = workbook.getSheetAt(0)
    DataFormatter fmt = new DataFormatter()

    // -------------------- FIND HEADER COLUMN --------------------

	List<String> headerNames = ["Skill Set","Secondary Job Title","<script>alert(\"Test!\");</script>",
		"Designation",
		"Tenure Range",
"Location",
"Serviceline",
"Band",
"Specialization",
"Business Unit",
"Region",
"Dynamic Attribute 9","Dynamic Attribute 10","Dynamic Attribute 11",
"Dynamic Attribute 12",
"Module",
"Dynamic Attribute 14",
"Dynamic Attribute 15"
		
]

	Map<String, Integer> headerMap = [:]
	
	int headerRowIdx = 0
	
	for (int r = headerRowIdx; r <= sheet.getLastRowNum(); r++) {
	
		Row row = sheet.getRow(r)
		if (row == null) continue
		boolean isRowEmpty = true
		for (int c = 0; c < row.getLastCellNum(); c++) {
	
			Cell cell = row.getCell(c, Row.MissingCellPolicy.CREATE_NULL_AS_BLANK)
			String value = fmt.formatCellValue(cell).trim()
			if (value) {
				isRowEmpty = false
			}
			headerNames.each { header ->
	
				if (header.equalsIgnoreCase(value)) {
					headerMap[header] = c
					if (headerRowIdx == 0 && r != 0) {
                    headerRowIdx = r
  }
				
				}
			}
		}
		if (isRowEmpty) {
			continue
		}
	
		if (headerMap.size() == headerNames.size()) {
			break
		}
	}

    // -------------------- READ COLUMN DATA --------------------
//
  Map<String, List<String>> resultMap = [:]

headerMap.each { header, colIndex ->

    List<String> columnData = []

    for (int r = headerRowIdx + 1; r <= sheet.getLastRowNum(); r++) {

        Row row = sheet.getRow(r)
        if (row == null) continue

        short lastCell = row.getLastCellNum()

        if (colIndex >= lastCell) {
            continue
        }

        Cell cell = row.getCell( colIndex, Row.MissingCellPolicy.CREATE_NULL_AS_BLANK )

        String value = fmt.formatCellValue(cell)?.trim()

        //println("Header=${header} Row=${r} Value=${value}")

        if (!value) continue
		//if (value.trim().equalsIgnoreCase(header.trim())) continue

        columnData.add(value)
    }

    resultMap[header] = columnData
    
}
    // -------------------- REMOVE DUPLICATES --------------------

    println("===== UNIQUE COLUMN VALUES =====")

  resultMap.each { header, values ->
 
   Set<String> uniqueValues = new LinkedHashSet<>(values)

	Map<String, List<String>> dropdownMap = [:]
	
	dropdownMap[header] = uniqueValues
	
	dropdownMap.sort().each { attributeName, dropdownValues  ->
		
			println("User Attribute: " + attributeName)
		
			println("Dropdown Options: " + dropdownValues.findAll { !it.equalsIgnoreCase("Unknown Tenure")
			} )
		
			println("--------------------------------")}
	
	
  }
	return resultMap

    } finally {
        if (workbook != null) workbook.close()
    }
	}
}
		

