package com.prohance.workoutput.common

import org.apache.poi.ss.usermodel.*
import com.kms.katalon.core.annotation.Keyword

import java.util.zip.ZipEntry
import java.util.zip.ZipInputStream

class Readzipfile {

    @Keyword
    def readExcel() {

        // -------------------- FIND LATEST FILE --------------------

        String downloadDir = System.getProperty("user.home") + "\\Downloads"

        File folder = new File(downloadDir)

        assert folder.exists() : "Downloads folder not found"

        File file = null

        long lastSize = -1

        int stableCount = 0

        for (int i = 0; i < 60; i++) {

            File[] all = folder.listFiles()

            file = all.findAll { f ->

                def name = f.name.toLowerCase()

                f.isFile() &&

                name.startsWith("woworktype_config_data".toLowerCase()) &&

                (
                    name.endsWith(".xlsx") ||
                    name.endsWith(".xls")  ||
                    name.endsWith(".zip")
                ) &&

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

                // stable for 3 seconds
                if (stableCount >= 3) {
                    break
                }
            }

            Thread.sleep(1000)
        }

        assert file != null : "No completed download found"

        println("Download completed file: " + file.getAbsolutePath())

        // -------------------- HANDLE ZIP --------------------

        String filePath = file.getAbsolutePath()

        if (filePath.toLowerCase().endsWith(".zip")) {

            println("ZIP file detected")

            String extractDir = downloadDir + "\\ExtractedFiles"

            File dir = new File(extractDir)

            if (!dir.exists()) {
                dir.mkdirs()
            }

            // cleanup old extracted files
            dir.eachFileRecurse { f ->
                try {
                    f.delete()
                } catch (Exception e) {
                }
            }

            byte[] buffer = new byte[1024]

            ZipInputStream zis =
                    new ZipInputStream(new FileInputStream(file))

            ZipEntry zipEntry = zis.getNextEntry()

            while (zipEntry != null) {

                File newFile = new File(extractDir, zipEntry.getName())

                if (zipEntry.isDirectory()) {

                    newFile.mkdirs()

                } else {

                    new File(newFile.parent).mkdirs()

                    FileOutputStream fos =
                            new FileOutputStream(newFile)

                    int len

                    while ((len = zis.read(buffer)) > 0) {
                        fos.write(buffer, 0, len)
                    }

                    fos.close()
                }

                zipEntry = zis.getNextEntry()
            }

            zis.closeEntry()
            zis.close()

            // -------------------- FIND EXTRACTED EXCEL --------------------

            File extractedExcel = null

            dir.eachFileRecurse(groovy.io.FileType.FILES) { f ->

                if (
                    f.name.toLowerCase().endsWith(".xlsx") ||
                    f.name.toLowerCase().endsWith(".xls")
                ) {

                    extractedExcel = f
                }
            }

            assert extractedExcel != null :
                    "No Excel file found inside ZIP"

            println("Extracted Excel: " +
                    extractedExcel.getAbsolutePath())

            println("Extracted Excel Size: " +
                    extractedExcel.length())

            filePath = extractedExcel.getAbsolutePath()
        }

        println("Final Excel path: " + filePath)

        // -------------------- WAIT UNTIL FILE STABLE --------------------

        File finalFile = new File(filePath)

        long finalLastSize = -1

        int stable = 0

        for (int j = 0; j < 30; j++) {

            long sizeNow = finalFile.length()

            if (sizeNow > 0 && sizeNow == finalLastSize) {

                stable++

                if (stable >= 3) {
                    break
                }

            } else {

                stable = 0
            }

            finalLastSize = sizeNow

            Thread.sleep(1000)
        }

        assert finalFile.length() > 0 :
                "File not fully downloaded/extracted"

        // -------------------- OPEN EXCEL --------------------

        Workbook workbook = null

        try {

            workbook =
                    WorkbookFactory.create(new File(filePath))

            Sheet sheet = workbook.getSheetAt(0)

            DataFormatter fmt = new DataFormatter()

            // -------------------- FIND HEADERS --------------------

            List<String> headerNames = [
                    "Work Type",
                    "EHT (Minutes)"
            ]

            Map<String, Integer> headerMap = [:]

            int headerRowIdx = 0

            for (int r = 0; r <= sheet.getLastRowNum(); r++) {

                Row row = sheet.getRow(r)

                if (row == null) {
                    continue
                }

                boolean isRowEmpty = true

                for (int c = 0; c < row.getLastCellNum(); c++) {

                    Cell cell = row.getCell(
                            c,
                            Row.MissingCellPolicy.CREATE_NULL_AS_BLANK
                    )

                    String value =
                            fmt.formatCellValue(cell).trim()

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

                if (headerMap.size() ==
                        headerNames.size()) {

                    break
                }
            }

            // -------------------- READ COLUMN DATA --------------------

            Map<String, List<String>> resultMap = [:]

            headerMap.each { header, colIndex ->

                List<String> columnData = []

                for (int r = headerRowIdx + 1;
                     r <= sheet.getLastRowNum();
                     r++) {

                    Row row = sheet.getRow(r)

                    if (row == null) {
                        continue
                    }

                    short lastCell = row.getLastCellNum()

                    if (colIndex >= lastCell) {
                        continue
                    }

                    Cell cell = row.getCell(
                            colIndex,
                            Row.MissingCellPolicy.CREATE_NULL_AS_BLANK
                    )

                    String value =
                            fmt.formatCellValue(cell)?.trim()

                    if (!value) {
                        continue
                    }

                    columnData.add(value)
                }
			def	wtandehtvalues= columnData
			
			
			//println wtandehtvalues
			
			//return wtandehtvalues
			 resultMap[header] = columnData
            }

            // -------------------- REMOVE DUPLICATES --------------------

            println("===== UNIQUE COLUMN VALUES =====")
		
        resultMap.each { header, values ->

                Set<String> uniqueValues =new LinkedHashSet<>(values)
                        

                //println("work type name: " + header)

                println("data: " +uniqueValues)
                        

                //println("--------------------------------")
				
			
            }

				
           

        } finally {

            if (workbook != null) {
                workbook.close()
            }
        }
    }
}