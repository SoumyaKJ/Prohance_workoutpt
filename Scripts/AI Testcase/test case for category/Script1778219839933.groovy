

/**
 * TEST CASE: Category/category work type list
 *
 * Output:
 *  - Returns List<String> in format: "<Category>-><WorkType>"
 *  - Also prints the final mapping to log.
 *
 * Pre-req Test Objects (as used below):
 *  - Worktype Definition Screen/Page_ProHance/a_WORK OUTPUT
 *  - Worktype Definition Screen/Page_ProHance Work Output/div_SIDEBAR MENU
 *  - Worktype Definition Screen/Page_ProHance Work Output/a_Administration
 *  - Object Repository/Page_ProHance Work Output/Page_ProHance Work Output/li_Work Type Category
 *  - Worktype Definition Screen/Page_ProHance Work Output/iframe
 *  - Object Repository/Category/Page_ProHance Work Output/row
 *  - Worktype Definition Screen/Page_ProHance Work Output/pagination
 *  - Worktype Definition Screen/Page_ProHance Work Output/pagination_next button
 *  - Object Repository/Page_ProHance Work Output/Page_ProHance Work Output/category_wt
 *  - Category/Page_ProHance Work Output/back button
 */

Test Steps:
dashboard:
Open browser and navigate to app URL.
Log in with valid credentials.
Verify dashboard page loads.
Verify dashboard header/title is visible.
Verify global navigation elements are present (e.g., sidebar/top nav).
Verify key metric tiles/cards are visible and show values (not blank).
Verify at least one chart/widget is visible.
Verify filters (date range/status/etc.) are visible.
Change a filter value and click Apply (if applicable).
Verify dashboard refreshes and widgets update (or loading indicator appears then disappears).
Click a widget/card that should drill down.
Verify drilldown page/modal opens and contains relevant data.
Log out.