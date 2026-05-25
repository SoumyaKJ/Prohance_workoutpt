import org.apache.poi.ss.usermodel.*
import org.apache.poi.ss.usermodel.WorkbookFactory

import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

import java.text.SimpleDateFormat

// -------------------- FIND LATEST FILE --------------------

String downloadDir = System.getProperty("user.home") + "\\Downloads"
File folder = new File(downloadDir)

assert folder.exists() : "Downloads folder not found"

File[] all = folder.listFiles()
assert all != null : "Cannot read Downloads folder"

// pick latest Users_*.xlsx file
File file = all.findAll { f ->
    f.isFile() &&
    f.name.startsWith("Users") &&
    f.name.toLowerCase().endsWith(".xlsx") &&
    !f.name.endsWith(".crdownload") &&
    f.length() > 0
}.sort { a, b -> b.lastModified() <=> a.lastModified() }
 .find { it != null }

assert file != null : "No Excel file found"

println("Selected file: " + file.getAbsolutePath())

// -------------------- WAIT UNTIL STABLE --------------------

long lastSize = -1
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

    String headerName = "Role"
    int headerRowIdx = -1
    int targetCol = -1

    for (int r = 0; r <= sheet.getLastRowNum(); r++) {

        Row row = sheet.getRow(r)
        if (row == null) continue

        for (int c = 0; c < row.getLastCellNum(); c++) {

            Cell cell = row.getCell(c, Row.MissingCellPolicy.CREATE_NULL_AS_BLANK)
            String value = fmt.formatCellValue(cell).trim()

            if (value.equalsIgnoreCase(headerName)) {
                headerRowIdx = r
                targetCol = c
                break
            }
        }

        if (targetCol != -1) break
    }

    assert targetCol != -1 : "Header not found: " + headerName

    println("Found header '${headerName}' at row=${headerRowIdx}, col=${targetCol}")

    // -------------------- READ COLUMN DATA --------------------

    List<String> columnData = []

    for (int r = headerRowIdx + 1; r <= sheet.getLastRowNum(); r++) {

        Row row = sheet.getRow(r)
        if (row == null) continue

        Cell cell = row.getCell(targetCol, Row.MissingCellPolicy.CREATE_NULL_AS_BLANK)

        String value = fmt.formatCellValue(cell).trim()

        if (value) {
            columnData.add(value)
        }
    }

    // -------------------- REMOVE DUPLICATES --------------------

    Set<String> uniqueValues = new LinkedHashSet<>(columnData)

    println("===== UNIQUE ROLE VALUES =====")

    uniqueValues.each {
        println(it)
    }

} finally {
    if (workbook != null) workbook.close()
}

WebUI.closeBrowser()