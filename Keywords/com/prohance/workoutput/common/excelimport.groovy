package com.prohance.workoutput.common

import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.util.KeywordUtil

import java.io.File
import java.io.FileInputStream

import org.apache.poi.ss.usermodel.*
import org.apache.poi.xssf.usermodel.XSSFWorkbook
import org.apache.poi.hssf.usermodel.HSSFWorkbook

public class excelimport {

    @Keyword
    def readExcelFromDownloads() {

        String downloadDir = System.getProperty("user.home") + "\\Downloads"
File folder = new File(downloadDir)
assert folder.exists() : "Downloads folder does not exist: ${downloadDir}" 

File[] all = folder.listFiles()
assert all != null : "Cannot read files in folder: ${downloadDir}" 

// Pick latest real .xlsx (ignore temp/in-progress downloads)
File file = all.findAll { f ->
    f.isFile() &&
    f.name.startsWith('Users') &&
    f.name.toLowerCase().endsWith('.xlsx') &&
    !new File(f.absolutePath + '.crdownload').exists() &&
    !f.name.toLowerCase().endsWith('.tmp')
}.sort { a, b -> b.lastModified() <=> a.lastModified() }
 .find { it != null }

assert file != null : "Excel file not found!"
println("Selected file: ${file.absolutePath} (size=${file.length()} bytes)")

// Wait until file size stabilizes (download completed)
long lastSize = -1
int stableCount = 0
for (int t = 0; t < 30; t++) { // up to ~30 seconds
    long sizeNow = file.length()
    if (sizeNow > 0 && sizeNow == lastSize) {
        stableCount++
        if (stableCount >= 3) break // stable for ~3 seconds
    } else {
        stableCount = 0
    }
    lastSize = sizeNow
    Thread.sleep(1000)
}
assert file.length() > 0 : "Excel file is empty (0 bytes): ${file.absolutePath}"

// Open workbook safely (no need to manage FileInputStream yourself)
Workbook workbook = null
try {
    workbook = WorkbookFactory.create(file)
    Sheet sheet = workbook.getSheetAt(0)

    for (int i = 0; i <= sheet.getLastRowNum(); i++) {
        Row row = sheet.getRow(i)
        if (row == null) continue

        for (int c = 0; c < row.getLastCellNum(); c++) {
            Cell cell = row.getCell(c, Row.MissingCellPolicy.CREATE_NULL_AS_BLANK)
            print(cell.toString() + " | ")
        }
        println()
    }
} finally {
    if (workbook != null) workbook.close()
}
}}