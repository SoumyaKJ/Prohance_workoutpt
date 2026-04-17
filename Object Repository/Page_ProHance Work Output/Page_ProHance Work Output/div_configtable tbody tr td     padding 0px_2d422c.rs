<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_configtable tbody tr td     padding 0px_2d422c</name>
   <tag></tag>
   <elementGuidId>783e9199-3dc2-4990-934c-7da59ef125e8</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='adminBodyContent']</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>#adminBodyContent</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorCollection>
      <entry>
         <key>SMART_LOCATOR</key>
         <value>internal:text=&quot;%> × Administration Work Output Work Type Category Work Type Category Details [D&quot;i</value>
      </entry>
   </smartLocatorCollection>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
      <webElementGuid>2dc42946-14ab-4136-b810-a127eb60d6a9</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>adminBodyContent</value>
      <webElementGuid>4a23d725-90b6-427f-b99f-cfbfb9b38bcc</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>adminBodyContent ph-wo-overflow-y-auto ph-wo-overflow-x-hidden ph-wo-height-100percentage ph-wo-margin-top-min-17px</value>
      <webElementGuid>ba2ce0d6-315b-45b6-8161-ea18a034f154</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tabindex</name>
      <type>Main</type>
      <value>1</value>
      <webElementGuid>d0d334a6-d153-4808-8b0f-31fbb57bbd29</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
	








%>


.configtable tbody tr td {
    padding: 0px; 
    height: 38px;
    border: none;
    padding-top: 0px;
    padding-bottom: 0px;
    text-align: left;
}

 #tableDivId
 {
 	margin-left: 5px;
 	padding-right: 10px;
 }
 
 .disclaimerDiv {
    padding: 10px 11px 30px 35px !important;
}













	
		













	
		×
		
	



	
	
	
	


	
		
			
				
					
						
							
							
							
								
							
							
						
						
							
								
										
						
					
					
			
			
			
			
		
	



 $( document ).ready(function() {
	fnFadeoutEvent(&quot;#dialogsuccessspan&quot;, 20000, &quot;ph-wo-display-none&quot;);
	fnFadeoutEvent(&quot;#dialogfailurespan&quot;, 20000, &quot;ph-wo-display-none&quot;);
	fnFadeoutEvent(&quot;#dialogdependencyspan&quot;, 20000, &quot;ph-wo-display-none&quot;);
	
	if ($(&quot;#dialogsuccessspan&quot;).is(':hidden')) 
	{
		fnRemoveClassWithSameStyle(&quot;#dialogsuccessspan&quot;, &quot;ph-wo-height&quot;, &quot;&quot;, false)
	 	$(&quot;#dialogsuccessspan&quot;).addClass(&quot;ph-wo-height-0px&quot;);
	}
	if ($(&quot;#dialogfailurespan&quot;).is(':hidden')) 
	{
		fnRemoveClassWithSameStyle(&quot;#dialogfailurespan&quot;, &quot;ph-wo-height&quot;, &quot;&quot;, false)
	 	$(&quot;#dialogfailurespan&quot;).addClass(&quot;ph-wo-height-0px&quot;);
	}
});

function fnSucessClose()
{
	fnRemoveClassWithSameStyle(&quot;#dialogsuccessspan&quot;, &quot;ph-wo-height&quot;, &quot;&quot;, false)
 	$(&quot;#dialogsuccessspan&quot;).addClass(&quot;ph-wo-height-0px&quot;);
}

function fnFailureClose()
{
	fnRemoveClassWithSameStyle(&quot;#dialogfailurespan&quot;, &quot;ph-wo-height&quot;, &quot;&quot;, false)
 	$(&quot;#dialogfailurespan&quot;).addClass(&quot;ph-wo-height-0px&quot;);
}

function fnShowDependency()
{
	$('.modal').draggable({
	    handle: &quot;.modal-header&quot;
	});
	
	var form;
	if ('CategoryForm' == 'null')
	{
		//
	}
	else
	{
		form = document.CategoryForm;
	}
	var url = &quot;/phworkoutput/getDataDependency/getDependencyList.htm?deleteRecordId=&quot;+-1;
	windowTitle = fnGetWindowName();
	var winObj =  window.open(&quot;&quot;, windowTitle, &quot;status=1, toolbar=0, scrollbars=1, resizable=1, width=1330, height=600, left=20, top=20&quot;);
	winObj.document.body.appendChild(fnGetDependencyPopUpLoader());
	window.parent.parent.fnAddToChildWindows(winObj);
	tempAction = form.action;
    tempTarget = form.target;
    form.action = url;
    form.target = windowTitle;
    form.submit();
    winObj.focus();
    form.action = tempAction;
    form.target = tempTarget;
}

function fnGetDependencyPopUpLoader()
{
	var loaderDiv = window.parent.document.createElement('div');
    loaderDiv.style.textAlign = 'center';
    loaderDiv.style.width = '100%';
    loaderDiv.style.position = 'absolute';
    loaderDiv.style.top = '48%';
    loaderDiv.setAttribute('nonce', 'xZrLNGLeef26xpNnnF1igA==');
 
    var loaderImg =  window.parent.document.createElement('img');
    loaderImg.src = &quot;/phworkoutput/images/loader-trans.gif&quot;;
    loaderImg.setAttribute('nonce', 'xZrLNGLeef26xpNnnF1igA==');
    loaderDiv.appendChild(loaderImg);
    return loaderDiv;
}

/* function fnDeleteDependency()
{
	var primaryKeyId = '';
	var moduleName = '';
	var menuItemId = '302';
	var formDefinitionId  = '';
	
	var actionName = &quot;/phworkoutput/getDataDependency/deleteDataDependency.htm?primaryKeyId=&quot;+primaryKeyId+&quot;&amp;moduleName=&quot;+moduleName+&quot;&amp;menuItemId=&quot;+menuItemId+&quot;&amp;formDefinitionId=&quot;+formDefinitionId;
	doAjaxCall(actionName, false, false);
} */


$(document).off('click', '[data-onclick=&quot;hideMsg&quot;]').on('click', '[data-onclick=&quot;hideMsg&quot;]', function() {
	
	fnRemoveClassWithSameStyle(&quot;.internalmessagefailure&quot;, &quot;ph-wo-display&quot;, &quot;&quot;, false);
	$(&quot;.internalmessagefailure&quot;).addClass(&quot;ph-wo-display-none&quot;);
	fnRemoveClassWithSameStyle(&quot;#displayErrorMessage&quot;, &quot;ph-wo-display&quot;, &quot;&quot;, false);
	$(&quot;#displayErrorMessage&quot;).addClass(&quot;ph-wo-display-none&quot;);
});

$(document).off('click', '[data-onclick=&quot;fnSucessClose&quot;]').on('click', '[data-onclick=&quot;fnSucessClose&quot;]', function() {
	fnSucessClose();
});

$(document).off('click', '[data-onclick=&quot;fnFailureClose&quot;]').on('click', '[data-onclick=&quot;fnFailureClose&quot;]', function() {
	fnFailureClose();
});

$(document).off('click', '[data-onclick=&quot;fnShowDependency&quot;]').on('click', '[data-onclick=&quot;fnShowDependency&quot;]', function() {
	fnShowDependency();
});


	
	
		







	
		
			
				
				
					
					
						Administration
					
				
				
			
		
		
			
				
				
					
					
						Work Output
					
				
				
			
		
		
			
				
					
				
				
				
					
					
						Work Type Category
					
				
				
					
				
			
		
		
			
				
				
				
					
					
						Work Type Category Details
					
				
				
			
		
		
		
		
		
			[Development Metrics] 
		
		
		
		
	




		
		
			
			 	
			
					
							BACK
					
					
					
			
		
	



	
	function fnBackToSummary()
	{
		form = document.WODataCollectionForm;
		if($(&quot;#isFistTab&quot;).val() != &quot;true&quot; || !document.WODataCollectionForm[1])
		{
			form = document.WODataCollectionForm[1];
		} 
		if(isNaN($(&quot;#lastXDays&quot;).val()))
		{
			$(&quot;#lastXDays&quot;).val(&quot;&quot;);
		}
		form.action = &quot;/phworkoutput/DataCollectionSummary.htm&quot;;
		doSubmit(form);
	}

	function fnRedirectPage(actionName)
	{
		form = document.CategoryForm;
		var tempAction = '';
		if(typeof form == 'undefined' &amp;&amp; actionName == &quot;/phworkoutput/WorkTypeSummary.htm&quot;)
		{
			fnBackToWorkTypeSummary();
		}
		else
		{
			if (form.searchXML != null &amp;&amp; form.searchXML != 'undefined')
			{
				form.searchXML.value=&quot;&quot;;
			}
			tempAction = form.action;
			document.forms[0].action = actionName;
		}
		doSubmit(document.forms[0]);
		form.action = tempAction;
	}

	function fnBackToParameterSummary()
	{
		form = document.ParameterForm;
		document.forms[0].action = &quot;/phworkoutput/WOParameterSummary.htm&quot;;
		doSubmit(document.forms[0]);
	}
	
	function fnBackToWOMetricSummary()
	{
		form = document.WOMetricForm;
		form.action = &quot;/phworkoutput/WOMetricSummary.htm&quot;;
		doSubmit(form);
	}
	
	
	function fnBackToWorkTypeSummary()
	{
		form = document.WorkTypeForm;
		document.forms[0].action = &quot;/phworkoutput/WorkTypeSummary.htm&quot;;
		doSubmit(document.forms[0]);
	}

	function fnBackToCategorySummary()
	{
		form = document.CategoryForm;
		document.forms[0].action = &quot;/phworkoutput/WorkOutputCategorySummary.htm&quot;;
		doSubmit(document.forms[0]);
	}
	
	function fnBackToWOWorkTypeAttributeSummary()
	{
		form = document.ConfigureWorkTypeAttrForm;
		document.forms[0].action = &quot;/phworkoutput/WOWorktypeAttributeSummary.htm&quot;;
		doSubmit(document.forms[0]);
	}

	$(&quot;[name = back]&quot;).click(function(){
		$(this).attr(&quot;disabled&quot;, &quot;disabled&quot;);
	});
	
	$(&quot;#reDirectPageArg0, #reDirectPageArg1, #reDirectPageArg2, #reDirectPageArg3&quot;).click(function()
 	{
	 	 var value = $(this).data('action');
	   	 if(value != null &amp;&amp; value != &quot;&quot; &amp;&amp; value != '')
		 {
	   		fnRedirectPage(value);
		 }
    });

	$(document).off('click', '[data-onclick=&quot;fnQuickLinks&quot;]').on('click', '[data-onclick=&quot;fnQuickLinks&quot;]', function() {
		fnQuickLinks();
	});

	$(document).off('click', '[data-onclick=&quot;fnDeleteQuickLink&quot;]').on('click', '[data-onclick=&quot;fnDeleteQuickLink&quot;]', function() {
		fnDeleteQuickLink();
	});

	$(document).off('click', '[data-onclick=&quot;fnBackToSummary&quot;]').on('click', '[data-onclick=&quot;fnBackToSummary&quot;]', function() {
		fnBackToSummary();
	});

	$(document).off('click', '[data-onclick=&quot;fnBackToParameterSummary&quot;]').on('click', '[data-onclick=&quot;fnBackToParameterSummary&quot;]', function() {
		fnBackToParameterSummary();
	});

	$(document).off('click', '[data-onclick=&quot;fnBackToWorkTypeSummary&quot;]').on('click', '[data-onclick=&quot;fnBackToWorkTypeSummary&quot;]', function() {
		fnBackToWorkTypeSummary();
	});

	$(document).off('click', '[data-onclick=&quot;fnBackToCategorySummary&quot;]').on('click', '[data-onclick=&quot;fnBackToCategorySummary&quot;]', function() {
		fnBackToCategorySummary();
	});

	$(document).off('click', '[data-onclick=&quot;fnBackToWOMetricSummary&quot;]').on('click', '[data-onclick=&quot;fnBackToWOMetricSummary&quot;]', function() {
		fnBackToWOMetricSummary();
	});
	
	$(document).off('click', '[data-onclick=&quot;fnBackToWOWorkTypeAttributeSummary&quot;]').on('click', '[data-onclick=&quot;fnBackToWOWorkTypeAttributeSummary&quot;]', function() {
		fnBackToWOWorkTypeAttributeSummary();
	});
	
	$(document).off('click', '[data-onclick=&quot;fnRedirectPage&quot;]').on('click', '[data-onclick=&quot;fnRedirectPage&quot;]', function() {
		var param = $(this).data(&quot;param&quot;);
		fnRedirectPage(param);
	});
	
	$(&quot;.historyback&quot;).click(function() 
	{
	    history.back(-1); 
	});

	
	
		
			
				
					
						
							 
								
									
									1
										Basic Details
									
								
							
							
								
									2
										Work Type Mapping
									
								
							
							
						
							
								 
								
							
					  
					
					
						
							
								
									

#mainTable > tbody > tr
{
	height: 0px !important;
}

#target_table > tbody > tr
{
	height: 0px !important;
}

table
{
  border-collapse : unset;
}

th
{
	border-right: none !important;
}

#CommonDataTableId_wrapper > #CommonDataTableId_filter
{
	border-bottom: none !important;
}

#CommonDataTableId
{
	border-top: 1px solid #e5e9ea;
	margin-top: 0px !important;
	border-right: 1px solid #e5e9ea;
}

table.dataTable td
{
	border-top: 1px solid #e5e9ea;
	border-left: 1px solid #e5e9ea;
	padding-left: 8px;
	padding-right: 10px;
}

.dataTables_info
{
	margin-left: 0px !important;
	border-top: 1px solid #e5e9ea;
}

.dataTables_paginate 
{
	border-top: 1px solid #e5e9ea;
}

.dataTables_empty
{
	vertical-align: middle;
}

.userSummaryTable
{
	margin: 0px;
	height: 550px;
	overflow-y: auto;
	overflow-x: hidden;
	padding-right: 5px;
	padding-bottom: 5px;
}

.chkbox-padding
{
	padding-bottom: 5px;
}

#right_table > tbody > tr > td
{
	border-bottom: 1px solid #e5e9ea;
}














        
                
                
        


























 table.dataTable thead .sorting:after
{
   content: &quot;&quot; !important;
}
table.dataTable thead .sorting:before
{
    content: &quot;&quot; !important;
}

table.dataTable thead .sorting_asc:after {
    content: url(/phworkoutput/images/sort_asc.png) !important;
   	opacity:1 !important;
    margin-bottom: 5px;
    margin-right:-10px;
    top: 20% !important;
}
table.dataTable thead .sorting_desc:after {
    content: url(/phworkoutput/images/sort_desc.png) !important;
    opacity:1 !important;
    margin-bottom: 5px;
    margin-right:-10px;
    top: 20% !important;
}
v.dataTables_scrollHead table.dataTable {
		    width:100%;
		}
		
.no-footer
{
	width:100% !important;
} 

.form-inline .form-control 
{
	vertical-align: unset !important;
}

table.dataTable
{
	border-collapse: collapse !important; 
}






















var dynPageLength;
var dynExportColumns;
var dynTitleHeaderName;
var dynFileName;
var dynPdfPageSize;
var dynSortingIndex;
var dynNumericAlignColArr;
var dynWidthArr;
var isMultiOrgAdmin;
var reportScheduleReq;
var sortingOrder;
var recordSize;

var isPaginationExtendEnable = false;
var lengthMenu = isPaginationExtendEnable ? [10, 20, 50, 100, 200, 500, 1000]  : [10, 20, 50, 100, 200];

/* Build Data Tabel */
function buildDataTableStructure(jsonData, pageLength, columns, sortingIndex)
{
	buildDataTableStructureImproved(jsonData, pageLength, columns, sortingIndex, &quot;CommonDataTableId&quot;)
}

/* Build Data Tabel by passing Dynamic Table Id*/
function buildDataTableStructureImproved(jsonData, pageLength, columns, sortingIndex, dynTableId)
{
	var isReqSorting = true;
	if(sortingIndex &lt; 0)
	{
		isReqSorting  = false;
		sortingIndex = 0;
	}
	var table = $('#' +dynTableId+'').DataTable( {
        data: jsonData,
        deferRender:    true,
        &quot;pageLength&quot;: pageLength,
        &quot;ordering&quot; : isReqSorting,
        &quot;autoWidth&quot;:false,
        &quot;scrollX&quot;: true,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class='ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px'> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class='ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100'>[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       '&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>',
                &quot;previous&quot;:   '&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>'
            },
        },
         columns: columns,
         &quot;order&quot;: [[ sortingIndex, 'asc' ]],
         dom: 'Blfrtip',
         buttons: [
            //Empty for no exports
         ]
    } );
}

/* Build Data Tabel With Default Data Table Export */
function buildDataTableStructureWithExports(jsonData, pageLength, columns, exportColumns, titleHeaderName, fileName, pdfPageSize, sortingIndex)
{
	var isReqSorting = true;
	if(sortingIndex &lt; 0)
	{
		isReqSorting  = false;
		sortingIndex = 0;
	}
	var excelReportType = $(&quot;#excelReportType&quot;).val();
	excelReportType = &quot;.&quot; + excelReportType.toLowerCase();
	var disclaimerText = $(&quot;#disclaimerText&quot;).val();
	titleHeaderName = replaceBackXMLEntities(titleHeaderName);
	fileName = replaceBackXMLEntities(fileName);
	var table = $('#CommonDataTableId').DataTable( {
        data: 	jsonData,
        deferRender:    true,
        &quot;pageLength&quot;: pageLength,
        &quot;ordering&quot; : isReqSorting,
        &quot;scrollX&quot;: true,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	 &quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
             &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class='ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px'> Displaying _START_ to _END_ &lt;label>&quot;,
             &quot;infoEmpty&quot;: &quot;No records to display&quot;,
             &quot;infoFiltered&quot;: &quot;&lt;label class='ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100' >[Filtered from _MAX_ records] &lt;/label>&quot;,
             &quot;search&quot;:         &quot;Search&quot;,
             &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
             &quot;paginate&quot;: { 
                &quot;next&quot;:       '&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>',
                &quot;previous&quot;:   '&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>'
            },
        },
         columns: columns,
         &quot;order&quot;: [[ sortingIndex, 'asc' ]],
         dom: 'Blfrtip',
         buttons: [
        	 {
        		 text:   '&lt;label class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot; class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>|&amp;nbsp;&amp;nbsp; Export:&amp;nbsp;&lt;/label>',
        		 autoClose: 'false',
                 tag: 'span'
        	 },
             {
                 extend: 'excelHtml5',
                 exportOptions: {
                 	columns: exportColumns,
                 	orthogonal: 'export',
                 	 stripHtml: true
                 },
                 filename: fileName,
                 text:   '&lt;img src=&quot;/phworkoutput/images/excel.png&quot; title=&quot;Export to Excel&quot; align=&quot;middle&quot; class=&quot;ph-wo-vertical-align-middle ph-wo-cursor-pointer&quot;>',
                 autoClose: 'true',
                 tag: 'span',
                 extension: excelReportType,
                 createEmptyCells: true,
                 title : titleHeaderName,
                 messageBottom: function () {
                     return disclaimerText;
                 	},
                     customize: function ( xlsx ) {
						var sheet = xlsx.xl.worksheets['sheet1.xml'];
						$('row c', sheet).attr( 's', '25' );
                        $('c[r=A1] t', sheet).text( titleHeaderName );
                        $('row:first c', sheet).attr( 's', '27' ); 
                        $('row:eq(1) c', sheet).attr('s', '27');
                        
                        if(disclaimerText != null &amp;&amp; disclaimerText != '')
                        {
                        	$('row:last c', sheet).attr( 's', '55' );
	                        $('row:last', sheet).attr('customHeight', 1);
	                        $('row:last', sheet).attr('ht', '150');
                        }
                      },
                      
                 button: ['excel']

             },
             {
                 extend: 'pdfHtml5',
                 orientation: 'landscape',
                 exportOptions: {
                 	columns: exportColumns,
                 	orthogonal: 'export',
                 	 stripHtml: true
                 },
                 text:      '&lt;img src=&quot;/phworkoutput/images/pdf.png&quot; title=&quot;Export to PDF&quot; align=&quot;middle&quot; class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>',
                 filename: fileName,
                 autoClose: 'true',
                 footer:'true',
                 messageBottom: function () {
                     return disclaimerText;
                  	},
               	 customize: function ( doc ) {
               		if(exportColumns.length &lt;= 4 || pdfPageSize == 'A4')
               		{
	               		doc.content[1].table.widths = Array(doc.content[1].table.body[0].length + 1).join('*').split('');
               		}
               		
               	   var cols = [];
               	   cols[0] = {text: '© JaMocha Tech Pvt. Ltd. 2009-2026', alignment: 'center', fontSize:'10', margin:[0, 0, 0, 0] };
               	   var objFooter = {};
               	   objFooter['columns'] = cols;
               	   doc['footer']=objFooter;
               	   doc.styles.tableHeader.fillColor = '#006bb7';
               	   doc.styles.tableHeader.alignment=&quot;left&quot;;
               	 	
               	   doc['header']=(function(page, pages) {
							return {
								columns: [
									{
										alignment: 'left',
										text: 'ProHance',
										fontSize: 12,
									},
									{
										alignment: 'right',
										fontSize: 8,
										text: ['Page ', { text: page.toString() }]
									}
								],
								margin: 20
							}
						});
               	doc.content.forEach(function(item) {
	               		if (item.table)
	               		{
	               			item.layout = {
	               					hLineColor: function(i, node) { return '#000000'; },
	               					vLineColor: function(i, node) { return '#000000'; }
	               			};
	                    }
	               });
		           doc.styles.tableHeader = {
		        		   fontSize		: 11,
		        		   bold			: true,
		        		   color		: '#ffffff',
		        		   fillColor	: '#006bb7'
			       };
               	 },
                 tag: 'span',
                 pageSize : pdfPageSize,
                 titleAttr : 'PDF',
                 title : titleHeaderName,
                 button: [ 'pdf' ]
             }
         ],
         drawCallback: function() {
          	  var hasRows = this.api().rows({ filter: 'applied' }).data().length > 0;
          	 $('.buttons-excel')[0].style.visibility = hasRows ? 'visible' : 'hidden';
          	 $('.buttons-pdf')[0].style.visibility = hasRows ? 'visible' : 'hidden';
          	}
    } );
	
}

/* Build Data Tabel With Custom Export (Custom Excel And PDF doExport) along With Email Report Schedular */
function buildDataTableStructureWithReportScheduler(jsonData, columns, jsonObjArr)
{
	fnUpdateProperties(jsonObjArr);
	var isReqSorting = true;
	if(dynSortingIndex &lt; 0)
	{
		isReqSorting  = false;
		dynSortingIndex = 0;
	}
	
	var exportExcel = '';
	var exportPdf = '';
	var emailSpan = '';
	if(reportScheduleReq == 'true')
	{
		emailSpan = '&lt;span id=&quot;emailSpan&quot; class=&quot;ph-wo-cursor-pointer&quot; data-onclick=&quot;showSchedulerDialogOnClick&quot;>&lt;img src=&quot;/phworkoutput/images/email.png&quot; title=&quot;Email&quot; align=&quot;middle&quot; class=&quot;ph-wo-vertical-align-middle&quot;>&lt;/span>';
		if($(&quot;#isAllowEmailExport&quot;).val() == 'false')
		{
			emailSpan = '&lt;span id=&quot;emailSpan&quot; class=&quot;ph-wo-cursor-pointer&quot; title=&quot;You do not have permission to download this content&quot;>&lt;img src=&quot;/phworkoutput/images/email.png&quot; title=&quot;Email&quot; align=&quot;middle&quot; class=&quot;ph-wo-vertical-align-middle&quot;>&lt;/span>';
		}
	}
	if(isMultiOrgAdmin == 'true')
	{
		exportExcel = 'excelHtml5';
		exportPdf = 'pdfHtml5';
	}
	var excelIcon = '';
	var pdfIcon = '';
	if($(&quot;#isAllowExcelExport&quot;).val() == 'false')
	{
		excelIcon = '&lt;img src=&quot;/phworkoutput/images/excel.png&quot; title=&quot;You do not have permission to download this content&quot; align=&quot;middle&quot; class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>';
		exportExcel = '';
	}
	else
	{
		excelIcon = '&lt;img src=&quot;/phworkoutput/images/excel.png&quot; title=&quot;Export to Excel&quot; data-onclick=&quot;fnDataTableExportOnClick&quot; data-etype=&quot;excel&quot; align=&quot;middle&quot; class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>';
	}
	if($(&quot;#isAllowPdfExport&quot;).val() == 'false')
	{
		pdfIcon = '&lt;img src=&quot;/phworkoutput/images/pdf.png&quot; title=&quot;You do not have permission to download this content&quot; align=&quot;middle&quot; class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>';
		exportPdf = '';
	}
	else if(recordSize > 10000)
	{
		pdfIcon = '&lt;img src=&quot;/phworkoutput/images/pdf.png&quot; title=&quot;PDF file cannot be downloaded, as number of records more then 10000&quot; align=&quot;middle&quot; class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>';
		exportPdf = '';
	}	
	else
	{
		pdfIcon = '&lt;img src=&quot;/phworkoutput/images/pdf.png&quot; title=&quot;Export to PDF&quot; data-onclick=&quot;fnDataTableExportOnClick&quot; data-etype=&quot;pdf&quot; align=&quot;middle&quot; class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>';
	}
	
	var excelReportType = $(&quot;#excelReportType&quot;).val();
	excelReportType = &quot;.&quot; + excelReportType.toLowerCase();
	titleHeaderName = replaceBackXMLEntities(dynTitleHeaderName);
	fileName = replaceBackXMLEntities(dynFileName);
	var disclaimerText = $(&quot;#disclaimerText&quot;).val();
	var table = $('#CommonDataTableId').DataTable( {
		 &quot;fnDrawCallback&quot;: function() {
	            // after table is redrawndo something here
	        	$(&quot;#thirdViewSortOrder&quot;).val($(&quot;#CommonDataTableId&quot;).dataTable().fnSettings().aaSorting);
	        },
        data: 	jsonData,
        deferRender:    true,
        &quot;pageLength&quot;: dynPageLength,
        &quot;scrollX&quot;: true,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;ordering&quot; : isReqSorting,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	 &quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
             &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class='ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px'> Displaying _START_ to _END_ &lt;label>&quot;,
             &quot;infoEmpty&quot;: &quot;No records to display&quot;,
             &quot;infoFiltered&quot;: &quot;&lt;label class='ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100' >[Filtered from _MAX_ records] &lt;/label>&quot;,
             &quot;search&quot;:         &quot;Search&quot;,
             &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       '&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>',
                &quot;previous&quot;:   '&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>'
            },
        },
         columns: columns,
         &quot;order&quot;: [[ dynSortingIndex, sortingOrder ]],
         dom: 'Blfrtip',
         buttons: [
        	 {
        		 text:   '&lt;label class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>|&amp;nbsp;&amp;nbsp; Export:&amp;nbsp;&lt;/label>',
        		 autoClose: 'false',
                 tag: 'span'
        	 },
        	 {
        		 text:   emailSpan,
        		 autoClose: 'false',
                 tag: 'span'
        	 },
        	 {
                 extend: exportExcel,
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: 'export',
                 	 stripHtml: true
                 },
                 filename: fileName,
                 text:   excelIcon,
                 autoClose: 'true',
                 tag: 'span',
                 extension: excelReportType,
                 createEmptyCells: true,
                 title : titleHeaderName,
                 messageBottom: function () {
                     return disclaimerText;
                 	},
                     customize: function ( xlsx ) {
						var sheet = xlsx.xl.worksheets['sheet1.xml'];
						$('row c', sheet).attr( 's', '25' );
                        $('c[r=A1] t', sheet).text( titleHeaderName );
                        $('row:first c', sheet).attr( 's', '27' ); 
                        $('row:eq(1) c', sheet).attr('s', '27');
                        
                        if(disclaimerText != null &amp;&amp; disclaimerText != '')
                        {
                        	$('row:last c', sheet).attr( 's', '55' );
	                        $('row:last', sheet).attr('customHeight', 1);
	                        $('row:last', sheet).attr('ht', '150');
                        }
                        
                      },
                 button: ['']
             },
             {
                 extend: exportPdf,
                 orientation: 'landscape',
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: 'export',
                 	 stripHtml: true
                 },
                 text:    pdfIcon ,
                 filename: fileName,
                 autoClose: 'true',
                 footer:'true',
                 messageBottom: function () {
                     return disclaimerText;
                  	},
               	 customize: function ( doc ) {
               		
               	   var cols = [];
               	   cols[0] = {text: '© JaMocha Tech Pvt. Ltd. 2009-2026', alignment: 'center', fontSize:'10', margin:[0, 0, 0, 0] };
               	   var objFooter = {};
               	   objFooter['columns'] = cols;
               	   doc['footer']=objFooter;
               	   doc.styles.tableHeader.fillColor = '#006bb7';
               	   doc.styles.tableHeader.alignment=&quot;left&quot;;
               	 	
               	   doc['header']=(function(page, pages) {
							return {
								columns: [
									{
										alignment: 'left',
										text: 'ProHance',
										fontSize: 12,
									},
									{
										alignment: 'right',
										fontSize: 8,
										text: ['Page ', { text: page.toString() }]
									}
								],
								margin: 20
							}
						});
               	   
	               	if(dynNumericAlignColArr.length > 0)
	               	{
	               		var rowCount = doc.content[1].table.body.length;
		               	for (i = 0; i &lt; rowCount; i++) 
		               	{
		               		dynNumericAlignColArr.forEach(function(number) {
		                   		doc.content[1].table.body[i][number].alignment = 'right';
		                   	  });
		               	}
               		}
	               	
	               	doc.content[1].table.widths = dynWidthArr;
	               	doc.content.forEach(function(item) {
	               		if (item.table)
	               		{
	               			item.layout = {
	               					hLineColor: function(i, node) { return '#000000'; },
	               					vLineColor: function(i, node) { return '#000000'; }
	               			};
	                    }
	               });
		           doc.styles.tableHeader = {
		        		   fontSize		: 11,
		        		   bold			: true,
		        		   color		: '#ffffff',
		        		   fillColor	: '#006bb7'
			       };
               	 },
                 tag: 'span',
                 pageSize : 'LEGAL',
                 titleAttr : 'PDF',
                 title : titleHeaderName,
                 button: [ 'pdf' ]
             }
         ]
    } );
}

/* Build Data Tabel With Row Grouping and Default Data Table Export */
function buildDataTableStructureRowsGroups(jsonData, pageLength, columns, exportColumns, titleHeaderName, fileName, pdfPageSize, sortingIndex, rowsGroup, jsonObjArr)
{
	//fnUpdateProperties(jsonObjArr);
	var isReqSorting = true;
	if(sortingIndex &lt; 0)
	{
		isReqSorting  = false;
		sortingIndex = 0;
	}
	var excelReportType = $(&quot;#excelReportType&quot;).val();
	excelReportType = &quot;.&quot; + excelReportType.toLowerCase();  
	var disclaimerText = $(&quot;#disclaimerText&quot;).val();
    titleHeaderName = replaceBackXMLEntities(titleHeaderName);
    fileName = replaceBackXMLEntities(fileName);
	var table = $('#CommonDataTableId').DataTable( {
        data: 	jsonData,
        deferRender:    false,
        &quot;pageLength&quot;: pageLength,
        &quot;ordering&quot; : isReqSorting,
        &quot;scrollX&quot;: false,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	 &quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
             &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class='ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px'> Displaying _START_ to _END_ &lt;label>&quot;,
             &quot;infoEmpty&quot;: &quot;No records to display&quot;,
             &quot;infoFiltered&quot;: &quot;&lt;label class='ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100' >[Filtered from _MAX_ records] &lt;/label>&quot;,
             &quot;search&quot;:         &quot;Search&quot;,
             &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       '&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>',
                &quot;previous&quot;:   '&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>'
            },
        },
         columns: columns,
         columnDefs: [{
        	    &quot;defaultContent&quot;: &quot;&quot;,
        	    &quot;targets&quot;: &quot;_all&quot;
        	  }],    
         &quot;order&quot;: [[ sortingIndex, 'asc' ]],
         rowGroup: rowsGroup,      
         dom: 'Blfrtip',
         buttons: [
             //Empty for no exports
          ]
    });
	
}

function fnUpdateProperties(jsonObjArr) 
{
	dynPageLength = jsonObjArr[&quot;pageLength&quot;];
	dynExportColumns = jsonObjArr[&quot;exportColumns&quot;];
	dynTitleHeaderName = jsonObjArr[&quot;titleHeaderName&quot;];
	dynFileName = jsonObjArr[&quot;fileName&quot;];
	dynPdfPageSize = jsonObjArr[&quot;pdfPageSize&quot;];
	dynSortingIndex = jsonObjArr[&quot;sortingIndex&quot;];
	dynNumericAlignColArr = jsonObjArr[&quot;rightAlignColArr&quot;];
	dynWidthArr	=	jsonObjArr[&quot;dynWidthArr&quot;];
	isMultiOrgAdmin	=	jsonObjArr[&quot;isMultiOrgAdmin&quot;];
	reportScheduleReq	=	jsonObjArr[&quot;reportScheduleReq&quot;];
	if(jsonObjArr[&quot;order&quot;] != null &amp;&amp; jsonObjArr[&quot;order&quot;] != '')
	{
		sortingOrder	=	jsonObjArr[&quot;order&quot;];
	}
	else 
	{
		sortingOrder = &quot;asc&quot;;
	}
	recordSize	=	jsonObjArr[&quot;recordSize&quot;];
}

/* Build Data Tabel with Default Data Table Export along with JSONArray param*/
function buildDataTableStructureWithExportsArr(jsonData, columns, jsonObjArr)
{
	fnUpdateProperties(jsonObjArr);
	var isReqSorting = true;
	if(dynSortingIndex &lt; 0)
	{
		isReqSorting  = false;
		dynSortingIndex = 0;
	}
	var excelReportType = $(&quot;#excelReportType&quot;).val();
	excelReportType = &quot;.&quot; + excelReportType.toLowerCase();
	var disclaimerText = $(&quot;#disclaimerText&quot;).val();
	titleHeaderName = replaceBackXMLEntities(dynTitleHeaderName);
	fileName = replaceBackXMLEntities(dynFileName);
	var table = $('#CommonDataTableId').DataTable( {
        data: 	jsonData,
        deferRender:    true,
        &quot;pageLength&quot;: dynPageLength,
        &quot;bDestroy&quot;: true,
        &quot;scrollX&quot;: true,
        &quot;ordering&quot; : isReqSorting,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class='ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px'> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class='ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100' >[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       '&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>',
                &quot;previous&quot;:   '&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>'
            },
        },
         columns: columns,
         &quot;order&quot;: [[ dynSortingIndex, 'asc' ]],
         rowGroup: {
             dataSrc: 88
         },
         dom: 'Blfrtip',
         buttons: [
        	 {
				text:   '&lt;label class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>|&amp;nbsp;&amp;nbsp; Export:&amp;nbsp;&lt;/label>',
        		 autoClose: 'false',
                 tag: 'span'
        	 },
             {
                 extend: 'excelHtml5',
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: 'export',
                 	 stripHtml: true
                 },
                 destroy:true,
                 filename: fileName,
				 text:   '&lt;img src=&quot;/phworkoutput/images/excel.png&quot; title=&quot;Export to Excel&quot; align=&quot;middle&quot; class=&quot;ph-wo-vertical-align-middle ph-wo-cursor-pointer&quot;>',
                 autoClose: 'true',
                 tag: 'span',
                 extension: excelReportType,
                 createEmptyCells: true,
                 title : titleHeaderName,
                 messageBottom: function () {
                     return disclaimerText;
                 	},
                     customize: function ( xlsx ) {
						 var sheet = xlsx.xl.worksheets['sheet1.xml'];
						$('row c', sheet).attr( 's', '25' );
                        $('c[r=A1] t', sheet).text( titleHeaderName );
                        $('row:first c', sheet).attr( 's', '27' ); 
                        $('row:eq(1) c', sheet).attr('s', '27');
                        
                        if(disclaimerText != null &amp;&amp; disclaimerText != '')
                        {
                        	$('row:last c', sheet).attr( 's', '55' );
	                        $('row:last', sheet).attr('customHeight', 1);
	                        $('row:last', sheet).attr('ht', '150');
                        }
                      },
                      
                 button: ['excel']

             },
             {
                 extend: 'pdfHtml5',
                 orientation: 'landscape',
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: 'export',
                 	 stripHtml: true
                 },
				 text:      '&lt;img src=&quot;/phworkoutput/images/pdf.png&quot; title=&quot;Export to PDF&quot; align=&quot;middle&quot; class=&quot;ph-wo-vertical-align-middle ph-wo-cursor-pointer&quot;>',
                 filename: fileName,
                 autoClose: 'true',
                 footer:'true',
                 destroy:true,
                 messageBottom: function () {
                     return disclaimerText;
                  	},
               	 customize: function ( doc ) {
               		
               	   var cols = [];
               	   cols[0] = {text: '© JaMocha Tech Pvt. Ltd. 2009-2026', alignment: 'center', fontSize:'10', margin:[0, 0, 0, 0] };
               	   var objFooter = {};
               	   objFooter['columns'] = cols;
               	   doc['footer']=objFooter;
               	   doc.styles.tableHeader.fillColor = '#006bb7';
               	   doc.styles.tableHeader.alignment=&quot;left&quot;;
               	 	
               	   doc['header']=(function(page, pages) {
							return {
								columns: [
									{
										alignment: 'left',
										text: 'ProHance',
										fontSize: 12,
									},
									{
										alignment: 'right',
										fontSize: 8,
										text: ['Page ', { text: page.toString() }]
									}
								],
								margin: 20
							}
						});
	               	if(dynNumericAlignColArr.length > 0)
	               	{
	               		var rowCount = doc.content[1].table.body.length;
		               	for (i = 0; i &lt; rowCount; i++) 
		               	{
		               		dynNumericAlignColArr.forEach(function(number) {
		                   		doc.content[1].table.body[i][number].alignment = 'right';
		                   	  });
		               	}
	               	}
               	
	               doc.content[1].table.widths=	dynWidthArr;
               	
               		doc.content.forEach(function(item) {
	               		if (item.table)
	               		{
	               			item.layout = {
	               					hLineColor: function(i, node) { return '#000000'; },
	               					vLineColor: function(i, node) { return '#000000'; }
	               			};
	                    }
	               });
		           doc.styles.tableHeader = {
		        		   fontSize		: 11,
		        		   bold			: true,
		        		   color		: '#ffffff',
		        		   fillColor	: '#006bb7'
			       };
               		doc.content.forEach(function(item) {
	               		if (item.table)
	               		{
	               			item.layout = {
	               					hLineColor: function(i, node) { return '#000000'; },
	               					vLineColor: function(i, node) { return '#000000'; }
	               			};
	                    }
	               });
		           doc.styles.tableHeader = {
		        		   fontSize		: 11,
		        		   bold			: true,
		        		   color		: '#ffffff',
		        		   fillColor	: '#006bb7'
			       };
               	 },
                 tag: 'span',
                 pageSize : 'LEGAL',
                 titleAttr : 'PDF',
                 title : titleHeaderName,
                 button: [ 'pdf' ]
             }
         ],
         drawCallback: function() {
       	  var hasRows = this.api().rows({ filter: 'applied' }).data().length > 0;
       	 $('.buttons-excel')[0].style.visibility = hasRows ? 'visible' : 'hidden';
       	 $('.buttons-pdf')[0].style.visibility = hasRows ? 'visible' : 'hidden';
       	}
    } );
	
}

/* Build Data table for WO Work Type Attributes*/
function buildDataTableStructureForWorkTypeAttributesBulkUpload(jsonData, columns, jsonObjArr)
{
	fnUpdateProperties(jsonObjArr);
	var isReqSorting = true;
	if(dynSortingIndex &lt; 0)
	{
		isReqSorting  = false;
		dynSortingIndex = 0;
	}
	var excelReportType = $(&quot;#excelReportType&quot;).val();
	excelReportType = &quot;.&quot; + excelReportType.toLowerCase();
	var disclaimerText = $(&quot;#disclaimerText&quot;).val();
	titleHeaderName = replaceBackXMLEntities(dynTitleHeaderName);
	fileName = replaceBackXMLEntities(dynFileName);
	var table = $('#CommonDataTableId').DataTable( {
        data: 	jsonData,
        deferRender:    true,
        &quot;pageLength&quot;: dynPageLength,
        &quot;bDestroy&quot;: true,
        &quot;scrollX&quot;: true,
        &quot;ordering&quot; : isReqSorting,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class='ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px'> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class='ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100' >[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       '&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>',
                &quot;previous&quot;:   '&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>'
            },
        },
         columns: columns,
         &quot;order&quot;: [[ dynSortingIndex, sortingOrder ]],
         rowGroup: {
             dataSrc: 88
         },
         dom: 'Blfrtip',
         buttons: [
        	 {
        		 text:   '&lt;label class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>|&amp;nbsp;&amp;nbsp; Export:&amp;nbsp;&lt;/label>',
        		 autoClose: 'false',
                 tag: 'span'
        	 },
             {
                 extend: 'excelHtml5',
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: 'export',
                 	 stripHtml: true
                 },
                 destroy:true,
                 filename: fileName,
                 text:   '&lt;img src=&quot;/phworkoutput/images/excel.png&quot; title=&quot;Export to Excel&quot; align=&quot;middle&quot; class=&quot;ph-wo-vertical-align-middle ph-wo-cursor-pointer&quot;>',
                 autoClose: 'true',
                 tag: 'span',
                 extension: excelReportType,
                 createEmptyCells: true,
                 title : titleHeaderName,
                 messageBottom: function () {
                     return disclaimerText;
                 	},
                     customize: function ( xlsx ) {
						 var sheet = xlsx.xl.worksheets['sheet1.xml'];
						$('row c', sheet).attr( 's', '25' );
                        $('c[r=A1] t', sheet).text( titleHeaderName );
                        $('row:first c', sheet).attr( 's', '27' ); 
                        $('row:eq(1) c', sheet).attr('s', '27');
                        
                        if(disclaimerText != null &amp;&amp; disclaimerText != '')
                        {
                        	$('row:last c', sheet).attr( 's', '55' );
	                        $('row:last', sheet).attr('customHeight', 1);
	                        $('row:last', sheet).attr('ht', '150');
                        }
                         
                      },
                      
                 button: ['excel']

             },
             {
                 extend: 'pdfHtml5',
                 orientation: 'landscape',
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: 'export',
                 	 stripHtml: true
                 },
                 text:      '&lt;img src=&quot;/phworkoutput/images/pdf.png&quot; title=&quot;Export to PDF&quot; align=&quot;middle&quot; class=&quot;ph-wo-vertical-align-middle ph-wo-cursor-pointer&quot;>',
                 filename: fileName,
                 autoClose: 'true',
                 footer:'true',
                 destroy:true,
                 messageBottom: function () {
                     return disclaimerText;
                  	},
               	 customize: function ( doc ) {
               		
               	   var cols = [];
               	   cols[0] = {text: '© JaMocha Tech Pvt. Ltd. 2009-2026', alignment: 'center', fontSize:'10', margin:[0, 0, 0, 0] };
               	   var objFooter = {};
               	   objFooter['columns'] = cols;
               	   doc['footer']=objFooter;
               	   doc.styles.tableHeader.fillColor = '#006bb7';
               	   doc.styles.tableHeader.alignment=&quot;left&quot;;
               	 	
               	   doc['header']=(function(page, pages) {
							return {
								columns: [
									{
										alignment: 'left',
										text: 'ProHance',
										fontSize: 12,
									},
									{
										alignment: 'right',
										fontSize: 8,
										text: ['Page ', { text: page.toString() }]
									}
								],
								margin: 20
							}
						});
	               	if(dynNumericAlignColArr.length > 0)
	               	{
	               		var rowCount = doc.content[1].table.body.length;
		               	for (i = 0; i &lt; rowCount; i++) 
		               	{
		               		dynNumericAlignColArr.forEach(function(number) {
		                   		doc.content[1].table.body[i][number].alignment = 'right';
		                   	  });
		               	}
	               	}
               	
	               doc.content[1].table.widths=	dynWidthArr;
	               doc.content.forEach(function(item) {
	               		if (item.table)
	               		{
	               			item.layout = {
	               					hLineColor: function(i, node) { return '#000000'; },
	               					vLineColor: function(i, node) { return '#000000'; }
	               			};
	                    }
	               });
		           doc.styles.tableHeader = {
		        		   fontSize		: 11,
		        		   bold			: true,
		        		   color		: '#ffffff',
		        		   fillColor	: '#006bb7'
			       };
               	
               	 },
                 tag: 'span',
                 pageSize : 'LEGAL',
                 titleAttr : 'PDF',
                 title : titleHeaderName,
                 button: [ 'pdf' ]
             }
         ],
         drawCallback: function() {
       	  var hasRows = this.api().rows({ filter: 'applied' }).data().length > 0;
       	 $('.buttons-excel')[0].style.visibility = hasRows ? 'visible' : 'hidden';
       	 $('.buttons-pdf')[0].style.visibility = hasRows ? 'visible' : 'hidden';
       	}
    } );
	
}

/* Build Data Tabel along with SX - scrollX param */
function buildScrollXDataTableStructure(jsonData, pageLength, columns, sortingIndex, isScrollX)
{
	var isReqSorting = true;
	if(sortingIndex &lt; 0)
	{
		isReqSorting  = false;
		sortingIndex = 0;
	}
	var table = $('#CommonDataTableId').DataTable( {
        data: jsonData,
        deferRender:    true,
        &quot;pageLength&quot;: pageLength,
        &quot;ordering&quot; : isReqSorting,
        &quot;scrollX&quot;: isScrollX,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class='ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px'> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class='ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100' >[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       '&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>',
                &quot;previous&quot;:   '&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>'
            },
        },
         columns: columns,
         &quot;order&quot;: [[ sortingIndex, 'asc' ]],
         dom: 'Blfrtip',
         buttons: [
            //Empty for no exports
         ]
    } );
}

/* Build Data Tabel with Defalt Data Table Export along with SX - scrollX param */
function buildDataTableStructureWithExportsArrScrollX(jsonData, columns, jsonObjArr, isScrollX)
{
	fnUpdateProperties(jsonObjArr);
	var isReqSorting = true;
	if(dynSortingIndex &lt; 0)
	{
		isReqSorting  = false;
		dynSortingIndex = 0;
	}
	var excelReportType = $(&quot;#excelReportType&quot;).val();
	excelReportType = &quot;.&quot; + excelReportType.toLowerCase();
	var disclaimerText = $(&quot;#disclaimerText&quot;).val();
	titleHeaderName = replaceBackXMLEntities(dynTitleHeaderName);
	fileName = replaceBackXMLEntities(dynFileName);
	var table = $('#CommonDataTableId').DataTable( {
        data: 	jsonData,
        deferRender:    true,
        &quot;pageLength&quot;: dynPageLength,
        &quot;bDestroy&quot;: true,
        &quot;scrollX&quot;: isScrollX,
        &quot;ordering&quot; : isReqSorting,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class='ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px'> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class='ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100' >[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       '&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>',
                &quot;previous&quot;:   '&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>'
            },
        },
         columns: columns,
         &quot;order&quot;: [[ dynSortingIndex, 'asc' ]],
         rowGroup: {
             dataSrc: 88
         },
         dom: 'Blfrtip',
         buttons: [
        	 {
        		 text:   '&lt;label class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>|&amp;nbsp;&amp;nbsp; Export:&amp;nbsp;&lt;/label>',
        		 autoClose: 'false',
                 tag: 'span'
        	 },
             {
                 extend: 'excelHtml5',
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: 'export',
                 	 stripHtml: true,
                 	 format: {
                        header: function (data, columnIdx) {
                            if ((columnIdx === 6 &amp;&amp; columns.length == 8) || (columnIdx === 5 &amp;&amp; columns.length === 7))
                            {   
                                return 'Part of Filter';
                            }
                            return data;
                        }
                    }
                 },
                 destroy:true,
                 filename: fileName,
                 text:   '&lt;img src=&quot;/phworkoutput/images/excel.png&quot; title=&quot;Export to Excel&quot; align=&quot;middle&quot; class=&quot;ph-wo-vertical-align-middle ph-wo-cursor-pointer&quot;>',
                 autoClose: 'true',
                 tag: 'span',
                 extension: excelReportType,
                 createEmptyCells: true,
                 title : titleHeaderName,
                 messageBottom: function () {
                     return disclaimerText;
                 	},
                     customize: function ( xlsx ) {
						 var sheet = xlsx.xl.worksheets['sheet1.xml'];
						$('row c', sheet).attr( 's', '25' );
                        $('c[r=A1] t', sheet).text( titleHeaderName );
                        $('row:first c', sheet).attr( 's', '27' ); 
                        $('row:eq(1) c', sheet).attr('s', '27');
                        
                        if(disclaimerText != null &amp;&amp; disclaimerText != '')
                        {
                        	$('row:last c', sheet).attr( 's', '55' );
	                        $('row:last', sheet).attr('customHeight', 1);
	                        $('row:last', sheet).attr('ht', '150');
                        }
                         
                      },
                      
                 button: ['excel']

             },
             {
                 extend: 'pdfHtml5',
                 orientation: 'landscape',
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: 'export',
                 	stripHtml: true,
                 	format: {
                        header: function (data, columnIdx) {
                            if ((columnIdx === 6 &amp;&amp; columns.length == 8) || (columnIdx === 5 &amp;&amp; columns.length === 7))
                            {   
                                return 'Part of Filter';
                            }
                            return data;
                        }
                    }
                 },
                 text:      '&lt;img src=&quot;/phworkoutput/images/pdf.png&quot; title=&quot;Export to PDF&quot; align=&quot;middle&quot; class=&quot;ph-wo-vertical-align-middle ph-wo-cursor-pointer&quot;>',
                 filename: fileName,
                 autoClose: 'true',
                 footer:'true',
                 destroy:true,
                 messageBottom: function () {
                     return disclaimerText;
                  	},
               	 customize: function ( doc ) {
               		
               	   var cols = [];
               	   cols[0] = {text: '© JaMocha Tech Pvt. Ltd. 2009-2026', alignment: 'center', fontSize:'10', margin:[0, 0, 0, 0] };
               	   var objFooter = {};
               	   objFooter['columns'] = cols;
               	   doc['footer']=objFooter;
               	   doc.styles.tableHeader.fillColor = '#006bb7';
               	   doc.styles.tableHeader.alignment=&quot;left&quot;;
               	 	
               	   doc['header']=(function(page, pages) {
							return {
								columns: [
									{
										alignment: 'left',
										text: 'ProHance',
										fontSize: 12,
									},
									{
										alignment: 'right',
										fontSize: 8,
										text: ['Page ', { text: page.toString() }]
									}
								],
								margin: 20
							}
						});
	               	if(dynNumericAlignColArr.length > 0)
	               	{
	               		var rowCount = doc.content[1].table.body.length;
		               	for (i = 0; i &lt; rowCount; i++) 
		               	{
		               		dynNumericAlignColArr.forEach(function(number) {
		                   		doc.content[1].table.body[i][number].alignment = 'right';
		                   	  });
		               	}
	               	}
               	
	               doc.content[1].table.widths=	dynWidthArr;
	               doc.content.forEach(function(item) {
	               		if (item.table)
	               		{
	               			item.layout = {
	               					hLineColor: function(i, node) { return '#000000'; },
	               					vLineColor: function(i, node) { return '#000000'; }
	               			};
	                    }
	               });
		           doc.styles.tableHeader = {
		        		   fontSize		: 11,
		        		   bold			: true,
		        		   color		: '#ffffff',
		        		   fillColor	: '#006bb7'
			       };
               	
               	 },
                 tag: 'span',
                 pageSize : 'LEGAL',
                 titleAttr : 'PDF',
                 title : titleHeaderName,
                 button: [ 'pdf' ]
             }
         ],
         drawCallback: function() {
        	 var api = this.api();
       	  	 var hasRows = api.rows({ filter: 'applied' }).data().length > 0;
       	  	 $('.buttons-excel')[0].style.visibility = hasRows ? 'visible' : 'hidden';
       	  	 $('.buttons-pdf')[0].style.visibility = hasRows ? 'visible' : 'hidden';
       	  	 api.rows({ page: 'current' }).every(function () {
	             var data = this.data();
	             if (data &amp;&amp; data.options) 
	             {
	                initSlimScroll(data.options.length, data.id);
	             }
          	 });
       	}
    } );
}

/* Build Data Tabel with Row Grouping, along with SX - scrollX param */
function buildDTRowGroupingWithoutExportSX(jsonData, pageLength, columns, sortingIndex, isScrollX, rowsGroup, ascnding)
{
	var isReqSorting = true;
	if(sortingIndex &lt; 0)
	{
		isReqSorting  = false;
		sortingIndex = 0;
	}
	var table = $('#CommonDataTableId').DataTable({
        data: jsonData,
        deferRender:    true,
        &quot;pageLength&quot;: pageLength,
        &quot;ordering&quot; : isReqSorting,
        &quot;scrollX&quot;: isScrollX,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class='ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px'> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class='ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100' >[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       '&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>',
                &quot;previous&quot;:   '&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>'
            },
        },
         columns: columns,
         columnDefs: [{
       	    &quot;defaultContent&quot;: &quot;&quot;,
       	    &quot;targets&quot;: &quot;_all&quot;
       	  }],
         &quot;order&quot;: [[ sortingIndex, ascnding ]],
         rowGroup: rowsGroup,
         dom: 'Blfrtip',
         &quot;bDestroy&quot;: true,
         buttons: [
            //Empty for no exports
         ]
    });
}

/* Build Data Tabel with Defalt Data Table, Export and Report Schedular Email along With SX - scrollX param */
function buildDTWithDTExportsWithRSEmailSX(jsonData, columns, jsonObjArr, isScrollX)
{
	fnUpdateProperties(jsonObjArr);
	var isReqSorting = true;
	if(dynSortingIndex &lt; 0)
	{
		isReqSorting  = false;
		dynSortingIndex = 0;
	}
	
	if(reportScheduleReq == 'true')
	{
		emailSpan = '&lt;span id=&quot;emailSpan&quot; class=&quot;ph-wo-cursor-pointer&quot; data-onclick=&quot;showSchedulerDialogOnClick&quot;>&lt;img src=&quot;/phworkoutput/images/email.png&quot; title=&quot;Email&quot; align=&quot;middle&quot; class=&quot;ph-wo-vertical-align-middle&quot;>&lt;/span>';
		if($(&quot;#isAllowEmailExport&quot;).val() == 'false')
		{
			emailSpan = '&lt;span id=&quot;emailSpan&quot; class=&quot;ph-wo-cursor-pointer&quot; title=&quot;You do not have permission to download this content&quot;>&lt;img src=&quot;/phworkoutput/images/email.png&quot; title=&quot;Email&quot; align=&quot;middle&quot; class=&quot;ph-wo-vertical-align-middle&quot;>&lt;/span>';
		}
	}
	
	var excelReportType = $(&quot;#excelReportType&quot;).val();
	excelReportType = &quot;.&quot; + excelReportType.toLowerCase();
	var disclaimerText = $(&quot;#disclaimerText&quot;).val();
	titleHeaderName = replaceBackXMLEntities(dynTitleHeaderName);
	fileName = replaceBackXMLEntities(dynFileName);
	var table = $('#CommonDataTableId').DataTable( {
        data: 	jsonData,
        deferRender:    true,
        &quot;pageLength&quot;: dynPageLength,
        &quot;bDestroy&quot;: true,
        &quot;scrollX&quot;: isScrollX,
        &quot;ordering&quot; : isReqSorting,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class='ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px'> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class='ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100' >[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       '&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>',
                &quot;previous&quot;:   '&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>'
            },
        },
         columns: columns,
         &quot;order&quot;: [[ dynSortingIndex, 'asc' ]],
         rowGroup: {
             dataSrc: 88
         },
         dom: 'Blfrtip',
         buttons: [
        	 {
        		 text:   '&lt;label class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>|&amp;nbsp;&amp;nbsp; Export:&amp;nbsp;&lt;/label>',
        		 autoClose: 'false',
                 tag: 'span'
        	 },
        	 {
        		 text:   emailSpan,
        		 autoClose: 'false',
                 tag: 'span'
        	 },
             {
                 extend: 'excelHtml5',
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: 'export',
                 	 stripHtml: true
                 },
                 destroy:true,
                 filename: fileName,
                 text:   '&lt;img src=&quot;/phworkoutput/images/excel.png&quot; title=&quot;Export to Excel&quot; align=&quot;middle&quot; class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>',
                 autoClose: 'true',
                 tag: 'span',
                 extension: excelReportType,
                 createEmptyCells: true,
                 title : titleHeaderName,
                 messageBottom: function () {
                     return disclaimerText;
                 	},
                     customize: function ( xlsx ) {
						 var sheet = xlsx.xl.worksheets['sheet1.xml'];
						$('row c', sheet).attr( 's', '25' );
                        $('c[r=A1] t', sheet).text( titleHeaderName );
                        $('row:first c', sheet).attr( 's', '27' ); 
                        $('row:eq(1) c', sheet).attr('s', '27');
                        
                        if(disclaimerText != null &amp;&amp; disclaimerText != '')
                        {
                        	$('row:last c', sheet).attr( 's', '55' );
	                        $('row:last', sheet).attr('customHeight', 1);
	                        $('row:last', sheet).attr('ht', '150');
                        }
                      },
                      
                 button: ['excel']
             },
             {
                 extend: 'pdfHtml5',
                 orientation: 'landscape',
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: 'export',
                 	 stripHtml: true
                 },
                 text:      '&lt;img src=&quot;/phworkoutput/images/pdf.png&quot; title=&quot;Export to PDF&quot; align=&quot;middle&quot; class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>',
                 filename: fileName,
                 autoClose: 'true',
                 footer:'true',
                 destroy:true,
                 messageBottom: function () {
                     return disclaimerText;
                  	},
               	 customize: function ( doc ) {
               		
               	   var cols = [];
               	   cols[0] = {text: '© JaMocha Tech Pvt. Ltd. 2009-2026', alignment: 'center', fontSize:'10', margin:[0, 0, 0, 0] };
               	   var objFooter = {};
               	   objFooter['columns'] = cols;
               	   doc['footer']=objFooter;
               	   doc.styles.tableHeader.fillColor = '#006bb7';
               	   doc.styles.tableHeader.alignment=&quot;left&quot;;
               	 	
               	   doc['header']=(function(page, pages) {
							return {
								columns: [
									{
										alignment: 'left',
										text: 'ProHance',
										fontSize: 12,
									},
									{
										alignment: 'right',
										fontSize: 8,
										text: ['Page ', { text: page.toString() }]
									}
								],
								margin: 20
							}
						});
	               	if(dynNumericAlignColArr.length > 0)
	               	{
	               		var rowCount = doc.content[1].table.body.length;
		               	for (i = 0; i &lt; rowCount; i++) 
		               	{
		               		dynNumericAlignColArr.forEach(function(number) {
		                   		doc.content[1].table.body[i][number].alignment = 'right';
		                   	  });
		               	}
	               	}
               	
	               doc.content[1].table.widths=	dynWidthArr;
	               doc.content.forEach(function(item) {
	               		if (item.table)
	               		{
	               			item.layout = {
	               					hLineColor: function(i, node) { return '#000000'; },
	               					vLineColor: function(i, node) { return '#000000'; }
	               			};
	                    }
	               });
		           doc.styles.tableHeader = {
		        		   fontSize		: 11,
		        		   bold			: true,
		        		   color		: '#ffffff',
		        		   fillColor	: '#006bb7'
			       };
               	
               	 },
                 tag: 'span',
                 pageSize : 'LEGAL',
                 titleAttr : 'PDF',
                 title : titleHeaderName,
                 button: [ 'pdf' ]
             }
         ],
         drawCallback: function() {
       	  var hasRows = this.api().rows({ filter: 'applied' }).data().length > 0;
       	 $('.buttons-excel')[0].style.visibility = hasRows ? 'visible' : 'hidden';
       	 $('.buttons-pdf')[0].style.visibility = hasRows ? 'visible' : 'hidden';
       	}
    } );
}

$(document).off('click', '[data-onclick=&quot;showSchedulerDialogOnClick&quot;]').on('click', '[data-onclick=&quot;showSchedulerDialogOnClick&quot;]', function() {
    showSchedulerDialog();
});

$(document).off('click', '[data-onclick=&quot;fnDataTableExportOnClick&quot;]').on('click', '[data-onclick=&quot;fnDataTableExportOnClick&quot;]', function() {
    var etype = $(this).data('etype');
    fnDataTableExport(etype);
});

/* Build Data Tabel With Custom Export (Custom Excel And PDF doExport) For Work Type Definition Screen */
function buildDataTableStructureWithExportsForWorkType(jsonData, columns, jsonObjArr)
{
	fnUpdateProperties(jsonObjArr);
	var isReqSorting = true;
	if(dynSortingIndex &lt; 0)
	{
		isReqSorting  = false;
		dynSortingIndex = 0;
	}
	
	var exportExcel = '';
	var exportPdf = '';
	var emailSpan = '';
	var excelIcon = '';
	var pdfIcon = '';
	
	excelIcon = '&lt;img src=&quot;/phworkoutput/images/excel.png&quot; title=&quot;Export to Excel&quot; data-onclick=&quot;fnDataTableExportexpdf&quot; data-etype=&quot;excel&quot; align=&quot;middle&quot; class=&quot;ph-wo-vertical-align-middle ph-wo-cursor-pointer&quot;>';
	
	if(recordSize > 10000)
	{
		pdfIcon = '&lt;img src=&quot;/phworkoutput/images/pdf.png&quot; title=&quot;PDF file cannot be downloaded, as number of records more then 10000&quot; align=&quot;middle&quot; class=&quot;ph-wo-vertical-align-middle ph-wo-cursor-pointer&quot;>';
		exportPdf = '';
	}	
	else
	{
		pdfIcon = '&lt;img src=&quot;/phworkoutput/images/pdf.png&quot; title=&quot;Export to PDF&quot; data-onclick=&quot;fnDataTableExportexpdf&quot; data-etype=&quot;pdf&quot; class=&quot;ph-wo-vertical-align-middle ph-wo-cursor-pointer&quot;>';
	}
	
	var excelReportType = $(&quot;#excelReportType&quot;).val();
	excelReportType = &quot;.&quot; + excelReportType.toLowerCase();
	
	titleHeaderName = replaceBackXMLEntities(dynTitleHeaderName);
	fileName = replaceBackXMLEntities(dynFileName);
	
	var disclaimerText = $(&quot;#disclaimerText&quot;).val();
	var table = $('#CommonDataTableId').DataTable( {
        data: 	jsonData,
        deferRender:    true,
        &quot;pageLength&quot;: dynPageLength,
        &quot;scrollX&quot;: true,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;ordering&quot; : isReqSorting,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
            &quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class='ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px'> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class='dataTables_ExportsInfoFilter' >[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
            &quot;next&quot;:       '&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>',
            &quot;previous&quot;:   '&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>'
            },
        },
         columns: columns,
         columnDefs: [{
      	    &quot;defaultContent&quot;: &quot;&quot;,
      	    &quot;targets&quot;: &quot;_all&quot;
      	  }],
         &quot;order&quot;: [[ dynSortingIndex, 'asc' ]],
         dom: 'Blfrtip',
         buttons: [
        	 {
        	 	 text:   '&lt;label class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>|&amp;nbsp;&amp;nbsp; Export:&amp;nbsp;&lt;/label>',
        		 autoClose: 'false',
                 tag: 'span'
        	 },
        	 {
        		 text:   emailSpan,
        		 autoClose: 'false',
                 tag: 'span'
        	 },
        	 {
                 extend: exportExcel,
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: 'export',
                 	 stripHtml: true
                 },
                 filename: fileName,
                 text:   excelIcon,
                 autoClose: 'true',
                 tag: 'span',
                 extension: excelReportType,
                 createEmptyCells: true,
                 title : titleHeaderName,
                 messageBottom: function () {
                     return disclaimerText;
                 	},
                     customize: function ( xlsx ) {
						var sheet = xlsx.xl.worksheets['sheet1.xml'];
						$('row c', sheet).attr( 's', '25' );
                        $('c[r=A1] t', sheet).text( titleHeaderName );
                        $('row:first c', sheet).attr( 's', '27' ); 
                        $('row:eq(1) c', sheet).attr('s', '27');
                        
                        if(disclaimerText != null &amp;&amp; disclaimerText != '')
                        {
                        	$('row:last c', sheet).attr( 's', '55' );
	                        $('row:last', sheet).attr('customHeight', 1);
	                        $('row:last', sheet).attr('ht', '150');
                        }
                        
                      },
                 button: ['']
             },
             {
                 extend: exportPdf,
                 orientation: 'landscape',
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: 'export',
                 	 stripHtml: true
                 },
                 text:    pdfIcon ,
                 filename: fileName,
                 autoClose: 'true',
                 footer:'true',
                 messageBottom: function () {
                     return disclaimerText;
                  	},
               	 customize: function ( doc ) {
               		
               	   var cols = [];
               	   cols[0] = {text: '© JaMocha Tech Pvt. Ltd. 2009-2026', alignment: 'center', fontSize:'10', margin:[0, 0, 0, 0] };
               	   var objFooter = {};
               	   objFooter['columns'] = cols;
               	   doc['footer']=objFooter;
               	   doc.styles.tableHeader.alignment=&quot;left&quot;;
               	 	
               	   doc['header']=(function(page, pages) {
							return {
								columns: [
									{
										alignment: 'left',
										text: 'ProHance',
										fontSize: 12,
									},
									{
										alignment: 'right',
										fontSize: 8,
										text: ['Page ', { text: page.toString() }]
									}
								],
								margin: 20
							}
						});
               	   
	               	if(dynNumericAlignColArr.length > 0)
	               	{
	               		var rowCount = doc.content[1].table.body.length;
		               	for (i = 0; i &lt; rowCount; i++) 
		               	{
		               		dynNumericAlignColArr.forEach(function(number) {
		                   		doc.content[1].table.body[i][number].alignment = 'right';
		                   	  });
		               	}
               		}
	               	
	               	doc.content[1].table.widths = dynWidthArr;
	                doc.content.forEach(function(item) {
	                       if (item.table) {
	                           item.layout = {
	                               hLineColor: function(i, node) { return '#000000'; }, // Set the border color here
	                               vLineColor: function(i, node) { return '#000000'; }, // Set the border color here
	                           };
	                       }
	                   });
	                doc.styles.tableHeader = {
	            		   fontSize: 11,
	                       bold: true,
	                       color: '#ffffff',  // Set font color
	                       fillColor: '#006bb7'
		               };
               	 },
                 tag: 'span',
                 pageSize : 'LEGAL',
                 titleAttr : 'PDF',
                 title : titleHeaderName,
                 button: [ 'pdf' ]
             }
         ]
    } );
	setTimeout(function() {
		table.columns.adjust().draw();
	}, 20); 
}

function fnDataTableExportexpdf(data)
{
	var param = data.data('param');
	fnDataTableExport(param);
}

function buildDataTableStructureForServerSideWorkTypes(pageLength, columnArr, sortingIndex, dynTableId, dataCollectionId, menuItemId, userGroupNameIds)
{
	$.fn.DataTable.ext.pager.numbers_length = 5;
	$.fn.DataTable.ext.pager.customPager = function (page, pages) {
	    const buttons = [];
	    const startPage = Math.max(page - 1, 1);
	    const endPage = Math.min(startPage + 1, pages - 2);

	    buttons.push(0);
	    if (startPage > 2)
	    {
	        buttons.push('ellipsis');
	    }

	    for (let i = startPage; i &lt;= endPage; i++)
	    {
	        buttons.push(i);
	    }

	    if (endPage &lt; pages - 1)
	    {
	        buttons.push('ellipsis');
	    }
	    buttons.push(pages - 1);
	    return buttons;
	};
	
	var table = $('#'+dynTableId).DataTable( {
	    &quot;processing&quot;: false,
        &quot;serverSide&quot;: true,
        &quot;ajax&quot;: {
        	&quot;async&quot;: true,
        	headers: {
      	        'X-Content-Security-Policy-Nonce': 'xZrLNGLeef26xpNnnF1igA=='
      	      },
            &quot;url&quot;: &quot;/phworkoutput/getWOWorkType.htm&quot;,
            &quot;type&quot;: &quot;POST&quot;,
            &quot;data&quot;: function (d) {
            	 d.groupName = userGroupNameIds;
                 d.paramGroupId = dataCollectionId;
                 d.menuItemId = menuItemId;
                return d;
            },
            &quot;beforeSend&quot;: function() {
            	fnRemoveClassWithSameStyle(&quot;#wtDiv&quot;, 'ph-wo-height', '');
            	$('#wtDiv').removeClass('ph-wo-display-none');
            	fnRemoveClassWithSameStyle(&quot;#loaderDiv&quot;, 'ph-wo-height', '');
            	$('#loaderDiv').addClass('ph-wo-display-none');
            },
            &quot;complete&quot;: function() {
            	fnRemoveClassWithSameStyle(&quot;#wtDiv&quot;, 'ph-wo-height', '');
            	$('#wtDiv').removeClass('ph-wo-display-none');
            	fnRemoveClassWithSameStyle(&quot;#loaderDiv&quot;, 'ph-wo-height', '');
            	$('#loaderDiv').addClass('ph-wo-display-none');
            	table.columns.adjust();
            	var totalRows = $('#'+dynTableId +&quot; tbody tr&quot;).length;
            	// Remove the existing scroll and add it based on a condition
            	if ($('.customScrollWT').parent().hasClass('slimScrollDiv')) 
				{
            	    $('.customScrollWT').slimScroll({ destroy: true });
            	    $('.customScrollWT').removeAttr('style');
            	    $('.customScrollWT').siblings('.slimScrollBar, .slimScrollRail').remove();
            	}
            	if ( totalRows > 6 || $('#wtDiv').height() > 330)
            	{
            		initSlimScroll();
            	}
            	setSelectedValue();
            	fnCheckSelectAllParameter();
            }
        },
        &quot;pageLength&quot;: pageLength,
        &quot;scrollX&quot;: true,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;ordering&quot; : true,
        &quot;lengthMenu&quot;: lengthMenu,
        initComplete : function() {
        	var self = this.api();
			var $filter = $('#CommonDataTableWorkTypeId_filter');
			var $input = $filter.find('input').unbind();
            var $searchButton = $('&lt;span title=&quot;Search&quot; class=&quot;custom-datatable-btn&quot;>&lt;i class=&quot;fa fa-search&quot;/>&lt;/span>')
                .click(function(event) {
                	event.preventDefault();
                    self.search($input.val()).draw();
                });
            var $clearButton = $('&lt;span title=&quot;Clear&quot; class=&quot;custom-datatable-btn&quot;>&lt;i class=&quot;fa fa-times&quot;/>&lt;/span>')
                .click(function(event) {
                	event.preventDefault();
                    self.search('').draw();
                });
            $filter.append($searchButton, $clearButton);
        },
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class='ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px'> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class='ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100' >[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       '&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>',
            	&quot;previous&quot;:   '&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>'
            },
            renderer: 'customPager',
        },
        columns: columnArr.map((column, index) => {
            if (index === 0) {
                column.orderable = false;
            }
            return column;
        }),
        order: [[1, 'asc']],
        dom: 'Blfrtip',
        buttons: [
        	 ]
    } );
}

function buildDataTableStructureForDynamicTable(jsonData, pageLength, columnArr, sortingIndex, dynTableId)
{
	$.fn.DataTable.ext.pager.numbers_length = 5;
	$.fn.DataTable.ext.pager.customPager = function (page, pages) {
	    const buttons = [];
	    const startPage = Math.max(page - 1, 1);
	    const endPage = Math.min(startPage + 1, pages - 2);

	    buttons.push(0);
	    if (startPage > 2)
	    {
	        buttons.push('ellipsis');
	    }

	    for (let i = startPage; i &lt;= endPage; i++)
	    {
	        buttons.push(i);
	    }

	    if (endPage &lt; pages - 1)
	    {
	        buttons.push('ellipsis');
	    }
	    buttons.push(pages - 1);
	    return buttons;
	};
	
	var table = $('#'+dynTableId).DataTable( {
		&quot;data&quot;: 	jsonData,
        &quot;deferRender&quot;:    true,
	    &quot;processing&quot;: false,
        &quot;pageLength&quot;: pageLength,
        &quot;scrollX&quot;: true,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;ordering&quot; : true,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class='ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px'> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class='ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100'>[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       '&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>',
                &quot;previous&quot;:   '&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>'
            },
            renderer: 'customPager',
        },
        &quot;drawCallback&quot;: function() {
        	var totalRows = $('#'+dynTableId +&quot; tbody tr&quot;).length;
        	if ( totalRows > 5 )
        	{
        		initSlimScroll();
        	}
        },
        columns: columnArr.map((column, index) => {
            if (index === 0) {
                column.orderable = false;
            }
            return column;
        }),
        order: [[1, 'asc']],
        dom: 'Blfrtip',
        buttons: []
        
    });
}

function buildDataTableStructureWithReportDataForWOWorkType(columns, jsonObjArr, menuItemId, persmissionObject)
{
	fnUpdateProperties(jsonObjArr);
	
	const dataKeys = columns.map(column => column.data);
	const dataKeysString = dataKeys.join(&quot;, &quot;);

	var isReqSorting = true;
	if(dynSortingIndex &lt; 0)
	{
		isReqSorting  = false;
		dynSortingIndex = 0;
	}
	var exportExcel = '';
	var exportPdf = '';
	var emailSpan = '';
	var excelIcon = '';
	var pdfIcon = '';
	
	
	excelIcon = '&lt;img src=&quot;/phworkoutput/images/excel.png&quot; title=&quot;Export to Excel&quot; data-onclick=&quot;fnDataTableExportexpdf&quot; data-etype=&quot;excel&quot; align=&quot;middle&quot; class=&quot;ph-wo-vertical-align-middle ph-wo-cursor-pointer&quot;>';
	
	if(recordSize > 10000)
	{
		pdfIcon = '&lt;img src=&quot;/phworkoutput/images/pdf.png&quot; title=&quot;PDF file cannot be downloaded, as number of records more then 10000&quot; align=&quot;middle&quot; class=&quot;ph-wo-vertical-align-middle ph-wo-cursor-pointer&quot;>';
		exportPdf = '';
	}	
	else
	{
		pdfIcon = '&lt;img src=&quot;/phworkoutput/images/pdf.png&quot; title=&quot;Export to PDF&quot; data-onclick=&quot;fnDataTableExportexpdf&quot; data-etype=&quot;pdf&quot; class=&quot;ph-wo-vertical-align-middle ph-wo-cursor-pointer&quot;>';
	}
	
	var excelReportType = $(&quot;#excelReportType&quot;).val();
	excelReportType = &quot;.&quot; + excelReportType.toLowerCase();
	
	titleHeaderName = replaceBackXMLEntities(dynTitleHeaderName);
	fileName = replaceBackXMLEntities(dynFileName);
	var counter = 1;
	var disclaimerText = $(&quot;#disclaimerText&quot;).val();
	var table = $('#CommonDataTableId').DataTable( {
	    &quot;processing&quot;: false,
        &quot;serverSide&quot;: true,
        &quot;ajax&quot;: {
        	&quot;async&quot;: true,
        	
            &quot;url&quot;: &quot;/phworkoutput/WOWorkTypeDetailTableAjax.htm&quot;,
            &quot;type&quot;: &quot;POST&quot;,
            headers	: {'X-Content-Security-Policy-Nonce': 'xZrLNGLeef26xpNnnF1igA=='},
             &quot;data&quot;: function (d) 
             {
            	 d.menuItemId = menuItemId;
                 d.persmissionObject = persmissionObject;
            	 d.selectedColumns = dataKeysString;
                return d;
            }
        },
        &quot;pageLength&quot;: dynPageLength,
        &quot;scrollX&quot;: true,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;ordering&quot; : isReqSorting,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class='ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px'> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class='ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100' >[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       '&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>',
            	&quot;previous&quot;:   '&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>'
            },
        },
         columns: columns,
         columnDefs: [{
     	    &quot;defaultContent&quot;: &quot;&quot;,
     	    &quot;targets&quot;: &quot;_all&quot;
     	  }],
         &quot;order&quot;: [[ dynSortingIndex, sortingOrder ]],
         dom: 'Blfrtip',
         buttons: [
        	 {
        		 text:   '&lt;label class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>|&amp;nbsp;&amp;nbsp; Export:&amp;nbsp;&lt;/label>',
        		 autoClose: 'false',
                 tag: 'span'
        	 },
        	 {
        		 text:   emailSpan,
        		 autoClose: 'false',
                 tag: 'span'
        	 },
        	 {
                 extend: exportExcel,
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: 'export',
                 	 stripHtml: true
                 },
                 filename: fileName,
                 text:   excelIcon,
                 autoClose: 'true',
                 tag: 'span',
                 extension: excelReportType,
                 createEmptyCells: true,
                 title : titleHeaderName,
                 messageBottom: function () {
                     return disclaimerText;
                 	},
                     customize: function ( xlsx ) {
						var sheet = xlsx.xl.worksheets['sheet1.xml'];
						$('row c', sheet).attr( 's', '25' );
                        $('c[r=A1] t', sheet).text( titleHeaderName );
                        $('row:first c', sheet).attr( 's', '27' ); 
                        $('row:eq(1) c', sheet).attr('s', '27');
                        
                        if(disclaimerText != null &amp;&amp; disclaimerText != '')
                        {
                        	$('row:last c', sheet).attr( 's', '55' );
	                        $('row:last', sheet).attr('customHeight', 1);
	                        $('row:last', sheet).attr('ht', '150');
                        }
                        
                      },
                 button: ['']
             },
             {
                 extend: exportPdf,
                 orientation: 'landscape',
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: 'export',
                 	 stripHtml: true
                 },
                 text:    pdfIcon ,
                 filename: fileName,
                 autoClose: 'true',
                 footer:'true',
                 messageBottom: function () {
                     return disclaimerText;
                  	},
               	 customize: function ( doc ) {
               		
               	   var cols = [];
               	   cols[0] = {text: '© JaMocha Tech Pvt. Ltd. 2009-2026', alignment: 'center', fontSize:'10', margin:[0, 0, 0, 0] };
               	   var objFooter = {};
               	   objFooter['columns'] = cols;
               	   doc['footer']=objFooter;
               	   doc.styles.tableHeader.alignment=&quot;left&quot;;
               	 	
               	   doc['header']=(function(page, pages) {
							return {
								columns: [
									{
										alignment: 'left',
										text: 'ProHance',
										fontSize: 12,
									},
									{
										alignment: 'right',
										fontSize: 8,
										text: ['Page ', { text: page.toString() }]
									}
								],
								margin: 20
							}
						});
               	   
	               	if(dynNumericAlignColArr.length > 0)
	               	{
	               		var rowCount = doc.content[1].table.body.length;
		               	for (i = 0; i &lt; rowCount; i++) 
		               	{
		               		dynNumericAlignColArr.forEach(function(number) {
		                   		doc.content[1].table.body[i][number].alignment = 'right';
		                   	  });
		               	}
               		}
	               	
	               	doc.content[1].table.widths = dynWidthArr;
	                doc.content.forEach(function(item) {
	                       if (item.table) {
	                           item.layout = {
	                               hLineColor: function(i, node) { return '#000000'; }, // Set the border color here
	                               vLineColor: function(i, node) { return '#000000'; }, // Set the border color here
	                           };
	                       }
	                   });
	                doc.styles.tableHeader = {
	            		   fontSize: 11,
	                       bold: true,
	                       color: '#ffffff',  // Set font color
	                       fillColor: '#006bb7'
		               };
               	 },
                 tag: 'span',
                 pageSize : 'LEGAL',
                 titleAttr : 'PDF',
                 title : titleHeaderName,
                 button: [ 'pdf' ]
             }
         ]
    } );
	setTimeout(function() {
		table.columns.adjust().draw();
	}, 20); 
}

function buildDataTableStructureForDashboardWorkType(pageLength, columnArr, sortingIndex, dynTableId, dataParam, urlStr)
{
	if ($.fn.dataTable.isDataTable('#'+dynTableId))
	{
		var existingTable = $('#'+dynTableId);
		existingTable.DataTable().clear().destroy();
		existingTable.off().empty();
	}
	$.fn.DataTable.ext.pager.numbers_length = 5;
	$.fn.DataTable.ext.pager.customPager = function (page, pages) {
	    const buttons = [];
	    const startPage = Math.max(page - 1, 1);
	    const endPage = Math.min(startPage + 1, pages - 2);

	    buttons.push(0);
	    if (startPage > 2)
	    {
	        buttons.push('ellipsis');
	    }

	    for (let i = startPage; i &lt;= endPage; i++)
	    {
	        buttons.push(i);
	    }

	    if (endPage &lt; pages - 1)
	    {
	        buttons.push('ellipsis');
	    }
	    buttons.push(pages - 1);
	    return buttons;
	};
	
	var table = $('#'+dynTableId).DataTable( {
	    &quot;processing&quot;: false,
        &quot;serverSide&quot;: true,
        &quot;ajax&quot;: {
        	&quot;async&quot;: true,
            &quot;url&quot;: urlStr,
            &quot;type&quot;: &quot;POST&quot;,
            headers	: {'X-Content-Security-Policy-Nonce': 'xZrLNGLeef26xpNnnF1igA=='},
            &quot;data&quot;: dataParam,
            &quot;beforeSend&quot;: function() {
            	fnRemoveClassWithSameStyle(&quot;.content-loading&quot;, 'ph-wo-display', '');
            	$(&quot;.content-loading&quot;).removeClass(&quot;ph-wo-display-none&quot;);
            	fnRemoveClassWithSameStyle(&quot;#specificWorktypeContent&quot;, 'ph-wo-display', '');
				$(&quot;#specificWorktypeContent&quot;).addClass(&quot;ph-wo-display-none&quot;);
            },
            &quot;complete&quot;: function() {
            	setSelectedValue();
            	fnRemoveClassWithSameStyle(&quot;.content-loading&quot;, 'ph-wo-display', '');
            	$(&quot;.content-loading&quot;).addClass(&quot;ph-wo-display-none&quot;);
            	fnRemoveClassWithSameStyle(&quot;#specificWorktypeContent&quot;, 'ph-wo-display', '');
				$(&quot;#specificWorktypeContent&quot;).removeClass(&quot;ph-wo-display-none&quot;);
			  	table.columns.adjust();
			  	initSlimScroll();
            }
        },
        initComplete : function() {
        	$('.dataTables_filter').removeClass(&quot;category-search&quot;);
		  	$('.dataTables_filter').addClass(&quot;workType-search&quot;);
        	var self = this.api();
            var $filter = $('.workType-search');
            var $input = $filter.find('input').unbind();
            var $searchButton = $('&lt;span title=&quot;Search&quot; class=&quot;custom-datatable-btn&quot;>&lt;i class=&quot;fa fa-search&quot;/>&lt;/span>')
                .click(function(event) {
                	event.preventDefault();
                	if($input.val().trim() != null &amp;&amp; $input.val().trim() != &quot;&quot;)
                	{
                    	self.search($input.val()).draw();
                    	initSlimScroll();
                	}
                });
            var $clearButton = $('&lt;span title=&quot;Clear&quot; class=&quot;custom-datatable-btn&quot;>&lt;i class=&quot;fa fa-times&quot;/>&lt;/span>')
                .click(function(event) {
                	event.preventDefault();
                    self.search('').draw();
                });
            if (!$filter.has($searchButton).length) {
            	  $filter.append($searchButton);
            	}

            	if (!$filter.has($clearButton).length) {
            	  $filter.append($clearButton);
            	}
        },
        &quot;pageLength&quot;: pageLength,
        &quot;scrollX&quot;: true,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;scrollCollapse&quot;: true,
        &quot;ordering&quot; : true,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class='ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px'> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class='ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100' >[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       '&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>',
            	&quot;previous&quot;:   '&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>'
            },
            renderer: 'customPager',
        },
		columns: columnArr.map((column, index) => {
			column.orderable = index !== 0;
            return column;
        }),
        order: [[1, 'asc']],
        dom: 'Blfrtip',
        buttons: [],
    });
}

function buildDataTableStructureForWorkTypeMapping(columns, jsonObjArr, menuItemId, persmissionObject)
{
	fnUpdateProperties(jsonObjArr);
	
	const dataKeys = columns.map(column => column.data);
	const dataKeysString = dataKeys.join(&quot;, &quot;);

	var isReqSorting = true;
	if(dynSortingIndex &lt; 0)
	{
		isReqSorting  = false;
		dynSortingIndex = 0;
	}
	titleHeaderName = replaceBackXMLEntities(dynTitleHeaderName);
	fileName = replaceBackXMLEntities(dynFileName);
	var counter = 1;
	var disclaimerText = $(&quot;#disclaimerText&quot;).val();
	var table = $('#CommonDataTableId').DataTable( {
	    &quot;processing&quot;: false,
        &quot;serverSide&quot;: true,
        &quot;ajax&quot;: {
        	&quot;async&quot;: true,
        	
            &quot;url&quot;: &quot;/phworkoutput/WorkTypeMappingDetailTableAjax.htm&quot;,
            &quot;type&quot;: &quot;POST&quot;,
            headers	: {'X-Content-Security-Policy-Nonce': 'xZrLNGLeef26xpNnnF1igA=='},
             &quot;data&quot;: function (d) 
             {
            	 d.menuItemId = menuItemId;
                 d.persmissionObject = persmissionObject;
            	 d.selectedColumns = dataKeysString;
                return d;
            }
        },
        &quot;pageLength&quot;: dynPageLength,
        &quot;scrollX&quot;: true,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;ordering&quot; : isReqSorting,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class='ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px'> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class='ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100' >[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       '&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>',
            	&quot;previous&quot;:   '&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>'
            },
        },
         columns: columns,
         columnDefs: [{
     	    &quot;defaultContent&quot;: &quot;&quot;,
     	    &quot;targets&quot;: &quot;_all&quot;
     	  }],
         &quot;order&quot;: [[ dynSortingIndex, sortingOrder ]],
         dom: 'Blfrtip',
         buttons: [
        	 ],
    });
	setTimeout(function() {
		table.columns.adjust().draw();
	}, 20); 
}

function buildDataTableStructureForWOCategories(pageLength, columnArr, sortingIndex, dynTableId, dataParam)
{
	$.fn.DataTable.ext.pager.numbers_length = 5;
	$.fn.DataTable.ext.pager.customPager = function (page, pages) {
	    const buttons = [];
	    const startPage = Math.max(page - 1, 1);
	    const endPage = Math.min(startPage + 1, pages - 2);

	    buttons.push(0);
	    if (startPage > 2)
	    {
	        buttons.push('ellipsis');
	    }

	    for (let i = startPage; i &lt;= endPage; i++)
	    {
	        buttons.push(i);
	    }

	    if (endPage &lt; pages - 1)
	    {
	        buttons.push('ellipsis');
	    }
	    buttons.push(pages - 1);
	    return buttons;
	};
	
	var table = $('#'+dynTableId).DataTable( {
	    &quot;processing&quot;: false,
        &quot;serverSide&quot;: true,
        &quot;ajax&quot;: {
        	&quot;async&quot;: true,
            &quot;url&quot;: &quot;/phworkoutput/getWOCategories.htm&quot;,
            &quot;type&quot;: &quot;POST&quot;,
            headers	: {'X-Content-Security-Policy-Nonce': 'xZrLNGLeef26xpNnnF1igA=='},
            &quot;data&quot;: dataParam,
            &quot;beforeSend&quot;: function() {
            	fnRemoveClassWithSameStyle(&quot;.content-loading&quot;, 'ph-wo-display', '');
            	$(&quot;.content-loading&quot;).removeClass(&quot;ph-wo-display-none&quot;);
            	$(&quot;#specificCategoryContent&quot;).removeClass(&quot;ph-wo-display-none&quot;);
            	fnRemoveClassWithSameStyle(&quot;.specificCategoryContent&quot;, 'ph-wo-display', '');
            	$(&quot;.specificCategoryContent&quot;).addClass(&quot;ph-wo-display-none&quot;);
				$(&quot;#specificCategoryContent&quot;).removeClass(&quot;ph-wo-display-none&quot;);
				fnRemoveClassWithSameStyle(&quot;.specificCategoryContent&quot;, 'ph-wo-display', '');
				$(&quot;#specificCategoryContent&quot;).addClass(&quot;ph-wo-display-none&quot;);
            },
            &quot;complete&quot;: function() {
            	setSelectedCategoryValue();
            	fnRemoveClassWithSameStyle(&quot;.content-loading&quot;, 'ph-wo-display', '');
            	$(&quot;.content-loading&quot;).addClass(&quot;ph-wo-display-none&quot;);
            	fnRemoveClassWithSameStyle(&quot;#specificCategoryContent&quot;, 'ph-wo-display', '');
				$(&quot;#specificCategoryContent&quot;).removeClass(&quot;ph-wo-display-none&quot;);
				fnRemoveClassWithSameStyle(&quot;.specificCategoryContent&quot;, 'ph-wo-display', '');
			  	table.columns.adjust();
			  	initCategorySlimScroll();
            }
        },
        initComplete : function() {
        	$('.dataTables_filter').removeClass(&quot;workType-search&quot;);
		  	$('.dataTables_filter').addClass(&quot;category-search&quot;);
        	var self = this.api();
            var $filter = $('.category-search');
            var $input = $filter.find('input').unbind();
            var $searchButton = $('&lt;span title=&quot;Search&quot; class=&quot;custom-datatable-btn&quot;>&lt;i class=&quot;fa fa-search category&quot;/>&lt;/span>')
                .click(function(event) {
                	event.preventDefault();
                	if($input.val().trim() != null &amp;&amp; $input.val().trim() != &quot;&quot;)
                	{
                    	self.search($input.val()).draw();
                	}
                });
            var $clearButton = $('&lt;span title=&quot;Clear&quot; class=&quot;custom-datatable-btn&quot;>&lt;i class=&quot;fa fa-times category&quot;/>&lt;/span>')
                .click(function(event) {
                	event.preventDefault();
                    self.search('').draw();
                });
            if (!$filter.has($searchButton).length) 
            {
            	  $filter.append($searchButton);
            }

            if (!$filter.has($clearButton).length) 
            {
            	  $filter.append($clearButton);
            }
        },
        &quot;pageLength&quot;: pageLength,
        &quot;scrollX&quot;: true,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;scrollCollapse&quot;: true,
        &quot;ordering&quot; : true,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class='ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px'> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class='ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100' >[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       '&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>',
            	&quot;previous&quot;:   '&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>'
            },
            renderer: 'customPager',
        },
		columns: columnArr.map((column, index) => {
			column.orderable = index !== 0;
            return column;
        }),
        order: [[1, 'asc']],
        dom: 'Blfrtip',
        buttons: [],
    });
}

/* Build Data Tabel with Default Data Table Export along with JSONArray param*/
function buildDataTableStructureForWOCategoryArr(columns, jsonObjArr, menuItemId, persmissionObject)
{
	
	fnUpdateProperties(jsonObjArr);
	const dataKeys = columns.map(column => column.data);
	const dataKeysString = dataKeys.join(&quot;, &quot;);
	var isReqSorting = true;
	if(dynSortingIndex &lt; 0)
	{
		isReqSorting  = false;
		dynSortingIndex = 0;
	}
	
	var exportExcel = '';
	var exportPdf = '';
	var emailSpan = '';
	var excelIcon = '';
	var pdfIcon = '';
	
	excelIcon = '&lt;img src=&quot;/phworkoutput/images/excel.png&quot; title=&quot;Export to Excel&quot; data-onclick=&quot;fnDataTableExportexpdf&quot; data-etype = &quot;excel&quot; class=&quot;ph-wo-vertical-align-middle ph-dash-cursor-pointer&quot;>';
	if(recordSize > 10000)
	{
		pdfIcon = '&lt;img src=&quot;/phworkoutput/images/pdf.png&quot; title=&quot;PDF file cannot be downloaded, as number of records more then 10000&quot; class=&quot;ph-wo-vertical-align-middle ph-dash-cursor-pointer&quot;>';
		exportPdf = '';
	}	
	else
	{
		pdfIcon = '&lt;img src=&quot;/phworkoutput/images/pdf.png&quot; title=&quot;Export to PDF&quot; data-onclick=&quot;fnDataTableExportexpdf&quot; data-etype=&quot;pdf&quot; class=&quot;ph-wo-vertical-align-middle ph-dash-cursor-pointer&quot;>';
	}
	
	var excelReportType = $(&quot;#excelReportType&quot;).val();
	excelReportType = &quot;.&quot; + excelReportType.toLowerCase();
	var disclaimerText = $(&quot;#disclaimerText&quot;).val();
	titleHeaderName = replaceBackXMLEntities(dynTitleHeaderName);
	fileName = replaceBackXMLEntities(dynFileName);
	var table = $('#CommonDataTableId').DataTable( {
        deferRender:    true,
        &quot;processing&quot;: false,
        &quot;serverSide&quot;: true,
        &quot;ajax&quot;: {
        	&quot;async&quot;: true,
        	
            &quot;url&quot;: &quot;/phworkoutput/WOCategoryDetailTableAjax.htm&quot;,
            &quot;type&quot;: &quot;POST&quot;,
            headers	: {'X-Content-Security-Policy-Nonce': 'xZrLNGLeef26xpNnnF1igA=='},
             &quot;data&quot;: function (d) 
             {
            	 d.menuItemId = menuItemId;
                 d.persmissionObject = persmissionObject;
            	 d.selectedColumns = dataKeysString;
                return d;
            }
        },
        &quot;pageLength&quot;: dynPageLength,
        &quot;bDestroy&quot;: true,
        &quot;scrollX&quot;: true,
        &quot;ordering&quot; : isReqSorting,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class='ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px'> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class='ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100' >[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       '&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>',
            	&quot;previous&quot;:   '&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>'
            },
        },
         columns: columns,
         &quot;order&quot;: [[ dynSortingIndex, 'asc' ]],
         rowGroup: {
             dataSrc: 88
         },
         dom: 'Blfrtip',
         buttons: [
        	 {
        		 text:   '&lt;label class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>|&amp;nbsp;&amp;nbsp; Export:&amp;nbsp;&lt;/label>',
        		 autoClose: 'false',
                 tag: 'span'
        	 },
             {
                 extend: exportExcel,
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: 'export',
                 	 stripHtml: true
                 },
                 destroy:true,
                 filename: fileName,
                 text:  excelIcon,
                 autoClose: 'true',
                 tag: 'span',
                 extension: excelReportType,
                 createEmptyCells: true,
                 title : titleHeaderName,
                 messageBottom: function () {
                     return disclaimerText;
                 	},
                     customize: function ( xlsx ) {
						 var sheet = xlsx.xl.worksheets['sheet1.xml'];
						$('row c', sheet).attr( 's', '25' );
                        $('c[r=A1] t', sheet).text( titleHeaderName );
                        $('row:first c', sheet).attr( 's', '27' ); 
                        $('row:eq(1) c', sheet).attr('s', '27');
                        
                        if(disclaimerText != null &amp;&amp; disclaimerText != '')
                        {
                        	$('row:last c', sheet).attr( 's', '55' );
	                        $('row:last', sheet).attr('customHeight', 1);
	                        $('row:last', sheet).attr('ht', '150');
                        }
                         
                      },
                      
                 button: ['excel']

             },
             {
                 extend: exportPdf,
                 orientation: 'landscape',
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: 'export',
                 	 stripHtml: true
                 },
                 text: pdfIcon,
                 filename: fileName,
                 autoClose: 'true',
                 footer:'true',
                 destroy:true,
                 messageBottom: function () {
                     return disclaimerText;
                  	},
               	 customize: function ( doc ) {
               		
               	   var cols = [];
               	   cols[0] = {text: '© JaMocha Tech Pvt. Ltd. 2009-2026', alignment: 'center', fontSize:'10', margin:[0, 0, 0, 0] };
               	   var objFooter = {};
               	   objFooter['columns'] = cols;
               	   doc['footer']=objFooter;
               	   doc.styles.tableHeader.fillColor = '#006bb7';
               	   doc.styles.tableHeader.alignment=&quot;left&quot;;
               	 	
               	   doc['header']=(function(page, pages) {
							return {
								columns: [
									{
										alignment: 'left',
										text: 'ProHance',
										fontSize: 12,
									},
									{
										alignment: 'right',
										fontSize: 8,
										text: ['Page ', { text: page.toString() }]
									}
								],
								margin: 20
							}
						});
	               	if(dynNumericAlignColArr.length > 0)
	               	{
	               		var rowCount = doc.content[1].table.body.length;
		               	for (i = 0; i &lt; rowCount; i++) 
		               	{
		               		dynNumericAlignColArr.forEach(function(number) {
		                   		doc.content[1].table.body[i][number].alignment = 'right';
		                   	  });
		               	}
	               	}
               	
	               doc.content[1].table.widths=	dynWidthArr;
               	
               	 },
                 tag: 'span',
                 pageSize : 'LEGAL',
                 titleAttr : 'PDF',
                 title : titleHeaderName,
                 button: [ 'pdf' ]
             }
         ],
         drawCallback: function() {
       	  var hasRows = this.api().rows({ filter: 'applied' }).data().length > 0;
       	 $('.buttons-excel')[0].style.visibility = hasRows ? 'visible' : 'hidden';
       	 $('.buttons-pdf')[0].style.visibility = hasRows ? 'visible' : 'hidden';
       	}
    } );
	
	setTimeout(function() {
		table.columns.adjust().draw();
	}, 20); 	
}


function buildScrollXDataTableStructure(jsonData, pageLength, columns, sortingIndex, isScrollX)
{
	var isReqSorting = true;
	if(sortingIndex &lt; 0)
	{
		isReqSorting  = false;
		sortingIndex = 0;
	}
	var table = $('#CommonDataTableId').DataTable( {
        data: jsonData,
        deferRender:    true,
        &quot;pageLength&quot;: pageLength,
        &quot;ordering&quot; : isReqSorting,
        &quot;autoWidth&quot;:false,
        &quot;scrollX&quot;: false,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class='ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px'> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class='ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100'>[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       '&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>',
                &quot;previous&quot;:   '&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>'
            },
        },
         columns: columns,
         &quot;order&quot;: [[ sortingIndex, 'asc' ]],
         dom: 'Blfrtip',
         buttons: []
    } );
}


function buildDataTableStructureForWorkType(jsonData, pageLength, columns, sortingIndex, isScrollX)
{
	$.fn.DataTable.ext.pager.numbers_length = 5;
	$.fn.DataTable.ext.pager.customPager = function (page, pages) {
	    const buttons = [];
	    const startPage = Math.max(page - 1, 1);
	    const endPage = Math.min(startPage + 1, pages - 2);
	    buttons.push(0);
	    if (startPage > 2) buttons.push('ellipsis');
	    for (let i = startPage; i &lt;= endPage; i++) buttons.push(i);
	    if (endPage &lt; pages - 1) buttons.push('ellipsis');
	    buttons.push(pages - 1);
	    return buttons;
	};
	var table = $('#CommonDataTableWorkTypeId').DataTable( {
	    data: jsonData,
	    deferRender: true,
	    &quot;processing&quot;: false,
	    &quot;bDestroy&quot;: true,
	    &quot;pageLength&quot;: pageLength,
	    &quot;scrollX&quot;: true,
	    &quot;sScrollXInner&quot;: &quot;100%&quot;,
	    &quot;scrollCollapse&quot;: true,
	    &quot;ordering&quot;: true,
	    &quot;lengthMenu&quot;: lengthMenu,
	    &quot;language&quot;: {
	        &quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
	        &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class='ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px'> Displaying _START_ to _END_ &lt;label>&quot;,
	        &quot;infoEmpty&quot;: &quot;No records to display&quot;,
	        &quot;infoFiltered&quot;: &quot;&lt;label class='ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100' >[Filtered from _MAX_ records] &lt;/label>&quot;,
	        &quot;search&quot;: &quot;Search&quot;,
	        &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
	        &quot;paginate&quot;: {
	            &quot;next&quot;: '&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>',
	            &quot;previous&quot;: '&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>'
	        },
	        renderer: 'customPager',
	    },
	    columns: columns.map((column, index) => {
	        column.orderable = index !== 0;
	        return column;
	    }),
	    order: [[1, 'asc']],
	    dom: 'Blfrtip',
	    buttons: [],
	} );
}

function buildDataTableStructureForWTAttributeOptions(pageLength, columnArr, sortingIndex, dynTableId, dataParam)
{
	$.fn.DataTable.ext.pager.numbers_length = 5;
	$.fn.DataTable.ext.pager.customPager = function (page, pages) {
	    const buttons = [];
	    const startPage = Math.max(page - 1, 1);
	    const endPage = Math.min(startPage + 1, pages - 2);

	    buttons.push(0);
	    if (startPage > 2)
	    {
	        buttons.push('ellipsis');
	    }

	    for (let i = startPage; i &lt;= endPage; i++)
	    {
	        buttons.push(i);
	    }

	    if (endPage &lt; pages - 1)
	    {
	        buttons.push('ellipsis');
	    }
	    buttons.push(pages - 1);
	    return buttons;
	};
	
	var table = $('#'+dynTableId).DataTable( {
	    &quot;processing&quot;: false,
        &quot;serverSide&quot;: true,
        &quot;ajax&quot;: {
        	&quot;async&quot;: true,
            &quot;url&quot;: &quot;/phworkoutput/getSpecificWorktypeAttrOptions.htm&quot;,
            &quot;type&quot;: &quot;POST&quot;,
            headers	: {'X-Content-Security-Policy-Nonce': 'xZrLNGLeef26xpNnnF1igA=='},
            &quot;data&quot;: dataParam,
            &quot;dataSrc&quot;: function(json) {
            	parameterJson = json.data;
                return json.data;
            },
            &quot;beforeSend&quot;: function() {
            	$(&quot;#loading_attr_options&quot;).removeClass(&quot;ph-wo-display-none&quot;);
				fnRemoveClassWithSameStyle(&quot;.content-loading&quot;, 'ph-wo-display', '');
            	$(&quot;.content-loading&quot;).removeClass(&quot;ph-wo-display-none&quot;);
            	$(&quot;#specificAttrOptionContent&quot;).removeClass(&quot;ph-wo-display-none&quot;);
            	fnRemoveClassWithSameStyle(&quot;.specificAttrOptionContent&quot;, 'ph-wo-display', '');
            	$(&quot;.specificAttrOptionContent&quot;).addClass(&quot;ph-wo-display-none&quot;);
				$(&quot;#specificAttrOptionContent&quot;).removeClass(&quot;ph-wo-display-none&quot;);
				fnRemoveClassWithSameStyle(&quot;.specificAttrOptionContent&quot;, 'ph-wo-display', '');
				$(&quot;#specificAttrOptionContent&quot;).addClass(&quot;ph-wo-display-none&quot;);
            },
            &quot;complete&quot;: function() {
            	$(&quot;#loading_attr_options&quot;).addClass(&quot;ph-wo-display-none&quot;);
            	fnRemoveClassWithSameStyle(&quot;.content-loading&quot;, 'ph-wo-display', '');
            	$(&quot;.content-loading&quot;).addClass(&quot;ph-wo-display-none&quot;);
            	fnRemoveClassWithSameStyle(&quot;#specificAttrOptionContent&quot;, 'ph-wo-display', '');
				$(&quot;#specificAttrOptionContent&quot;).removeClass(&quot;ph-wo-display-none&quot;);
				fnRemoveClassWithSameStyle(&quot;.specificAttrOptionContent&quot;, 'ph-wo-display', '');
			  	table.columns.adjust();
            }
        },
        initComplete : function() {
		  	$('.dataTables_filter').addClass(&quot;attrOption-search&quot;);
        	var self = this.api();
            var $filter = $('.attrOption-search');
            var $input = $filter.find('input').unbind();
            var $searchButton = $('&lt;span title=&quot;Search&quot; class=&quot;custom-datatable-btn&quot;>&lt;i class=&quot;fa fa-search attrOption&quot;/>&lt;/span>')
                .click(function(event) {
                	event.preventDefault();
                	if($input.val().trim() != null &amp;&amp; $input.val().trim() != &quot;&quot;)
                	{
                		$(&quot;#loading_attr_options&quot;).removeClass(&quot;ph-wo-display-none&quot;);
                    	self.search($input.val()).draw();
                	}
                });
            var $clearButton = $('&lt;span title=&quot;Clear&quot; class=&quot;custom-datatable-btn&quot;>&lt;i class=&quot;fa fa-times attrOption&quot;/>&lt;/span>')
                .click(function(event) {
                	event.preventDefault();
                	$(&quot;#loading_attr_options&quot;).removeClass(&quot;ph-wo-display-none&quot;);
                	self.search('').draw();
                });
            if (!$filter.has($searchButton).length) 
            {
            	  $filter.append($searchButton);
            }

            if (!$filter.has($clearButton).length) 
            {
            	  $filter.append($clearButton);
            }
        },
        &quot;pageLength&quot;: pageLength,
        &quot;scrollX&quot;: true,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;scrollCollapse&quot;: true,
        &quot;ordering&quot; : true,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class='ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px'> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class='ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100' >[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       '&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>',
            	&quot;previous&quot;:   '&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>'
            },
            renderer: 'customPager',
        },
		columns: columnArr.map((column, index) => {
			column.orderable = index !== 0;
            return column;
        }),
        order: [[1, 'asc']],
        dom: 'Blfrtip',
        buttons: [],
    });
}

$(document).off('click', '[data-onclick=&quot;fnDataTableExportexpdf&quot;]').on('click', '[data-onclick=&quot;fnDataTableExportexpdf&quot;]', function() {
    var etype = $(this).data('etype');
    fnDataTableExport(etype);
});
 










 
	

    
			













	
		×
		
	



	
	
	
	


	
		
			
				
					
						
							
							
							
								
							
							
						
						
							
								
										
						
					
					
			
			
			
			
		
	



 $( document ).ready(function() {
	fnFadeoutEvent(&quot;#dialogsuccessspan&quot;, 20000, &quot;ph-wo-display-none&quot;);
	fnFadeoutEvent(&quot;#dialogfailurespan&quot;, 20000, &quot;ph-wo-display-none&quot;);
	fnFadeoutEvent(&quot;#dialogdependencyspan&quot;, 20000, &quot;ph-wo-display-none&quot;);
	
	if ($(&quot;#dialogsuccessspan&quot;).is(':hidden')) 
	{
		fnRemoveClassWithSameStyle(&quot;#dialogsuccessspan&quot;, &quot;ph-wo-height&quot;, &quot;&quot;, false)
	 	$(&quot;#dialogsuccessspan&quot;).addClass(&quot;ph-wo-height-0px&quot;);
	}
	if ($(&quot;#dialogfailurespan&quot;).is(':hidden')) 
	{
		fnRemoveClassWithSameStyle(&quot;#dialogfailurespan&quot;, &quot;ph-wo-height&quot;, &quot;&quot;, false)
	 	$(&quot;#dialogfailurespan&quot;).addClass(&quot;ph-wo-height-0px&quot;);
	}
});

function fnSucessClose()
{
	fnRemoveClassWithSameStyle(&quot;#dialogsuccessspan&quot;, &quot;ph-wo-height&quot;, &quot;&quot;, false)
 	$(&quot;#dialogsuccessspan&quot;).addClass(&quot;ph-wo-height-0px&quot;);
}

function fnFailureClose()
{
	fnRemoveClassWithSameStyle(&quot;#dialogfailurespan&quot;, &quot;ph-wo-height&quot;, &quot;&quot;, false)
 	$(&quot;#dialogfailurespan&quot;).addClass(&quot;ph-wo-height-0px&quot;);
}

function fnShowDependency()
{
	$('.modal').draggable({
	    handle: &quot;.modal-header&quot;
	});
	
	var form;
	if ('WorkTypeForm' == 'null')
	{
		//
	}
	else
	{
		form = document.WorkTypeForm;
	}
	var url = &quot;/phworkoutput/getDataDependency/getDependencyList.htm?deleteRecordId=&quot;+-1;
	windowTitle = fnGetWindowName();
	var winObj =  window.open(&quot;&quot;, windowTitle, &quot;status=1, toolbar=0, scrollbars=1, resizable=1, width=1330, height=600, left=20, top=20&quot;);
	winObj.document.body.appendChild(fnGetDependencyPopUpLoader());
	window.parent.parent.fnAddToChildWindows(winObj);
	tempAction = form.action;
    tempTarget = form.target;
    form.action = url;
    form.target = windowTitle;
    form.submit();
    winObj.focus();
    form.action = tempAction;
    form.target = tempTarget;
}

function fnGetDependencyPopUpLoader()
{
	var loaderDiv = window.parent.document.createElement('div');
    loaderDiv.style.textAlign = 'center';
    loaderDiv.style.width = '100%';
    loaderDiv.style.position = 'absolute';
    loaderDiv.style.top = '48%';
    loaderDiv.setAttribute('nonce', 'xZrLNGLeef26xpNnnF1igA==');
 
    var loaderImg =  window.parent.document.createElement('img');
    loaderImg.src = &quot;/phworkoutput/images/loader-trans.gif&quot;;
    loaderImg.setAttribute('nonce', 'xZrLNGLeef26xpNnnF1igA==');
    loaderDiv.appendChild(loaderImg);
    return loaderDiv;
}

/* function fnDeleteDependency()
{
	var primaryKeyId = '';
	var moduleName = '';
	var menuItemId = '302';
	var formDefinitionId  = '';
	
	var actionName = &quot;/phworkoutput/getDataDependency/deleteDataDependency.htm?primaryKeyId=&quot;+primaryKeyId+&quot;&amp;moduleName=&quot;+moduleName+&quot;&amp;menuItemId=&quot;+menuItemId+&quot;&amp;formDefinitionId=&quot;+formDefinitionId;
	doAjaxCall(actionName, false, false);
} */


$(document).off('click', '[data-onclick=&quot;hideMsg&quot;]').on('click', '[data-onclick=&quot;hideMsg&quot;]', function() {
	
	fnRemoveClassWithSameStyle(&quot;.internalmessagefailure&quot;, &quot;ph-wo-display&quot;, &quot;&quot;, false);
	$(&quot;.internalmessagefailure&quot;).addClass(&quot;ph-wo-display-none&quot;);
	fnRemoveClassWithSameStyle(&quot;#displayErrorMessage&quot;, &quot;ph-wo-display&quot;, &quot;&quot;, false);
	$(&quot;#displayErrorMessage&quot;).addClass(&quot;ph-wo-display-none&quot;);
});

$(document).off('click', '[data-onclick=&quot;fnSucessClose&quot;]').on('click', '[data-onclick=&quot;fnSucessClose&quot;]', function() {
	fnSucessClose();
});

$(document).off('click', '[data-onclick=&quot;fnFailureClose&quot;]').on('click', '[data-onclick=&quot;fnFailureClose&quot;]', function() {
	fnFailureClose();
});

$(document).off('click', '[data-onclick=&quot;fnShowDependency&quot;]').on('click', '[data-onclick=&quot;fnShowDependency&quot;]', function() {
	fnShowDependency();
});


	 



	 
        
			
				 
			        
						
						
							
								
									
										ADD WORK TYPES
									
								
							
							
								
									
										
											
												Records to display 102050100200Search
											        
											        Work TypeDescription#User GroupsStatus#dipankar_processWO Scoredcard 2.01Active#Process_028WO Scoredcard 2.01Active#Process_0288 clonedWO Scoredcard 2.01Active#Process_05WO Scoredcard 2.01Active#Process_055WO Scoredcard 2.01Active123zc bz b0Active1_WT0Active2_WTWT10ActiveabWT1231Activeact1230Active
												Total Records: 32    Displaying 1 to 10 1234
											
											
											
									
								
							
						
					
				
			
		
	





$(document).off(&quot;click&quot;, &quot;.fnAddWorkTypes&quot;).on(&quot;click&quot;, &quot;.fnAddWorkTypes&quot;, function(){
	fnAddWorkTypes(this.form);
});

$(document).off(&quot;submit&quot;, &quot;#categoryForm&quot;).on(&quot;submit&quot;, &quot;#categoryForm&quot;, function(){
	return false;
});

form = document.CategoryForm;

var userGroupHashedMap = new Array();

function fnAddWorkTypes(form)
{
	
	mappedWorkTypes = '67,68,69,71,73,138,74,139,140,142,143,144,145,146,147,149,90,94,32,98,99,102,104,105,106,107,108,111,117,119,57,123';
	$(&quot;#mappedWorkTypes&quot;).val(mappedWorkTypes);
	windowTitle = fnGetWindowName();
	var url = &quot;/phworkoutput/WOCategoryDetails/popupWorkTypeSummary.htm?id=1&amp;menuItemId=302&quot;;
	console.log(url)
	var winObj =  window.open(&quot;&quot;, windowTitle, &quot;status=1, toolbar=0, scrollbars=1, resizable=1, width=1330, height=600, left=20, top=20&quot;);
	winObj.document.body.appendChild(fnGetPopUpLoader(winObj));
	window.parent.parent.fnAddToChildWindows(winObj);
	tempAction = form.action;
    tempTarget = form.target;
    form.action = url;
    form.target = windowTitle;
    form.submit();
    winObj.focus();
    form.action = tempAction;
    form.target = tempTarget;
}

function fnAddWorkTypeMapping(workTypIds)
{
	$(&quot;#workTypeIdStr&quot;).val(workTypIds);
	
	var url = &quot;/phworkoutput/WOCategoryDetails/addWorkTypeMapping.htm?menuItemId=302&quot;;
	$.ajax({
		url: url,
		headers	: {'X-Content-Security-Policy-Nonce': 'xZrLNGLeef26xpNnnF1igA=='},
		global: false,
		type: &quot;POST&quot;,
		data: ({ 			
				&quot;categoryId&quot; 	: $(&quot;#categoryId&quot;).val(),
				&quot;workTypeIdStr&quot;		: workTypIds
 	          }),
	         success: function(resp)
			 {
	        	var json = JSON.parse(resp);
	        	if(json[&quot;message&quot;] == 'SUCCESS')
		        {
	                $(&quot;#categotyWorkTypesMsg&quot;).val('SUCCESS');
		        }
		        else
		        {
	                $(&quot;#categotyWorkTypesMsg&quot;).val('Failed');
		        }
	        	
	        	 fnShowWorkTypeScreen(true);
			 },
		     error :  function(msg,arg1,arg2)
		     {
		    	 processErrorStatus(msg.status, &quot;/phworkoutput&quot;);
		    	 return false;
			 }
	});
}

if('' == &quot;fromGroupDelete&quot;)
{
	form.mappedUserGroupIds.value = '';
}

$(document).ready(function() 
{
	fnPrepareDataTable();
});

function fnPrepareDataTable()
{
	var Val = '[{\&quot;workTypeName\&quot;:\&quot;Test Cases Written\&quot;,\&quot;worktypeId\&quot;:32,\&quot;userGroupCount\&quot;:6,\&quot;description\&quot;:\&quot;Test Cases\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;WO Bugs Dev Verified\&quot;,\&quot;worktypeId\&quot;:57,\&quot;userGroupCount\&quot;:8,\&quot;description\&quot;:\&quot;Bugs Dev Verified [From Workflow jobs using Database Import]\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;Design\&quot;,\&quot;worktypeId\&quot;:67,\&quot;userGroupCount\&quot;:5,\&quot;description\&quot;:\&quot;WO Scoredcard 2.0\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;Dev. Support\&quot;,\&quot;worktypeId\&quot;:68,\&quot;userGroupCount\&quot;:2,\&quot;description\&quot;:\&quot;WO Scoredcard 2.0\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;Development\&quot;,\&quot;worktypeId\&quot;:69,\&quot;userGroupCount\&quot;:7,\&quot;description\&quot;:\&quot;WO Scoredcard 2.0\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;R&amp;amp;I Developement\&quot;,\&quot;worktypeId\&quot;:71,\&quot;userGroupCount\&quot;:1,\&quot;description\&quot;:\&quot;WO Scoredcard 2.0\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;Testing\&quot;,\&quot;worktypeId\&quot;:73,\&quot;userGroupCount\&quot;:8,\&quot;description\&quot;:\&quot;WO Scoredcard 2.0\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;workflowimport\&quot;,\&quot;worktypeId\&quot;:74,\&quot;userGroupCount\&quot;:15,\&quot;description\&quot;:\&quot;WO Scoredcard 2.0\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;ukS)g\&quot;,\&quot;worktypeId\&quot;:90,\&quot;userGroupCount\&quot;:0,\&quot;description\&quot;:\&quot;\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;x0-tN\&quot;,\&quot;worktypeId\&quot;:94,\&quot;userGroupCount\&quot;:0,\&quot;description\&quot;:\&quot;\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;jO_)d\&quot;,\&quot;worktypeId\&quot;:98,\&quot;userGroupCount\&quot;:0,\&quot;description\&quot;:\&quot;\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;#dipankar_process\&quot;,\&quot;worktypeId\&quot;:99,\&quot;userGroupCount\&quot;:1,\&quot;description\&quot;:\&quot;WO Scoredcard 2.0\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;#Process_05\&quot;,\&quot;worktypeId\&quot;:102,\&quot;userGroupCount\&quot;:1,\&quot;description\&quot;:\&quot;WO Scoredcard 2.0\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;Bugs Resolved\&quot;,\&quot;worktypeId\&quot;:104,\&quot;userGroupCount\&quot;:6,\&quot;description\&quot;:\&quot;WO Scoredcard 2.0\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;#Process_028\&quot;,\&quot;worktypeId\&quot;:105,\&quot;userGroupCount\&quot;:1,\&quot;description\&quot;:\&quot;WO Scoredcard 2.0\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;#Process_0288 cloned\&quot;,\&quot;worktypeId\&quot;:106,\&quot;userGroupCount\&quot;:1,\&quot;description\&quot;:\&quot;WO Scoredcard 2.0\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;Test_Claim Provider insurance\&quot;,\&quot;worktypeId\&quot;:107,\&quot;userGroupCount\&quot;:1,\&quot;description\&quot;:\&quot;WO Scoredcard 2.0\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;rkq3x\&quot;,\&quot;worktypeId\&quot;:108,\&quot;userGroupCount\&quot;:0,\&quot;description\&quot;:\&quot;\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;Fy5_n\&quot;,\&quot;worktypeId\&quot;:111,\&quot;userGroupCount\&quot;:0,\&quot;description\&quot;:\&quot;\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;zc bz b\&quot;,\&quot;worktypeId\&quot;:117,\&quot;userGroupCount\&quot;:1,\&quot;description\&quot;:\&quot;\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;New WT01\&quot;,\&quot;worktypeId\&quot;:119,\&quot;userGroupCount\&quot;:1,\&quot;description\&quot;:\&quot;New WT01\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;zc bz b123\&quot;,\&quot;worktypeId\&quot;:123,\&quot;userGroupCount\&quot;:0,\&quot;description\&quot;:\&quot;\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;Bugs Dev Fixed\&quot;,\&quot;worktypeId\&quot;:138,\&quot;userGroupCount\&quot;:0,\&quot;description\&quot;:\&quot;\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;soumya\&quot;,\&quot;worktypeId\&quot;:139,\&quot;userGroupCount\&quot;:1,\&quot;description\&quot;:\&quot;\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;scc,gvgbmj\&quot;,\&quot;worktypeId\&quot;:140,\&quot;userGroupCount\&quot;:2,\&quot;description\&quot;:\&quot;WO Scoredcard 2.0\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;WT1\&quot;,\&quot;worktypeId\&quot;:142,\&quot;userGroupCount\&quot;:0,\&quot;description\&quot;:\&quot;WT1\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;abWT123\&quot;,\&quot;worktypeId\&quot;:143,\&quot;userGroupCount\&quot;:1,\&quot;description\&quot;:\&quot;\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;act123\&quot;,\&quot;worktypeId\&quot;:144,\&quot;userGroupCount\&quot;:0,\&quot;description\&quot;:\&quot;\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;123zc bz b\&quot;,\&quot;worktypeId\&quot;:145,\&quot;userGroupCount\&quot;:0,\&quot;description\&quot;:\&quot;\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;1_WT\&quot;,\&quot;worktypeId\&quot;:146,\&quot;userGroupCount\&quot;:0,\&quot;description\&quot;:\&quot;\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;2_WT\&quot;,\&quot;worktypeId\&quot;:147,\&quot;userGroupCount\&quot;:0,\&quot;description\&quot;:\&quot;WT1\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;#Process_055\&quot;,\&quot;worktypeId\&quot;:149,\&quot;userGroupCount\&quot;:1,\&quot;description\&quot;:\&quot;WO Scoredcard 2.0\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true}]';
	var json = JSON.parse(Val);
	var columnArr = [];
	
	columnArr.push(
		{
			&quot;title&quot;		: &quot;Work Type&quot;, 
		 	&quot;data&quot;		: &quot;workTypeName&quot;, 
		 	&quot;width&quot; 	: &quot;45%&quot;,
		 	&quot;className&quot;	: &quot;text&quot;,
		 	&quot;render&quot;	: function (workTypeName, type, full, meta) 
		 	{
		 		var actions = '&lt;div class=&quot;ph-wo-word-break-break-all&quot;>'+workTypeName+'&lt;/div>';
				return actions;
			}
	 	});
	
	columnArr.push(
		{
			&quot;title&quot;		: &quot;Description&quot;, 
		 	&quot;data&quot;		: &quot;description&quot;, 
		 	&quot;width&quot; 	: &quot;40%&quot;,
		 	&quot;className&quot;	: &quot;text&quot;,
		 	&quot;render&quot;	: function (description, type, full, meta) 
		 	{
		 		var actions = '&lt;div class=&quot;ph-wo-word-break-break-all&quot;>'+description+'&lt;/div>';
				return actions;
			}
	 	});
	
	columnArr.push({&quot;title&quot;: &quot;#User Groups&quot;, &quot;data&quot;: &quot;userGroupCount&quot;, &quot;width&quot; : &quot;10%&quot;, &quot;class&quot;: &quot;numeric&quot;});
	
	columnArr.push({ 
   	 &quot;title&quot;	 : &quot;Status&quot;, 
     	 &quot;data&quot;		 : &quot;status&quot;,
     	 &quot;searchable&quot;: false,
     	 &quot;sortable&quot;	 : true,
     	&quot;width&quot; 	 : &quot;5%&quot;,
     	 &quot;className&quot; : &quot;text-center&quot;,
     	 &quot;render&quot;	 : function (json, type, full, meta) 
     	 {
          	var actions = '';
          	if(full.status == true)
      		{
      			actions = '&lt;i class=&quot;fa fa-circle circle-active&quot; title=&quot;Active&quot;>&lt;span class=&quot;ph-wo-display-none&quot;>Active&lt;/span>&lt;/i>';
      		}
      		else if(full.status == false)
      		{
      			actions = '&lt;i class=&quot;fa fa-circle circle-inactive&quot; title=&quot;Inactive&quot;>&lt;span class=&quot;ph-wo-display-none&quot;>Inactive&lt;/span>&lt;/i>';
      		}
			return actions;
 		}
    });
	
	
	var pageSize = 10;
	var sortingIndex = 0;
	
	if(json != '')
	{
		buildScrollXDataTableStructure(json, pageSize, columnArr, sortingIndex, false);
	}
	var table = $('#CommonDataTableId').DataTable();
	table.columns.adjust().draw();
}




								
							
						
					
				
				
					
						
							
								PREVIOUS
							
						
						
							CANCEL
						
						
							
								NEXT
							
						
						
							
								EXIT
							
						
					
				
			
		
	


	


$(document).off(&quot;click&quot;, &quot;.fnShowBasicScreen1&quot;).on(&quot;click&quot;, &quot;.fnShowBasicScreen1&quot;, function(){
	fnShowBasicScreen1();
});

$(document).off(&quot;click&quot;, &quot;.fnShowWorkTypeScreen&quot;).on(&quot;click&quot;, &quot;.fnShowWorkTypeScreen&quot;, function(){
	var flag = $(this).data(&quot;flag&quot;);
	fnShowWorkTypeScreen(flag);
});

$(document).off(&quot;click&quot;, &quot;.fnShowPreviousScreen&quot;).on(&quot;click&quot;, &quot;.fnShowPreviousScreen&quot;, function(){
	fnShowPreviousScreen();
});

$(document).off(&quot;click&quot;, &quot;.fnShowNextScreen&quot;).on(&quot;click&quot;, &quot;.fnShowNextScreen&quot;, function(){
	fnShowNextScreen();
});

$(document).off(&quot;click&quot;, &quot;.fnBackToCategorySummary&quot;).on(&quot;click&quot;, &quot;.fnBackToCategorySummary&quot;, function(){
	fnBackToCategorySummary();
});

form = document.CategoryForm;

previousScreenName = &quot;&quot;;
currentScreenName = &quot;&quot;;
nextScreenName =&quot;&quot;;


function fnShowPreviousScreen()
{
	fnShowScreen(previousScreenName)
}

function fnShowNextScreen()
{
	fnShowScreen(nextScreenName)
}

function fnShowScreen(screenName)
{
	if(screenName == &quot;BASIC_SCREEN&quot;)
	{
		fnShowBasicScreen1();
	}
	else if (screenName == &quot;WORKTYPEMAPPING_SCREEN&quot;)
	{
		fnShowWorkTypeScreen(false);
	}
	else
	{
		alert('Unknown screen');
	}
}
function fnHighLightMenu(screenName)
{
	$(&quot;#parameterGroupTD&quot;).addClass('wizardtdnohighlight');
	$(&quot;#basicsTD&quot;).addClass(&quot;wizardtdnohighlight&quot;);
	$(&quot;#heatCodeTD&quot;).addClass(&quot;wizardtdnohighlight&quot;);
	
	if(screenName == &quot;BASIC_SCREEN&quot;)
	{
		fnRemoveClassWithSameStyle(&quot;#ExitButton&quot;,&quot;ph-wo-display&quot;,&quot;&quot;, false);
		$(&quot;#ExitButton&quot;).addClass(&quot;ph-wo-display-none&quot;);
		previousScreenName = &quot;&quot;;
		nextScreenName =&quot;WORKTYPEMAPPING_SCREEN&quot;;
		fnRemoveClassWithSameStyle(&quot;#previousButton&quot;,&quot;ph-wo-display&quot;,&quot;&quot;, false);
		$(&quot;#previousButton&quot;).addClass(&quot;ph-wo-display-none&quot;);
		fnRemoveClassWithSameStyle(&quot;#ExitButton&quot;,&quot;ph-wo-display&quot;,&quot;&quot;, false);
		$(&quot;#ExitButton&quot;).addClass(&quot;ph-wo-display-none&quot;);
		fnRemoveClassWithSameStyle(&quot;#nextButton&quot;,&quot;ph-wo-display&quot;,&quot;&quot;, false);
		$(&quot;#nextButton&quot;).addClass(&quot;ph-wo-display-empty&quot;);
		
		$(&quot;#basicsTD&quot;).removeClass();
		$(&quot;#basicsTD&quot;).addClass(&quot;fnShowBasicScreen1&quot;);
		$(&quot;#parameterGroupTD&quot;).removeClass('wizardtdhighlight');
		$(&quot;#heatCodeTD&quot;).removeClass('wizardtdhighlight');
		$(&quot;#basicsTD&quot;).addClass('wizardtdhighlight');
	}
	else if (screenName == &quot;WORKTYPEMAPPING_SCREEN&quot;)
	{
		fnRemoveClassWithSameStyle(&quot;#ExitButton&quot;,&quot;ph-wo-display&quot;,&quot;&quot;, false);
		$(&quot;#ExitButton&quot;).addClass(&quot;ph-wo-display-empty&quot;);
		previousScreenName = &quot;BASIC_SCREEN&quot;;
		nextScreenName =&quot;&quot;;
		fnRemoveClassWithSameStyle(&quot;#ExitButton&quot;,&quot;ph-wo-display&quot;,&quot;&quot;, false);
		$(&quot;#ExitButton&quot;).addClass(&quot;ph-wo-display-empty&quot;);
		fnRemoveClassWithSameStyle(&quot;#previousButton&quot;,&quot;ph-wo-display&quot;,&quot;&quot;, false);
		$(&quot;#previousButton&quot;).addClass(&quot;ph-wo-display-empty&quot;);
		fnRemoveClassWithSameStyle(&quot;#nextButton&quot;,&quot;ph-wo-display&quot;,&quot;&quot;, false);
		$(&quot;#nextButton&quot;).addClass(&quot;ph-wo-display-none&quot;);
		
		$(&quot;#basicsTD&quot;).removeClass('wizardtdhighlight');
		$(&quot;#heatCodeTD&quot;).removeClass('wizardtdhighlight');
		$(&quot;#parameterGroupTD&quot;).removeClass();
		$(&quot;#parameterGroupTD&quot;).addClass(&quot;fnShowWorkTypeScreen&quot;);
		$(&quot;#parameterGroupTD&quot;).addClass('wizardtdhighlight');
		$(&quot;#right_table&quot;).height(185);
	}
	else
	{
		alert('Unknown screen');
	}
	fnRemoveClassWithSameStyle(&quot;#contentTD&quot;,&quot;ph-wo-display&quot;,&quot;&quot;, false);
	$(&quot;#contentTD&quot;).addClass(&quot;ph-wo-display-empty&quot;);
}


function fnRemoveClass()
{
	$(&quot;#basicsTD&quot;).removeClass();
	$(&quot;#basicsTD&quot;).addClass(&quot;fnShowBasicScreen1&quot;);
	$(&quot;#parameterGroupTD&quot;).removeClass();
}


function fnShowBasicScreen1()
{
	fnHighLightMenu('BASIC_SCREEN');
	actionType = &quot;add&quot;;
	if($(&quot;#categoryId&quot;).val() > 0)
	{
		showIFrameLoading();
		actionType = &quot;modify&quot;;
		form.action = &quot;/phworkoutput/WOCategoryDetails/modify.htm?menuItemId=302&amp;subActionType=&quot;+actionType+&quot;&amp;categoryId=&quot;+$(&quot;#categoryId&quot;).val();
		form.submit();
	}
}

function fnShowWorkTypeScreen(isAddedWorkTypes)
{
	fnHighLightMenu('WORKTYPEMAPPING_SCREEN');
	var url = &quot;/phworkoutput/WOCategoryDetails/workTypeMappingScreen.htm?menuItemId=302&amp;categoryId=&quot;+$(&quot;#categoryId&quot;).val();
	doAjaxCall(url, null, $(&quot;#mappedParameterIds&quot;).val(), isAddedWorkTypes);
}

function fnAddWorkTypeMapping(str)
{
	//Empty fn required for adding workType Ids in second wizard screen
}


function doAjaxCall(url, selectedIds, mappedParameterIds, isAddedWorkTypes)
{
	$(&quot;#contentTD&quot;).html(fnGetAjaxLoader());
	$.ajax({
		url: url,
		headers	: {'X-Content-Security-Policy-Nonce': 'xZrLNGLeef26xpNnnF1igA=='},
		global: false,
		type: &quot;POST&quot;,
		data: ({ 			
				&quot;menuItemId&quot;	: '302',
				&quot;isAjaxCall&quot; 	: true,
				&quot;mappedUserGroupIds&quot;: selectedIds,
				&quot;mappedParameterIds&quot;: mappedParameterIds
 	          }),
	         success: function(resp)
			 {

	        	$(&quot;#contentTD&quot;).html('');
		        $(&quot;#contentTD&quot;).html(resp);
		        fnRemoveClassWithSameStyle(&quot;#pagingTD2&quot;,&quot;ph-wo-display&quot;,&quot;&quot;, false);
		        $(&quot;#pagingTD2&quot;).addClass(&quot;ph-wo-display-none&quot;);
		        fnSetEmptyTdHeight($(&quot;#right_table&quot;).height(), false);
		        if(isAddedWorkTypes)
		        {
		        	var successMsg = &quot;Work Types added successfully.&quot;;
					var failureMsg = &quot;Work Types can not be added, due to Error.&quot;;
				    
		        	if($(&quot;#categotyWorkTypesMsg&quot;).val() == 'SUCCESS')
			        {
		                var html = '&lt;span class=&quot;alert alert-success config-alert ph-wo-width-50percentage ph-wo-margin-left-min-150px&quot; id=&quot;successspanSubHead&quot;>&lt;a href=&quot;&quot; class=&quot;close&quot; data-dismiss=&quot;alert&quot; aria-label=&quot;close&quot;>&amp;times;&lt;/a>';
		                        html += '&lt;span id=&quot;successdivSubHead&quot;>&lt;/span>&lt;/span>';
		                $(&quot;#successspanSubHead&quot;).html(html);
		                $(&quot;#successdivSubHead&quot;).html(successMsg);
		                fnRemoveClassWithSameStyle(&quot;#successspanSubHead&quot;,&quot;ph-wo-display&quot;,&quot;&quot;, false);
		                $(&quot;#successspanSubHead&quot;).addClass(&quot;ph-wo-display-empty&quot;);
		               setTimeout(function() { $('#successspanSubHead').html(''); }, 10000); 
		                $(&quot;#categotyWorkTypesMsg&quot;).val('SUCCESS');
			        }
			        else
			        {
		                var html = '&lt;span class=&quot;alert alert-danger config-alert ph-wo-width-50percentage ph-wo-margin-left-min-150px&quot; id=&quot;failurespanSubHead&quot;>&lt;a href=&quot;&quot; class=&quot;close&quot; data-dismiss=&quot;alert&quot; aria-label=&quot;close&quot;>&amp;times;&lt;/a>';
		                        html +=  '&lt;span id=&quot;failuredivSubHead&quot; >&lt;/span>&lt;/span>';
		                $(&quot;#failurespanSubHead&quot;).html(html);
		                  $(&quot;#failuredivSubHead&quot;).html(failureMsg);
		                fnRemoveClassWithSameStyle(&quot;#failurespanSubHead&quot;,&quot;ph-wo-display&quot;,&quot;&quot;, false);
		                $(&quot;#failurespanSubHead&quot;).addClass(&quot;ph-wo-display-empty&quot;);
		               setTimeout(function() { $('#failurespanSubHead').html(''); }, 10000); 
		                $(&quot;#categotyWorkTypesMsg&quot;).val('Failed');
			        }
		        }
		        if($.browser.chrome){
		        	fnRemoveClassWithSameStyle(&quot;#right_table&quot;,&quot;ph-wo-border-bottom&quot;,&quot;&quot;, false);
		    		$(&quot;#right_table&quot;).addClass(&quot;ph-wo-border-bottom-none&quot;);
		        }

			 },
		     error :  function(msg,arg1,arg2)
		     {
		    	 processErrorStatus(msg.status, &quot;/phworkoutput&quot;);
		    	 return false;
			 }
	});
}

fnRemoveClassWithSameStyle(&quot;#adminBodyContent&quot;,&quot;ph-wo-margin-top&quot;,&quot;&quot;, false);
$(&quot;#adminBodyContent&quot;).addClass('ph-wo-margin-top-min-17px');

$(document).ready(function() 
{
	fnHighLightMenu(&quot;BASIC_SCREEN&quot;);
	if('modify' == 'modify' || $(&quot;#workTypeId&quot;).val() > 0)
	{
		fnRemoveClassWithSameStyle(&quot;#naviButtons&quot;,&quot;ph-wo-display&quot;,&quot;&quot;, false);
		$(&quot;#naviButtons&quot;).addClass(&quot;ph-wo-display-empty&quot;);
	}
	else
	{
		fnRemoveClassWithSameStyle(&quot;#naviButtons&quot;,&quot;ph-wo-display&quot;,&quot;&quot;, false);
		$(&quot;#naviButtons&quot;).addClass(&quot;ph-wo-display-none&quot;);
	}
	fnSetEmptyTdHeight($(&quot;#right_table&quot;).outerHeight(), true);
	if('true' == 'true')
	{
		fnShowWorkTypeScreen(false);
	}
});

function fnSetEmptyTdHeight(rightTabHeight, onload)
{
	var subactionType = 'modify';
	var trHeight = ($(&quot;#tab_table&quot;).find(&quot;tr:visible&quot;).length - 1) * 22;
	var borderHeight = ($(&quot;#tab_table&quot;).outerHeight()) - (trHeight + $(&quot;#empty_td&quot;).height());
	if(borderHeight &lt; 0)
    	borderHeight = 0;
	var objAgent = navigator.userAgent;
	var objbrowserName  = navigator.appName;
	var objfullVersion  = ''+parseFloat(navigator.appVersion); 
	if ((objOffsetVersion=objAgent.indexOf(&quot;Firefox&quot;))!=-1) 
	{
		 objbrowserName = &quot;Firefox&quot;;
	 }
	 if (objbrowserName == 'Firefox') 
	 {
		 if(subactionType == &quot;modify&quot;)
		{
			 remHt = rightTabHeight  - (trHeight+50);
		}
		 else if(subactionType == &quot;add&quot;)
		 {
			 remHt = rightTabHeight  - (trHeight+24);
		 }
		
	 }
	 else
	 {
		 remHt = rightTabHeight - (trHeight + borderHeight);
	 }  
	if((remHt + trHeight + borderHeight) == rightTabHeight)
		remHt = remHt - 1;
	remHt = getEmptyTdHeightInSafari(remHt);
	$(&quot;#empty_td&quot;).height(remHt);
}



	



	.disclaimerDiv
	{
		padding: 10px 50px 30px 50px;
		font-size: 10px;
	}



	
	The information made available through this web portal is intended solely for authorized users and for general informational purposes. While we strive to ensure that the data and reports are accurate and up to date, we make no warranties or representations-express or implied-regarding the completeness, accuracy, reliability, or fitness for a particular purpose of the information presented. Unauthorized access, use, distribution, copying, or modification of any content or data from this platform is strictly prohibited and may be unlawful. If you are not an intended or authorized user, please exit immediately and notify the administrator. We take precautions to safeguard this platform against viruses and malicious code; however, users are responsible for performing their own virus scans before downloading any files. We do not accept any liability for loss or damage arising from the use of this platform or the information accessed through it. By continuing to use this system, you agree to these terms and acknowledge your responsibility for any actions taken based on the information provided herein.



$(document).ready(function () {
    placeDisclaimer();
});

$(window).resize(placeDisclaimer);

function placeDisclaimer() 
{
    var contentHeight = $(&quot;.table-responsive&quot;).outerHeight(true);
    var windowHeight = $(window).height();
    var disclaimerHeight = $(&quot;.disclaimerDiv&quot;).outerHeight(true);

    fnRemoveClassWithSameStyle(&quot;#contentFrame&quot;, &quot;ph-wo-min-height&quot;, &quot;&quot;, false);
	
    if (contentHeight + disclaimerHeight &lt; windowHeight) 
    {
        var dynamicMargin = windowHeight - (contentHeight + disclaimerHeight) - 80;
		var className = (&quot;ph-wo-margin-top-&quot; +dynamicMargin + &quot;px&quot;).replace('.', '-');
		addDynamicCSS(&quot;xZrLNGLeef26xpNnnF1igA==&quot;, className , { &quot;margin-top&quot;:  dynamicMargin +'px !important'});
        $(&quot;.disclaimerDiv&quot;).addClass(className);
    } 
}



	
		
			
				
					
						Help
						
							
								
							
						
					
					
			
      		
		
	

		

    



	
		
			
				×
				Warning
      		
      		
      			 
      			
      		
      		
        		OK
      		
		
	



	
		
			
				×
				Confirmation
      		
      		
      			 
      			
      		
      		
      			NO
        		YES
      		
		
	









function fnHelp()
{
	
	 
		alert(&quot;Help cannot be shown as it is not been configured.&quot;);
	 
}
  

document.onclick = window.parent.fnHideSideBar;
gloabalDeviceWidth = window.parent.gloabalDeviceWidth;

function adjustDataTableWidth()
{
	if(gloabalDeviceWidth &lt;= 1024)
	{
		var style=document.createElement('style');
		style.type=&quot;text/css&quot;;
		style.setAttribute('nonce', &quot;xZrLNGLeef26xpNnnF1igA==&quot;);
		var css='.admin-responsivetable{max-width:'+gloabalDeviceWidth - 75+'px !important};'
		style.appendChild(document.createTextNode(css));
		document.head.appendChild(style);
	}
}

function adjustContentOverflow(elementId)
{
	if(gloabalDeviceWidth &lt;= 1024)
	{
		var style=document.createElement('style');
		style.type=&quot;text/css&quot;;
		style.setAttribute('nonce', &quot;xZrLNGLeef26xpNnnF1igA==&quot;);
		var css='#'+elementId+'{max-width:'+gloabalDeviceWidth - 75+'!important};'
		style.appendChild(document.createTextNode(css));
		document.head.appendChild(style);
	}
}

$(document).ready(function() {
	window.parent.hideWindowScroll();
	window.parent.hideBottomScroll();
	window.parent.fnSetHelpPath('');
	hideIFrameLoading();
	
	// forcing slimscroll without mouse movement
	$('#adminBodyContent').mouseover();
	$('#adminBodyContent').focus(); 

	setAlertHideTimer();
	fnRemoveClassWithSameStyle(&quot;.disclaimerDiv&quot;, &quot;ph-wo-display&quot;, &quot;&quot;, false);
	if(isIE8 != -1)
	{
		$(&quot;.input-right-content-textarea&quot;).addClass(&quot;input-right-content-textarea_ie&quot;);
	}
});
	
$(function() {
    var e = 300,
        a = 500;
    navigator.userAgent.match(/iPhone|iPad|iPod/i) ? $(&quot;#adminBodyContent&quot;).bind(&quot;touchend touchcancel touchleave&quot;, function(t) {
        $(this).scrollTop() > e ? $(&quot;#scroll-to-top-admin&quot;).fadeIn(a) : $(&quot;#scroll-to-top-admin&quot;).fadeOut(a)
    }) : $(&quot;#adminBodyContent&quot;).scroll(function() {
    	checkAndToggleBottomScroll();
        $(this).scrollTop() > e ? $(&quot;#scroll-to-top-admin&quot;).fadeIn(a) : $(&quot;#scroll-to-top-admin&quot;).fadeOut(a)
    }), $(&quot;#scroll-to-top-admin&quot;).click(function(e) {
    	$('#adminBodyContent').focus(); 
        return e.preventDefault(), $(&quot;#adminBodyContent&quot;).animate({
            scrollTop: 0
        }, a), !1
    })
}); 

function isSessionExpired(response)
{
	return window.parent.isSessionExpired(response);
}

var alertHideInterval;
function setAlertHideTimer()
{
	alertHideInterval = window.setTimeout(function() {
		hideAlertMessage();
	}, 20000);
	
	$(&quot;.config-alert > a&quot;).click(function() {
		hideAlertMessage();
		return false;
	});
}

function hideAlertMessage() {
	$(&quot;.config-alert&quot;).fadeTo(500, 0).slideUp(500, function(){
        $(this).remove();
    });
	
	clearTimeout(alertHideInterval);
}

function fnShowDialogWithNote(label, key, note)
{
	var url = &quot;/phworkoutput/HelpFramework.htm?subActionType=helpDialog&quot;;

	$.ajax({
			url: url,
			global: false,
			headers	: {'X-Content-Security-Policy-Nonce': 'xZrLNGLeef26xpNnnF1igA=='},
			type: &quot;POST&quot;,
			data: ({
					&quot;menuItemId&quot;: form.menuItemId.value,
					&quot;label&quot; : replaceBackJSParamXMLEntities(label),
					&quot;key&quot; : key,
					&quot;note&quot; : note
				}),
			success: function(resp)
			{
				if(!isSessionExpired(resp))
				{
					$(&quot;#showData&quot;).html(resp);
					fnRemoveClassWithSameStyle(&quot;#helpDialog&quot;, &quot;ph-wo-display&quot;, &quot;&quot;, false);
					$(&quot;#helpDialog&quot;).modal(&quot;show&quot;);
		    	}	
			},
			error :  function(msg,arg1,arg2)
			{
				processErrorStatus(msg.status, &quot;/phworkoutput&quot;);
				return false;
			}
	});
}

function fnShowDialog(label, key)
{
	var url = &quot;/phworkoutput/HelpFramework.htm?subActionType=helpDialog&quot;;

	$.ajax({
			url: url,
			global: false,
			headers	: {'X-Content-Security-Policy-Nonce': 'xZrLNGLeef26xpNnnF1igA=='},
			type: &quot;POST&quot;,
			data: ({
					&quot;menuItemId&quot;: &quot;302&quot;,
					&quot;label&quot; : replaceBackJSParamXMLEntities(label),
					&quot;key&quot; : key
				}),
			success: function(resp)
			{
				if(!isSessionExpired(resp))
				{
					$(&quot;#showData&quot;).html(resp);
					fnRemoveClassWithSameStyle(&quot;#helpDialog&quot;, &quot;ph-wo-display&quot;, &quot;&quot;, false);
					$(&quot;#helpDialog&quot;).modal(&quot;show&quot;);
		    	}	
			},
			error :  function(msg,arg1,arg2)
			{
				processErrorStatus(msg.status, &quot;/phworkoutput&quot;);
			    return false;
			}
	});
}

function fnShowPopup(xmlHttp, label, key)
{
	if (xmlHttp.readyState == 4)
	{
		
	}
}

$(document).on('click','.dropdown-menu', function(e) {
	e.stopPropagation();
});

function showIFrameLoading()
{
	window.parent.showLoadingModel();
}

function hideIFrameLoading()
{
	window.parent.hideLoadingModel();
}

$(&quot;.adminform&quot;).submit(function(e) {
	if(!isAlertModalVisible)
	{
		showIFrameLoading();
	}
});

function doSubmit(form) {
	showIFrameLoading();
	form.submit();
}

function fnGetAjaxLoader()
{
	divBuffer = '&lt;div class=&quot;ph-wo-text-align-center ph-wo-width-100percentage&quot;>';
	divBuffer += '&lt;img src=&quot;/phworkoutput/images/ajax-loader.gif&quot;>';
	divBuffer +='&lt;/div>';
	  
	return divBuffer;
}

function fnGetPopUpLoader()
{
	divBuffer = '&lt;div class=&quot;ph-wo-text-align-center ph-wo-width-100percentage ph-wo-position-absolute ph-wo-top-48px&quot;>';
	divBuffer += '&lt;img src=&quot;/phworkoutput/images/loader-trans.gif&quot;>';
	divBuffer +='&lt;/div>';
	  
	return divBuffer;
} 

function fnGetPopUpLoader(winObj)
{
	var loaderDiv = winObj.document.createElement('div');
    loaderDiv.style.textAlign = 'center';
    loaderDiv.style.width = '100%';
    loaderDiv.style.position = 'absolute';
    loaderDiv.style.top = '48%';
    loaderDiv.setAttribute('nonce', 'xZrLNGLeef26xpNnnF1igA==');
 
    var loaderImg = winObj.document.createElement('img');
    loaderImg.src = &quot;/phworkoutput/images/loader-trans.gif&quot;;
    loaderImg.setAttribute('nonce', 'xZrLNGLeef26xpNnnF1igA==');
    loaderDiv.appendChild(loaderImg);
    return loaderDiv;
}

function checkAndToggleBottomScroll()
{
	if($(&quot;.pagebanner&quot;).is(&quot;:visible&quot;))
    {
		var scrollPos = $(&quot;#adminBodyContent&quot;).scrollTop() + $(window).height();
		var remHeight = ($('.table-responsive').position().top + $('#adminBodyContent').scrollTop()) + $('.table-responsive').outerHeight(true);
    	if(scrollPos >= remHeight)
      	{
        	hideBottomScroll();
      	}
        else
      	{
    		showBottomScroll();
      	} 
    }
	else
	{
		hideBottomScroll();
	} 
}


function hideBottomScroll()
{
	fnRemoveClassWithSameStyle(&quot;.scroll-bottom&quot;, &quot;ph-wo-display&quot;, &quot;&quot;, false);
	$(&quot;.scroll-bottom&quot;).addClass(&quot;ph-wo-display&quot;);
}

function showBottomScroll()
{
	if($(&quot;#scrollInnerDiv&quot;).width() > 0)
	{
		fnRemoveClassWithSameStyle(&quot;.scroll-bottom&quot;, &quot;ph-wo-display&quot;, &quot;&quot;, false);
	}
	if($('.pagebanner').is(&quot;:visible&quot;))
	{
		$(&quot;.scroll-bottom&quot;).scrollLeft($(&quot;.table-responsive:visible&quot;).scrollLeft());
	}
}

function getEmptyTdHeightInSafari(tdHeight)
{
	if($.browser.safari){
        tdHeight = tdHeight - 12;
        return tdHeight;
    }
	else
	{
		return tdHeight;
	}
}

function removeSlimScroll(objectId) 
{
    if($('.'+objectId).parent().prop('className') == 'slimScrollDiv')
    {
		$('.'+objectId).slimScroll().unbind('slimscroll');
		$('.'+objectId).parent().replaceWith($('.'+objectId));
		fnRemoveClassWithSameStyle(&quot;.&quot;+objectId, &quot;ph-wo-overflow&quot;, &quot;&quot;, false);
		fnRemoveClassWithSameStyle(&quot;.&quot;+objectId, &quot;ph-wo-height&quot;, &quot;&quot;, false);
    }
}

function addScrollToApplicationBody()
{
	var frameHeight = parseInt($(window).height());
	var style=document.createElement('style');
	style.type=&quot;text/css&quot;;
	style.setAttribute('nonce', &quot;xZrLNGLeef26xpNnnF1igA==&quot;);
	var css='#bodyContent{min-height:'+frameHeight+' !important};'
	style.appendChild(document.createTextNode(css));
	document.head.appendChild(style);
}  

</value>
      <webElementGuid>6cc98c68-ac0c-4f49-ab3b-076f7211f13f</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;adminBodyContent&quot;)</value>
      <webElementGuid>b977a44b-2555-4556-a99f-7be6806e0421</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>ref_element</name>
      <type>Main</type>
      <value>Object Repository/Page_ProHance Work Output/Page_ProHance Work Output/iframe_Work Output Settings_contentFrame_1</value>
      <webElementGuid>1ddaad8f-9862-4953-915e-3363d12b9bcc</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//div[@id='adminBodyContent']</value>
      <webElementGuid>dadcd085-7483-4468-8526-ce21f2b248d1</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>//*/text()[normalize-space(.)='%>']/parent::*</value>
      <webElementGuid>a33d4e22-60e9-41d5-a2c6-a8ddd5044575</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div</value>
      <webElementGuid>f63b8b57-e401-46f6-97e7-a51e6d3a136b</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[@id = 'adminBodyContent' and (text() = concat(&quot;
	








%>


.configtable tbody tr td {
    padding: 0px; 
    height: 38px;
    border: none;
    padding-top: 0px;
    padding-bottom: 0px;
    text-align: left;
}

 #tableDivId
 {
 	margin-left: 5px;
 	padding-right: 10px;
 }
 
 .disclaimerDiv {
    padding: 10px 11px 30px 35px !important;
}













	
		













	
		×
		
	



	
	
	
	


	
		
			
				
					
						
							
							
							
								
							
							
						
						
							
								
										
						
					
					
			
			
			
			
		
	



 $( document ).ready(function() {
	fnFadeoutEvent(&quot;#dialogsuccessspan&quot;, 20000, &quot;ph-wo-display-none&quot;);
	fnFadeoutEvent(&quot;#dialogfailurespan&quot;, 20000, &quot;ph-wo-display-none&quot;);
	fnFadeoutEvent(&quot;#dialogdependencyspan&quot;, 20000, &quot;ph-wo-display-none&quot;);
	
	if ($(&quot;#dialogsuccessspan&quot;).is(&quot; , &quot;'&quot; , &quot;:hidden&quot; , &quot;'&quot; , &quot;)) 
	{
		fnRemoveClassWithSameStyle(&quot;#dialogsuccessspan&quot;, &quot;ph-wo-height&quot;, &quot;&quot;, false)
	 	$(&quot;#dialogsuccessspan&quot;).addClass(&quot;ph-wo-height-0px&quot;);
	}
	if ($(&quot;#dialogfailurespan&quot;).is(&quot; , &quot;'&quot; , &quot;:hidden&quot; , &quot;'&quot; , &quot;)) 
	{
		fnRemoveClassWithSameStyle(&quot;#dialogfailurespan&quot;, &quot;ph-wo-height&quot;, &quot;&quot;, false)
	 	$(&quot;#dialogfailurespan&quot;).addClass(&quot;ph-wo-height-0px&quot;);
	}
});

function fnSucessClose()
{
	fnRemoveClassWithSameStyle(&quot;#dialogsuccessspan&quot;, &quot;ph-wo-height&quot;, &quot;&quot;, false)
 	$(&quot;#dialogsuccessspan&quot;).addClass(&quot;ph-wo-height-0px&quot;);
}

function fnFailureClose()
{
	fnRemoveClassWithSameStyle(&quot;#dialogfailurespan&quot;, &quot;ph-wo-height&quot;, &quot;&quot;, false)
 	$(&quot;#dialogfailurespan&quot;).addClass(&quot;ph-wo-height-0px&quot;);
}

function fnShowDependency()
{
	$(&quot; , &quot;'&quot; , &quot;.modal&quot; , &quot;'&quot; , &quot;).draggable({
	    handle: &quot;.modal-header&quot;
	});
	
	var form;
	if (&quot; , &quot;'&quot; , &quot;CategoryForm&quot; , &quot;'&quot; , &quot; == &quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;)
	{
		//
	}
	else
	{
		form = document.CategoryForm;
	}
	var url = &quot;/phworkoutput/getDataDependency/getDependencyList.htm?deleteRecordId=&quot;+-1;
	windowTitle = fnGetWindowName();
	var winObj =  window.open(&quot;&quot;, windowTitle, &quot;status=1, toolbar=0, scrollbars=1, resizable=1, width=1330, height=600, left=20, top=20&quot;);
	winObj.document.body.appendChild(fnGetDependencyPopUpLoader());
	window.parent.parent.fnAddToChildWindows(winObj);
	tempAction = form.action;
    tempTarget = form.target;
    form.action = url;
    form.target = windowTitle;
    form.submit();
    winObj.focus();
    form.action = tempAction;
    form.target = tempTarget;
}

function fnGetDependencyPopUpLoader()
{
	var loaderDiv = window.parent.document.createElement(&quot; , &quot;'&quot; , &quot;div&quot; , &quot;'&quot; , &quot;);
    loaderDiv.style.textAlign = &quot; , &quot;'&quot; , &quot;center&quot; , &quot;'&quot; , &quot;;
    loaderDiv.style.width = &quot; , &quot;'&quot; , &quot;100%&quot; , &quot;'&quot; , &quot;;
    loaderDiv.style.position = &quot; , &quot;'&quot; , &quot;absolute&quot; , &quot;'&quot; , &quot;;
    loaderDiv.style.top = &quot; , &quot;'&quot; , &quot;48%&quot; , &quot;'&quot; , &quot;;
    loaderDiv.setAttribute(&quot; , &quot;'&quot; , &quot;nonce&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;xZrLNGLeef26xpNnnF1igA==&quot; , &quot;'&quot; , &quot;);
 
    var loaderImg =  window.parent.document.createElement(&quot; , &quot;'&quot; , &quot;img&quot; , &quot;'&quot; , &quot;);
    loaderImg.src = &quot;/phworkoutput/images/loader-trans.gif&quot;;
    loaderImg.setAttribute(&quot; , &quot;'&quot; , &quot;nonce&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;xZrLNGLeef26xpNnnF1igA==&quot; , &quot;'&quot; , &quot;);
    loaderDiv.appendChild(loaderImg);
    return loaderDiv;
}

/* function fnDeleteDependency()
{
	var primaryKeyId = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	var moduleName = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	var menuItemId = &quot; , &quot;'&quot; , &quot;302&quot; , &quot;'&quot; , &quot;;
	var formDefinitionId  = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	
	var actionName = &quot;/phworkoutput/getDataDependency/deleteDataDependency.htm?primaryKeyId=&quot;+primaryKeyId+&quot;&amp;moduleName=&quot;+moduleName+&quot;&amp;menuItemId=&quot;+menuItemId+&quot;&amp;formDefinitionId=&quot;+formDefinitionId;
	doAjaxCall(actionName, false, false);
} */


$(document).off(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;hideMsg&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;hideMsg&quot;]&quot; , &quot;'&quot; , &quot;, function() {
	
	fnRemoveClassWithSameStyle(&quot;.internalmessagefailure&quot;, &quot;ph-wo-display&quot;, &quot;&quot;, false);
	$(&quot;.internalmessagefailure&quot;).addClass(&quot;ph-wo-display-none&quot;);
	fnRemoveClassWithSameStyle(&quot;#displayErrorMessage&quot;, &quot;ph-wo-display&quot;, &quot;&quot;, false);
	$(&quot;#displayErrorMessage&quot;).addClass(&quot;ph-wo-display-none&quot;);
});

$(document).off(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnSucessClose&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnSucessClose&quot;]&quot; , &quot;'&quot; , &quot;, function() {
	fnSucessClose();
});

$(document).off(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnFailureClose&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnFailureClose&quot;]&quot; , &quot;'&quot; , &quot;, function() {
	fnFailureClose();
});

$(document).off(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnShowDependency&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnShowDependency&quot;]&quot; , &quot;'&quot; , &quot;, function() {
	fnShowDependency();
});


	
	
		







	
		
			
				
				
					
					
						Administration
					
				
				
			
		
		
			
				
				
					
					
						Work Output
					
				
				
			
		
		
			
				
					
				
				
				
					
					
						Work Type Category
					
				
				
					
				
			
		
		
			
				
				
				
					
					
						Work Type Category Details
					
				
				
			
		
		
		
		
		
			[Development Metrics] 
		
		
		
		
	




		
		
			
			 	
			
					
							BACK
					
					
					
			
		
	



	
	function fnBackToSummary()
	{
		form = document.WODataCollectionForm;
		if($(&quot;#isFistTab&quot;).val() != &quot;true&quot; || !document.WODataCollectionForm[1])
		{
			form = document.WODataCollectionForm[1];
		} 
		if(isNaN($(&quot;#lastXDays&quot;).val()))
		{
			$(&quot;#lastXDays&quot;).val(&quot;&quot;);
		}
		form.action = &quot;/phworkoutput/DataCollectionSummary.htm&quot;;
		doSubmit(form);
	}

	function fnRedirectPage(actionName)
	{
		form = document.CategoryForm;
		var tempAction = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
		if(typeof form == &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot; &amp;&amp; actionName == &quot;/phworkoutput/WorkTypeSummary.htm&quot;)
		{
			fnBackToWorkTypeSummary();
		}
		else
		{
			if (form.searchXML != null &amp;&amp; form.searchXML != &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot;)
			{
				form.searchXML.value=&quot;&quot;;
			}
			tempAction = form.action;
			document.forms[0].action = actionName;
		}
		doSubmit(document.forms[0]);
		form.action = tempAction;
	}

	function fnBackToParameterSummary()
	{
		form = document.ParameterForm;
		document.forms[0].action = &quot;/phworkoutput/WOParameterSummary.htm&quot;;
		doSubmit(document.forms[0]);
	}
	
	function fnBackToWOMetricSummary()
	{
		form = document.WOMetricForm;
		form.action = &quot;/phworkoutput/WOMetricSummary.htm&quot;;
		doSubmit(form);
	}
	
	
	function fnBackToWorkTypeSummary()
	{
		form = document.WorkTypeForm;
		document.forms[0].action = &quot;/phworkoutput/WorkTypeSummary.htm&quot;;
		doSubmit(document.forms[0]);
	}

	function fnBackToCategorySummary()
	{
		form = document.CategoryForm;
		document.forms[0].action = &quot;/phworkoutput/WorkOutputCategorySummary.htm&quot;;
		doSubmit(document.forms[0]);
	}
	
	function fnBackToWOWorkTypeAttributeSummary()
	{
		form = document.ConfigureWorkTypeAttrForm;
		document.forms[0].action = &quot;/phworkoutput/WOWorktypeAttributeSummary.htm&quot;;
		doSubmit(document.forms[0]);
	}

	$(&quot;[name = back]&quot;).click(function(){
		$(this).attr(&quot;disabled&quot;, &quot;disabled&quot;);
	});
	
	$(&quot;#reDirectPageArg0, #reDirectPageArg1, #reDirectPageArg2, #reDirectPageArg3&quot;).click(function()
 	{
	 	 var value = $(this).data(&quot; , &quot;'&quot; , &quot;action&quot; , &quot;'&quot; , &quot;);
	   	 if(value != null &amp;&amp; value != &quot;&quot; &amp;&amp; value != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
		 {
	   		fnRedirectPage(value);
		 }
    });

	$(document).off(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnQuickLinks&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnQuickLinks&quot;]&quot; , &quot;'&quot; , &quot;, function() {
		fnQuickLinks();
	});

	$(document).off(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnDeleteQuickLink&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnDeleteQuickLink&quot;]&quot; , &quot;'&quot; , &quot;, function() {
		fnDeleteQuickLink();
	});

	$(document).off(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnBackToSummary&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnBackToSummary&quot;]&quot; , &quot;'&quot; , &quot;, function() {
		fnBackToSummary();
	});

	$(document).off(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnBackToParameterSummary&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnBackToParameterSummary&quot;]&quot; , &quot;'&quot; , &quot;, function() {
		fnBackToParameterSummary();
	});

	$(document).off(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnBackToWorkTypeSummary&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnBackToWorkTypeSummary&quot;]&quot; , &quot;'&quot; , &quot;, function() {
		fnBackToWorkTypeSummary();
	});

	$(document).off(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnBackToCategorySummary&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnBackToCategorySummary&quot;]&quot; , &quot;'&quot; , &quot;, function() {
		fnBackToCategorySummary();
	});

	$(document).off(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnBackToWOMetricSummary&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnBackToWOMetricSummary&quot;]&quot; , &quot;'&quot; , &quot;, function() {
		fnBackToWOMetricSummary();
	});
	
	$(document).off(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnBackToWOWorkTypeAttributeSummary&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnBackToWOWorkTypeAttributeSummary&quot;]&quot; , &quot;'&quot; , &quot;, function() {
		fnBackToWOWorkTypeAttributeSummary();
	});
	
	$(document).off(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnRedirectPage&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnRedirectPage&quot;]&quot; , &quot;'&quot; , &quot;, function() {
		var param = $(this).data(&quot;param&quot;);
		fnRedirectPage(param);
	});
	
	$(&quot;.historyback&quot;).click(function() 
	{
	    history.back(-1); 
	});

	
	
		
			
				
					
						
							 
								
									
									1
										Basic Details
									
								
							
							
								
									2
										Work Type Mapping
									
								
							
							
						
							
								 
								
							
					  
					
					
						
							
								
									

#mainTable > tbody > tr
{
	height: 0px !important;
}

#target_table > tbody > tr
{
	height: 0px !important;
}

table
{
  border-collapse : unset;
}

th
{
	border-right: none !important;
}

#CommonDataTableId_wrapper > #CommonDataTableId_filter
{
	border-bottom: none !important;
}

#CommonDataTableId
{
	border-top: 1px solid #e5e9ea;
	margin-top: 0px !important;
	border-right: 1px solid #e5e9ea;
}

table.dataTable td
{
	border-top: 1px solid #e5e9ea;
	border-left: 1px solid #e5e9ea;
	padding-left: 8px;
	padding-right: 10px;
}

.dataTables_info
{
	margin-left: 0px !important;
	border-top: 1px solid #e5e9ea;
}

.dataTables_paginate 
{
	border-top: 1px solid #e5e9ea;
}

.dataTables_empty
{
	vertical-align: middle;
}

.userSummaryTable
{
	margin: 0px;
	height: 550px;
	overflow-y: auto;
	overflow-x: hidden;
	padding-right: 5px;
	padding-bottom: 5px;
}

.chkbox-padding
{
	padding-bottom: 5px;
}

#right_table > tbody > tr > td
{
	border-bottom: 1px solid #e5e9ea;
}














        
                
                
        


























 table.dataTable thead .sorting:after
{
   content: &quot;&quot; !important;
}
table.dataTable thead .sorting:before
{
    content: &quot;&quot; !important;
}

table.dataTable thead .sorting_asc:after {
    content: url(/phworkoutput/images/sort_asc.png) !important;
   	opacity:1 !important;
    margin-bottom: 5px;
    margin-right:-10px;
    top: 20% !important;
}
table.dataTable thead .sorting_desc:after {
    content: url(/phworkoutput/images/sort_desc.png) !important;
    opacity:1 !important;
    margin-bottom: 5px;
    margin-right:-10px;
    top: 20% !important;
}
v.dataTables_scrollHead table.dataTable {
		    width:100%;
		}
		
.no-footer
{
	width:100% !important;
} 

.form-inline .form-control 
{
	vertical-align: unset !important;
}

table.dataTable
{
	border-collapse: collapse !important; 
}






















var dynPageLength;
var dynExportColumns;
var dynTitleHeaderName;
var dynFileName;
var dynPdfPageSize;
var dynSortingIndex;
var dynNumericAlignColArr;
var dynWidthArr;
var isMultiOrgAdmin;
var reportScheduleReq;
var sortingOrder;
var recordSize;

var isPaginationExtendEnable = false;
var lengthMenu = isPaginationExtendEnable ? [10, 20, 50, 100, 200, 500, 1000]  : [10, 20, 50, 100, 200];

/* Build Data Tabel */
function buildDataTableStructure(jsonData, pageLength, columns, sortingIndex)
{
	buildDataTableStructureImproved(jsonData, pageLength, columns, sortingIndex, &quot;CommonDataTableId&quot;)
}

/* Build Data Tabel by passing Dynamic Table Id*/
function buildDataTableStructureImproved(jsonData, pageLength, columns, sortingIndex, dynTableId)
{
	var isReqSorting = true;
	if(sortingIndex &lt; 0)
	{
		isReqSorting  = false;
		sortingIndex = 0;
	}
	var table = $(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; +dynTableId+&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;).DataTable( {
        data: jsonData,
        deferRender:    true,
        &quot;pageLength&quot;: pageLength,
        &quot;ordering&quot; : isReqSorting,
        &quot;autoWidth&quot;:false,
        &quot;scrollX&quot;: true,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px&quot; , &quot;'&quot; , &quot;> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100&quot; , &quot;'&quot; , &quot;>[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
                &quot;previous&quot;:   &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;
            },
        },
         columns: columns,
         &quot;order&quot;: [[ sortingIndex, &quot; , &quot;'&quot; , &quot;asc&quot; , &quot;'&quot; , &quot; ]],
         dom: &quot; , &quot;'&quot; , &quot;Blfrtip&quot; , &quot;'&quot; , &quot;,
         buttons: [
            //Empty for no exports
         ]
    } );
}

/* Build Data Tabel With Default Data Table Export */
function buildDataTableStructureWithExports(jsonData, pageLength, columns, exportColumns, titleHeaderName, fileName, pdfPageSize, sortingIndex)
{
	var isReqSorting = true;
	if(sortingIndex &lt; 0)
	{
		isReqSorting  = false;
		sortingIndex = 0;
	}
	var excelReportType = $(&quot;#excelReportType&quot;).val();
	excelReportType = &quot;.&quot; + excelReportType.toLowerCase();
	var disclaimerText = $(&quot;#disclaimerText&quot;).val();
	titleHeaderName = replaceBackXMLEntities(titleHeaderName);
	fileName = replaceBackXMLEntities(fileName);
	var table = $(&quot; , &quot;'&quot; , &quot;#CommonDataTableId&quot; , &quot;'&quot; , &quot;).DataTable( {
        data: 	jsonData,
        deferRender:    true,
        &quot;pageLength&quot;: pageLength,
        &quot;ordering&quot; : isReqSorting,
        &quot;scrollX&quot;: true,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	 &quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
             &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px&quot; , &quot;'&quot; , &quot;> Displaying _START_ to _END_ &lt;label>&quot;,
             &quot;infoEmpty&quot;: &quot;No records to display&quot;,
             &quot;infoFiltered&quot;: &quot;&lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100&quot; , &quot;'&quot; , &quot; >[Filtered from _MAX_ records] &lt;/label>&quot;,
             &quot;search&quot;:         &quot;Search&quot;,
             &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
             &quot;paginate&quot;: { 
                &quot;next&quot;:       &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
                &quot;previous&quot;:   &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;
            },
        },
         columns: columns,
         &quot;order&quot;: [[ sortingIndex, &quot; , &quot;'&quot; , &quot;asc&quot; , &quot;'&quot; , &quot; ]],
         dom: &quot; , &quot;'&quot; , &quot;Blfrtip&quot; , &quot;'&quot; , &quot;,
         buttons: [
        	 {
        		 text:   &quot; , &quot;'&quot; , &quot;&lt;label class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot; class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>|&amp;nbsp;&amp;nbsp; Export:&amp;nbsp;&lt;/label>&quot; , &quot;'&quot; , &quot;,
        		 autoClose: &quot; , &quot;'&quot; , &quot;false&quot; , &quot;'&quot; , &quot;,
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;
        	 },
             {
                 extend: &quot; , &quot;'&quot; , &quot;excelHtml5&quot; , &quot;'&quot; , &quot;,
                 exportOptions: {
                 	columns: exportColumns,
                 	orthogonal: &quot; , &quot;'&quot; , &quot;export&quot; , &quot;'&quot; , &quot;,
                 	 stripHtml: true
                 },
                 filename: fileName,
                 text:   &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/excel.png&quot; title=&quot;Export to Excel&quot; align=&quot;middle&quot; class=&quot;ph-wo-vertical-align-middle ph-wo-cursor-pointer&quot;>&quot; , &quot;'&quot; , &quot;,
                 autoClose: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;,
                 extension: excelReportType,
                 createEmptyCells: true,
                 title : titleHeaderName,
                 messageBottom: function () {
                     return disclaimerText;
                 	},
                     customize: function ( xlsx ) {
						var sheet = xlsx.xl.worksheets[&quot; , &quot;'&quot; , &quot;sheet1.xml&quot; , &quot;'&quot; , &quot;];
						$(&quot; , &quot;'&quot; , &quot;row c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;25&quot; , &quot;'&quot; , &quot; );
                        $(&quot; , &quot;'&quot; , &quot;c[r=A1] t&quot; , &quot;'&quot; , &quot;, sheet).text( titleHeaderName );
                        $(&quot; , &quot;'&quot; , &quot;row:first c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;27&quot; , &quot;'&quot; , &quot; ); 
                        $(&quot; , &quot;'&quot; , &quot;row:eq(1) c&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;27&quot; , &quot;'&quot; , &quot;);
                        
                        if(disclaimerText != null &amp;&amp; disclaimerText != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
                        {
                        	$(&quot; , &quot;'&quot; , &quot;row:last c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;55&quot; , &quot;'&quot; , &quot; );
	                        $(&quot; , &quot;'&quot; , &quot;row:last&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;customHeight&quot; , &quot;'&quot; , &quot;, 1);
	                        $(&quot; , &quot;'&quot; , &quot;row:last&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;ht&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;150&quot; , &quot;'&quot; , &quot;);
                        }
                      },
                      
                 button: [&quot; , &quot;'&quot; , &quot;excel&quot; , &quot;'&quot; , &quot;]

             },
             {
                 extend: &quot; , &quot;'&quot; , &quot;pdfHtml5&quot; , &quot;'&quot; , &quot;,
                 orientation: &quot; , &quot;'&quot; , &quot;landscape&quot; , &quot;'&quot; , &quot;,
                 exportOptions: {
                 	columns: exportColumns,
                 	orthogonal: &quot; , &quot;'&quot; , &quot;export&quot; , &quot;'&quot; , &quot;,
                 	 stripHtml: true
                 },
                 text:      &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/pdf.png&quot; title=&quot;Export to PDF&quot; align=&quot;middle&quot; class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>&quot; , &quot;'&quot; , &quot;,
                 filename: fileName,
                 autoClose: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 footer:&quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 messageBottom: function () {
                     return disclaimerText;
                  	},
               	 customize: function ( doc ) {
               		if(exportColumns.length &lt;= 4 || pdfPageSize == &quot; , &quot;'&quot; , &quot;A4&quot; , &quot;'&quot; , &quot;)
               		{
	               		doc.content[1].table.widths = Array(doc.content[1].table.body[0].length + 1).join(&quot; , &quot;'&quot; , &quot;*&quot; , &quot;'&quot; , &quot;).split(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
               		}
               		
               	   var cols = [];
               	   cols[0] = {text: &quot; , &quot;'&quot; , &quot;© JaMocha Tech Pvt. Ltd. 2009-2026&quot; , &quot;'&quot; , &quot;, alignment: &quot; , &quot;'&quot; , &quot;center&quot; , &quot;'&quot; , &quot;, fontSize:&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;, margin:[0, 0, 0, 0] };
               	   var objFooter = {};
               	   objFooter[&quot; , &quot;'&quot; , &quot;columns&quot; , &quot;'&quot; , &quot;] = cols;
               	   doc[&quot; , &quot;'&quot; , &quot;footer&quot; , &quot;'&quot; , &quot;]=objFooter;
               	   doc.styles.tableHeader.fillColor = &quot; , &quot;'&quot; , &quot;#006bb7&quot; , &quot;'&quot; , &quot;;
               	   doc.styles.tableHeader.alignment=&quot;left&quot;;
               	 	
               	   doc[&quot; , &quot;'&quot; , &quot;header&quot; , &quot;'&quot; , &quot;]=(function(page, pages) {
							return {
								columns: [
									{
										alignment: &quot; , &quot;'&quot; , &quot;left&quot; , &quot;'&quot; , &quot;,
										text: &quot; , &quot;'&quot; , &quot;ProHance&quot; , &quot;'&quot; , &quot;,
										fontSize: 12,
									},
									{
										alignment: &quot; , &quot;'&quot; , &quot;right&quot; , &quot;'&quot; , &quot;,
										fontSize: 8,
										text: [&quot; , &quot;'&quot; , &quot;Page &quot; , &quot;'&quot; , &quot;, { text: page.toString() }]
									}
								],
								margin: 20
							}
						});
               	doc.content.forEach(function(item) {
	               		if (item.table)
	               		{
	               			item.layout = {
	               					hLineColor: function(i, node) { return &quot; , &quot;'&quot; , &quot;#000000&quot; , &quot;'&quot; , &quot;; },
	               					vLineColor: function(i, node) { return &quot; , &quot;'&quot; , &quot;#000000&quot; , &quot;'&quot; , &quot;; }
	               			};
	                    }
	               });
		           doc.styles.tableHeader = {
		        		   fontSize		: 11,
		        		   bold			: true,
		        		   color		: &quot; , &quot;'&quot; , &quot;#ffffff&quot; , &quot;'&quot; , &quot;,
		        		   fillColor	: &quot; , &quot;'&quot; , &quot;#006bb7&quot; , &quot;'&quot; , &quot;
			       };
               	 },
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;,
                 pageSize : pdfPageSize,
                 titleAttr : &quot; , &quot;'&quot; , &quot;PDF&quot; , &quot;'&quot; , &quot;,
                 title : titleHeaderName,
                 button: [ &quot; , &quot;'&quot; , &quot;pdf&quot; , &quot;'&quot; , &quot; ]
             }
         ],
         drawCallback: function() {
          	  var hasRows = this.api().rows({ filter: &quot; , &quot;'&quot; , &quot;applied&quot; , &quot;'&quot; , &quot; }).data().length > 0;
          	 $(&quot; , &quot;'&quot; , &quot;.buttons-excel&quot; , &quot;'&quot; , &quot;)[0].style.visibility = hasRows ? &quot; , &quot;'&quot; , &quot;visible&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;;
          	 $(&quot; , &quot;'&quot; , &quot;.buttons-pdf&quot; , &quot;'&quot; , &quot;)[0].style.visibility = hasRows ? &quot; , &quot;'&quot; , &quot;visible&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;;
          	}
    } );
	
}

/* Build Data Tabel With Custom Export (Custom Excel And PDF doExport) along With Email Report Schedular */
function buildDataTableStructureWithReportScheduler(jsonData, columns, jsonObjArr)
{
	fnUpdateProperties(jsonObjArr);
	var isReqSorting = true;
	if(dynSortingIndex &lt; 0)
	{
		isReqSorting  = false;
		dynSortingIndex = 0;
	}
	
	var exportExcel = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	var exportPdf = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	var emailSpan = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	if(reportScheduleReq == &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;)
	{
		emailSpan = &quot; , &quot;'&quot; , &quot;&lt;span id=&quot;emailSpan&quot; class=&quot;ph-wo-cursor-pointer&quot; data-onclick=&quot;showSchedulerDialogOnClick&quot;>&lt;img src=&quot;/phworkoutput/images/email.png&quot; title=&quot;Email&quot; align=&quot;middle&quot; class=&quot;ph-wo-vertical-align-middle&quot;>&lt;/span>&quot; , &quot;'&quot; , &quot;;
		if($(&quot;#isAllowEmailExport&quot;).val() == &quot; , &quot;'&quot; , &quot;false&quot; , &quot;'&quot; , &quot;)
		{
			emailSpan = &quot; , &quot;'&quot; , &quot;&lt;span id=&quot;emailSpan&quot; class=&quot;ph-wo-cursor-pointer&quot; title=&quot;You do not have permission to download this content&quot;>&lt;img src=&quot;/phworkoutput/images/email.png&quot; title=&quot;Email&quot; align=&quot;middle&quot; class=&quot;ph-wo-vertical-align-middle&quot;>&lt;/span>&quot; , &quot;'&quot; , &quot;;
		}
	}
	if(isMultiOrgAdmin == &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;)
	{
		exportExcel = &quot; , &quot;'&quot; , &quot;excelHtml5&quot; , &quot;'&quot; , &quot;;
		exportPdf = &quot; , &quot;'&quot; , &quot;pdfHtml5&quot; , &quot;'&quot; , &quot;;
	}
	var excelIcon = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	var pdfIcon = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	if($(&quot;#isAllowExcelExport&quot;).val() == &quot; , &quot;'&quot; , &quot;false&quot; , &quot;'&quot; , &quot;)
	{
		excelIcon = &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/excel.png&quot; title=&quot;You do not have permission to download this content&quot; align=&quot;middle&quot; class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>&quot; , &quot;'&quot; , &quot;;
		exportExcel = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	}
	else
	{
		excelIcon = &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/excel.png&quot; title=&quot;Export to Excel&quot; data-onclick=&quot;fnDataTableExportOnClick&quot; data-etype=&quot;excel&quot; align=&quot;middle&quot; class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>&quot; , &quot;'&quot; , &quot;;
	}
	if($(&quot;#isAllowPdfExport&quot;).val() == &quot; , &quot;'&quot; , &quot;false&quot; , &quot;'&quot; , &quot;)
	{
		pdfIcon = &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/pdf.png&quot; title=&quot;You do not have permission to download this content&quot; align=&quot;middle&quot; class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>&quot; , &quot;'&quot; , &quot;;
		exportPdf = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	}
	else if(recordSize > 10000)
	{
		pdfIcon = &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/pdf.png&quot; title=&quot;PDF file cannot be downloaded, as number of records more then 10000&quot; align=&quot;middle&quot; class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>&quot; , &quot;'&quot; , &quot;;
		exportPdf = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	}	
	else
	{
		pdfIcon = &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/pdf.png&quot; title=&quot;Export to PDF&quot; data-onclick=&quot;fnDataTableExportOnClick&quot; data-etype=&quot;pdf&quot; align=&quot;middle&quot; class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>&quot; , &quot;'&quot; , &quot;;
	}
	
	var excelReportType = $(&quot;#excelReportType&quot;).val();
	excelReportType = &quot;.&quot; + excelReportType.toLowerCase();
	titleHeaderName = replaceBackXMLEntities(dynTitleHeaderName);
	fileName = replaceBackXMLEntities(dynFileName);
	var disclaimerText = $(&quot;#disclaimerText&quot;).val();
	var table = $(&quot; , &quot;'&quot; , &quot;#CommonDataTableId&quot; , &quot;'&quot; , &quot;).DataTable( {
		 &quot;fnDrawCallback&quot;: function() {
	            // after table is redrawndo something here
	        	$(&quot;#thirdViewSortOrder&quot;).val($(&quot;#CommonDataTableId&quot;).dataTable().fnSettings().aaSorting);
	        },
        data: 	jsonData,
        deferRender:    true,
        &quot;pageLength&quot;: dynPageLength,
        &quot;scrollX&quot;: true,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;ordering&quot; : isReqSorting,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	 &quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
             &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px&quot; , &quot;'&quot; , &quot;> Displaying _START_ to _END_ &lt;label>&quot;,
             &quot;infoEmpty&quot;: &quot;No records to display&quot;,
             &quot;infoFiltered&quot;: &quot;&lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100&quot; , &quot;'&quot; , &quot; >[Filtered from _MAX_ records] &lt;/label>&quot;,
             &quot;search&quot;:         &quot;Search&quot;,
             &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
                &quot;previous&quot;:   &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;
            },
        },
         columns: columns,
         &quot;order&quot;: [[ dynSortingIndex, sortingOrder ]],
         dom: &quot; , &quot;'&quot; , &quot;Blfrtip&quot; , &quot;'&quot; , &quot;,
         buttons: [
        	 {
        		 text:   &quot; , &quot;'&quot; , &quot;&lt;label class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>|&amp;nbsp;&amp;nbsp; Export:&amp;nbsp;&lt;/label>&quot; , &quot;'&quot; , &quot;,
        		 autoClose: &quot; , &quot;'&quot; , &quot;false&quot; , &quot;'&quot; , &quot;,
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;
        	 },
        	 {
        		 text:   emailSpan,
        		 autoClose: &quot; , &quot;'&quot; , &quot;false&quot; , &quot;'&quot; , &quot;,
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;
        	 },
        	 {
                 extend: exportExcel,
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: &quot; , &quot;'&quot; , &quot;export&quot; , &quot;'&quot; , &quot;,
                 	 stripHtml: true
                 },
                 filename: fileName,
                 text:   excelIcon,
                 autoClose: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;,
                 extension: excelReportType,
                 createEmptyCells: true,
                 title : titleHeaderName,
                 messageBottom: function () {
                     return disclaimerText;
                 	},
                     customize: function ( xlsx ) {
						var sheet = xlsx.xl.worksheets[&quot; , &quot;'&quot; , &quot;sheet1.xml&quot; , &quot;'&quot; , &quot;];
						$(&quot; , &quot;'&quot; , &quot;row c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;25&quot; , &quot;'&quot; , &quot; );
                        $(&quot; , &quot;'&quot; , &quot;c[r=A1] t&quot; , &quot;'&quot; , &quot;, sheet).text( titleHeaderName );
                        $(&quot; , &quot;'&quot; , &quot;row:first c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;27&quot; , &quot;'&quot; , &quot; ); 
                        $(&quot; , &quot;'&quot; , &quot;row:eq(1) c&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;27&quot; , &quot;'&quot; , &quot;);
                        
                        if(disclaimerText != null &amp;&amp; disclaimerText != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
                        {
                        	$(&quot; , &quot;'&quot; , &quot;row:last c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;55&quot; , &quot;'&quot; , &quot; );
	                        $(&quot; , &quot;'&quot; , &quot;row:last&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;customHeight&quot; , &quot;'&quot; , &quot;, 1);
	                        $(&quot; , &quot;'&quot; , &quot;row:last&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;ht&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;150&quot; , &quot;'&quot; , &quot;);
                        }
                        
                      },
                 button: [&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;]
             },
             {
                 extend: exportPdf,
                 orientation: &quot; , &quot;'&quot; , &quot;landscape&quot; , &quot;'&quot; , &quot;,
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: &quot; , &quot;'&quot; , &quot;export&quot; , &quot;'&quot; , &quot;,
                 	 stripHtml: true
                 },
                 text:    pdfIcon ,
                 filename: fileName,
                 autoClose: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 footer:&quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 messageBottom: function () {
                     return disclaimerText;
                  	},
               	 customize: function ( doc ) {
               		
               	   var cols = [];
               	   cols[0] = {text: &quot; , &quot;'&quot; , &quot;© JaMocha Tech Pvt. Ltd. 2009-2026&quot; , &quot;'&quot; , &quot;, alignment: &quot; , &quot;'&quot; , &quot;center&quot; , &quot;'&quot; , &quot;, fontSize:&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;, margin:[0, 0, 0, 0] };
               	   var objFooter = {};
               	   objFooter[&quot; , &quot;'&quot; , &quot;columns&quot; , &quot;'&quot; , &quot;] = cols;
               	   doc[&quot; , &quot;'&quot; , &quot;footer&quot; , &quot;'&quot; , &quot;]=objFooter;
               	   doc.styles.tableHeader.fillColor = &quot; , &quot;'&quot; , &quot;#006bb7&quot; , &quot;'&quot; , &quot;;
               	   doc.styles.tableHeader.alignment=&quot;left&quot;;
               	 	
               	   doc[&quot; , &quot;'&quot; , &quot;header&quot; , &quot;'&quot; , &quot;]=(function(page, pages) {
							return {
								columns: [
									{
										alignment: &quot; , &quot;'&quot; , &quot;left&quot; , &quot;'&quot; , &quot;,
										text: &quot; , &quot;'&quot; , &quot;ProHance&quot; , &quot;'&quot; , &quot;,
										fontSize: 12,
									},
									{
										alignment: &quot; , &quot;'&quot; , &quot;right&quot; , &quot;'&quot; , &quot;,
										fontSize: 8,
										text: [&quot; , &quot;'&quot; , &quot;Page &quot; , &quot;'&quot; , &quot;, { text: page.toString() }]
									}
								],
								margin: 20
							}
						});
               	   
	               	if(dynNumericAlignColArr.length > 0)
	               	{
	               		var rowCount = doc.content[1].table.body.length;
		               	for (i = 0; i &lt; rowCount; i++) 
		               	{
		               		dynNumericAlignColArr.forEach(function(number) {
		                   		doc.content[1].table.body[i][number].alignment = &quot; , &quot;'&quot; , &quot;right&quot; , &quot;'&quot; , &quot;;
		                   	  });
		               	}
               		}
	               	
	               	doc.content[1].table.widths = dynWidthArr;
	               	doc.content.forEach(function(item) {
	               		if (item.table)
	               		{
	               			item.layout = {
	               					hLineColor: function(i, node) { return &quot; , &quot;'&quot; , &quot;#000000&quot; , &quot;'&quot; , &quot;; },
	               					vLineColor: function(i, node) { return &quot; , &quot;'&quot; , &quot;#000000&quot; , &quot;'&quot; , &quot;; }
	               			};
	                    }
	               });
		           doc.styles.tableHeader = {
		        		   fontSize		: 11,
		        		   bold			: true,
		        		   color		: &quot; , &quot;'&quot; , &quot;#ffffff&quot; , &quot;'&quot; , &quot;,
		        		   fillColor	: &quot; , &quot;'&quot; , &quot;#006bb7&quot; , &quot;'&quot; , &quot;
			       };
               	 },
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;,
                 pageSize : &quot; , &quot;'&quot; , &quot;LEGAL&quot; , &quot;'&quot; , &quot;,
                 titleAttr : &quot; , &quot;'&quot; , &quot;PDF&quot; , &quot;'&quot; , &quot;,
                 title : titleHeaderName,
                 button: [ &quot; , &quot;'&quot; , &quot;pdf&quot; , &quot;'&quot; , &quot; ]
             }
         ]
    } );
}

/* Build Data Tabel With Row Grouping and Default Data Table Export */
function buildDataTableStructureRowsGroups(jsonData, pageLength, columns, exportColumns, titleHeaderName, fileName, pdfPageSize, sortingIndex, rowsGroup, jsonObjArr)
{
	//fnUpdateProperties(jsonObjArr);
	var isReqSorting = true;
	if(sortingIndex &lt; 0)
	{
		isReqSorting  = false;
		sortingIndex = 0;
	}
	var excelReportType = $(&quot;#excelReportType&quot;).val();
	excelReportType = &quot;.&quot; + excelReportType.toLowerCase();  
	var disclaimerText = $(&quot;#disclaimerText&quot;).val();
    titleHeaderName = replaceBackXMLEntities(titleHeaderName);
    fileName = replaceBackXMLEntities(fileName);
	var table = $(&quot; , &quot;'&quot; , &quot;#CommonDataTableId&quot; , &quot;'&quot; , &quot;).DataTable( {
        data: 	jsonData,
        deferRender:    false,
        &quot;pageLength&quot;: pageLength,
        &quot;ordering&quot; : isReqSorting,
        &quot;scrollX&quot;: false,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	 &quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
             &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px&quot; , &quot;'&quot; , &quot;> Displaying _START_ to _END_ &lt;label>&quot;,
             &quot;infoEmpty&quot;: &quot;No records to display&quot;,
             &quot;infoFiltered&quot;: &quot;&lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100&quot; , &quot;'&quot; , &quot; >[Filtered from _MAX_ records] &lt;/label>&quot;,
             &quot;search&quot;:         &quot;Search&quot;,
             &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
                &quot;previous&quot;:   &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;
            },
        },
         columns: columns,
         columnDefs: [{
        	    &quot;defaultContent&quot;: &quot;&quot;,
        	    &quot;targets&quot;: &quot;_all&quot;
        	  }],    
         &quot;order&quot;: [[ sortingIndex, &quot; , &quot;'&quot; , &quot;asc&quot; , &quot;'&quot; , &quot; ]],
         rowGroup: rowsGroup,      
         dom: &quot; , &quot;'&quot; , &quot;Blfrtip&quot; , &quot;'&quot; , &quot;,
         buttons: [
             //Empty for no exports
          ]
    });
	
}

function fnUpdateProperties(jsonObjArr) 
{
	dynPageLength = jsonObjArr[&quot;pageLength&quot;];
	dynExportColumns = jsonObjArr[&quot;exportColumns&quot;];
	dynTitleHeaderName = jsonObjArr[&quot;titleHeaderName&quot;];
	dynFileName = jsonObjArr[&quot;fileName&quot;];
	dynPdfPageSize = jsonObjArr[&quot;pdfPageSize&quot;];
	dynSortingIndex = jsonObjArr[&quot;sortingIndex&quot;];
	dynNumericAlignColArr = jsonObjArr[&quot;rightAlignColArr&quot;];
	dynWidthArr	=	jsonObjArr[&quot;dynWidthArr&quot;];
	isMultiOrgAdmin	=	jsonObjArr[&quot;isMultiOrgAdmin&quot;];
	reportScheduleReq	=	jsonObjArr[&quot;reportScheduleReq&quot;];
	if(jsonObjArr[&quot;order&quot;] != null &amp;&amp; jsonObjArr[&quot;order&quot;] != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
	{
		sortingOrder	=	jsonObjArr[&quot;order&quot;];
	}
	else 
	{
		sortingOrder = &quot;asc&quot;;
	}
	recordSize	=	jsonObjArr[&quot;recordSize&quot;];
}

/* Build Data Tabel with Default Data Table Export along with JSONArray param*/
function buildDataTableStructureWithExportsArr(jsonData, columns, jsonObjArr)
{
	fnUpdateProperties(jsonObjArr);
	var isReqSorting = true;
	if(dynSortingIndex &lt; 0)
	{
		isReqSorting  = false;
		dynSortingIndex = 0;
	}
	var excelReportType = $(&quot;#excelReportType&quot;).val();
	excelReportType = &quot;.&quot; + excelReportType.toLowerCase();
	var disclaimerText = $(&quot;#disclaimerText&quot;).val();
	titleHeaderName = replaceBackXMLEntities(dynTitleHeaderName);
	fileName = replaceBackXMLEntities(dynFileName);
	var table = $(&quot; , &quot;'&quot; , &quot;#CommonDataTableId&quot; , &quot;'&quot; , &quot;).DataTable( {
        data: 	jsonData,
        deferRender:    true,
        &quot;pageLength&quot;: dynPageLength,
        &quot;bDestroy&quot;: true,
        &quot;scrollX&quot;: true,
        &quot;ordering&quot; : isReqSorting,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px&quot; , &quot;'&quot; , &quot;> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100&quot; , &quot;'&quot; , &quot; >[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
                &quot;previous&quot;:   &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;
            },
        },
         columns: columns,
         &quot;order&quot;: [[ dynSortingIndex, &quot; , &quot;'&quot; , &quot;asc&quot; , &quot;'&quot; , &quot; ]],
         rowGroup: {
             dataSrc: 88
         },
         dom: &quot; , &quot;'&quot; , &quot;Blfrtip&quot; , &quot;'&quot; , &quot;,
         buttons: [
        	 {
				text:   &quot; , &quot;'&quot; , &quot;&lt;label class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>|&amp;nbsp;&amp;nbsp; Export:&amp;nbsp;&lt;/label>&quot; , &quot;'&quot; , &quot;,
        		 autoClose: &quot; , &quot;'&quot; , &quot;false&quot; , &quot;'&quot; , &quot;,
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;
        	 },
             {
                 extend: &quot; , &quot;'&quot; , &quot;excelHtml5&quot; , &quot;'&quot; , &quot;,
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: &quot; , &quot;'&quot; , &quot;export&quot; , &quot;'&quot; , &quot;,
                 	 stripHtml: true
                 },
                 destroy:true,
                 filename: fileName,
				 text:   &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/excel.png&quot; title=&quot;Export to Excel&quot; align=&quot;middle&quot; class=&quot;ph-wo-vertical-align-middle ph-wo-cursor-pointer&quot;>&quot; , &quot;'&quot; , &quot;,
                 autoClose: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;,
                 extension: excelReportType,
                 createEmptyCells: true,
                 title : titleHeaderName,
                 messageBottom: function () {
                     return disclaimerText;
                 	},
                     customize: function ( xlsx ) {
						 var sheet = xlsx.xl.worksheets[&quot; , &quot;'&quot; , &quot;sheet1.xml&quot; , &quot;'&quot; , &quot;];
						$(&quot; , &quot;'&quot; , &quot;row c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;25&quot; , &quot;'&quot; , &quot; );
                        $(&quot; , &quot;'&quot; , &quot;c[r=A1] t&quot; , &quot;'&quot; , &quot;, sheet).text( titleHeaderName );
                        $(&quot; , &quot;'&quot; , &quot;row:first c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;27&quot; , &quot;'&quot; , &quot; ); 
                        $(&quot; , &quot;'&quot; , &quot;row:eq(1) c&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;27&quot; , &quot;'&quot; , &quot;);
                        
                        if(disclaimerText != null &amp;&amp; disclaimerText != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
                        {
                        	$(&quot; , &quot;'&quot; , &quot;row:last c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;55&quot; , &quot;'&quot; , &quot; );
	                        $(&quot; , &quot;'&quot; , &quot;row:last&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;customHeight&quot; , &quot;'&quot; , &quot;, 1);
	                        $(&quot; , &quot;'&quot; , &quot;row:last&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;ht&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;150&quot; , &quot;'&quot; , &quot;);
                        }
                      },
                      
                 button: [&quot; , &quot;'&quot; , &quot;excel&quot; , &quot;'&quot; , &quot;]

             },
             {
                 extend: &quot; , &quot;'&quot; , &quot;pdfHtml5&quot; , &quot;'&quot; , &quot;,
                 orientation: &quot; , &quot;'&quot; , &quot;landscape&quot; , &quot;'&quot; , &quot;,
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: &quot; , &quot;'&quot; , &quot;export&quot; , &quot;'&quot; , &quot;,
                 	 stripHtml: true
                 },
				 text:      &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/pdf.png&quot; title=&quot;Export to PDF&quot; align=&quot;middle&quot; class=&quot;ph-wo-vertical-align-middle ph-wo-cursor-pointer&quot;>&quot; , &quot;'&quot; , &quot;,
                 filename: fileName,
                 autoClose: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 footer:&quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 destroy:true,
                 messageBottom: function () {
                     return disclaimerText;
                  	},
               	 customize: function ( doc ) {
               		
               	   var cols = [];
               	   cols[0] = {text: &quot; , &quot;'&quot; , &quot;© JaMocha Tech Pvt. Ltd. 2009-2026&quot; , &quot;'&quot; , &quot;, alignment: &quot; , &quot;'&quot; , &quot;center&quot; , &quot;'&quot; , &quot;, fontSize:&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;, margin:[0, 0, 0, 0] };
               	   var objFooter = {};
               	   objFooter[&quot; , &quot;'&quot; , &quot;columns&quot; , &quot;'&quot; , &quot;] = cols;
               	   doc[&quot; , &quot;'&quot; , &quot;footer&quot; , &quot;'&quot; , &quot;]=objFooter;
               	   doc.styles.tableHeader.fillColor = &quot; , &quot;'&quot; , &quot;#006bb7&quot; , &quot;'&quot; , &quot;;
               	   doc.styles.tableHeader.alignment=&quot;left&quot;;
               	 	
               	   doc[&quot; , &quot;'&quot; , &quot;header&quot; , &quot;'&quot; , &quot;]=(function(page, pages) {
							return {
								columns: [
									{
										alignment: &quot; , &quot;'&quot; , &quot;left&quot; , &quot;'&quot; , &quot;,
										text: &quot; , &quot;'&quot; , &quot;ProHance&quot; , &quot;'&quot; , &quot;,
										fontSize: 12,
									},
									{
										alignment: &quot; , &quot;'&quot; , &quot;right&quot; , &quot;'&quot; , &quot;,
										fontSize: 8,
										text: [&quot; , &quot;'&quot; , &quot;Page &quot; , &quot;'&quot; , &quot;, { text: page.toString() }]
									}
								],
								margin: 20
							}
						});
	               	if(dynNumericAlignColArr.length > 0)
	               	{
	               		var rowCount = doc.content[1].table.body.length;
		               	for (i = 0; i &lt; rowCount; i++) 
		               	{
		               		dynNumericAlignColArr.forEach(function(number) {
		                   		doc.content[1].table.body[i][number].alignment = &quot; , &quot;'&quot; , &quot;right&quot; , &quot;'&quot; , &quot;;
		                   	  });
		               	}
	               	}
               	
	               doc.content[1].table.widths=	dynWidthArr;
               	
               		doc.content.forEach(function(item) {
	               		if (item.table)
	               		{
	               			item.layout = {
	               					hLineColor: function(i, node) { return &quot; , &quot;'&quot; , &quot;#000000&quot; , &quot;'&quot; , &quot;; },
	               					vLineColor: function(i, node) { return &quot; , &quot;'&quot; , &quot;#000000&quot; , &quot;'&quot; , &quot;; }
	               			};
	                    }
	               });
		           doc.styles.tableHeader = {
		        		   fontSize		: 11,
		        		   bold			: true,
		        		   color		: &quot; , &quot;'&quot; , &quot;#ffffff&quot; , &quot;'&quot; , &quot;,
		        		   fillColor	: &quot; , &quot;'&quot; , &quot;#006bb7&quot; , &quot;'&quot; , &quot;
			       };
               		doc.content.forEach(function(item) {
	               		if (item.table)
	               		{
	               			item.layout = {
	               					hLineColor: function(i, node) { return &quot; , &quot;'&quot; , &quot;#000000&quot; , &quot;'&quot; , &quot;; },
	               					vLineColor: function(i, node) { return &quot; , &quot;'&quot; , &quot;#000000&quot; , &quot;'&quot; , &quot;; }
	               			};
	                    }
	               });
		           doc.styles.tableHeader = {
		        		   fontSize		: 11,
		        		   bold			: true,
		        		   color		: &quot; , &quot;'&quot; , &quot;#ffffff&quot; , &quot;'&quot; , &quot;,
		        		   fillColor	: &quot; , &quot;'&quot; , &quot;#006bb7&quot; , &quot;'&quot; , &quot;
			       };
               	 },
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;,
                 pageSize : &quot; , &quot;'&quot; , &quot;LEGAL&quot; , &quot;'&quot; , &quot;,
                 titleAttr : &quot; , &quot;'&quot; , &quot;PDF&quot; , &quot;'&quot; , &quot;,
                 title : titleHeaderName,
                 button: [ &quot; , &quot;'&quot; , &quot;pdf&quot; , &quot;'&quot; , &quot; ]
             }
         ],
         drawCallback: function() {
       	  var hasRows = this.api().rows({ filter: &quot; , &quot;'&quot; , &quot;applied&quot; , &quot;'&quot; , &quot; }).data().length > 0;
       	 $(&quot; , &quot;'&quot; , &quot;.buttons-excel&quot; , &quot;'&quot; , &quot;)[0].style.visibility = hasRows ? &quot; , &quot;'&quot; , &quot;visible&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;;
       	 $(&quot; , &quot;'&quot; , &quot;.buttons-pdf&quot; , &quot;'&quot; , &quot;)[0].style.visibility = hasRows ? &quot; , &quot;'&quot; , &quot;visible&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;;
       	}
    } );
	
}

/* Build Data table for WO Work Type Attributes*/
function buildDataTableStructureForWorkTypeAttributesBulkUpload(jsonData, columns, jsonObjArr)
{
	fnUpdateProperties(jsonObjArr);
	var isReqSorting = true;
	if(dynSortingIndex &lt; 0)
	{
		isReqSorting  = false;
		dynSortingIndex = 0;
	}
	var excelReportType = $(&quot;#excelReportType&quot;).val();
	excelReportType = &quot;.&quot; + excelReportType.toLowerCase();
	var disclaimerText = $(&quot;#disclaimerText&quot;).val();
	titleHeaderName = replaceBackXMLEntities(dynTitleHeaderName);
	fileName = replaceBackXMLEntities(dynFileName);
	var table = $(&quot; , &quot;'&quot; , &quot;#CommonDataTableId&quot; , &quot;'&quot; , &quot;).DataTable( {
        data: 	jsonData,
        deferRender:    true,
        &quot;pageLength&quot;: dynPageLength,
        &quot;bDestroy&quot;: true,
        &quot;scrollX&quot;: true,
        &quot;ordering&quot; : isReqSorting,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px&quot; , &quot;'&quot; , &quot;> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100&quot; , &quot;'&quot; , &quot; >[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
                &quot;previous&quot;:   &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;
            },
        },
         columns: columns,
         &quot;order&quot;: [[ dynSortingIndex, sortingOrder ]],
         rowGroup: {
             dataSrc: 88
         },
         dom: &quot; , &quot;'&quot; , &quot;Blfrtip&quot; , &quot;'&quot; , &quot;,
         buttons: [
        	 {
        		 text:   &quot; , &quot;'&quot; , &quot;&lt;label class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>|&amp;nbsp;&amp;nbsp; Export:&amp;nbsp;&lt;/label>&quot; , &quot;'&quot; , &quot;,
        		 autoClose: &quot; , &quot;'&quot; , &quot;false&quot; , &quot;'&quot; , &quot;,
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;
        	 },
             {
                 extend: &quot; , &quot;'&quot; , &quot;excelHtml5&quot; , &quot;'&quot; , &quot;,
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: &quot; , &quot;'&quot; , &quot;export&quot; , &quot;'&quot; , &quot;,
                 	 stripHtml: true
                 },
                 destroy:true,
                 filename: fileName,
                 text:   &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/excel.png&quot; title=&quot;Export to Excel&quot; align=&quot;middle&quot; class=&quot;ph-wo-vertical-align-middle ph-wo-cursor-pointer&quot;>&quot; , &quot;'&quot; , &quot;,
                 autoClose: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;,
                 extension: excelReportType,
                 createEmptyCells: true,
                 title : titleHeaderName,
                 messageBottom: function () {
                     return disclaimerText;
                 	},
                     customize: function ( xlsx ) {
						 var sheet = xlsx.xl.worksheets[&quot; , &quot;'&quot; , &quot;sheet1.xml&quot; , &quot;'&quot; , &quot;];
						$(&quot; , &quot;'&quot; , &quot;row c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;25&quot; , &quot;'&quot; , &quot; );
                        $(&quot; , &quot;'&quot; , &quot;c[r=A1] t&quot; , &quot;'&quot; , &quot;, sheet).text( titleHeaderName );
                        $(&quot; , &quot;'&quot; , &quot;row:first c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;27&quot; , &quot;'&quot; , &quot; ); 
                        $(&quot; , &quot;'&quot; , &quot;row:eq(1) c&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;27&quot; , &quot;'&quot; , &quot;);
                        
                        if(disclaimerText != null &amp;&amp; disclaimerText != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
                        {
                        	$(&quot; , &quot;'&quot; , &quot;row:last c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;55&quot; , &quot;'&quot; , &quot; );
	                        $(&quot; , &quot;'&quot; , &quot;row:last&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;customHeight&quot; , &quot;'&quot; , &quot;, 1);
	                        $(&quot; , &quot;'&quot; , &quot;row:last&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;ht&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;150&quot; , &quot;'&quot; , &quot;);
                        }
                         
                      },
                      
                 button: [&quot; , &quot;'&quot; , &quot;excel&quot; , &quot;'&quot; , &quot;]

             },
             {
                 extend: &quot; , &quot;'&quot; , &quot;pdfHtml5&quot; , &quot;'&quot; , &quot;,
                 orientation: &quot; , &quot;'&quot; , &quot;landscape&quot; , &quot;'&quot; , &quot;,
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: &quot; , &quot;'&quot; , &quot;export&quot; , &quot;'&quot; , &quot;,
                 	 stripHtml: true
                 },
                 text:      &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/pdf.png&quot; title=&quot;Export to PDF&quot; align=&quot;middle&quot; class=&quot;ph-wo-vertical-align-middle ph-wo-cursor-pointer&quot;>&quot; , &quot;'&quot; , &quot;,
                 filename: fileName,
                 autoClose: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 footer:&quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 destroy:true,
                 messageBottom: function () {
                     return disclaimerText;
                  	},
               	 customize: function ( doc ) {
               		
               	   var cols = [];
               	   cols[0] = {text: &quot; , &quot;'&quot; , &quot;© JaMocha Tech Pvt. Ltd. 2009-2026&quot; , &quot;'&quot; , &quot;, alignment: &quot; , &quot;'&quot; , &quot;center&quot; , &quot;'&quot; , &quot;, fontSize:&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;, margin:[0, 0, 0, 0] };
               	   var objFooter = {};
               	   objFooter[&quot; , &quot;'&quot; , &quot;columns&quot; , &quot;'&quot; , &quot;] = cols;
               	   doc[&quot; , &quot;'&quot; , &quot;footer&quot; , &quot;'&quot; , &quot;]=objFooter;
               	   doc.styles.tableHeader.fillColor = &quot; , &quot;'&quot; , &quot;#006bb7&quot; , &quot;'&quot; , &quot;;
               	   doc.styles.tableHeader.alignment=&quot;left&quot;;
               	 	
               	   doc[&quot; , &quot;'&quot; , &quot;header&quot; , &quot;'&quot; , &quot;]=(function(page, pages) {
							return {
								columns: [
									{
										alignment: &quot; , &quot;'&quot; , &quot;left&quot; , &quot;'&quot; , &quot;,
										text: &quot; , &quot;'&quot; , &quot;ProHance&quot; , &quot;'&quot; , &quot;,
										fontSize: 12,
									},
									{
										alignment: &quot; , &quot;'&quot; , &quot;right&quot; , &quot;'&quot; , &quot;,
										fontSize: 8,
										text: [&quot; , &quot;'&quot; , &quot;Page &quot; , &quot;'&quot; , &quot;, { text: page.toString() }]
									}
								],
								margin: 20
							}
						});
	               	if(dynNumericAlignColArr.length > 0)
	               	{
	               		var rowCount = doc.content[1].table.body.length;
		               	for (i = 0; i &lt; rowCount; i++) 
		               	{
		               		dynNumericAlignColArr.forEach(function(number) {
		                   		doc.content[1].table.body[i][number].alignment = &quot; , &quot;'&quot; , &quot;right&quot; , &quot;'&quot; , &quot;;
		                   	  });
		               	}
	               	}
               	
	               doc.content[1].table.widths=	dynWidthArr;
	               doc.content.forEach(function(item) {
	               		if (item.table)
	               		{
	               			item.layout = {
	               					hLineColor: function(i, node) { return &quot; , &quot;'&quot; , &quot;#000000&quot; , &quot;'&quot; , &quot;; },
	               					vLineColor: function(i, node) { return &quot; , &quot;'&quot; , &quot;#000000&quot; , &quot;'&quot; , &quot;; }
	               			};
	                    }
	               });
		           doc.styles.tableHeader = {
		        		   fontSize		: 11,
		        		   bold			: true,
		        		   color		: &quot; , &quot;'&quot; , &quot;#ffffff&quot; , &quot;'&quot; , &quot;,
		        		   fillColor	: &quot; , &quot;'&quot; , &quot;#006bb7&quot; , &quot;'&quot; , &quot;
			       };
               	
               	 },
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;,
                 pageSize : &quot; , &quot;'&quot; , &quot;LEGAL&quot; , &quot;'&quot; , &quot;,
                 titleAttr : &quot; , &quot;'&quot; , &quot;PDF&quot; , &quot;'&quot; , &quot;,
                 title : titleHeaderName,
                 button: [ &quot; , &quot;'&quot; , &quot;pdf&quot; , &quot;'&quot; , &quot; ]
             }
         ],
         drawCallback: function() {
       	  var hasRows = this.api().rows({ filter: &quot; , &quot;'&quot; , &quot;applied&quot; , &quot;'&quot; , &quot; }).data().length > 0;
       	 $(&quot; , &quot;'&quot; , &quot;.buttons-excel&quot; , &quot;'&quot; , &quot;)[0].style.visibility = hasRows ? &quot; , &quot;'&quot; , &quot;visible&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;;
       	 $(&quot; , &quot;'&quot; , &quot;.buttons-pdf&quot; , &quot;'&quot; , &quot;)[0].style.visibility = hasRows ? &quot; , &quot;'&quot; , &quot;visible&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;;
       	}
    } );
	
}

/* Build Data Tabel along with SX - scrollX param */
function buildScrollXDataTableStructure(jsonData, pageLength, columns, sortingIndex, isScrollX)
{
	var isReqSorting = true;
	if(sortingIndex &lt; 0)
	{
		isReqSorting  = false;
		sortingIndex = 0;
	}
	var table = $(&quot; , &quot;'&quot; , &quot;#CommonDataTableId&quot; , &quot;'&quot; , &quot;).DataTable( {
        data: jsonData,
        deferRender:    true,
        &quot;pageLength&quot;: pageLength,
        &quot;ordering&quot; : isReqSorting,
        &quot;scrollX&quot;: isScrollX,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px&quot; , &quot;'&quot; , &quot;> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100&quot; , &quot;'&quot; , &quot; >[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
                &quot;previous&quot;:   &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;
            },
        },
         columns: columns,
         &quot;order&quot;: [[ sortingIndex, &quot; , &quot;'&quot; , &quot;asc&quot; , &quot;'&quot; , &quot; ]],
         dom: &quot; , &quot;'&quot; , &quot;Blfrtip&quot; , &quot;'&quot; , &quot;,
         buttons: [
            //Empty for no exports
         ]
    } );
}

/* Build Data Tabel with Defalt Data Table Export along with SX - scrollX param */
function buildDataTableStructureWithExportsArrScrollX(jsonData, columns, jsonObjArr, isScrollX)
{
	fnUpdateProperties(jsonObjArr);
	var isReqSorting = true;
	if(dynSortingIndex &lt; 0)
	{
		isReqSorting  = false;
		dynSortingIndex = 0;
	}
	var excelReportType = $(&quot;#excelReportType&quot;).val();
	excelReportType = &quot;.&quot; + excelReportType.toLowerCase();
	var disclaimerText = $(&quot;#disclaimerText&quot;).val();
	titleHeaderName = replaceBackXMLEntities(dynTitleHeaderName);
	fileName = replaceBackXMLEntities(dynFileName);
	var table = $(&quot; , &quot;'&quot; , &quot;#CommonDataTableId&quot; , &quot;'&quot; , &quot;).DataTable( {
        data: 	jsonData,
        deferRender:    true,
        &quot;pageLength&quot;: dynPageLength,
        &quot;bDestroy&quot;: true,
        &quot;scrollX&quot;: isScrollX,
        &quot;ordering&quot; : isReqSorting,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px&quot; , &quot;'&quot; , &quot;> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100&quot; , &quot;'&quot; , &quot; >[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
                &quot;previous&quot;:   &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;
            },
        },
         columns: columns,
         &quot;order&quot;: [[ dynSortingIndex, &quot; , &quot;'&quot; , &quot;asc&quot; , &quot;'&quot; , &quot; ]],
         rowGroup: {
             dataSrc: 88
         },
         dom: &quot; , &quot;'&quot; , &quot;Blfrtip&quot; , &quot;'&quot; , &quot;,
         buttons: [
        	 {
        		 text:   &quot; , &quot;'&quot; , &quot;&lt;label class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>|&amp;nbsp;&amp;nbsp; Export:&amp;nbsp;&lt;/label>&quot; , &quot;'&quot; , &quot;,
        		 autoClose: &quot; , &quot;'&quot; , &quot;false&quot; , &quot;'&quot; , &quot;,
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;
        	 },
             {
                 extend: &quot; , &quot;'&quot; , &quot;excelHtml5&quot; , &quot;'&quot; , &quot;,
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: &quot; , &quot;'&quot; , &quot;export&quot; , &quot;'&quot; , &quot;,
                 	 stripHtml: true,
                 	 format: {
                        header: function (data, columnIdx) {
                            if ((columnIdx === 6 &amp;&amp; columns.length == 8) || (columnIdx === 5 &amp;&amp; columns.length === 7))
                            {   
                                return &quot; , &quot;'&quot; , &quot;Part of Filter&quot; , &quot;'&quot; , &quot;;
                            }
                            return data;
                        }
                    }
                 },
                 destroy:true,
                 filename: fileName,
                 text:   &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/excel.png&quot; title=&quot;Export to Excel&quot; align=&quot;middle&quot; class=&quot;ph-wo-vertical-align-middle ph-wo-cursor-pointer&quot;>&quot; , &quot;'&quot; , &quot;,
                 autoClose: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;,
                 extension: excelReportType,
                 createEmptyCells: true,
                 title : titleHeaderName,
                 messageBottom: function () {
                     return disclaimerText;
                 	},
                     customize: function ( xlsx ) {
						 var sheet = xlsx.xl.worksheets[&quot; , &quot;'&quot; , &quot;sheet1.xml&quot; , &quot;'&quot; , &quot;];
						$(&quot; , &quot;'&quot; , &quot;row c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;25&quot; , &quot;'&quot; , &quot; );
                        $(&quot; , &quot;'&quot; , &quot;c[r=A1] t&quot; , &quot;'&quot; , &quot;, sheet).text( titleHeaderName );
                        $(&quot; , &quot;'&quot; , &quot;row:first c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;27&quot; , &quot;'&quot; , &quot; ); 
                        $(&quot; , &quot;'&quot; , &quot;row:eq(1) c&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;27&quot; , &quot;'&quot; , &quot;);
                        
                        if(disclaimerText != null &amp;&amp; disclaimerText != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
                        {
                        	$(&quot; , &quot;'&quot; , &quot;row:last c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;55&quot; , &quot;'&quot; , &quot; );
	                        $(&quot; , &quot;'&quot; , &quot;row:last&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;customHeight&quot; , &quot;'&quot; , &quot;, 1);
	                        $(&quot; , &quot;'&quot; , &quot;row:last&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;ht&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;150&quot; , &quot;'&quot; , &quot;);
                        }
                         
                      },
                      
                 button: [&quot; , &quot;'&quot; , &quot;excel&quot; , &quot;'&quot; , &quot;]

             },
             {
                 extend: &quot; , &quot;'&quot; , &quot;pdfHtml5&quot; , &quot;'&quot; , &quot;,
                 orientation: &quot; , &quot;'&quot; , &quot;landscape&quot; , &quot;'&quot; , &quot;,
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: &quot; , &quot;'&quot; , &quot;export&quot; , &quot;'&quot; , &quot;,
                 	stripHtml: true,
                 	format: {
                        header: function (data, columnIdx) {
                            if ((columnIdx === 6 &amp;&amp; columns.length == 8) || (columnIdx === 5 &amp;&amp; columns.length === 7))
                            {   
                                return &quot; , &quot;'&quot; , &quot;Part of Filter&quot; , &quot;'&quot; , &quot;;
                            }
                            return data;
                        }
                    }
                 },
                 text:      &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/pdf.png&quot; title=&quot;Export to PDF&quot; align=&quot;middle&quot; class=&quot;ph-wo-vertical-align-middle ph-wo-cursor-pointer&quot;>&quot; , &quot;'&quot; , &quot;,
                 filename: fileName,
                 autoClose: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 footer:&quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 destroy:true,
                 messageBottom: function () {
                     return disclaimerText;
                  	},
               	 customize: function ( doc ) {
               		
               	   var cols = [];
               	   cols[0] = {text: &quot; , &quot;'&quot; , &quot;© JaMocha Tech Pvt. Ltd. 2009-2026&quot; , &quot;'&quot; , &quot;, alignment: &quot; , &quot;'&quot; , &quot;center&quot; , &quot;'&quot; , &quot;, fontSize:&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;, margin:[0, 0, 0, 0] };
               	   var objFooter = {};
               	   objFooter[&quot; , &quot;'&quot; , &quot;columns&quot; , &quot;'&quot; , &quot;] = cols;
               	   doc[&quot; , &quot;'&quot; , &quot;footer&quot; , &quot;'&quot; , &quot;]=objFooter;
               	   doc.styles.tableHeader.fillColor = &quot; , &quot;'&quot; , &quot;#006bb7&quot; , &quot;'&quot; , &quot;;
               	   doc.styles.tableHeader.alignment=&quot;left&quot;;
               	 	
               	   doc[&quot; , &quot;'&quot; , &quot;header&quot; , &quot;'&quot; , &quot;]=(function(page, pages) {
							return {
								columns: [
									{
										alignment: &quot; , &quot;'&quot; , &quot;left&quot; , &quot;'&quot; , &quot;,
										text: &quot; , &quot;'&quot; , &quot;ProHance&quot; , &quot;'&quot; , &quot;,
										fontSize: 12,
									},
									{
										alignment: &quot; , &quot;'&quot; , &quot;right&quot; , &quot;'&quot; , &quot;,
										fontSize: 8,
										text: [&quot; , &quot;'&quot; , &quot;Page &quot; , &quot;'&quot; , &quot;, { text: page.toString() }]
									}
								],
								margin: 20
							}
						});
	               	if(dynNumericAlignColArr.length > 0)
	               	{
	               		var rowCount = doc.content[1].table.body.length;
		               	for (i = 0; i &lt; rowCount; i++) 
		               	{
		               		dynNumericAlignColArr.forEach(function(number) {
		                   		doc.content[1].table.body[i][number].alignment = &quot; , &quot;'&quot; , &quot;right&quot; , &quot;'&quot; , &quot;;
		                   	  });
		               	}
	               	}
               	
	               doc.content[1].table.widths=	dynWidthArr;
	               doc.content.forEach(function(item) {
	               		if (item.table)
	               		{
	               			item.layout = {
	               					hLineColor: function(i, node) { return &quot; , &quot;'&quot; , &quot;#000000&quot; , &quot;'&quot; , &quot;; },
	               					vLineColor: function(i, node) { return &quot; , &quot;'&quot; , &quot;#000000&quot; , &quot;'&quot; , &quot;; }
	               			};
	                    }
	               });
		           doc.styles.tableHeader = {
		        		   fontSize		: 11,
		        		   bold			: true,
		        		   color		: &quot; , &quot;'&quot; , &quot;#ffffff&quot; , &quot;'&quot; , &quot;,
		        		   fillColor	: &quot; , &quot;'&quot; , &quot;#006bb7&quot; , &quot;'&quot; , &quot;
			       };
               	
               	 },
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;,
                 pageSize : &quot; , &quot;'&quot; , &quot;LEGAL&quot; , &quot;'&quot; , &quot;,
                 titleAttr : &quot; , &quot;'&quot; , &quot;PDF&quot; , &quot;'&quot; , &quot;,
                 title : titleHeaderName,
                 button: [ &quot; , &quot;'&quot; , &quot;pdf&quot; , &quot;'&quot; , &quot; ]
             }
         ],
         drawCallback: function() {
        	 var api = this.api();
       	  	 var hasRows = api.rows({ filter: &quot; , &quot;'&quot; , &quot;applied&quot; , &quot;'&quot; , &quot; }).data().length > 0;
       	  	 $(&quot; , &quot;'&quot; , &quot;.buttons-excel&quot; , &quot;'&quot; , &quot;)[0].style.visibility = hasRows ? &quot; , &quot;'&quot; , &quot;visible&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;;
       	  	 $(&quot; , &quot;'&quot; , &quot;.buttons-pdf&quot; , &quot;'&quot; , &quot;)[0].style.visibility = hasRows ? &quot; , &quot;'&quot; , &quot;visible&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;;
       	  	 api.rows({ page: &quot; , &quot;'&quot; , &quot;current&quot; , &quot;'&quot; , &quot; }).every(function () {
	             var data = this.data();
	             if (data &amp;&amp; data.options) 
	             {
	                initSlimScroll(data.options.length, data.id);
	             }
          	 });
       	}
    } );
}

/* Build Data Tabel with Row Grouping, along with SX - scrollX param */
function buildDTRowGroupingWithoutExportSX(jsonData, pageLength, columns, sortingIndex, isScrollX, rowsGroup, ascnding)
{
	var isReqSorting = true;
	if(sortingIndex &lt; 0)
	{
		isReqSorting  = false;
		sortingIndex = 0;
	}
	var table = $(&quot; , &quot;'&quot; , &quot;#CommonDataTableId&quot; , &quot;'&quot; , &quot;).DataTable({
        data: jsonData,
        deferRender:    true,
        &quot;pageLength&quot;: pageLength,
        &quot;ordering&quot; : isReqSorting,
        &quot;scrollX&quot;: isScrollX,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px&quot; , &quot;'&quot; , &quot;> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100&quot; , &quot;'&quot; , &quot; >[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
                &quot;previous&quot;:   &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;
            },
        },
         columns: columns,
         columnDefs: [{
       	    &quot;defaultContent&quot;: &quot;&quot;,
       	    &quot;targets&quot;: &quot;_all&quot;
       	  }],
         &quot;order&quot;: [[ sortingIndex, ascnding ]],
         rowGroup: rowsGroup,
         dom: &quot; , &quot;'&quot; , &quot;Blfrtip&quot; , &quot;'&quot; , &quot;,
         &quot;bDestroy&quot;: true,
         buttons: [
            //Empty for no exports
         ]
    });
}

/* Build Data Tabel with Defalt Data Table, Export and Report Schedular Email along With SX - scrollX param */
function buildDTWithDTExportsWithRSEmailSX(jsonData, columns, jsonObjArr, isScrollX)
{
	fnUpdateProperties(jsonObjArr);
	var isReqSorting = true;
	if(dynSortingIndex &lt; 0)
	{
		isReqSorting  = false;
		dynSortingIndex = 0;
	}
	
	if(reportScheduleReq == &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;)
	{
		emailSpan = &quot; , &quot;'&quot; , &quot;&lt;span id=&quot;emailSpan&quot; class=&quot;ph-wo-cursor-pointer&quot; data-onclick=&quot;showSchedulerDialogOnClick&quot;>&lt;img src=&quot;/phworkoutput/images/email.png&quot; title=&quot;Email&quot; align=&quot;middle&quot; class=&quot;ph-wo-vertical-align-middle&quot;>&lt;/span>&quot; , &quot;'&quot; , &quot;;
		if($(&quot;#isAllowEmailExport&quot;).val() == &quot; , &quot;'&quot; , &quot;false&quot; , &quot;'&quot; , &quot;)
		{
			emailSpan = &quot; , &quot;'&quot; , &quot;&lt;span id=&quot;emailSpan&quot; class=&quot;ph-wo-cursor-pointer&quot; title=&quot;You do not have permission to download this content&quot;>&lt;img src=&quot;/phworkoutput/images/email.png&quot; title=&quot;Email&quot; align=&quot;middle&quot; class=&quot;ph-wo-vertical-align-middle&quot;>&lt;/span>&quot; , &quot;'&quot; , &quot;;
		}
	}
	
	var excelReportType = $(&quot;#excelReportType&quot;).val();
	excelReportType = &quot;.&quot; + excelReportType.toLowerCase();
	var disclaimerText = $(&quot;#disclaimerText&quot;).val();
	titleHeaderName = replaceBackXMLEntities(dynTitleHeaderName);
	fileName = replaceBackXMLEntities(dynFileName);
	var table = $(&quot; , &quot;'&quot; , &quot;#CommonDataTableId&quot; , &quot;'&quot; , &quot;).DataTable( {
        data: 	jsonData,
        deferRender:    true,
        &quot;pageLength&quot;: dynPageLength,
        &quot;bDestroy&quot;: true,
        &quot;scrollX&quot;: isScrollX,
        &quot;ordering&quot; : isReqSorting,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px&quot; , &quot;'&quot; , &quot;> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100&quot; , &quot;'&quot; , &quot; >[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
                &quot;previous&quot;:   &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;
            },
        },
         columns: columns,
         &quot;order&quot;: [[ dynSortingIndex, &quot; , &quot;'&quot; , &quot;asc&quot; , &quot;'&quot; , &quot; ]],
         rowGroup: {
             dataSrc: 88
         },
         dom: &quot; , &quot;'&quot; , &quot;Blfrtip&quot; , &quot;'&quot; , &quot;,
         buttons: [
        	 {
        		 text:   &quot; , &quot;'&quot; , &quot;&lt;label class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>|&amp;nbsp;&amp;nbsp; Export:&amp;nbsp;&lt;/label>&quot; , &quot;'&quot; , &quot;,
        		 autoClose: &quot; , &quot;'&quot; , &quot;false&quot; , &quot;'&quot; , &quot;,
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;
        	 },
        	 {
        		 text:   emailSpan,
        		 autoClose: &quot; , &quot;'&quot; , &quot;false&quot; , &quot;'&quot; , &quot;,
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;
        	 },
             {
                 extend: &quot; , &quot;'&quot; , &quot;excelHtml5&quot; , &quot;'&quot; , &quot;,
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: &quot; , &quot;'&quot; , &quot;export&quot; , &quot;'&quot; , &quot;,
                 	 stripHtml: true
                 },
                 destroy:true,
                 filename: fileName,
                 text:   &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/excel.png&quot; title=&quot;Export to Excel&quot; align=&quot;middle&quot; class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>&quot; , &quot;'&quot; , &quot;,
                 autoClose: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;,
                 extension: excelReportType,
                 createEmptyCells: true,
                 title : titleHeaderName,
                 messageBottom: function () {
                     return disclaimerText;
                 	},
                     customize: function ( xlsx ) {
						 var sheet = xlsx.xl.worksheets[&quot; , &quot;'&quot; , &quot;sheet1.xml&quot; , &quot;'&quot; , &quot;];
						$(&quot; , &quot;'&quot; , &quot;row c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;25&quot; , &quot;'&quot; , &quot; );
                        $(&quot; , &quot;'&quot; , &quot;c[r=A1] t&quot; , &quot;'&quot; , &quot;, sheet).text( titleHeaderName );
                        $(&quot; , &quot;'&quot; , &quot;row:first c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;27&quot; , &quot;'&quot; , &quot; ); 
                        $(&quot; , &quot;'&quot; , &quot;row:eq(1) c&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;27&quot; , &quot;'&quot; , &quot;);
                        
                        if(disclaimerText != null &amp;&amp; disclaimerText != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
                        {
                        	$(&quot; , &quot;'&quot; , &quot;row:last c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;55&quot; , &quot;'&quot; , &quot; );
	                        $(&quot; , &quot;'&quot; , &quot;row:last&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;customHeight&quot; , &quot;'&quot; , &quot;, 1);
	                        $(&quot; , &quot;'&quot; , &quot;row:last&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;ht&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;150&quot; , &quot;'&quot; , &quot;);
                        }
                      },
                      
                 button: [&quot; , &quot;'&quot; , &quot;excel&quot; , &quot;'&quot; , &quot;]
             },
             {
                 extend: &quot; , &quot;'&quot; , &quot;pdfHtml5&quot; , &quot;'&quot; , &quot;,
                 orientation: &quot; , &quot;'&quot; , &quot;landscape&quot; , &quot;'&quot; , &quot;,
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: &quot; , &quot;'&quot; , &quot;export&quot; , &quot;'&quot; , &quot;,
                 	 stripHtml: true
                 },
                 text:      &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/pdf.png&quot; title=&quot;Export to PDF&quot; align=&quot;middle&quot; class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>&quot; , &quot;'&quot; , &quot;,
                 filename: fileName,
                 autoClose: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 footer:&quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 destroy:true,
                 messageBottom: function () {
                     return disclaimerText;
                  	},
               	 customize: function ( doc ) {
               		
               	   var cols = [];
               	   cols[0] = {text: &quot; , &quot;'&quot; , &quot;© JaMocha Tech Pvt. Ltd. 2009-2026&quot; , &quot;'&quot; , &quot;, alignment: &quot; , &quot;'&quot; , &quot;center&quot; , &quot;'&quot; , &quot;, fontSize:&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;, margin:[0, 0, 0, 0] };
               	   var objFooter = {};
               	   objFooter[&quot; , &quot;'&quot; , &quot;columns&quot; , &quot;'&quot; , &quot;] = cols;
               	   doc[&quot; , &quot;'&quot; , &quot;footer&quot; , &quot;'&quot; , &quot;]=objFooter;
               	   doc.styles.tableHeader.fillColor = &quot; , &quot;'&quot; , &quot;#006bb7&quot; , &quot;'&quot; , &quot;;
               	   doc.styles.tableHeader.alignment=&quot;left&quot;;
               	 	
               	   doc[&quot; , &quot;'&quot; , &quot;header&quot; , &quot;'&quot; , &quot;]=(function(page, pages) {
							return {
								columns: [
									{
										alignment: &quot; , &quot;'&quot; , &quot;left&quot; , &quot;'&quot; , &quot;,
										text: &quot; , &quot;'&quot; , &quot;ProHance&quot; , &quot;'&quot; , &quot;,
										fontSize: 12,
									},
									{
										alignment: &quot; , &quot;'&quot; , &quot;right&quot; , &quot;'&quot; , &quot;,
										fontSize: 8,
										text: [&quot; , &quot;'&quot; , &quot;Page &quot; , &quot;'&quot; , &quot;, { text: page.toString() }]
									}
								],
								margin: 20
							}
						});
	               	if(dynNumericAlignColArr.length > 0)
	               	{
	               		var rowCount = doc.content[1].table.body.length;
		               	for (i = 0; i &lt; rowCount; i++) 
		               	{
		               		dynNumericAlignColArr.forEach(function(number) {
		                   		doc.content[1].table.body[i][number].alignment = &quot; , &quot;'&quot; , &quot;right&quot; , &quot;'&quot; , &quot;;
		                   	  });
		               	}
	               	}
               	
	               doc.content[1].table.widths=	dynWidthArr;
	               doc.content.forEach(function(item) {
	               		if (item.table)
	               		{
	               			item.layout = {
	               					hLineColor: function(i, node) { return &quot; , &quot;'&quot; , &quot;#000000&quot; , &quot;'&quot; , &quot;; },
	               					vLineColor: function(i, node) { return &quot; , &quot;'&quot; , &quot;#000000&quot; , &quot;'&quot; , &quot;; }
	               			};
	                    }
	               });
		           doc.styles.tableHeader = {
		        		   fontSize		: 11,
		        		   bold			: true,
		        		   color		: &quot; , &quot;'&quot; , &quot;#ffffff&quot; , &quot;'&quot; , &quot;,
		        		   fillColor	: &quot; , &quot;'&quot; , &quot;#006bb7&quot; , &quot;'&quot; , &quot;
			       };
               	
               	 },
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;,
                 pageSize : &quot; , &quot;'&quot; , &quot;LEGAL&quot; , &quot;'&quot; , &quot;,
                 titleAttr : &quot; , &quot;'&quot; , &quot;PDF&quot; , &quot;'&quot; , &quot;,
                 title : titleHeaderName,
                 button: [ &quot; , &quot;'&quot; , &quot;pdf&quot; , &quot;'&quot; , &quot; ]
             }
         ],
         drawCallback: function() {
       	  var hasRows = this.api().rows({ filter: &quot; , &quot;'&quot; , &quot;applied&quot; , &quot;'&quot; , &quot; }).data().length > 0;
       	 $(&quot; , &quot;'&quot; , &quot;.buttons-excel&quot; , &quot;'&quot; , &quot;)[0].style.visibility = hasRows ? &quot; , &quot;'&quot; , &quot;visible&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;;
       	 $(&quot; , &quot;'&quot; , &quot;.buttons-pdf&quot; , &quot;'&quot; , &quot;)[0].style.visibility = hasRows ? &quot; , &quot;'&quot; , &quot;visible&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;;
       	}
    } );
}

$(document).off(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;showSchedulerDialogOnClick&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;showSchedulerDialogOnClick&quot;]&quot; , &quot;'&quot; , &quot;, function() {
    showSchedulerDialog();
});

$(document).off(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnDataTableExportOnClick&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnDataTableExportOnClick&quot;]&quot; , &quot;'&quot; , &quot;, function() {
    var etype = $(this).data(&quot; , &quot;'&quot; , &quot;etype&quot; , &quot;'&quot; , &quot;);
    fnDataTableExport(etype);
});

/* Build Data Tabel With Custom Export (Custom Excel And PDF doExport) For Work Type Definition Screen */
function buildDataTableStructureWithExportsForWorkType(jsonData, columns, jsonObjArr)
{
	fnUpdateProperties(jsonObjArr);
	var isReqSorting = true;
	if(dynSortingIndex &lt; 0)
	{
		isReqSorting  = false;
		dynSortingIndex = 0;
	}
	
	var exportExcel = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	var exportPdf = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	var emailSpan = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	var excelIcon = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	var pdfIcon = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	
	excelIcon = &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/excel.png&quot; title=&quot;Export to Excel&quot; data-onclick=&quot;fnDataTableExportexpdf&quot; data-etype=&quot;excel&quot; align=&quot;middle&quot; class=&quot;ph-wo-vertical-align-middle ph-wo-cursor-pointer&quot;>&quot; , &quot;'&quot; , &quot;;
	
	if(recordSize > 10000)
	{
		pdfIcon = &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/pdf.png&quot; title=&quot;PDF file cannot be downloaded, as number of records more then 10000&quot; align=&quot;middle&quot; class=&quot;ph-wo-vertical-align-middle ph-wo-cursor-pointer&quot;>&quot; , &quot;'&quot; , &quot;;
		exportPdf = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	}	
	else
	{
		pdfIcon = &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/pdf.png&quot; title=&quot;Export to PDF&quot; data-onclick=&quot;fnDataTableExportexpdf&quot; data-etype=&quot;pdf&quot; class=&quot;ph-wo-vertical-align-middle ph-wo-cursor-pointer&quot;>&quot; , &quot;'&quot; , &quot;;
	}
	
	var excelReportType = $(&quot;#excelReportType&quot;).val();
	excelReportType = &quot;.&quot; + excelReportType.toLowerCase();
	
	titleHeaderName = replaceBackXMLEntities(dynTitleHeaderName);
	fileName = replaceBackXMLEntities(dynFileName);
	
	var disclaimerText = $(&quot;#disclaimerText&quot;).val();
	var table = $(&quot; , &quot;'&quot; , &quot;#CommonDataTableId&quot; , &quot;'&quot; , &quot;).DataTable( {
        data: 	jsonData,
        deferRender:    true,
        &quot;pageLength&quot;: dynPageLength,
        &quot;scrollX&quot;: true,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;ordering&quot; : isReqSorting,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
            &quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px&quot; , &quot;'&quot; , &quot;> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class=&quot; , &quot;'&quot; , &quot;dataTables_ExportsInfoFilter&quot; , &quot;'&quot; , &quot; >[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
            &quot;next&quot;:       &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
            &quot;previous&quot;:   &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;
            },
        },
         columns: columns,
         columnDefs: [{
      	    &quot;defaultContent&quot;: &quot;&quot;,
      	    &quot;targets&quot;: &quot;_all&quot;
      	  }],
         &quot;order&quot;: [[ dynSortingIndex, &quot; , &quot;'&quot; , &quot;asc&quot; , &quot;'&quot; , &quot; ]],
         dom: &quot; , &quot;'&quot; , &quot;Blfrtip&quot; , &quot;'&quot; , &quot;,
         buttons: [
        	 {
        	 	 text:   &quot; , &quot;'&quot; , &quot;&lt;label class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>|&amp;nbsp;&amp;nbsp; Export:&amp;nbsp;&lt;/label>&quot; , &quot;'&quot; , &quot;,
        		 autoClose: &quot; , &quot;'&quot; , &quot;false&quot; , &quot;'&quot; , &quot;,
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;
        	 },
        	 {
        		 text:   emailSpan,
        		 autoClose: &quot; , &quot;'&quot; , &quot;false&quot; , &quot;'&quot; , &quot;,
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;
        	 },
        	 {
                 extend: exportExcel,
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: &quot; , &quot;'&quot; , &quot;export&quot; , &quot;'&quot; , &quot;,
                 	 stripHtml: true
                 },
                 filename: fileName,
                 text:   excelIcon,
                 autoClose: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;,
                 extension: excelReportType,
                 createEmptyCells: true,
                 title : titleHeaderName,
                 messageBottom: function () {
                     return disclaimerText;
                 	},
                     customize: function ( xlsx ) {
						var sheet = xlsx.xl.worksheets[&quot; , &quot;'&quot; , &quot;sheet1.xml&quot; , &quot;'&quot; , &quot;];
						$(&quot; , &quot;'&quot; , &quot;row c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;25&quot; , &quot;'&quot; , &quot; );
                        $(&quot; , &quot;'&quot; , &quot;c[r=A1] t&quot; , &quot;'&quot; , &quot;, sheet).text( titleHeaderName );
                        $(&quot; , &quot;'&quot; , &quot;row:first c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;27&quot; , &quot;'&quot; , &quot; ); 
                        $(&quot; , &quot;'&quot; , &quot;row:eq(1) c&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;27&quot; , &quot;'&quot; , &quot;);
                        
                        if(disclaimerText != null &amp;&amp; disclaimerText != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
                        {
                        	$(&quot; , &quot;'&quot; , &quot;row:last c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;55&quot; , &quot;'&quot; , &quot; );
	                        $(&quot; , &quot;'&quot; , &quot;row:last&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;customHeight&quot; , &quot;'&quot; , &quot;, 1);
	                        $(&quot; , &quot;'&quot; , &quot;row:last&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;ht&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;150&quot; , &quot;'&quot; , &quot;);
                        }
                        
                      },
                 button: [&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;]
             },
             {
                 extend: exportPdf,
                 orientation: &quot; , &quot;'&quot; , &quot;landscape&quot; , &quot;'&quot; , &quot;,
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: &quot; , &quot;'&quot; , &quot;export&quot; , &quot;'&quot; , &quot;,
                 	 stripHtml: true
                 },
                 text:    pdfIcon ,
                 filename: fileName,
                 autoClose: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 footer:&quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 messageBottom: function () {
                     return disclaimerText;
                  	},
               	 customize: function ( doc ) {
               		
               	   var cols = [];
               	   cols[0] = {text: &quot; , &quot;'&quot; , &quot;© JaMocha Tech Pvt. Ltd. 2009-2026&quot; , &quot;'&quot; , &quot;, alignment: &quot; , &quot;'&quot; , &quot;center&quot; , &quot;'&quot; , &quot;, fontSize:&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;, margin:[0, 0, 0, 0] };
               	   var objFooter = {};
               	   objFooter[&quot; , &quot;'&quot; , &quot;columns&quot; , &quot;'&quot; , &quot;] = cols;
               	   doc[&quot; , &quot;'&quot; , &quot;footer&quot; , &quot;'&quot; , &quot;]=objFooter;
               	   doc.styles.tableHeader.alignment=&quot;left&quot;;
               	 	
               	   doc[&quot; , &quot;'&quot; , &quot;header&quot; , &quot;'&quot; , &quot;]=(function(page, pages) {
							return {
								columns: [
									{
										alignment: &quot; , &quot;'&quot; , &quot;left&quot; , &quot;'&quot; , &quot;,
										text: &quot; , &quot;'&quot; , &quot;ProHance&quot; , &quot;'&quot; , &quot;,
										fontSize: 12,
									},
									{
										alignment: &quot; , &quot;'&quot; , &quot;right&quot; , &quot;'&quot; , &quot;,
										fontSize: 8,
										text: [&quot; , &quot;'&quot; , &quot;Page &quot; , &quot;'&quot; , &quot;, { text: page.toString() }]
									}
								],
								margin: 20
							}
						});
               	   
	               	if(dynNumericAlignColArr.length > 0)
	               	{
	               		var rowCount = doc.content[1].table.body.length;
		               	for (i = 0; i &lt; rowCount; i++) 
		               	{
		               		dynNumericAlignColArr.forEach(function(number) {
		                   		doc.content[1].table.body[i][number].alignment = &quot; , &quot;'&quot; , &quot;right&quot; , &quot;'&quot; , &quot;;
		                   	  });
		               	}
               		}
	               	
	               	doc.content[1].table.widths = dynWidthArr;
	                doc.content.forEach(function(item) {
	                       if (item.table) {
	                           item.layout = {
	                               hLineColor: function(i, node) { return &quot; , &quot;'&quot; , &quot;#000000&quot; , &quot;'&quot; , &quot;; }, // Set the border color here
	                               vLineColor: function(i, node) { return &quot; , &quot;'&quot; , &quot;#000000&quot; , &quot;'&quot; , &quot;; }, // Set the border color here
	                           };
	                       }
	                   });
	                doc.styles.tableHeader = {
	            		   fontSize: 11,
	                       bold: true,
	                       color: &quot; , &quot;'&quot; , &quot;#ffffff&quot; , &quot;'&quot; , &quot;,  // Set font color
	                       fillColor: &quot; , &quot;'&quot; , &quot;#006bb7&quot; , &quot;'&quot; , &quot;
		               };
               	 },
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;,
                 pageSize : &quot; , &quot;'&quot; , &quot;LEGAL&quot; , &quot;'&quot; , &quot;,
                 titleAttr : &quot; , &quot;'&quot; , &quot;PDF&quot; , &quot;'&quot; , &quot;,
                 title : titleHeaderName,
                 button: [ &quot; , &quot;'&quot; , &quot;pdf&quot; , &quot;'&quot; , &quot; ]
             }
         ]
    } );
	setTimeout(function() {
		table.columns.adjust().draw();
	}, 20); 
}

function fnDataTableExportexpdf(data)
{
	var param = data.data(&quot; , &quot;'&quot; , &quot;param&quot; , &quot;'&quot; , &quot;);
	fnDataTableExport(param);
}

function buildDataTableStructureForServerSideWorkTypes(pageLength, columnArr, sortingIndex, dynTableId, dataCollectionId, menuItemId, userGroupNameIds)
{
	$.fn.DataTable.ext.pager.numbers_length = 5;
	$.fn.DataTable.ext.pager.customPager = function (page, pages) {
	    const buttons = [];
	    const startPage = Math.max(page - 1, 1);
	    const endPage = Math.min(startPage + 1, pages - 2);

	    buttons.push(0);
	    if (startPage > 2)
	    {
	        buttons.push(&quot; , &quot;'&quot; , &quot;ellipsis&quot; , &quot;'&quot; , &quot;);
	    }

	    for (let i = startPage; i &lt;= endPage; i++)
	    {
	        buttons.push(i);
	    }

	    if (endPage &lt; pages - 1)
	    {
	        buttons.push(&quot; , &quot;'&quot; , &quot;ellipsis&quot; , &quot;'&quot; , &quot;);
	    }
	    buttons.push(pages - 1);
	    return buttons;
	};
	
	var table = $(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot;+dynTableId).DataTable( {
	    &quot;processing&quot;: false,
        &quot;serverSide&quot;: true,
        &quot;ajax&quot;: {
        	&quot;async&quot;: true,
        	headers: {
      	        &quot; , &quot;'&quot; , &quot;X-Content-Security-Policy-Nonce&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;xZrLNGLeef26xpNnnF1igA==&quot; , &quot;'&quot; , &quot;
      	      },
            &quot;url&quot;: &quot;/phworkoutput/getWOWorkType.htm&quot;,
            &quot;type&quot;: &quot;POST&quot;,
            &quot;data&quot;: function (d) {
            	 d.groupName = userGroupNameIds;
                 d.paramGroupId = dataCollectionId;
                 d.menuItemId = menuItemId;
                return d;
            },
            &quot;beforeSend&quot;: function() {
            	fnRemoveClassWithSameStyle(&quot;#wtDiv&quot;, &quot; , &quot;'&quot; , &quot;ph-wo-height&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            	$(&quot; , &quot;'&quot; , &quot;#wtDiv&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;ph-wo-display-none&quot; , &quot;'&quot; , &quot;);
            	fnRemoveClassWithSameStyle(&quot;#loaderDiv&quot;, &quot; , &quot;'&quot; , &quot;ph-wo-height&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            	$(&quot; , &quot;'&quot; , &quot;#loaderDiv&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;ph-wo-display-none&quot; , &quot;'&quot; , &quot;);
            },
            &quot;complete&quot;: function() {
            	fnRemoveClassWithSameStyle(&quot;#wtDiv&quot;, &quot; , &quot;'&quot; , &quot;ph-wo-height&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            	$(&quot; , &quot;'&quot; , &quot;#wtDiv&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;ph-wo-display-none&quot; , &quot;'&quot; , &quot;);
            	fnRemoveClassWithSameStyle(&quot;#loaderDiv&quot;, &quot; , &quot;'&quot; , &quot;ph-wo-height&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            	$(&quot; , &quot;'&quot; , &quot;#loaderDiv&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;ph-wo-display-none&quot; , &quot;'&quot; , &quot;);
            	table.columns.adjust();
            	var totalRows = $(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot;+dynTableId +&quot; tbody tr&quot;).length;
            	// Remove the existing scroll and add it based on a condition
            	if ($(&quot; , &quot;'&quot; , &quot;.customScrollWT&quot; , &quot;'&quot; , &quot;).parent().hasClass(&quot; , &quot;'&quot; , &quot;slimScrollDiv&quot; , &quot;'&quot; , &quot;)) 
				{
            	    $(&quot; , &quot;'&quot; , &quot;.customScrollWT&quot; , &quot;'&quot; , &quot;).slimScroll({ destroy: true });
            	    $(&quot; , &quot;'&quot; , &quot;.customScrollWT&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;);
            	    $(&quot; , &quot;'&quot; , &quot;.customScrollWT&quot; , &quot;'&quot; , &quot;).siblings(&quot; , &quot;'&quot; , &quot;.slimScrollBar, .slimScrollRail&quot; , &quot;'&quot; , &quot;).remove();
            	}
            	if ( totalRows > 6 || $(&quot; , &quot;'&quot; , &quot;#wtDiv&quot; , &quot;'&quot; , &quot;).height() > 330)
            	{
            		initSlimScroll();
            	}
            	setSelectedValue();
            	fnCheckSelectAllParameter();
            }
        },
        &quot;pageLength&quot;: pageLength,
        &quot;scrollX&quot;: true,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;ordering&quot; : true,
        &quot;lengthMenu&quot;: lengthMenu,
        initComplete : function() {
        	var self = this.api();
			var $filter = $(&quot; , &quot;'&quot; , &quot;#CommonDataTableWorkTypeId_filter&quot; , &quot;'&quot; , &quot;);
			var $input = $filter.find(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).unbind();
            var $searchButton = $(&quot; , &quot;'&quot; , &quot;&lt;span title=&quot;Search&quot; class=&quot;custom-datatable-btn&quot;>&lt;i class=&quot;fa fa-search&quot;/>&lt;/span>&quot; , &quot;'&quot; , &quot;)
                .click(function(event) {
                	event.preventDefault();
                    self.search($input.val()).draw();
                });
            var $clearButton = $(&quot; , &quot;'&quot; , &quot;&lt;span title=&quot;Clear&quot; class=&quot;custom-datatable-btn&quot;>&lt;i class=&quot;fa fa-times&quot;/>&lt;/span>&quot; , &quot;'&quot; , &quot;)
                .click(function(event) {
                	event.preventDefault();
                    self.search(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;).draw();
                });
            $filter.append($searchButton, $clearButton);
        },
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px&quot; , &quot;'&quot; , &quot;> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100&quot; , &quot;'&quot; , &quot; >[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
            	&quot;previous&quot;:   &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;
            },
            renderer: &quot; , &quot;'&quot; , &quot;customPager&quot; , &quot;'&quot; , &quot;,
        },
        columns: columnArr.map((column, index) => {
            if (index === 0) {
                column.orderable = false;
            }
            return column;
        }),
        order: [[1, &quot; , &quot;'&quot; , &quot;asc&quot; , &quot;'&quot; , &quot;]],
        dom: &quot; , &quot;'&quot; , &quot;Blfrtip&quot; , &quot;'&quot; , &quot;,
        buttons: [
        	 ]
    } );
}

function buildDataTableStructureForDynamicTable(jsonData, pageLength, columnArr, sortingIndex, dynTableId)
{
	$.fn.DataTable.ext.pager.numbers_length = 5;
	$.fn.DataTable.ext.pager.customPager = function (page, pages) {
	    const buttons = [];
	    const startPage = Math.max(page - 1, 1);
	    const endPage = Math.min(startPage + 1, pages - 2);

	    buttons.push(0);
	    if (startPage > 2)
	    {
	        buttons.push(&quot; , &quot;'&quot; , &quot;ellipsis&quot; , &quot;'&quot; , &quot;);
	    }

	    for (let i = startPage; i &lt;= endPage; i++)
	    {
	        buttons.push(i);
	    }

	    if (endPage &lt; pages - 1)
	    {
	        buttons.push(&quot; , &quot;'&quot; , &quot;ellipsis&quot; , &quot;'&quot; , &quot;);
	    }
	    buttons.push(pages - 1);
	    return buttons;
	};
	
	var table = $(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot;+dynTableId).DataTable( {
		&quot;data&quot;: 	jsonData,
        &quot;deferRender&quot;:    true,
	    &quot;processing&quot;: false,
        &quot;pageLength&quot;: pageLength,
        &quot;scrollX&quot;: true,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;ordering&quot; : true,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px&quot; , &quot;'&quot; , &quot;> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100&quot; , &quot;'&quot; , &quot;>[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
                &quot;previous&quot;:   &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;
            },
            renderer: &quot; , &quot;'&quot; , &quot;customPager&quot; , &quot;'&quot; , &quot;,
        },
        &quot;drawCallback&quot;: function() {
        	var totalRows = $(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot;+dynTableId +&quot; tbody tr&quot;).length;
        	if ( totalRows > 5 )
        	{
        		initSlimScroll();
        	}
        },
        columns: columnArr.map((column, index) => {
            if (index === 0) {
                column.orderable = false;
            }
            return column;
        }),
        order: [[1, &quot; , &quot;'&quot; , &quot;asc&quot; , &quot;'&quot; , &quot;]],
        dom: &quot; , &quot;'&quot; , &quot;Blfrtip&quot; , &quot;'&quot; , &quot;,
        buttons: []
        
    });
}

function buildDataTableStructureWithReportDataForWOWorkType(columns, jsonObjArr, menuItemId, persmissionObject)
{
	fnUpdateProperties(jsonObjArr);
	
	const dataKeys = columns.map(column => column.data);
	const dataKeysString = dataKeys.join(&quot;, &quot;);

	var isReqSorting = true;
	if(dynSortingIndex &lt; 0)
	{
		isReqSorting  = false;
		dynSortingIndex = 0;
	}
	var exportExcel = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	var exportPdf = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	var emailSpan = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	var excelIcon = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	var pdfIcon = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	
	
	excelIcon = &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/excel.png&quot; title=&quot;Export to Excel&quot; data-onclick=&quot;fnDataTableExportexpdf&quot; data-etype=&quot;excel&quot; align=&quot;middle&quot; class=&quot;ph-wo-vertical-align-middle ph-wo-cursor-pointer&quot;>&quot; , &quot;'&quot; , &quot;;
	
	if(recordSize > 10000)
	{
		pdfIcon = &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/pdf.png&quot; title=&quot;PDF file cannot be downloaded, as number of records more then 10000&quot; align=&quot;middle&quot; class=&quot;ph-wo-vertical-align-middle ph-wo-cursor-pointer&quot;>&quot; , &quot;'&quot; , &quot;;
		exportPdf = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	}	
	else
	{
		pdfIcon = &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/pdf.png&quot; title=&quot;Export to PDF&quot; data-onclick=&quot;fnDataTableExportexpdf&quot; data-etype=&quot;pdf&quot; class=&quot;ph-wo-vertical-align-middle ph-wo-cursor-pointer&quot;>&quot; , &quot;'&quot; , &quot;;
	}
	
	var excelReportType = $(&quot;#excelReportType&quot;).val();
	excelReportType = &quot;.&quot; + excelReportType.toLowerCase();
	
	titleHeaderName = replaceBackXMLEntities(dynTitleHeaderName);
	fileName = replaceBackXMLEntities(dynFileName);
	var counter = 1;
	var disclaimerText = $(&quot;#disclaimerText&quot;).val();
	var table = $(&quot; , &quot;'&quot; , &quot;#CommonDataTableId&quot; , &quot;'&quot; , &quot;).DataTable( {
	    &quot;processing&quot;: false,
        &quot;serverSide&quot;: true,
        &quot;ajax&quot;: {
        	&quot;async&quot;: true,
        	
            &quot;url&quot;: &quot;/phworkoutput/WOWorkTypeDetailTableAjax.htm&quot;,
            &quot;type&quot;: &quot;POST&quot;,
            headers	: {&quot; , &quot;'&quot; , &quot;X-Content-Security-Policy-Nonce&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;xZrLNGLeef26xpNnnF1igA==&quot; , &quot;'&quot; , &quot;},
             &quot;data&quot;: function (d) 
             {
            	 d.menuItemId = menuItemId;
                 d.persmissionObject = persmissionObject;
            	 d.selectedColumns = dataKeysString;
                return d;
            }
        },
        &quot;pageLength&quot;: dynPageLength,
        &quot;scrollX&quot;: true,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;ordering&quot; : isReqSorting,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px&quot; , &quot;'&quot; , &quot;> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100&quot; , &quot;'&quot; , &quot; >[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
            	&quot;previous&quot;:   &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;
            },
        },
         columns: columns,
         columnDefs: [{
     	    &quot;defaultContent&quot;: &quot;&quot;,
     	    &quot;targets&quot;: &quot;_all&quot;
     	  }],
         &quot;order&quot;: [[ dynSortingIndex, sortingOrder ]],
         dom: &quot; , &quot;'&quot; , &quot;Blfrtip&quot; , &quot;'&quot; , &quot;,
         buttons: [
        	 {
        		 text:   &quot; , &quot;'&quot; , &quot;&lt;label class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>|&amp;nbsp;&amp;nbsp; Export:&amp;nbsp;&lt;/label>&quot; , &quot;'&quot; , &quot;,
        		 autoClose: &quot; , &quot;'&quot; , &quot;false&quot; , &quot;'&quot; , &quot;,
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;
        	 },
        	 {
        		 text:   emailSpan,
        		 autoClose: &quot; , &quot;'&quot; , &quot;false&quot; , &quot;'&quot; , &quot;,
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;
        	 },
        	 {
                 extend: exportExcel,
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: &quot; , &quot;'&quot; , &quot;export&quot; , &quot;'&quot; , &quot;,
                 	 stripHtml: true
                 },
                 filename: fileName,
                 text:   excelIcon,
                 autoClose: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;,
                 extension: excelReportType,
                 createEmptyCells: true,
                 title : titleHeaderName,
                 messageBottom: function () {
                     return disclaimerText;
                 	},
                     customize: function ( xlsx ) {
						var sheet = xlsx.xl.worksheets[&quot; , &quot;'&quot; , &quot;sheet1.xml&quot; , &quot;'&quot; , &quot;];
						$(&quot; , &quot;'&quot; , &quot;row c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;25&quot; , &quot;'&quot; , &quot; );
                        $(&quot; , &quot;'&quot; , &quot;c[r=A1] t&quot; , &quot;'&quot; , &quot;, sheet).text( titleHeaderName );
                        $(&quot; , &quot;'&quot; , &quot;row:first c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;27&quot; , &quot;'&quot; , &quot; ); 
                        $(&quot; , &quot;'&quot; , &quot;row:eq(1) c&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;27&quot; , &quot;'&quot; , &quot;);
                        
                        if(disclaimerText != null &amp;&amp; disclaimerText != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
                        {
                        	$(&quot; , &quot;'&quot; , &quot;row:last c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;55&quot; , &quot;'&quot; , &quot; );
	                        $(&quot; , &quot;'&quot; , &quot;row:last&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;customHeight&quot; , &quot;'&quot; , &quot;, 1);
	                        $(&quot; , &quot;'&quot; , &quot;row:last&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;ht&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;150&quot; , &quot;'&quot; , &quot;);
                        }
                        
                      },
                 button: [&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;]
             },
             {
                 extend: exportPdf,
                 orientation: &quot; , &quot;'&quot; , &quot;landscape&quot; , &quot;'&quot; , &quot;,
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: &quot; , &quot;'&quot; , &quot;export&quot; , &quot;'&quot; , &quot;,
                 	 stripHtml: true
                 },
                 text:    pdfIcon ,
                 filename: fileName,
                 autoClose: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 footer:&quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 messageBottom: function () {
                     return disclaimerText;
                  	},
               	 customize: function ( doc ) {
               		
               	   var cols = [];
               	   cols[0] = {text: &quot; , &quot;'&quot; , &quot;© JaMocha Tech Pvt. Ltd. 2009-2026&quot; , &quot;'&quot; , &quot;, alignment: &quot; , &quot;'&quot; , &quot;center&quot; , &quot;'&quot; , &quot;, fontSize:&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;, margin:[0, 0, 0, 0] };
               	   var objFooter = {};
               	   objFooter[&quot; , &quot;'&quot; , &quot;columns&quot; , &quot;'&quot; , &quot;] = cols;
               	   doc[&quot; , &quot;'&quot; , &quot;footer&quot; , &quot;'&quot; , &quot;]=objFooter;
               	   doc.styles.tableHeader.alignment=&quot;left&quot;;
               	 	
               	   doc[&quot; , &quot;'&quot; , &quot;header&quot; , &quot;'&quot; , &quot;]=(function(page, pages) {
							return {
								columns: [
									{
										alignment: &quot; , &quot;'&quot; , &quot;left&quot; , &quot;'&quot; , &quot;,
										text: &quot; , &quot;'&quot; , &quot;ProHance&quot; , &quot;'&quot; , &quot;,
										fontSize: 12,
									},
									{
										alignment: &quot; , &quot;'&quot; , &quot;right&quot; , &quot;'&quot; , &quot;,
										fontSize: 8,
										text: [&quot; , &quot;'&quot; , &quot;Page &quot; , &quot;'&quot; , &quot;, { text: page.toString() }]
									}
								],
								margin: 20
							}
						});
               	   
	               	if(dynNumericAlignColArr.length > 0)
	               	{
	               		var rowCount = doc.content[1].table.body.length;
		               	for (i = 0; i &lt; rowCount; i++) 
		               	{
		               		dynNumericAlignColArr.forEach(function(number) {
		                   		doc.content[1].table.body[i][number].alignment = &quot; , &quot;'&quot; , &quot;right&quot; , &quot;'&quot; , &quot;;
		                   	  });
		               	}
               		}
	               	
	               	doc.content[1].table.widths = dynWidthArr;
	                doc.content.forEach(function(item) {
	                       if (item.table) {
	                           item.layout = {
	                               hLineColor: function(i, node) { return &quot; , &quot;'&quot; , &quot;#000000&quot; , &quot;'&quot; , &quot;; }, // Set the border color here
	                               vLineColor: function(i, node) { return &quot; , &quot;'&quot; , &quot;#000000&quot; , &quot;'&quot; , &quot;; }, // Set the border color here
	                           };
	                       }
	                   });
	                doc.styles.tableHeader = {
	            		   fontSize: 11,
	                       bold: true,
	                       color: &quot; , &quot;'&quot; , &quot;#ffffff&quot; , &quot;'&quot; , &quot;,  // Set font color
	                       fillColor: &quot; , &quot;'&quot; , &quot;#006bb7&quot; , &quot;'&quot; , &quot;
		               };
               	 },
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;,
                 pageSize : &quot; , &quot;'&quot; , &quot;LEGAL&quot; , &quot;'&quot; , &quot;,
                 titleAttr : &quot; , &quot;'&quot; , &quot;PDF&quot; , &quot;'&quot; , &quot;,
                 title : titleHeaderName,
                 button: [ &quot; , &quot;'&quot; , &quot;pdf&quot; , &quot;'&quot; , &quot; ]
             }
         ]
    } );
	setTimeout(function() {
		table.columns.adjust().draw();
	}, 20); 
}

function buildDataTableStructureForDashboardWorkType(pageLength, columnArr, sortingIndex, dynTableId, dataParam, urlStr)
{
	if ($.fn.dataTable.isDataTable(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot;+dynTableId))
	{
		var existingTable = $(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot;+dynTableId);
		existingTable.DataTable().clear().destroy();
		existingTable.off().empty();
	}
	$.fn.DataTable.ext.pager.numbers_length = 5;
	$.fn.DataTable.ext.pager.customPager = function (page, pages) {
	    const buttons = [];
	    const startPage = Math.max(page - 1, 1);
	    const endPage = Math.min(startPage + 1, pages - 2);

	    buttons.push(0);
	    if (startPage > 2)
	    {
	        buttons.push(&quot; , &quot;'&quot; , &quot;ellipsis&quot; , &quot;'&quot; , &quot;);
	    }

	    for (let i = startPage; i &lt;= endPage; i++)
	    {
	        buttons.push(i);
	    }

	    if (endPage &lt; pages - 1)
	    {
	        buttons.push(&quot; , &quot;'&quot; , &quot;ellipsis&quot; , &quot;'&quot; , &quot;);
	    }
	    buttons.push(pages - 1);
	    return buttons;
	};
	
	var table = $(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot;+dynTableId).DataTable( {
	    &quot;processing&quot;: false,
        &quot;serverSide&quot;: true,
        &quot;ajax&quot;: {
        	&quot;async&quot;: true,
            &quot;url&quot;: urlStr,
            &quot;type&quot;: &quot;POST&quot;,
            headers	: {&quot; , &quot;'&quot; , &quot;X-Content-Security-Policy-Nonce&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;xZrLNGLeef26xpNnnF1igA==&quot; , &quot;'&quot; , &quot;},
            &quot;data&quot;: dataParam,
            &quot;beforeSend&quot;: function() {
            	fnRemoveClassWithSameStyle(&quot;.content-loading&quot;, &quot; , &quot;'&quot; , &quot;ph-wo-display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            	$(&quot;.content-loading&quot;).removeClass(&quot;ph-wo-display-none&quot;);
            	fnRemoveClassWithSameStyle(&quot;#specificWorktypeContent&quot;, &quot; , &quot;'&quot; , &quot;ph-wo-display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
				$(&quot;#specificWorktypeContent&quot;).addClass(&quot;ph-wo-display-none&quot;);
            },
            &quot;complete&quot;: function() {
            	setSelectedValue();
            	fnRemoveClassWithSameStyle(&quot;.content-loading&quot;, &quot; , &quot;'&quot; , &quot;ph-wo-display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            	$(&quot;.content-loading&quot;).addClass(&quot;ph-wo-display-none&quot;);
            	fnRemoveClassWithSameStyle(&quot;#specificWorktypeContent&quot;, &quot; , &quot;'&quot; , &quot;ph-wo-display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
				$(&quot;#specificWorktypeContent&quot;).removeClass(&quot;ph-wo-display-none&quot;);
			  	table.columns.adjust();
			  	initSlimScroll();
            }
        },
        initComplete : function() {
        	$(&quot; , &quot;'&quot; , &quot;.dataTables_filter&quot; , &quot;'&quot; , &quot;).removeClass(&quot;category-search&quot;);
		  	$(&quot; , &quot;'&quot; , &quot;.dataTables_filter&quot; , &quot;'&quot; , &quot;).addClass(&quot;workType-search&quot;);
        	var self = this.api();
            var $filter = $(&quot; , &quot;'&quot; , &quot;.workType-search&quot; , &quot;'&quot; , &quot;);
            var $input = $filter.find(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).unbind();
            var $searchButton = $(&quot; , &quot;'&quot; , &quot;&lt;span title=&quot;Search&quot; class=&quot;custom-datatable-btn&quot;>&lt;i class=&quot;fa fa-search&quot;/>&lt;/span>&quot; , &quot;'&quot; , &quot;)
                .click(function(event) {
                	event.preventDefault();
                	if($input.val().trim() != null &amp;&amp; $input.val().trim() != &quot;&quot;)
                	{
                    	self.search($input.val()).draw();
                    	initSlimScroll();
                	}
                });
            var $clearButton = $(&quot; , &quot;'&quot; , &quot;&lt;span title=&quot;Clear&quot; class=&quot;custom-datatable-btn&quot;>&lt;i class=&quot;fa fa-times&quot;/>&lt;/span>&quot; , &quot;'&quot; , &quot;)
                .click(function(event) {
                	event.preventDefault();
                    self.search(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;).draw();
                });
            if (!$filter.has($searchButton).length) {
            	  $filter.append($searchButton);
            	}

            	if (!$filter.has($clearButton).length) {
            	  $filter.append($clearButton);
            	}
        },
        &quot;pageLength&quot;: pageLength,
        &quot;scrollX&quot;: true,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;scrollCollapse&quot;: true,
        &quot;ordering&quot; : true,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px&quot; , &quot;'&quot; , &quot;> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100&quot; , &quot;'&quot; , &quot; >[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
            	&quot;previous&quot;:   &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;
            },
            renderer: &quot; , &quot;'&quot; , &quot;customPager&quot; , &quot;'&quot; , &quot;,
        },
		columns: columnArr.map((column, index) => {
			column.orderable = index !== 0;
            return column;
        }),
        order: [[1, &quot; , &quot;'&quot; , &quot;asc&quot; , &quot;'&quot; , &quot;]],
        dom: &quot; , &quot;'&quot; , &quot;Blfrtip&quot; , &quot;'&quot; , &quot;,
        buttons: [],
    });
}

function buildDataTableStructureForWorkTypeMapping(columns, jsonObjArr, menuItemId, persmissionObject)
{
	fnUpdateProperties(jsonObjArr);
	
	const dataKeys = columns.map(column => column.data);
	const dataKeysString = dataKeys.join(&quot;, &quot;);

	var isReqSorting = true;
	if(dynSortingIndex &lt; 0)
	{
		isReqSorting  = false;
		dynSortingIndex = 0;
	}
	titleHeaderName = replaceBackXMLEntities(dynTitleHeaderName);
	fileName = replaceBackXMLEntities(dynFileName);
	var counter = 1;
	var disclaimerText = $(&quot;#disclaimerText&quot;).val();
	var table = $(&quot; , &quot;'&quot; , &quot;#CommonDataTableId&quot; , &quot;'&quot; , &quot;).DataTable( {
	    &quot;processing&quot;: false,
        &quot;serverSide&quot;: true,
        &quot;ajax&quot;: {
        	&quot;async&quot;: true,
        	
            &quot;url&quot;: &quot;/phworkoutput/WorkTypeMappingDetailTableAjax.htm&quot;,
            &quot;type&quot;: &quot;POST&quot;,
            headers	: {&quot; , &quot;'&quot; , &quot;X-Content-Security-Policy-Nonce&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;xZrLNGLeef26xpNnnF1igA==&quot; , &quot;'&quot; , &quot;},
             &quot;data&quot;: function (d) 
             {
            	 d.menuItemId = menuItemId;
                 d.persmissionObject = persmissionObject;
            	 d.selectedColumns = dataKeysString;
                return d;
            }
        },
        &quot;pageLength&quot;: dynPageLength,
        &quot;scrollX&quot;: true,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;ordering&quot; : isReqSorting,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px&quot; , &quot;'&quot; , &quot;> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100&quot; , &quot;'&quot; , &quot; >[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
            	&quot;previous&quot;:   &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;
            },
        },
         columns: columns,
         columnDefs: [{
     	    &quot;defaultContent&quot;: &quot;&quot;,
     	    &quot;targets&quot;: &quot;_all&quot;
     	  }],
         &quot;order&quot;: [[ dynSortingIndex, sortingOrder ]],
         dom: &quot; , &quot;'&quot; , &quot;Blfrtip&quot; , &quot;'&quot; , &quot;,
         buttons: [
        	 ],
    });
	setTimeout(function() {
		table.columns.adjust().draw();
	}, 20); 
}

function buildDataTableStructureForWOCategories(pageLength, columnArr, sortingIndex, dynTableId, dataParam)
{
	$.fn.DataTable.ext.pager.numbers_length = 5;
	$.fn.DataTable.ext.pager.customPager = function (page, pages) {
	    const buttons = [];
	    const startPage = Math.max(page - 1, 1);
	    const endPage = Math.min(startPage + 1, pages - 2);

	    buttons.push(0);
	    if (startPage > 2)
	    {
	        buttons.push(&quot; , &quot;'&quot; , &quot;ellipsis&quot; , &quot;'&quot; , &quot;);
	    }

	    for (let i = startPage; i &lt;= endPage; i++)
	    {
	        buttons.push(i);
	    }

	    if (endPage &lt; pages - 1)
	    {
	        buttons.push(&quot; , &quot;'&quot; , &quot;ellipsis&quot; , &quot;'&quot; , &quot;);
	    }
	    buttons.push(pages - 1);
	    return buttons;
	};
	
	var table = $(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot;+dynTableId).DataTable( {
	    &quot;processing&quot;: false,
        &quot;serverSide&quot;: true,
        &quot;ajax&quot;: {
        	&quot;async&quot;: true,
            &quot;url&quot;: &quot;/phworkoutput/getWOCategories.htm&quot;,
            &quot;type&quot;: &quot;POST&quot;,
            headers	: {&quot; , &quot;'&quot; , &quot;X-Content-Security-Policy-Nonce&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;xZrLNGLeef26xpNnnF1igA==&quot; , &quot;'&quot; , &quot;},
            &quot;data&quot;: dataParam,
            &quot;beforeSend&quot;: function() {
            	fnRemoveClassWithSameStyle(&quot;.content-loading&quot;, &quot; , &quot;'&quot; , &quot;ph-wo-display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            	$(&quot;.content-loading&quot;).removeClass(&quot;ph-wo-display-none&quot;);
            	$(&quot;#specificCategoryContent&quot;).removeClass(&quot;ph-wo-display-none&quot;);
            	fnRemoveClassWithSameStyle(&quot;.specificCategoryContent&quot;, &quot; , &quot;'&quot; , &quot;ph-wo-display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            	$(&quot;.specificCategoryContent&quot;).addClass(&quot;ph-wo-display-none&quot;);
				$(&quot;#specificCategoryContent&quot;).removeClass(&quot;ph-wo-display-none&quot;);
				fnRemoveClassWithSameStyle(&quot;.specificCategoryContent&quot;, &quot; , &quot;'&quot; , &quot;ph-wo-display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
				$(&quot;#specificCategoryContent&quot;).addClass(&quot;ph-wo-display-none&quot;);
            },
            &quot;complete&quot;: function() {
            	setSelectedCategoryValue();
            	fnRemoveClassWithSameStyle(&quot;.content-loading&quot;, &quot; , &quot;'&quot; , &quot;ph-wo-display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            	$(&quot;.content-loading&quot;).addClass(&quot;ph-wo-display-none&quot;);
            	fnRemoveClassWithSameStyle(&quot;#specificCategoryContent&quot;, &quot; , &quot;'&quot; , &quot;ph-wo-display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
				$(&quot;#specificCategoryContent&quot;).removeClass(&quot;ph-wo-display-none&quot;);
				fnRemoveClassWithSameStyle(&quot;.specificCategoryContent&quot;, &quot; , &quot;'&quot; , &quot;ph-wo-display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
			  	table.columns.adjust();
			  	initCategorySlimScroll();
            }
        },
        initComplete : function() {
        	$(&quot; , &quot;'&quot; , &quot;.dataTables_filter&quot; , &quot;'&quot; , &quot;).removeClass(&quot;workType-search&quot;);
		  	$(&quot; , &quot;'&quot; , &quot;.dataTables_filter&quot; , &quot;'&quot; , &quot;).addClass(&quot;category-search&quot;);
        	var self = this.api();
            var $filter = $(&quot; , &quot;'&quot; , &quot;.category-search&quot; , &quot;'&quot; , &quot;);
            var $input = $filter.find(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).unbind();
            var $searchButton = $(&quot; , &quot;'&quot; , &quot;&lt;span title=&quot;Search&quot; class=&quot;custom-datatable-btn&quot;>&lt;i class=&quot;fa fa-search category&quot;/>&lt;/span>&quot; , &quot;'&quot; , &quot;)
                .click(function(event) {
                	event.preventDefault();
                	if($input.val().trim() != null &amp;&amp; $input.val().trim() != &quot;&quot;)
                	{
                    	self.search($input.val()).draw();
                	}
                });
            var $clearButton = $(&quot; , &quot;'&quot; , &quot;&lt;span title=&quot;Clear&quot; class=&quot;custom-datatable-btn&quot;>&lt;i class=&quot;fa fa-times category&quot;/>&lt;/span>&quot; , &quot;'&quot; , &quot;)
                .click(function(event) {
                	event.preventDefault();
                    self.search(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;).draw();
                });
            if (!$filter.has($searchButton).length) 
            {
            	  $filter.append($searchButton);
            }

            if (!$filter.has($clearButton).length) 
            {
            	  $filter.append($clearButton);
            }
        },
        &quot;pageLength&quot;: pageLength,
        &quot;scrollX&quot;: true,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;scrollCollapse&quot;: true,
        &quot;ordering&quot; : true,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px&quot; , &quot;'&quot; , &quot;> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100&quot; , &quot;'&quot; , &quot; >[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
            	&quot;previous&quot;:   &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;
            },
            renderer: &quot; , &quot;'&quot; , &quot;customPager&quot; , &quot;'&quot; , &quot;,
        },
		columns: columnArr.map((column, index) => {
			column.orderable = index !== 0;
            return column;
        }),
        order: [[1, &quot; , &quot;'&quot; , &quot;asc&quot; , &quot;'&quot; , &quot;]],
        dom: &quot; , &quot;'&quot; , &quot;Blfrtip&quot; , &quot;'&quot; , &quot;,
        buttons: [],
    });
}

/* Build Data Tabel with Default Data Table Export along with JSONArray param*/
function buildDataTableStructureForWOCategoryArr(columns, jsonObjArr, menuItemId, persmissionObject)
{
	
	fnUpdateProperties(jsonObjArr);
	const dataKeys = columns.map(column => column.data);
	const dataKeysString = dataKeys.join(&quot;, &quot;);
	var isReqSorting = true;
	if(dynSortingIndex &lt; 0)
	{
		isReqSorting  = false;
		dynSortingIndex = 0;
	}
	
	var exportExcel = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	var exportPdf = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	var emailSpan = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	var excelIcon = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	var pdfIcon = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	
	excelIcon = &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/excel.png&quot; title=&quot;Export to Excel&quot; data-onclick=&quot;fnDataTableExportexpdf&quot; data-etype = &quot;excel&quot; class=&quot;ph-wo-vertical-align-middle ph-dash-cursor-pointer&quot;>&quot; , &quot;'&quot; , &quot;;
	if(recordSize > 10000)
	{
		pdfIcon = &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/pdf.png&quot; title=&quot;PDF file cannot be downloaded, as number of records more then 10000&quot; class=&quot;ph-wo-vertical-align-middle ph-dash-cursor-pointer&quot;>&quot; , &quot;'&quot; , &quot;;
		exportPdf = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	}	
	else
	{
		pdfIcon = &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/pdf.png&quot; title=&quot;Export to PDF&quot; data-onclick=&quot;fnDataTableExportexpdf&quot; data-etype=&quot;pdf&quot; class=&quot;ph-wo-vertical-align-middle ph-dash-cursor-pointer&quot;>&quot; , &quot;'&quot; , &quot;;
	}
	
	var excelReportType = $(&quot;#excelReportType&quot;).val();
	excelReportType = &quot;.&quot; + excelReportType.toLowerCase();
	var disclaimerText = $(&quot;#disclaimerText&quot;).val();
	titleHeaderName = replaceBackXMLEntities(dynTitleHeaderName);
	fileName = replaceBackXMLEntities(dynFileName);
	var table = $(&quot; , &quot;'&quot; , &quot;#CommonDataTableId&quot; , &quot;'&quot; , &quot;).DataTable( {
        deferRender:    true,
        &quot;processing&quot;: false,
        &quot;serverSide&quot;: true,
        &quot;ajax&quot;: {
        	&quot;async&quot;: true,
        	
            &quot;url&quot;: &quot;/phworkoutput/WOCategoryDetailTableAjax.htm&quot;,
            &quot;type&quot;: &quot;POST&quot;,
            headers	: {&quot; , &quot;'&quot; , &quot;X-Content-Security-Policy-Nonce&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;xZrLNGLeef26xpNnnF1igA==&quot; , &quot;'&quot; , &quot;},
             &quot;data&quot;: function (d) 
             {
            	 d.menuItemId = menuItemId;
                 d.persmissionObject = persmissionObject;
            	 d.selectedColumns = dataKeysString;
                return d;
            }
        },
        &quot;pageLength&quot;: dynPageLength,
        &quot;bDestroy&quot;: true,
        &quot;scrollX&quot;: true,
        &quot;ordering&quot; : isReqSorting,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px&quot; , &quot;'&quot; , &quot;> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100&quot; , &quot;'&quot; , &quot; >[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
            	&quot;previous&quot;:   &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;
            },
        },
         columns: columns,
         &quot;order&quot;: [[ dynSortingIndex, &quot; , &quot;'&quot; , &quot;asc&quot; , &quot;'&quot; , &quot; ]],
         rowGroup: {
             dataSrc: 88
         },
         dom: &quot; , &quot;'&quot; , &quot;Blfrtip&quot; , &quot;'&quot; , &quot;,
         buttons: [
        	 {
        		 text:   &quot; , &quot;'&quot; , &quot;&lt;label class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>|&amp;nbsp;&amp;nbsp; Export:&amp;nbsp;&lt;/label>&quot; , &quot;'&quot; , &quot;,
        		 autoClose: &quot; , &quot;'&quot; , &quot;false&quot; , &quot;'&quot; , &quot;,
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;
        	 },
             {
                 extend: exportExcel,
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: &quot; , &quot;'&quot; , &quot;export&quot; , &quot;'&quot; , &quot;,
                 	 stripHtml: true
                 },
                 destroy:true,
                 filename: fileName,
                 text:  excelIcon,
                 autoClose: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;,
                 extension: excelReportType,
                 createEmptyCells: true,
                 title : titleHeaderName,
                 messageBottom: function () {
                     return disclaimerText;
                 	},
                     customize: function ( xlsx ) {
						 var sheet = xlsx.xl.worksheets[&quot; , &quot;'&quot; , &quot;sheet1.xml&quot; , &quot;'&quot; , &quot;];
						$(&quot; , &quot;'&quot; , &quot;row c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;25&quot; , &quot;'&quot; , &quot; );
                        $(&quot; , &quot;'&quot; , &quot;c[r=A1] t&quot; , &quot;'&quot; , &quot;, sheet).text( titleHeaderName );
                        $(&quot; , &quot;'&quot; , &quot;row:first c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;27&quot; , &quot;'&quot; , &quot; ); 
                        $(&quot; , &quot;'&quot; , &quot;row:eq(1) c&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;27&quot; , &quot;'&quot; , &quot;);
                        
                        if(disclaimerText != null &amp;&amp; disclaimerText != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
                        {
                        	$(&quot; , &quot;'&quot; , &quot;row:last c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;55&quot; , &quot;'&quot; , &quot; );
	                        $(&quot; , &quot;'&quot; , &quot;row:last&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;customHeight&quot; , &quot;'&quot; , &quot;, 1);
	                        $(&quot; , &quot;'&quot; , &quot;row:last&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;ht&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;150&quot; , &quot;'&quot; , &quot;);
                        }
                         
                      },
                      
                 button: [&quot; , &quot;'&quot; , &quot;excel&quot; , &quot;'&quot; , &quot;]

             },
             {
                 extend: exportPdf,
                 orientation: &quot; , &quot;'&quot; , &quot;landscape&quot; , &quot;'&quot; , &quot;,
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: &quot; , &quot;'&quot; , &quot;export&quot; , &quot;'&quot; , &quot;,
                 	 stripHtml: true
                 },
                 text: pdfIcon,
                 filename: fileName,
                 autoClose: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 footer:&quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 destroy:true,
                 messageBottom: function () {
                     return disclaimerText;
                  	},
               	 customize: function ( doc ) {
               		
               	   var cols = [];
               	   cols[0] = {text: &quot; , &quot;'&quot; , &quot;© JaMocha Tech Pvt. Ltd. 2009-2026&quot; , &quot;'&quot; , &quot;, alignment: &quot; , &quot;'&quot; , &quot;center&quot; , &quot;'&quot; , &quot;, fontSize:&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;, margin:[0, 0, 0, 0] };
               	   var objFooter = {};
               	   objFooter[&quot; , &quot;'&quot; , &quot;columns&quot; , &quot;'&quot; , &quot;] = cols;
               	   doc[&quot; , &quot;'&quot; , &quot;footer&quot; , &quot;'&quot; , &quot;]=objFooter;
               	   doc.styles.tableHeader.fillColor = &quot; , &quot;'&quot; , &quot;#006bb7&quot; , &quot;'&quot; , &quot;;
               	   doc.styles.tableHeader.alignment=&quot;left&quot;;
               	 	
               	   doc[&quot; , &quot;'&quot; , &quot;header&quot; , &quot;'&quot; , &quot;]=(function(page, pages) {
							return {
								columns: [
									{
										alignment: &quot; , &quot;'&quot; , &quot;left&quot; , &quot;'&quot; , &quot;,
										text: &quot; , &quot;'&quot; , &quot;ProHance&quot; , &quot;'&quot; , &quot;,
										fontSize: 12,
									},
									{
										alignment: &quot; , &quot;'&quot; , &quot;right&quot; , &quot;'&quot; , &quot;,
										fontSize: 8,
										text: [&quot; , &quot;'&quot; , &quot;Page &quot; , &quot;'&quot; , &quot;, { text: page.toString() }]
									}
								],
								margin: 20
							}
						});
	               	if(dynNumericAlignColArr.length > 0)
	               	{
	               		var rowCount = doc.content[1].table.body.length;
		               	for (i = 0; i &lt; rowCount; i++) 
		               	{
		               		dynNumericAlignColArr.forEach(function(number) {
		                   		doc.content[1].table.body[i][number].alignment = &quot; , &quot;'&quot; , &quot;right&quot; , &quot;'&quot; , &quot;;
		                   	  });
		               	}
	               	}
               	
	               doc.content[1].table.widths=	dynWidthArr;
               	
               	 },
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;,
                 pageSize : &quot; , &quot;'&quot; , &quot;LEGAL&quot; , &quot;'&quot; , &quot;,
                 titleAttr : &quot; , &quot;'&quot; , &quot;PDF&quot; , &quot;'&quot; , &quot;,
                 title : titleHeaderName,
                 button: [ &quot; , &quot;'&quot; , &quot;pdf&quot; , &quot;'&quot; , &quot; ]
             }
         ],
         drawCallback: function() {
       	  var hasRows = this.api().rows({ filter: &quot; , &quot;'&quot; , &quot;applied&quot; , &quot;'&quot; , &quot; }).data().length > 0;
       	 $(&quot; , &quot;'&quot; , &quot;.buttons-excel&quot; , &quot;'&quot; , &quot;)[0].style.visibility = hasRows ? &quot; , &quot;'&quot; , &quot;visible&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;;
       	 $(&quot; , &quot;'&quot; , &quot;.buttons-pdf&quot; , &quot;'&quot; , &quot;)[0].style.visibility = hasRows ? &quot; , &quot;'&quot; , &quot;visible&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;;
       	}
    } );
	
	setTimeout(function() {
		table.columns.adjust().draw();
	}, 20); 	
}


function buildScrollXDataTableStructure(jsonData, pageLength, columns, sortingIndex, isScrollX)
{
	var isReqSorting = true;
	if(sortingIndex &lt; 0)
	{
		isReqSorting  = false;
		sortingIndex = 0;
	}
	var table = $(&quot; , &quot;'&quot; , &quot;#CommonDataTableId&quot; , &quot;'&quot; , &quot;).DataTable( {
        data: jsonData,
        deferRender:    true,
        &quot;pageLength&quot;: pageLength,
        &quot;ordering&quot; : isReqSorting,
        &quot;autoWidth&quot;:false,
        &quot;scrollX&quot;: false,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px&quot; , &quot;'&quot; , &quot;> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100&quot; , &quot;'&quot; , &quot;>[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
                &quot;previous&quot;:   &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;
            },
        },
         columns: columns,
         &quot;order&quot;: [[ sortingIndex, &quot; , &quot;'&quot; , &quot;asc&quot; , &quot;'&quot; , &quot; ]],
         dom: &quot; , &quot;'&quot; , &quot;Blfrtip&quot; , &quot;'&quot; , &quot;,
         buttons: []
    } );
}


function buildDataTableStructureForWorkType(jsonData, pageLength, columns, sortingIndex, isScrollX)
{
	$.fn.DataTable.ext.pager.numbers_length = 5;
	$.fn.DataTable.ext.pager.customPager = function (page, pages) {
	    const buttons = [];
	    const startPage = Math.max(page - 1, 1);
	    const endPage = Math.min(startPage + 1, pages - 2);
	    buttons.push(0);
	    if (startPage > 2) buttons.push(&quot; , &quot;'&quot; , &quot;ellipsis&quot; , &quot;'&quot; , &quot;);
	    for (let i = startPage; i &lt;= endPage; i++) buttons.push(i);
	    if (endPage &lt; pages - 1) buttons.push(&quot; , &quot;'&quot; , &quot;ellipsis&quot; , &quot;'&quot; , &quot;);
	    buttons.push(pages - 1);
	    return buttons;
	};
	var table = $(&quot; , &quot;'&quot; , &quot;#CommonDataTableWorkTypeId&quot; , &quot;'&quot; , &quot;).DataTable( {
	    data: jsonData,
	    deferRender: true,
	    &quot;processing&quot;: false,
	    &quot;bDestroy&quot;: true,
	    &quot;pageLength&quot;: pageLength,
	    &quot;scrollX&quot;: true,
	    &quot;sScrollXInner&quot;: &quot;100%&quot;,
	    &quot;scrollCollapse&quot;: true,
	    &quot;ordering&quot;: true,
	    &quot;lengthMenu&quot;: lengthMenu,
	    &quot;language&quot;: {
	        &quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
	        &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px&quot; , &quot;'&quot; , &quot;> Displaying _START_ to _END_ &lt;label>&quot;,
	        &quot;infoEmpty&quot;: &quot;No records to display&quot;,
	        &quot;infoFiltered&quot;: &quot;&lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100&quot; , &quot;'&quot; , &quot; >[Filtered from _MAX_ records] &lt;/label>&quot;,
	        &quot;search&quot;: &quot;Search&quot;,
	        &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
	        &quot;paginate&quot;: {
	            &quot;next&quot;: &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
	            &quot;previous&quot;: &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;
	        },
	        renderer: &quot; , &quot;'&quot; , &quot;customPager&quot; , &quot;'&quot; , &quot;,
	    },
	    columns: columns.map((column, index) => {
	        column.orderable = index !== 0;
	        return column;
	    }),
	    order: [[1, &quot; , &quot;'&quot; , &quot;asc&quot; , &quot;'&quot; , &quot;]],
	    dom: &quot; , &quot;'&quot; , &quot;Blfrtip&quot; , &quot;'&quot; , &quot;,
	    buttons: [],
	} );
}

function buildDataTableStructureForWTAttributeOptions(pageLength, columnArr, sortingIndex, dynTableId, dataParam)
{
	$.fn.DataTable.ext.pager.numbers_length = 5;
	$.fn.DataTable.ext.pager.customPager = function (page, pages) {
	    const buttons = [];
	    const startPage = Math.max(page - 1, 1);
	    const endPage = Math.min(startPage + 1, pages - 2);

	    buttons.push(0);
	    if (startPage > 2)
	    {
	        buttons.push(&quot; , &quot;'&quot; , &quot;ellipsis&quot; , &quot;'&quot; , &quot;);
	    }

	    for (let i = startPage; i &lt;= endPage; i++)
	    {
	        buttons.push(i);
	    }

	    if (endPage &lt; pages - 1)
	    {
	        buttons.push(&quot; , &quot;'&quot; , &quot;ellipsis&quot; , &quot;'&quot; , &quot;);
	    }
	    buttons.push(pages - 1);
	    return buttons;
	};
	
	var table = $(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot;+dynTableId).DataTable( {
	    &quot;processing&quot;: false,
        &quot;serverSide&quot;: true,
        &quot;ajax&quot;: {
        	&quot;async&quot;: true,
            &quot;url&quot;: &quot;/phworkoutput/getSpecificWorktypeAttrOptions.htm&quot;,
            &quot;type&quot;: &quot;POST&quot;,
            headers	: {&quot; , &quot;'&quot; , &quot;X-Content-Security-Policy-Nonce&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;xZrLNGLeef26xpNnnF1igA==&quot; , &quot;'&quot; , &quot;},
            &quot;data&quot;: dataParam,
            &quot;dataSrc&quot;: function(json) {
            	parameterJson = json.data;
                return json.data;
            },
            &quot;beforeSend&quot;: function() {
            	$(&quot;#loading_attr_options&quot;).removeClass(&quot;ph-wo-display-none&quot;);
				fnRemoveClassWithSameStyle(&quot;.content-loading&quot;, &quot; , &quot;'&quot; , &quot;ph-wo-display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            	$(&quot;.content-loading&quot;).removeClass(&quot;ph-wo-display-none&quot;);
            	$(&quot;#specificAttrOptionContent&quot;).removeClass(&quot;ph-wo-display-none&quot;);
            	fnRemoveClassWithSameStyle(&quot;.specificAttrOptionContent&quot;, &quot; , &quot;'&quot; , &quot;ph-wo-display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            	$(&quot;.specificAttrOptionContent&quot;).addClass(&quot;ph-wo-display-none&quot;);
				$(&quot;#specificAttrOptionContent&quot;).removeClass(&quot;ph-wo-display-none&quot;);
				fnRemoveClassWithSameStyle(&quot;.specificAttrOptionContent&quot;, &quot; , &quot;'&quot; , &quot;ph-wo-display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
				$(&quot;#specificAttrOptionContent&quot;).addClass(&quot;ph-wo-display-none&quot;);
            },
            &quot;complete&quot;: function() {
            	$(&quot;#loading_attr_options&quot;).addClass(&quot;ph-wo-display-none&quot;);
            	fnRemoveClassWithSameStyle(&quot;.content-loading&quot;, &quot; , &quot;'&quot; , &quot;ph-wo-display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            	$(&quot;.content-loading&quot;).addClass(&quot;ph-wo-display-none&quot;);
            	fnRemoveClassWithSameStyle(&quot;#specificAttrOptionContent&quot;, &quot; , &quot;'&quot; , &quot;ph-wo-display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
				$(&quot;#specificAttrOptionContent&quot;).removeClass(&quot;ph-wo-display-none&quot;);
				fnRemoveClassWithSameStyle(&quot;.specificAttrOptionContent&quot;, &quot; , &quot;'&quot; , &quot;ph-wo-display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
			  	table.columns.adjust();
            }
        },
        initComplete : function() {
		  	$(&quot; , &quot;'&quot; , &quot;.dataTables_filter&quot; , &quot;'&quot; , &quot;).addClass(&quot;attrOption-search&quot;);
        	var self = this.api();
            var $filter = $(&quot; , &quot;'&quot; , &quot;.attrOption-search&quot; , &quot;'&quot; , &quot;);
            var $input = $filter.find(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).unbind();
            var $searchButton = $(&quot; , &quot;'&quot; , &quot;&lt;span title=&quot;Search&quot; class=&quot;custom-datatable-btn&quot;>&lt;i class=&quot;fa fa-search attrOption&quot;/>&lt;/span>&quot; , &quot;'&quot; , &quot;)
                .click(function(event) {
                	event.preventDefault();
                	if($input.val().trim() != null &amp;&amp; $input.val().trim() != &quot;&quot;)
                	{
                		$(&quot;#loading_attr_options&quot;).removeClass(&quot;ph-wo-display-none&quot;);
                    	self.search($input.val()).draw();
                	}
                });
            var $clearButton = $(&quot; , &quot;'&quot; , &quot;&lt;span title=&quot;Clear&quot; class=&quot;custom-datatable-btn&quot;>&lt;i class=&quot;fa fa-times attrOption&quot;/>&lt;/span>&quot; , &quot;'&quot; , &quot;)
                .click(function(event) {
                	event.preventDefault();
                	$(&quot;#loading_attr_options&quot;).removeClass(&quot;ph-wo-display-none&quot;);
                	self.search(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;).draw();
                });
            if (!$filter.has($searchButton).length) 
            {
            	  $filter.append($searchButton);
            }

            if (!$filter.has($clearButton).length) 
            {
            	  $filter.append($clearButton);
            }
        },
        &quot;pageLength&quot;: pageLength,
        &quot;scrollX&quot;: true,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;scrollCollapse&quot;: true,
        &quot;ordering&quot; : true,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px&quot; , &quot;'&quot; , &quot;> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100&quot; , &quot;'&quot; , &quot; >[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
            	&quot;previous&quot;:   &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;
            },
            renderer: &quot; , &quot;'&quot; , &quot;customPager&quot; , &quot;'&quot; , &quot;,
        },
		columns: columnArr.map((column, index) => {
			column.orderable = index !== 0;
            return column;
        }),
        order: [[1, &quot; , &quot;'&quot; , &quot;asc&quot; , &quot;'&quot; , &quot;]],
        dom: &quot; , &quot;'&quot; , &quot;Blfrtip&quot; , &quot;'&quot; , &quot;,
        buttons: [],
    });
}

$(document).off(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnDataTableExportexpdf&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnDataTableExportexpdf&quot;]&quot; , &quot;'&quot; , &quot;, function() {
    var etype = $(this).data(&quot; , &quot;'&quot; , &quot;etype&quot; , &quot;'&quot; , &quot;);
    fnDataTableExport(etype);
});
 










 
	

    
			













	
		×
		
	



	
	
	
	


	
		
			
				
					
						
							
							
							
								
							
							
						
						
							
								
										
						
					
					
			
			
			
			
		
	



 $( document ).ready(function() {
	fnFadeoutEvent(&quot;#dialogsuccessspan&quot;, 20000, &quot;ph-wo-display-none&quot;);
	fnFadeoutEvent(&quot;#dialogfailurespan&quot;, 20000, &quot;ph-wo-display-none&quot;);
	fnFadeoutEvent(&quot;#dialogdependencyspan&quot;, 20000, &quot;ph-wo-display-none&quot;);
	
	if ($(&quot;#dialogsuccessspan&quot;).is(&quot; , &quot;'&quot; , &quot;:hidden&quot; , &quot;'&quot; , &quot;)) 
	{
		fnRemoveClassWithSameStyle(&quot;#dialogsuccessspan&quot;, &quot;ph-wo-height&quot;, &quot;&quot;, false)
	 	$(&quot;#dialogsuccessspan&quot;).addClass(&quot;ph-wo-height-0px&quot;);
	}
	if ($(&quot;#dialogfailurespan&quot;).is(&quot; , &quot;'&quot; , &quot;:hidden&quot; , &quot;'&quot; , &quot;)) 
	{
		fnRemoveClassWithSameStyle(&quot;#dialogfailurespan&quot;, &quot;ph-wo-height&quot;, &quot;&quot;, false)
	 	$(&quot;#dialogfailurespan&quot;).addClass(&quot;ph-wo-height-0px&quot;);
	}
});

function fnSucessClose()
{
	fnRemoveClassWithSameStyle(&quot;#dialogsuccessspan&quot;, &quot;ph-wo-height&quot;, &quot;&quot;, false)
 	$(&quot;#dialogsuccessspan&quot;).addClass(&quot;ph-wo-height-0px&quot;);
}

function fnFailureClose()
{
	fnRemoveClassWithSameStyle(&quot;#dialogfailurespan&quot;, &quot;ph-wo-height&quot;, &quot;&quot;, false)
 	$(&quot;#dialogfailurespan&quot;).addClass(&quot;ph-wo-height-0px&quot;);
}

function fnShowDependency()
{
	$(&quot; , &quot;'&quot; , &quot;.modal&quot; , &quot;'&quot; , &quot;).draggable({
	    handle: &quot;.modal-header&quot;
	});
	
	var form;
	if (&quot; , &quot;'&quot; , &quot;WorkTypeForm&quot; , &quot;'&quot; , &quot; == &quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;)
	{
		//
	}
	else
	{
		form = document.WorkTypeForm;
	}
	var url = &quot;/phworkoutput/getDataDependency/getDependencyList.htm?deleteRecordId=&quot;+-1;
	windowTitle = fnGetWindowName();
	var winObj =  window.open(&quot;&quot;, windowTitle, &quot;status=1, toolbar=0, scrollbars=1, resizable=1, width=1330, height=600, left=20, top=20&quot;);
	winObj.document.body.appendChild(fnGetDependencyPopUpLoader());
	window.parent.parent.fnAddToChildWindows(winObj);
	tempAction = form.action;
    tempTarget = form.target;
    form.action = url;
    form.target = windowTitle;
    form.submit();
    winObj.focus();
    form.action = tempAction;
    form.target = tempTarget;
}

function fnGetDependencyPopUpLoader()
{
	var loaderDiv = window.parent.document.createElement(&quot; , &quot;'&quot; , &quot;div&quot; , &quot;'&quot; , &quot;);
    loaderDiv.style.textAlign = &quot; , &quot;'&quot; , &quot;center&quot; , &quot;'&quot; , &quot;;
    loaderDiv.style.width = &quot; , &quot;'&quot; , &quot;100%&quot; , &quot;'&quot; , &quot;;
    loaderDiv.style.position = &quot; , &quot;'&quot; , &quot;absolute&quot; , &quot;'&quot; , &quot;;
    loaderDiv.style.top = &quot; , &quot;'&quot; , &quot;48%&quot; , &quot;'&quot; , &quot;;
    loaderDiv.setAttribute(&quot; , &quot;'&quot; , &quot;nonce&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;xZrLNGLeef26xpNnnF1igA==&quot; , &quot;'&quot; , &quot;);
 
    var loaderImg =  window.parent.document.createElement(&quot; , &quot;'&quot; , &quot;img&quot; , &quot;'&quot; , &quot;);
    loaderImg.src = &quot;/phworkoutput/images/loader-trans.gif&quot;;
    loaderImg.setAttribute(&quot; , &quot;'&quot; , &quot;nonce&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;xZrLNGLeef26xpNnnF1igA==&quot; , &quot;'&quot; , &quot;);
    loaderDiv.appendChild(loaderImg);
    return loaderDiv;
}

/* function fnDeleteDependency()
{
	var primaryKeyId = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	var moduleName = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	var menuItemId = &quot; , &quot;'&quot; , &quot;302&quot; , &quot;'&quot; , &quot;;
	var formDefinitionId  = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	
	var actionName = &quot;/phworkoutput/getDataDependency/deleteDataDependency.htm?primaryKeyId=&quot;+primaryKeyId+&quot;&amp;moduleName=&quot;+moduleName+&quot;&amp;menuItemId=&quot;+menuItemId+&quot;&amp;formDefinitionId=&quot;+formDefinitionId;
	doAjaxCall(actionName, false, false);
} */


$(document).off(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;hideMsg&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;hideMsg&quot;]&quot; , &quot;'&quot; , &quot;, function() {
	
	fnRemoveClassWithSameStyle(&quot;.internalmessagefailure&quot;, &quot;ph-wo-display&quot;, &quot;&quot;, false);
	$(&quot;.internalmessagefailure&quot;).addClass(&quot;ph-wo-display-none&quot;);
	fnRemoveClassWithSameStyle(&quot;#displayErrorMessage&quot;, &quot;ph-wo-display&quot;, &quot;&quot;, false);
	$(&quot;#displayErrorMessage&quot;).addClass(&quot;ph-wo-display-none&quot;);
});

$(document).off(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnSucessClose&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnSucessClose&quot;]&quot; , &quot;'&quot; , &quot;, function() {
	fnSucessClose();
});

$(document).off(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnFailureClose&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnFailureClose&quot;]&quot; , &quot;'&quot; , &quot;, function() {
	fnFailureClose();
});

$(document).off(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnShowDependency&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnShowDependency&quot;]&quot; , &quot;'&quot; , &quot;, function() {
	fnShowDependency();
});


	 



	 
        
			
				 
			        
						
						
							
								
									
										ADD WORK TYPES
									
								
							
							
								
									
										
											
												Records to display 102050100200Search
											        
											        Work TypeDescription#User GroupsStatus#dipankar_processWO Scoredcard 2.01Active#Process_028WO Scoredcard 2.01Active#Process_0288 clonedWO Scoredcard 2.01Active#Process_05WO Scoredcard 2.01Active#Process_055WO Scoredcard 2.01Active123zc bz b0Active1_WT0Active2_WTWT10ActiveabWT1231Activeact1230Active
												Total Records: 32    Displaying 1 to 10 1234
											
											
											
									
								
							
						
					
				
			
		
	





$(document).off(&quot;click&quot;, &quot;.fnAddWorkTypes&quot;).on(&quot;click&quot;, &quot;.fnAddWorkTypes&quot;, function(){
	fnAddWorkTypes(this.form);
});

$(document).off(&quot;submit&quot;, &quot;#categoryForm&quot;).on(&quot;submit&quot;, &quot;#categoryForm&quot;, function(){
	return false;
});

form = document.CategoryForm;

var userGroupHashedMap = new Array();

function fnAddWorkTypes(form)
{
	
	mappedWorkTypes = &quot; , &quot;'&quot; , &quot;67,68,69,71,73,138,74,139,140,142,143,144,145,146,147,149,90,94,32,98,99,102,104,105,106,107,108,111,117,119,57,123&quot; , &quot;'&quot; , &quot;;
	$(&quot;#mappedWorkTypes&quot;).val(mappedWorkTypes);
	windowTitle = fnGetWindowName();
	var url = &quot;/phworkoutput/WOCategoryDetails/popupWorkTypeSummary.htm?id=1&amp;menuItemId=302&quot;;
	console.log(url)
	var winObj =  window.open(&quot;&quot;, windowTitle, &quot;status=1, toolbar=0, scrollbars=1, resizable=1, width=1330, height=600, left=20, top=20&quot;);
	winObj.document.body.appendChild(fnGetPopUpLoader(winObj));
	window.parent.parent.fnAddToChildWindows(winObj);
	tempAction = form.action;
    tempTarget = form.target;
    form.action = url;
    form.target = windowTitle;
    form.submit();
    winObj.focus();
    form.action = tempAction;
    form.target = tempTarget;
}

function fnAddWorkTypeMapping(workTypIds)
{
	$(&quot;#workTypeIdStr&quot;).val(workTypIds);
	
	var url = &quot;/phworkoutput/WOCategoryDetails/addWorkTypeMapping.htm?menuItemId=302&quot;;
	$.ajax({
		url: url,
		headers	: {&quot; , &quot;'&quot; , &quot;X-Content-Security-Policy-Nonce&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;xZrLNGLeef26xpNnnF1igA==&quot; , &quot;'&quot; , &quot;},
		global: false,
		type: &quot;POST&quot;,
		data: ({ 			
				&quot;categoryId&quot; 	: $(&quot;#categoryId&quot;).val(),
				&quot;workTypeIdStr&quot;		: workTypIds
 	          }),
	         success: function(resp)
			 {
	        	var json = JSON.parse(resp);
	        	if(json[&quot;message&quot;] == &quot; , &quot;'&quot; , &quot;SUCCESS&quot; , &quot;'&quot; , &quot;)
		        {
	                $(&quot;#categotyWorkTypesMsg&quot;).val(&quot; , &quot;'&quot; , &quot;SUCCESS&quot; , &quot;'&quot; , &quot;);
		        }
		        else
		        {
	                $(&quot;#categotyWorkTypesMsg&quot;).val(&quot; , &quot;'&quot; , &quot;Failed&quot; , &quot;'&quot; , &quot;);
		        }
	        	
	        	 fnShowWorkTypeScreen(true);
			 },
		     error :  function(msg,arg1,arg2)
		     {
		    	 processErrorStatus(msg.status, &quot;/phworkoutput&quot;);
		    	 return false;
			 }
	});
}

if(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; == &quot;fromGroupDelete&quot;)
{
	form.mappedUserGroupIds.value = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
}

$(document).ready(function() 
{
	fnPrepareDataTable();
});

function fnPrepareDataTable()
{
	var Val = &quot; , &quot;'&quot; , &quot;[{\&quot;workTypeName\&quot;:\&quot;Test Cases Written\&quot;,\&quot;worktypeId\&quot;:32,\&quot;userGroupCount\&quot;:6,\&quot;description\&quot;:\&quot;Test Cases\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;WO Bugs Dev Verified\&quot;,\&quot;worktypeId\&quot;:57,\&quot;userGroupCount\&quot;:8,\&quot;description\&quot;:\&quot;Bugs Dev Verified [From Workflow jobs using Database Import]\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;Design\&quot;,\&quot;worktypeId\&quot;:67,\&quot;userGroupCount\&quot;:5,\&quot;description\&quot;:\&quot;WO Scoredcard 2.0\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;Dev. Support\&quot;,\&quot;worktypeId\&quot;:68,\&quot;userGroupCount\&quot;:2,\&quot;description\&quot;:\&quot;WO Scoredcard 2.0\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;Development\&quot;,\&quot;worktypeId\&quot;:69,\&quot;userGroupCount\&quot;:7,\&quot;description\&quot;:\&quot;WO Scoredcard 2.0\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;R&amp;amp;I Developement\&quot;,\&quot;worktypeId\&quot;:71,\&quot;userGroupCount\&quot;:1,\&quot;description\&quot;:\&quot;WO Scoredcard 2.0\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;Testing\&quot;,\&quot;worktypeId\&quot;:73,\&quot;userGroupCount\&quot;:8,\&quot;description\&quot;:\&quot;WO Scoredcard 2.0\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;workflowimport\&quot;,\&quot;worktypeId\&quot;:74,\&quot;userGroupCount\&quot;:15,\&quot;description\&quot;:\&quot;WO Scoredcard 2.0\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;ukS)g\&quot;,\&quot;worktypeId\&quot;:90,\&quot;userGroupCount\&quot;:0,\&quot;description\&quot;:\&quot;\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;x0-tN\&quot;,\&quot;worktypeId\&quot;:94,\&quot;userGroupCount\&quot;:0,\&quot;description\&quot;:\&quot;\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;jO_)d\&quot;,\&quot;worktypeId\&quot;:98,\&quot;userGroupCount\&quot;:0,\&quot;description\&quot;:\&quot;\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;#dipankar_process\&quot;,\&quot;worktypeId\&quot;:99,\&quot;userGroupCount\&quot;:1,\&quot;description\&quot;:\&quot;WO Scoredcard 2.0\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;#Process_05\&quot;,\&quot;worktypeId\&quot;:102,\&quot;userGroupCount\&quot;:1,\&quot;description\&quot;:\&quot;WO Scoredcard 2.0\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;Bugs Resolved\&quot;,\&quot;worktypeId\&quot;:104,\&quot;userGroupCount\&quot;:6,\&quot;description\&quot;:\&quot;WO Scoredcard 2.0\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;#Process_028\&quot;,\&quot;worktypeId\&quot;:105,\&quot;userGroupCount\&quot;:1,\&quot;description\&quot;:\&quot;WO Scoredcard 2.0\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;#Process_0288 cloned\&quot;,\&quot;worktypeId\&quot;:106,\&quot;userGroupCount\&quot;:1,\&quot;description\&quot;:\&quot;WO Scoredcard 2.0\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;Test_Claim Provider insurance\&quot;,\&quot;worktypeId\&quot;:107,\&quot;userGroupCount\&quot;:1,\&quot;description\&quot;:\&quot;WO Scoredcard 2.0\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;rkq3x\&quot;,\&quot;worktypeId\&quot;:108,\&quot;userGroupCount\&quot;:0,\&quot;description\&quot;:\&quot;\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;Fy5_n\&quot;,\&quot;worktypeId\&quot;:111,\&quot;userGroupCount\&quot;:0,\&quot;description\&quot;:\&quot;\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;zc bz b\&quot;,\&quot;worktypeId\&quot;:117,\&quot;userGroupCount\&quot;:1,\&quot;description\&quot;:\&quot;\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;New WT01\&quot;,\&quot;worktypeId\&quot;:119,\&quot;userGroupCount\&quot;:1,\&quot;description\&quot;:\&quot;New WT01\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;zc bz b123\&quot;,\&quot;worktypeId\&quot;:123,\&quot;userGroupCount\&quot;:0,\&quot;description\&quot;:\&quot;\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;Bugs Dev Fixed\&quot;,\&quot;worktypeId\&quot;:138,\&quot;userGroupCount\&quot;:0,\&quot;description\&quot;:\&quot;\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;soumya\&quot;,\&quot;worktypeId\&quot;:139,\&quot;userGroupCount\&quot;:1,\&quot;description\&quot;:\&quot;\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;scc,gvgbmj\&quot;,\&quot;worktypeId\&quot;:140,\&quot;userGroupCount\&quot;:2,\&quot;description\&quot;:\&quot;WO Scoredcard 2.0\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;WT1\&quot;,\&quot;worktypeId\&quot;:142,\&quot;userGroupCount\&quot;:0,\&quot;description\&quot;:\&quot;WT1\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;abWT123\&quot;,\&quot;worktypeId\&quot;:143,\&quot;userGroupCount\&quot;:1,\&quot;description\&quot;:\&quot;\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;act123\&quot;,\&quot;worktypeId\&quot;:144,\&quot;userGroupCount\&quot;:0,\&quot;description\&quot;:\&quot;\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;123zc bz b\&quot;,\&quot;worktypeId\&quot;:145,\&quot;userGroupCount\&quot;:0,\&quot;description\&quot;:\&quot;\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;1_WT\&quot;,\&quot;worktypeId\&quot;:146,\&quot;userGroupCount\&quot;:0,\&quot;description\&quot;:\&quot;\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;2_WT\&quot;,\&quot;worktypeId\&quot;:147,\&quot;userGroupCount\&quot;:0,\&quot;description\&quot;:\&quot;WT1\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;#Process_055\&quot;,\&quot;worktypeId\&quot;:149,\&quot;userGroupCount\&quot;:1,\&quot;description\&quot;:\&quot;WO Scoredcard 2.0\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true}]&quot; , &quot;'&quot; , &quot;;
	var json = JSON.parse(Val);
	var columnArr = [];
	
	columnArr.push(
		{
			&quot;title&quot;		: &quot;Work Type&quot;, 
		 	&quot;data&quot;		: &quot;workTypeName&quot;, 
		 	&quot;width&quot; 	: &quot;45%&quot;,
		 	&quot;className&quot;	: &quot;text&quot;,
		 	&quot;render&quot;	: function (workTypeName, type, full, meta) 
		 	{
		 		var actions = &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;ph-wo-word-break-break-all&quot;>&quot; , &quot;'&quot; , &quot;+workTypeName+&quot; , &quot;'&quot; , &quot;&lt;/div>&quot; , &quot;'&quot; , &quot;;
				return actions;
			}
	 	});
	
	columnArr.push(
		{
			&quot;title&quot;		: &quot;Description&quot;, 
		 	&quot;data&quot;		: &quot;description&quot;, 
		 	&quot;width&quot; 	: &quot;40%&quot;,
		 	&quot;className&quot;	: &quot;text&quot;,
		 	&quot;render&quot;	: function (description, type, full, meta) 
		 	{
		 		var actions = &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;ph-wo-word-break-break-all&quot;>&quot; , &quot;'&quot; , &quot;+description+&quot; , &quot;'&quot; , &quot;&lt;/div>&quot; , &quot;'&quot; , &quot;;
				return actions;
			}
	 	});
	
	columnArr.push({&quot;title&quot;: &quot;#User Groups&quot;, &quot;data&quot;: &quot;userGroupCount&quot;, &quot;width&quot; : &quot;10%&quot;, &quot;class&quot;: &quot;numeric&quot;});
	
	columnArr.push({ 
   	 &quot;title&quot;	 : &quot;Status&quot;, 
     	 &quot;data&quot;		 : &quot;status&quot;,
     	 &quot;searchable&quot;: false,
     	 &quot;sortable&quot;	 : true,
     	&quot;width&quot; 	 : &quot;5%&quot;,
     	 &quot;className&quot; : &quot;text-center&quot;,
     	 &quot;render&quot;	 : function (json, type, full, meta) 
     	 {
          	var actions = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
          	if(full.status == true)
      		{
      			actions = &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-circle circle-active&quot; title=&quot;Active&quot;>&lt;span class=&quot;ph-wo-display-none&quot;>Active&lt;/span>&lt;/i>&quot; , &quot;'&quot; , &quot;;
      		}
      		else if(full.status == false)
      		{
      			actions = &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-circle circle-inactive&quot; title=&quot;Inactive&quot;>&lt;span class=&quot;ph-wo-display-none&quot;>Inactive&lt;/span>&lt;/i>&quot; , &quot;'&quot; , &quot;;
      		}
			return actions;
 		}
    });
	
	
	var pageSize = 10;
	var sortingIndex = 0;
	
	if(json != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
	{
		buildScrollXDataTableStructure(json, pageSize, columnArr, sortingIndex, false);
	}
	var table = $(&quot; , &quot;'&quot; , &quot;#CommonDataTableId&quot; , &quot;'&quot; , &quot;).DataTable();
	table.columns.adjust().draw();
}




								
							
						
					
				
				
					
						
							
								PREVIOUS
							
						
						
							CANCEL
						
						
							
								NEXT
							
						
						
							
								EXIT
							
						
					
				
			
		
	


	


$(document).off(&quot;click&quot;, &quot;.fnShowBasicScreen1&quot;).on(&quot;click&quot;, &quot;.fnShowBasicScreen1&quot;, function(){
	fnShowBasicScreen1();
});

$(document).off(&quot;click&quot;, &quot;.fnShowWorkTypeScreen&quot;).on(&quot;click&quot;, &quot;.fnShowWorkTypeScreen&quot;, function(){
	var flag = $(this).data(&quot;flag&quot;);
	fnShowWorkTypeScreen(flag);
});

$(document).off(&quot;click&quot;, &quot;.fnShowPreviousScreen&quot;).on(&quot;click&quot;, &quot;.fnShowPreviousScreen&quot;, function(){
	fnShowPreviousScreen();
});

$(document).off(&quot;click&quot;, &quot;.fnShowNextScreen&quot;).on(&quot;click&quot;, &quot;.fnShowNextScreen&quot;, function(){
	fnShowNextScreen();
});

$(document).off(&quot;click&quot;, &quot;.fnBackToCategorySummary&quot;).on(&quot;click&quot;, &quot;.fnBackToCategorySummary&quot;, function(){
	fnBackToCategorySummary();
});

form = document.CategoryForm;

previousScreenName = &quot;&quot;;
currentScreenName = &quot;&quot;;
nextScreenName =&quot;&quot;;


function fnShowPreviousScreen()
{
	fnShowScreen(previousScreenName)
}

function fnShowNextScreen()
{
	fnShowScreen(nextScreenName)
}

function fnShowScreen(screenName)
{
	if(screenName == &quot;BASIC_SCREEN&quot;)
	{
		fnShowBasicScreen1();
	}
	else if (screenName == &quot;WORKTYPEMAPPING_SCREEN&quot;)
	{
		fnShowWorkTypeScreen(false);
	}
	else
	{
		alert(&quot; , &quot;'&quot; , &quot;Unknown screen&quot; , &quot;'&quot; , &quot;);
	}
}
function fnHighLightMenu(screenName)
{
	$(&quot;#parameterGroupTD&quot;).addClass(&quot; , &quot;'&quot; , &quot;wizardtdnohighlight&quot; , &quot;'&quot; , &quot;);
	$(&quot;#basicsTD&quot;).addClass(&quot;wizardtdnohighlight&quot;);
	$(&quot;#heatCodeTD&quot;).addClass(&quot;wizardtdnohighlight&quot;);
	
	if(screenName == &quot;BASIC_SCREEN&quot;)
	{
		fnRemoveClassWithSameStyle(&quot;#ExitButton&quot;,&quot;ph-wo-display&quot;,&quot;&quot;, false);
		$(&quot;#ExitButton&quot;).addClass(&quot;ph-wo-display-none&quot;);
		previousScreenName = &quot;&quot;;
		nextScreenName =&quot;WORKTYPEMAPPING_SCREEN&quot;;
		fnRemoveClassWithSameStyle(&quot;#previousButton&quot;,&quot;ph-wo-display&quot;,&quot;&quot;, false);
		$(&quot;#previousButton&quot;).addClass(&quot;ph-wo-display-none&quot;);
		fnRemoveClassWithSameStyle(&quot;#ExitButton&quot;,&quot;ph-wo-display&quot;,&quot;&quot;, false);
		$(&quot;#ExitButton&quot;).addClass(&quot;ph-wo-display-none&quot;);
		fnRemoveClassWithSameStyle(&quot;#nextButton&quot;,&quot;ph-wo-display&quot;,&quot;&quot;, false);
		$(&quot;#nextButton&quot;).addClass(&quot;ph-wo-display-empty&quot;);
		
		$(&quot;#basicsTD&quot;).removeClass();
		$(&quot;#basicsTD&quot;).addClass(&quot;fnShowBasicScreen1&quot;);
		$(&quot;#parameterGroupTD&quot;).removeClass(&quot; , &quot;'&quot; , &quot;wizardtdhighlight&quot; , &quot;'&quot; , &quot;);
		$(&quot;#heatCodeTD&quot;).removeClass(&quot; , &quot;'&quot; , &quot;wizardtdhighlight&quot; , &quot;'&quot; , &quot;);
		$(&quot;#basicsTD&quot;).addClass(&quot; , &quot;'&quot; , &quot;wizardtdhighlight&quot; , &quot;'&quot; , &quot;);
	}
	else if (screenName == &quot;WORKTYPEMAPPING_SCREEN&quot;)
	{
		fnRemoveClassWithSameStyle(&quot;#ExitButton&quot;,&quot;ph-wo-display&quot;,&quot;&quot;, false);
		$(&quot;#ExitButton&quot;).addClass(&quot;ph-wo-display-empty&quot;);
		previousScreenName = &quot;BASIC_SCREEN&quot;;
		nextScreenName =&quot;&quot;;
		fnRemoveClassWithSameStyle(&quot;#ExitButton&quot;,&quot;ph-wo-display&quot;,&quot;&quot;, false);
		$(&quot;#ExitButton&quot;).addClass(&quot;ph-wo-display-empty&quot;);
		fnRemoveClassWithSameStyle(&quot;#previousButton&quot;,&quot;ph-wo-display&quot;,&quot;&quot;, false);
		$(&quot;#previousButton&quot;).addClass(&quot;ph-wo-display-empty&quot;);
		fnRemoveClassWithSameStyle(&quot;#nextButton&quot;,&quot;ph-wo-display&quot;,&quot;&quot;, false);
		$(&quot;#nextButton&quot;).addClass(&quot;ph-wo-display-none&quot;);
		
		$(&quot;#basicsTD&quot;).removeClass(&quot; , &quot;'&quot; , &quot;wizardtdhighlight&quot; , &quot;'&quot; , &quot;);
		$(&quot;#heatCodeTD&quot;).removeClass(&quot; , &quot;'&quot; , &quot;wizardtdhighlight&quot; , &quot;'&quot; , &quot;);
		$(&quot;#parameterGroupTD&quot;).removeClass();
		$(&quot;#parameterGroupTD&quot;).addClass(&quot;fnShowWorkTypeScreen&quot;);
		$(&quot;#parameterGroupTD&quot;).addClass(&quot; , &quot;'&quot; , &quot;wizardtdhighlight&quot; , &quot;'&quot; , &quot;);
		$(&quot;#right_table&quot;).height(185);
	}
	else
	{
		alert(&quot; , &quot;'&quot; , &quot;Unknown screen&quot; , &quot;'&quot; , &quot;);
	}
	fnRemoveClassWithSameStyle(&quot;#contentTD&quot;,&quot;ph-wo-display&quot;,&quot;&quot;, false);
	$(&quot;#contentTD&quot;).addClass(&quot;ph-wo-display-empty&quot;);
}


function fnRemoveClass()
{
	$(&quot;#basicsTD&quot;).removeClass();
	$(&quot;#basicsTD&quot;).addClass(&quot;fnShowBasicScreen1&quot;);
	$(&quot;#parameterGroupTD&quot;).removeClass();
}


function fnShowBasicScreen1()
{
	fnHighLightMenu(&quot; , &quot;'&quot; , &quot;BASIC_SCREEN&quot; , &quot;'&quot; , &quot;);
	actionType = &quot;add&quot;;
	if($(&quot;#categoryId&quot;).val() > 0)
	{
		showIFrameLoading();
		actionType = &quot;modify&quot;;
		form.action = &quot;/phworkoutput/WOCategoryDetails/modify.htm?menuItemId=302&amp;subActionType=&quot;+actionType+&quot;&amp;categoryId=&quot;+$(&quot;#categoryId&quot;).val();
		form.submit();
	}
}

function fnShowWorkTypeScreen(isAddedWorkTypes)
{
	fnHighLightMenu(&quot; , &quot;'&quot; , &quot;WORKTYPEMAPPING_SCREEN&quot; , &quot;'&quot; , &quot;);
	var url = &quot;/phworkoutput/WOCategoryDetails/workTypeMappingScreen.htm?menuItemId=302&amp;categoryId=&quot;+$(&quot;#categoryId&quot;).val();
	doAjaxCall(url, null, $(&quot;#mappedParameterIds&quot;).val(), isAddedWorkTypes);
}

function fnAddWorkTypeMapping(str)
{
	//Empty fn required for adding workType Ids in second wizard screen
}


function doAjaxCall(url, selectedIds, mappedParameterIds, isAddedWorkTypes)
{
	$(&quot;#contentTD&quot;).html(fnGetAjaxLoader());
	$.ajax({
		url: url,
		headers	: {&quot; , &quot;'&quot; , &quot;X-Content-Security-Policy-Nonce&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;xZrLNGLeef26xpNnnF1igA==&quot; , &quot;'&quot; , &quot;},
		global: false,
		type: &quot;POST&quot;,
		data: ({ 			
				&quot;menuItemId&quot;	: &quot; , &quot;'&quot; , &quot;302&quot; , &quot;'&quot; , &quot;,
				&quot;isAjaxCall&quot; 	: true,
				&quot;mappedUserGroupIds&quot;: selectedIds,
				&quot;mappedParameterIds&quot;: mappedParameterIds
 	          }),
	         success: function(resp)
			 {

	        	$(&quot;#contentTD&quot;).html(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		        $(&quot;#contentTD&quot;).html(resp);
		        fnRemoveClassWithSameStyle(&quot;#pagingTD2&quot;,&quot;ph-wo-display&quot;,&quot;&quot;, false);
		        $(&quot;#pagingTD2&quot;).addClass(&quot;ph-wo-display-none&quot;);
		        fnSetEmptyTdHeight($(&quot;#right_table&quot;).height(), false);
		        if(isAddedWorkTypes)
		        {
		        	var successMsg = &quot;Work Types added successfully.&quot;;
					var failureMsg = &quot;Work Types can not be added, due to Error.&quot;;
				    
		        	if($(&quot;#categotyWorkTypesMsg&quot;).val() == &quot; , &quot;'&quot; , &quot;SUCCESS&quot; , &quot;'&quot; , &quot;)
			        {
		                var html = &quot; , &quot;'&quot; , &quot;&lt;span class=&quot;alert alert-success config-alert ph-wo-width-50percentage ph-wo-margin-left-min-150px&quot; id=&quot;successspanSubHead&quot;>&lt;a href=&quot;&quot; class=&quot;close&quot; data-dismiss=&quot;alert&quot; aria-label=&quot;close&quot;>&amp;times;&lt;/a>&quot; , &quot;'&quot; , &quot;;
		                        html += &quot; , &quot;'&quot; , &quot;&lt;span id=&quot;successdivSubHead&quot;>&lt;/span>&lt;/span>&quot; , &quot;'&quot; , &quot;;
		                $(&quot;#successspanSubHead&quot;).html(html);
		                $(&quot;#successdivSubHead&quot;).html(successMsg);
		                fnRemoveClassWithSameStyle(&quot;#successspanSubHead&quot;,&quot;ph-wo-display&quot;,&quot;&quot;, false);
		                $(&quot;#successspanSubHead&quot;).addClass(&quot;ph-wo-display-empty&quot;);
		               setTimeout(function() { $(&quot; , &quot;'&quot; , &quot;#successspanSubHead&quot; , &quot;'&quot; , &quot;).html(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;); }, 10000); 
		                $(&quot;#categotyWorkTypesMsg&quot;).val(&quot; , &quot;'&quot; , &quot;SUCCESS&quot; , &quot;'&quot; , &quot;);
			        }
			        else
			        {
		                var html = &quot; , &quot;'&quot; , &quot;&lt;span class=&quot;alert alert-danger config-alert ph-wo-width-50percentage ph-wo-margin-left-min-150px&quot; id=&quot;failurespanSubHead&quot;>&lt;a href=&quot;&quot; class=&quot;close&quot; data-dismiss=&quot;alert&quot; aria-label=&quot;close&quot;>&amp;times;&lt;/a>&quot; , &quot;'&quot; , &quot;;
		                        html +=  &quot; , &quot;'&quot; , &quot;&lt;span id=&quot;failuredivSubHead&quot; >&lt;/span>&lt;/span>&quot; , &quot;'&quot; , &quot;;
		                $(&quot;#failurespanSubHead&quot;).html(html);
		                  $(&quot;#failuredivSubHead&quot;).html(failureMsg);
		                fnRemoveClassWithSameStyle(&quot;#failurespanSubHead&quot;,&quot;ph-wo-display&quot;,&quot;&quot;, false);
		                $(&quot;#failurespanSubHead&quot;).addClass(&quot;ph-wo-display-empty&quot;);
		               setTimeout(function() { $(&quot; , &quot;'&quot; , &quot;#failurespanSubHead&quot; , &quot;'&quot; , &quot;).html(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;); }, 10000); 
		                $(&quot;#categotyWorkTypesMsg&quot;).val(&quot; , &quot;'&quot; , &quot;Failed&quot; , &quot;'&quot; , &quot;);
			        }
		        }
		        if($.browser.chrome){
		        	fnRemoveClassWithSameStyle(&quot;#right_table&quot;,&quot;ph-wo-border-bottom&quot;,&quot;&quot;, false);
		    		$(&quot;#right_table&quot;).addClass(&quot;ph-wo-border-bottom-none&quot;);
		        }

			 },
		     error :  function(msg,arg1,arg2)
		     {
		    	 processErrorStatus(msg.status, &quot;/phworkoutput&quot;);
		    	 return false;
			 }
	});
}

fnRemoveClassWithSameStyle(&quot;#adminBodyContent&quot;,&quot;ph-wo-margin-top&quot;,&quot;&quot;, false);
$(&quot;#adminBodyContent&quot;).addClass(&quot; , &quot;'&quot; , &quot;ph-wo-margin-top-min-17px&quot; , &quot;'&quot; , &quot;);

$(document).ready(function() 
{
	fnHighLightMenu(&quot;BASIC_SCREEN&quot;);
	if(&quot; , &quot;'&quot; , &quot;modify&quot; , &quot;'&quot; , &quot; == &quot; , &quot;'&quot; , &quot;modify&quot; , &quot;'&quot; , &quot; || $(&quot;#workTypeId&quot;).val() > 0)
	{
		fnRemoveClassWithSameStyle(&quot;#naviButtons&quot;,&quot;ph-wo-display&quot;,&quot;&quot;, false);
		$(&quot;#naviButtons&quot;).addClass(&quot;ph-wo-display-empty&quot;);
	}
	else
	{
		fnRemoveClassWithSameStyle(&quot;#naviButtons&quot;,&quot;ph-wo-display&quot;,&quot;&quot;, false);
		$(&quot;#naviButtons&quot;).addClass(&quot;ph-wo-display-none&quot;);
	}
	fnSetEmptyTdHeight($(&quot;#right_table&quot;).outerHeight(), true);
	if(&quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot; == &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;)
	{
		fnShowWorkTypeScreen(false);
	}
});

function fnSetEmptyTdHeight(rightTabHeight, onload)
{
	var subactionType = &quot; , &quot;'&quot; , &quot;modify&quot; , &quot;'&quot; , &quot;;
	var trHeight = ($(&quot;#tab_table&quot;).find(&quot;tr:visible&quot;).length - 1) * 22;
	var borderHeight = ($(&quot;#tab_table&quot;).outerHeight()) - (trHeight + $(&quot;#empty_td&quot;).height());
	if(borderHeight &lt; 0)
    	borderHeight = 0;
	var objAgent = navigator.userAgent;
	var objbrowserName  = navigator.appName;
	var objfullVersion  = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;+parseFloat(navigator.appVersion); 
	if ((objOffsetVersion=objAgent.indexOf(&quot;Firefox&quot;))!=-1) 
	{
		 objbrowserName = &quot;Firefox&quot;;
	 }
	 if (objbrowserName == &quot; , &quot;'&quot; , &quot;Firefox&quot; , &quot;'&quot; , &quot;) 
	 {
		 if(subactionType == &quot;modify&quot;)
		{
			 remHt = rightTabHeight  - (trHeight+50);
		}
		 else if(subactionType == &quot;add&quot;)
		 {
			 remHt = rightTabHeight  - (trHeight+24);
		 }
		
	 }
	 else
	 {
		 remHt = rightTabHeight - (trHeight + borderHeight);
	 }  
	if((remHt + trHeight + borderHeight) == rightTabHeight)
		remHt = remHt - 1;
	remHt = getEmptyTdHeightInSafari(remHt);
	$(&quot;#empty_td&quot;).height(remHt);
}



	



	.disclaimerDiv
	{
		padding: 10px 50px 30px 50px;
		font-size: 10px;
	}



	
	The information made available through this web portal is intended solely for authorized users and for general informational purposes. While we strive to ensure that the data and reports are accurate and up to date, we make no warranties or representations-express or implied-regarding the completeness, accuracy, reliability, or fitness for a particular purpose of the information presented. Unauthorized access, use, distribution, copying, or modification of any content or data from this platform is strictly prohibited and may be unlawful. If you are not an intended or authorized user, please exit immediately and notify the administrator. We take precautions to safeguard this platform against viruses and malicious code; however, users are responsible for performing their own virus scans before downloading any files. We do not accept any liability for loss or damage arising from the use of this platform or the information accessed through it. By continuing to use this system, you agree to these terms and acknowledge your responsibility for any actions taken based on the information provided herein.



$(document).ready(function () {
    placeDisclaimer();
});

$(window).resize(placeDisclaimer);

function placeDisclaimer() 
{
    var contentHeight = $(&quot;.table-responsive&quot;).outerHeight(true);
    var windowHeight = $(window).height();
    var disclaimerHeight = $(&quot;.disclaimerDiv&quot;).outerHeight(true);

    fnRemoveClassWithSameStyle(&quot;#contentFrame&quot;, &quot;ph-wo-min-height&quot;, &quot;&quot;, false);
	
    if (contentHeight + disclaimerHeight &lt; windowHeight) 
    {
        var dynamicMargin = windowHeight - (contentHeight + disclaimerHeight) - 80;
		var className = (&quot;ph-wo-margin-top-&quot; +dynamicMargin + &quot;px&quot;).replace(&quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;);
		addDynamicCSS(&quot;xZrLNGLeef26xpNnnF1igA==&quot;, className , { &quot;margin-top&quot;:  dynamicMargin +&quot; , &quot;'&quot; , &quot;px !important&quot; , &quot;'&quot; , &quot;});
        $(&quot;.disclaimerDiv&quot;).addClass(className);
    } 
}



	
		
			
				
					
						Help
						
							
								
							
						
					
					
			
      		
		
	

		

    



	
		
			
				×
				Warning
      		
      		
      			 
      			
      		
      		
        		OK
      		
		
	



	
		
			
				×
				Confirmation
      		
      		
      			 
      			
      		
      		
      			NO
        		YES
      		
		
	









function fnHelp()
{
	
	 
		alert(&quot;Help cannot be shown as it is not been configured.&quot;);
	 
}
  

document.onclick = window.parent.fnHideSideBar;
gloabalDeviceWidth = window.parent.gloabalDeviceWidth;

function adjustDataTableWidth()
{
	if(gloabalDeviceWidth &lt;= 1024)
	{
		var style=document.createElement(&quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;);
		style.type=&quot;text/css&quot;;
		style.setAttribute(&quot; , &quot;'&quot; , &quot;nonce&quot; , &quot;'&quot; , &quot;, &quot;xZrLNGLeef26xpNnnF1igA==&quot;);
		var css=&quot; , &quot;'&quot; , &quot;.admin-responsivetable{max-width:&quot; , &quot;'&quot; , &quot;+gloabalDeviceWidth - 75+&quot; , &quot;'&quot; , &quot;px !important};&quot; , &quot;'&quot; , &quot;
		style.appendChild(document.createTextNode(css));
		document.head.appendChild(style);
	}
}

function adjustContentOverflow(elementId)
{
	if(gloabalDeviceWidth &lt;= 1024)
	{
		var style=document.createElement(&quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;);
		style.type=&quot;text/css&quot;;
		style.setAttribute(&quot; , &quot;'&quot; , &quot;nonce&quot; , &quot;'&quot; , &quot;, &quot;xZrLNGLeef26xpNnnF1igA==&quot;);
		var css=&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot;+elementId+&quot; , &quot;'&quot; , &quot;{max-width:&quot; , &quot;'&quot; , &quot;+gloabalDeviceWidth - 75+&quot; , &quot;'&quot; , &quot;!important};&quot; , &quot;'&quot; , &quot;
		style.appendChild(document.createTextNode(css));
		document.head.appendChild(style);
	}
}

$(document).ready(function() {
	window.parent.hideWindowScroll();
	window.parent.hideBottomScroll();
	window.parent.fnSetHelpPath(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
	hideIFrameLoading();
	
	// forcing slimscroll without mouse movement
	$(&quot; , &quot;'&quot; , &quot;#adminBodyContent&quot; , &quot;'&quot; , &quot;).mouseover();
	$(&quot; , &quot;'&quot; , &quot;#adminBodyContent&quot; , &quot;'&quot; , &quot;).focus(); 

	setAlertHideTimer();
	fnRemoveClassWithSameStyle(&quot;.disclaimerDiv&quot;, &quot;ph-wo-display&quot;, &quot;&quot;, false);
	if(isIE8 != -1)
	{
		$(&quot;.input-right-content-textarea&quot;).addClass(&quot;input-right-content-textarea_ie&quot;);
	}
});
	
$(function() {
    var e = 300,
        a = 500;
    navigator.userAgent.match(/iPhone|iPad|iPod/i) ? $(&quot;#adminBodyContent&quot;).bind(&quot;touchend touchcancel touchleave&quot;, function(t) {
        $(this).scrollTop() > e ? $(&quot;#scroll-to-top-admin&quot;).fadeIn(a) : $(&quot;#scroll-to-top-admin&quot;).fadeOut(a)
    }) : $(&quot;#adminBodyContent&quot;).scroll(function() {
    	checkAndToggleBottomScroll();
        $(this).scrollTop() > e ? $(&quot;#scroll-to-top-admin&quot;).fadeIn(a) : $(&quot;#scroll-to-top-admin&quot;).fadeOut(a)
    }), $(&quot;#scroll-to-top-admin&quot;).click(function(e) {
    	$(&quot; , &quot;'&quot; , &quot;#adminBodyContent&quot; , &quot;'&quot; , &quot;).focus(); 
        return e.preventDefault(), $(&quot;#adminBodyContent&quot;).animate({
            scrollTop: 0
        }, a), !1
    })
}); 

function isSessionExpired(response)
{
	return window.parent.isSessionExpired(response);
}

var alertHideInterval;
function setAlertHideTimer()
{
	alertHideInterval = window.setTimeout(function() {
		hideAlertMessage();
	}, 20000);
	
	$(&quot;.config-alert > a&quot;).click(function() {
		hideAlertMessage();
		return false;
	});
}

function hideAlertMessage() {
	$(&quot;.config-alert&quot;).fadeTo(500, 0).slideUp(500, function(){
        $(this).remove();
    });
	
	clearTimeout(alertHideInterval);
}

function fnShowDialogWithNote(label, key, note)
{
	var url = &quot;/phworkoutput/HelpFramework.htm?subActionType=helpDialog&quot;;

	$.ajax({
			url: url,
			global: false,
			headers	: {&quot; , &quot;'&quot; , &quot;X-Content-Security-Policy-Nonce&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;xZrLNGLeef26xpNnnF1igA==&quot; , &quot;'&quot; , &quot;},
			type: &quot;POST&quot;,
			data: ({
					&quot;menuItemId&quot;: form.menuItemId.value,
					&quot;label&quot; : replaceBackJSParamXMLEntities(label),
					&quot;key&quot; : key,
					&quot;note&quot; : note
				}),
			success: function(resp)
			{
				if(!isSessionExpired(resp))
				{
					$(&quot;#showData&quot;).html(resp);
					fnRemoveClassWithSameStyle(&quot;#helpDialog&quot;, &quot;ph-wo-display&quot;, &quot;&quot;, false);
					$(&quot;#helpDialog&quot;).modal(&quot;show&quot;);
		    	}	
			},
			error :  function(msg,arg1,arg2)
			{
				processErrorStatus(msg.status, &quot;/phworkoutput&quot;);
				return false;
			}
	});
}

function fnShowDialog(label, key)
{
	var url = &quot;/phworkoutput/HelpFramework.htm?subActionType=helpDialog&quot;;

	$.ajax({
			url: url,
			global: false,
			headers	: {&quot; , &quot;'&quot; , &quot;X-Content-Security-Policy-Nonce&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;xZrLNGLeef26xpNnnF1igA==&quot; , &quot;'&quot; , &quot;},
			type: &quot;POST&quot;,
			data: ({
					&quot;menuItemId&quot;: &quot;302&quot;,
					&quot;label&quot; : replaceBackJSParamXMLEntities(label),
					&quot;key&quot; : key
				}),
			success: function(resp)
			{
				if(!isSessionExpired(resp))
				{
					$(&quot;#showData&quot;).html(resp);
					fnRemoveClassWithSameStyle(&quot;#helpDialog&quot;, &quot;ph-wo-display&quot;, &quot;&quot;, false);
					$(&quot;#helpDialog&quot;).modal(&quot;show&quot;);
		    	}	
			},
			error :  function(msg,arg1,arg2)
			{
				processErrorStatus(msg.status, &quot;/phworkoutput&quot;);
			    return false;
			}
	});
}

function fnShowPopup(xmlHttp, label, key)
{
	if (xmlHttp.readyState == 4)
	{
		
	}
}

$(document).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;.dropdown-menu&quot; , &quot;'&quot; , &quot;, function(e) {
	e.stopPropagation();
});

function showIFrameLoading()
{
	window.parent.showLoadingModel();
}

function hideIFrameLoading()
{
	window.parent.hideLoadingModel();
}

$(&quot;.adminform&quot;).submit(function(e) {
	if(!isAlertModalVisible)
	{
		showIFrameLoading();
	}
});

function doSubmit(form) {
	showIFrameLoading();
	form.submit();
}

function fnGetAjaxLoader()
{
	divBuffer = &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;ph-wo-text-align-center ph-wo-width-100percentage&quot;>&quot; , &quot;'&quot; , &quot;;
	divBuffer += &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/ajax-loader.gif&quot;>&quot; , &quot;'&quot; , &quot;;
	divBuffer +=&quot; , &quot;'&quot; , &quot;&lt;/div>&quot; , &quot;'&quot; , &quot;;
	  
	return divBuffer;
}

function fnGetPopUpLoader()
{
	divBuffer = &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;ph-wo-text-align-center ph-wo-width-100percentage ph-wo-position-absolute ph-wo-top-48px&quot;>&quot; , &quot;'&quot; , &quot;;
	divBuffer += &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/loader-trans.gif&quot;>&quot; , &quot;'&quot; , &quot;;
	divBuffer +=&quot; , &quot;'&quot; , &quot;&lt;/div>&quot; , &quot;'&quot; , &quot;;
	  
	return divBuffer;
} 

function fnGetPopUpLoader(winObj)
{
	var loaderDiv = winObj.document.createElement(&quot; , &quot;'&quot; , &quot;div&quot; , &quot;'&quot; , &quot;);
    loaderDiv.style.textAlign = &quot; , &quot;'&quot; , &quot;center&quot; , &quot;'&quot; , &quot;;
    loaderDiv.style.width = &quot; , &quot;'&quot; , &quot;100%&quot; , &quot;'&quot; , &quot;;
    loaderDiv.style.position = &quot; , &quot;'&quot; , &quot;absolute&quot; , &quot;'&quot; , &quot;;
    loaderDiv.style.top = &quot; , &quot;'&quot; , &quot;48%&quot; , &quot;'&quot; , &quot;;
    loaderDiv.setAttribute(&quot; , &quot;'&quot; , &quot;nonce&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;xZrLNGLeef26xpNnnF1igA==&quot; , &quot;'&quot; , &quot;);
 
    var loaderImg = winObj.document.createElement(&quot; , &quot;'&quot; , &quot;img&quot; , &quot;'&quot; , &quot;);
    loaderImg.src = &quot;/phworkoutput/images/loader-trans.gif&quot;;
    loaderImg.setAttribute(&quot; , &quot;'&quot; , &quot;nonce&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;xZrLNGLeef26xpNnnF1igA==&quot; , &quot;'&quot; , &quot;);
    loaderDiv.appendChild(loaderImg);
    return loaderDiv;
}

function checkAndToggleBottomScroll()
{
	if($(&quot;.pagebanner&quot;).is(&quot;:visible&quot;))
    {
		var scrollPos = $(&quot;#adminBodyContent&quot;).scrollTop() + $(window).height();
		var remHeight = ($(&quot; , &quot;'&quot; , &quot;.table-responsive&quot; , &quot;'&quot; , &quot;).position().top + $(&quot; , &quot;'&quot; , &quot;#adminBodyContent&quot; , &quot;'&quot; , &quot;).scrollTop()) + $(&quot; , &quot;'&quot; , &quot;.table-responsive&quot; , &quot;'&quot; , &quot;).outerHeight(true);
    	if(scrollPos >= remHeight)
      	{
        	hideBottomScroll();
      	}
        else
      	{
    		showBottomScroll();
      	} 
    }
	else
	{
		hideBottomScroll();
	} 
}


function hideBottomScroll()
{
	fnRemoveClassWithSameStyle(&quot;.scroll-bottom&quot;, &quot;ph-wo-display&quot;, &quot;&quot;, false);
	$(&quot;.scroll-bottom&quot;).addClass(&quot;ph-wo-display&quot;);
}

function showBottomScroll()
{
	if($(&quot;#scrollInnerDiv&quot;).width() > 0)
	{
		fnRemoveClassWithSameStyle(&quot;.scroll-bottom&quot;, &quot;ph-wo-display&quot;, &quot;&quot;, false);
	}
	if($(&quot; , &quot;'&quot; , &quot;.pagebanner&quot; , &quot;'&quot; , &quot;).is(&quot;:visible&quot;))
	{
		$(&quot;.scroll-bottom&quot;).scrollLeft($(&quot;.table-responsive:visible&quot;).scrollLeft());
	}
}

function getEmptyTdHeightInSafari(tdHeight)
{
	if($.browser.safari){
        tdHeight = tdHeight - 12;
        return tdHeight;
    }
	else
	{
		return tdHeight;
	}
}

function removeSlimScroll(objectId) 
{
    if($(&quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot;+objectId).parent().prop(&quot; , &quot;'&quot; , &quot;className&quot; , &quot;'&quot; , &quot;) == &quot; , &quot;'&quot; , &quot;slimScrollDiv&quot; , &quot;'&quot; , &quot;)
    {
		$(&quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot;+objectId).slimScroll().unbind(&quot; , &quot;'&quot; , &quot;slimscroll&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot;+objectId).parent().replaceWith($(&quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot;+objectId));
		fnRemoveClassWithSameStyle(&quot;.&quot;+objectId, &quot;ph-wo-overflow&quot;, &quot;&quot;, false);
		fnRemoveClassWithSameStyle(&quot;.&quot;+objectId, &quot;ph-wo-height&quot;, &quot;&quot;, false);
    }
}

function addScrollToApplicationBody()
{
	var frameHeight = parseInt($(window).height());
	var style=document.createElement(&quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;);
	style.type=&quot;text/css&quot;;
	style.setAttribute(&quot; , &quot;'&quot; , &quot;nonce&quot; , &quot;'&quot; , &quot;, &quot;xZrLNGLeef26xpNnnF1igA==&quot;);
	var css=&quot; , &quot;'&quot; , &quot;#bodyContent{min-height:&quot; , &quot;'&quot; , &quot;+frameHeight+&quot; , &quot;'&quot; , &quot; !important};&quot; , &quot;'&quot; , &quot;
	style.appendChild(document.createTextNode(css));
	document.head.appendChild(style);
}  

&quot;) or . = concat(&quot;
	








%>


.configtable tbody tr td {
    padding: 0px; 
    height: 38px;
    border: none;
    padding-top: 0px;
    padding-bottom: 0px;
    text-align: left;
}

 #tableDivId
 {
 	margin-left: 5px;
 	padding-right: 10px;
 }
 
 .disclaimerDiv {
    padding: 10px 11px 30px 35px !important;
}













	
		













	
		×
		
	



	
	
	
	


	
		
			
				
					
						
							
							
							
								
							
							
						
						
							
								
										
						
					
					
			
			
			
			
		
	



 $( document ).ready(function() {
	fnFadeoutEvent(&quot;#dialogsuccessspan&quot;, 20000, &quot;ph-wo-display-none&quot;);
	fnFadeoutEvent(&quot;#dialogfailurespan&quot;, 20000, &quot;ph-wo-display-none&quot;);
	fnFadeoutEvent(&quot;#dialogdependencyspan&quot;, 20000, &quot;ph-wo-display-none&quot;);
	
	if ($(&quot;#dialogsuccessspan&quot;).is(&quot; , &quot;'&quot; , &quot;:hidden&quot; , &quot;'&quot; , &quot;)) 
	{
		fnRemoveClassWithSameStyle(&quot;#dialogsuccessspan&quot;, &quot;ph-wo-height&quot;, &quot;&quot;, false)
	 	$(&quot;#dialogsuccessspan&quot;).addClass(&quot;ph-wo-height-0px&quot;);
	}
	if ($(&quot;#dialogfailurespan&quot;).is(&quot; , &quot;'&quot; , &quot;:hidden&quot; , &quot;'&quot; , &quot;)) 
	{
		fnRemoveClassWithSameStyle(&quot;#dialogfailurespan&quot;, &quot;ph-wo-height&quot;, &quot;&quot;, false)
	 	$(&quot;#dialogfailurespan&quot;).addClass(&quot;ph-wo-height-0px&quot;);
	}
});

function fnSucessClose()
{
	fnRemoveClassWithSameStyle(&quot;#dialogsuccessspan&quot;, &quot;ph-wo-height&quot;, &quot;&quot;, false)
 	$(&quot;#dialogsuccessspan&quot;).addClass(&quot;ph-wo-height-0px&quot;);
}

function fnFailureClose()
{
	fnRemoveClassWithSameStyle(&quot;#dialogfailurespan&quot;, &quot;ph-wo-height&quot;, &quot;&quot;, false)
 	$(&quot;#dialogfailurespan&quot;).addClass(&quot;ph-wo-height-0px&quot;);
}

function fnShowDependency()
{
	$(&quot; , &quot;'&quot; , &quot;.modal&quot; , &quot;'&quot; , &quot;).draggable({
	    handle: &quot;.modal-header&quot;
	});
	
	var form;
	if (&quot; , &quot;'&quot; , &quot;CategoryForm&quot; , &quot;'&quot; , &quot; == &quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;)
	{
		//
	}
	else
	{
		form = document.CategoryForm;
	}
	var url = &quot;/phworkoutput/getDataDependency/getDependencyList.htm?deleteRecordId=&quot;+-1;
	windowTitle = fnGetWindowName();
	var winObj =  window.open(&quot;&quot;, windowTitle, &quot;status=1, toolbar=0, scrollbars=1, resizable=1, width=1330, height=600, left=20, top=20&quot;);
	winObj.document.body.appendChild(fnGetDependencyPopUpLoader());
	window.parent.parent.fnAddToChildWindows(winObj);
	tempAction = form.action;
    tempTarget = form.target;
    form.action = url;
    form.target = windowTitle;
    form.submit();
    winObj.focus();
    form.action = tempAction;
    form.target = tempTarget;
}

function fnGetDependencyPopUpLoader()
{
	var loaderDiv = window.parent.document.createElement(&quot; , &quot;'&quot; , &quot;div&quot; , &quot;'&quot; , &quot;);
    loaderDiv.style.textAlign = &quot; , &quot;'&quot; , &quot;center&quot; , &quot;'&quot; , &quot;;
    loaderDiv.style.width = &quot; , &quot;'&quot; , &quot;100%&quot; , &quot;'&quot; , &quot;;
    loaderDiv.style.position = &quot; , &quot;'&quot; , &quot;absolute&quot; , &quot;'&quot; , &quot;;
    loaderDiv.style.top = &quot; , &quot;'&quot; , &quot;48%&quot; , &quot;'&quot; , &quot;;
    loaderDiv.setAttribute(&quot; , &quot;'&quot; , &quot;nonce&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;xZrLNGLeef26xpNnnF1igA==&quot; , &quot;'&quot; , &quot;);
 
    var loaderImg =  window.parent.document.createElement(&quot; , &quot;'&quot; , &quot;img&quot; , &quot;'&quot; , &quot;);
    loaderImg.src = &quot;/phworkoutput/images/loader-trans.gif&quot;;
    loaderImg.setAttribute(&quot; , &quot;'&quot; , &quot;nonce&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;xZrLNGLeef26xpNnnF1igA==&quot; , &quot;'&quot; , &quot;);
    loaderDiv.appendChild(loaderImg);
    return loaderDiv;
}

/* function fnDeleteDependency()
{
	var primaryKeyId = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	var moduleName = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	var menuItemId = &quot; , &quot;'&quot; , &quot;302&quot; , &quot;'&quot; , &quot;;
	var formDefinitionId  = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	
	var actionName = &quot;/phworkoutput/getDataDependency/deleteDataDependency.htm?primaryKeyId=&quot;+primaryKeyId+&quot;&amp;moduleName=&quot;+moduleName+&quot;&amp;menuItemId=&quot;+menuItemId+&quot;&amp;formDefinitionId=&quot;+formDefinitionId;
	doAjaxCall(actionName, false, false);
} */


$(document).off(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;hideMsg&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;hideMsg&quot;]&quot; , &quot;'&quot; , &quot;, function() {
	
	fnRemoveClassWithSameStyle(&quot;.internalmessagefailure&quot;, &quot;ph-wo-display&quot;, &quot;&quot;, false);
	$(&quot;.internalmessagefailure&quot;).addClass(&quot;ph-wo-display-none&quot;);
	fnRemoveClassWithSameStyle(&quot;#displayErrorMessage&quot;, &quot;ph-wo-display&quot;, &quot;&quot;, false);
	$(&quot;#displayErrorMessage&quot;).addClass(&quot;ph-wo-display-none&quot;);
});

$(document).off(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnSucessClose&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnSucessClose&quot;]&quot; , &quot;'&quot; , &quot;, function() {
	fnSucessClose();
});

$(document).off(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnFailureClose&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnFailureClose&quot;]&quot; , &quot;'&quot; , &quot;, function() {
	fnFailureClose();
});

$(document).off(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnShowDependency&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnShowDependency&quot;]&quot; , &quot;'&quot; , &quot;, function() {
	fnShowDependency();
});


	
	
		







	
		
			
				
				
					
					
						Administration
					
				
				
			
		
		
			
				
				
					
					
						Work Output
					
				
				
			
		
		
			
				
					
				
				
				
					
					
						Work Type Category
					
				
				
					
				
			
		
		
			
				
				
				
					
					
						Work Type Category Details
					
				
				
			
		
		
		
		
		
			[Development Metrics] 
		
		
		
		
	




		
		
			
			 	
			
					
							BACK
					
					
					
			
		
	



	
	function fnBackToSummary()
	{
		form = document.WODataCollectionForm;
		if($(&quot;#isFistTab&quot;).val() != &quot;true&quot; || !document.WODataCollectionForm[1])
		{
			form = document.WODataCollectionForm[1];
		} 
		if(isNaN($(&quot;#lastXDays&quot;).val()))
		{
			$(&quot;#lastXDays&quot;).val(&quot;&quot;);
		}
		form.action = &quot;/phworkoutput/DataCollectionSummary.htm&quot;;
		doSubmit(form);
	}

	function fnRedirectPage(actionName)
	{
		form = document.CategoryForm;
		var tempAction = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
		if(typeof form == &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot; &amp;&amp; actionName == &quot;/phworkoutput/WorkTypeSummary.htm&quot;)
		{
			fnBackToWorkTypeSummary();
		}
		else
		{
			if (form.searchXML != null &amp;&amp; form.searchXML != &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot;)
			{
				form.searchXML.value=&quot;&quot;;
			}
			tempAction = form.action;
			document.forms[0].action = actionName;
		}
		doSubmit(document.forms[0]);
		form.action = tempAction;
	}

	function fnBackToParameterSummary()
	{
		form = document.ParameterForm;
		document.forms[0].action = &quot;/phworkoutput/WOParameterSummary.htm&quot;;
		doSubmit(document.forms[0]);
	}
	
	function fnBackToWOMetricSummary()
	{
		form = document.WOMetricForm;
		form.action = &quot;/phworkoutput/WOMetricSummary.htm&quot;;
		doSubmit(form);
	}
	
	
	function fnBackToWorkTypeSummary()
	{
		form = document.WorkTypeForm;
		document.forms[0].action = &quot;/phworkoutput/WorkTypeSummary.htm&quot;;
		doSubmit(document.forms[0]);
	}

	function fnBackToCategorySummary()
	{
		form = document.CategoryForm;
		document.forms[0].action = &quot;/phworkoutput/WorkOutputCategorySummary.htm&quot;;
		doSubmit(document.forms[0]);
	}
	
	function fnBackToWOWorkTypeAttributeSummary()
	{
		form = document.ConfigureWorkTypeAttrForm;
		document.forms[0].action = &quot;/phworkoutput/WOWorktypeAttributeSummary.htm&quot;;
		doSubmit(document.forms[0]);
	}

	$(&quot;[name = back]&quot;).click(function(){
		$(this).attr(&quot;disabled&quot;, &quot;disabled&quot;);
	});
	
	$(&quot;#reDirectPageArg0, #reDirectPageArg1, #reDirectPageArg2, #reDirectPageArg3&quot;).click(function()
 	{
	 	 var value = $(this).data(&quot; , &quot;'&quot; , &quot;action&quot; , &quot;'&quot; , &quot;);
	   	 if(value != null &amp;&amp; value != &quot;&quot; &amp;&amp; value != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
		 {
	   		fnRedirectPage(value);
		 }
    });

	$(document).off(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnQuickLinks&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnQuickLinks&quot;]&quot; , &quot;'&quot; , &quot;, function() {
		fnQuickLinks();
	});

	$(document).off(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnDeleteQuickLink&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnDeleteQuickLink&quot;]&quot; , &quot;'&quot; , &quot;, function() {
		fnDeleteQuickLink();
	});

	$(document).off(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnBackToSummary&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnBackToSummary&quot;]&quot; , &quot;'&quot; , &quot;, function() {
		fnBackToSummary();
	});

	$(document).off(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnBackToParameterSummary&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnBackToParameterSummary&quot;]&quot; , &quot;'&quot; , &quot;, function() {
		fnBackToParameterSummary();
	});

	$(document).off(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnBackToWorkTypeSummary&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnBackToWorkTypeSummary&quot;]&quot; , &quot;'&quot; , &quot;, function() {
		fnBackToWorkTypeSummary();
	});

	$(document).off(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnBackToCategorySummary&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnBackToCategorySummary&quot;]&quot; , &quot;'&quot; , &quot;, function() {
		fnBackToCategorySummary();
	});

	$(document).off(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnBackToWOMetricSummary&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnBackToWOMetricSummary&quot;]&quot; , &quot;'&quot; , &quot;, function() {
		fnBackToWOMetricSummary();
	});
	
	$(document).off(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnBackToWOWorkTypeAttributeSummary&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnBackToWOWorkTypeAttributeSummary&quot;]&quot; , &quot;'&quot; , &quot;, function() {
		fnBackToWOWorkTypeAttributeSummary();
	});
	
	$(document).off(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnRedirectPage&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnRedirectPage&quot;]&quot; , &quot;'&quot; , &quot;, function() {
		var param = $(this).data(&quot;param&quot;);
		fnRedirectPage(param);
	});
	
	$(&quot;.historyback&quot;).click(function() 
	{
	    history.back(-1); 
	});

	
	
		
			
				
					
						
							 
								
									
									1
										Basic Details
									
								
							
							
								
									2
										Work Type Mapping
									
								
							
							
						
							
								 
								
							
					  
					
					
						
							
								
									

#mainTable > tbody > tr
{
	height: 0px !important;
}

#target_table > tbody > tr
{
	height: 0px !important;
}

table
{
  border-collapse : unset;
}

th
{
	border-right: none !important;
}

#CommonDataTableId_wrapper > #CommonDataTableId_filter
{
	border-bottom: none !important;
}

#CommonDataTableId
{
	border-top: 1px solid #e5e9ea;
	margin-top: 0px !important;
	border-right: 1px solid #e5e9ea;
}

table.dataTable td
{
	border-top: 1px solid #e5e9ea;
	border-left: 1px solid #e5e9ea;
	padding-left: 8px;
	padding-right: 10px;
}

.dataTables_info
{
	margin-left: 0px !important;
	border-top: 1px solid #e5e9ea;
}

.dataTables_paginate 
{
	border-top: 1px solid #e5e9ea;
}

.dataTables_empty
{
	vertical-align: middle;
}

.userSummaryTable
{
	margin: 0px;
	height: 550px;
	overflow-y: auto;
	overflow-x: hidden;
	padding-right: 5px;
	padding-bottom: 5px;
}

.chkbox-padding
{
	padding-bottom: 5px;
}

#right_table > tbody > tr > td
{
	border-bottom: 1px solid #e5e9ea;
}














        
                
                
        


























 table.dataTable thead .sorting:after
{
   content: &quot;&quot; !important;
}
table.dataTable thead .sorting:before
{
    content: &quot;&quot; !important;
}

table.dataTable thead .sorting_asc:after {
    content: url(/phworkoutput/images/sort_asc.png) !important;
   	opacity:1 !important;
    margin-bottom: 5px;
    margin-right:-10px;
    top: 20% !important;
}
table.dataTable thead .sorting_desc:after {
    content: url(/phworkoutput/images/sort_desc.png) !important;
    opacity:1 !important;
    margin-bottom: 5px;
    margin-right:-10px;
    top: 20% !important;
}
v.dataTables_scrollHead table.dataTable {
		    width:100%;
		}
		
.no-footer
{
	width:100% !important;
} 

.form-inline .form-control 
{
	vertical-align: unset !important;
}

table.dataTable
{
	border-collapse: collapse !important; 
}






















var dynPageLength;
var dynExportColumns;
var dynTitleHeaderName;
var dynFileName;
var dynPdfPageSize;
var dynSortingIndex;
var dynNumericAlignColArr;
var dynWidthArr;
var isMultiOrgAdmin;
var reportScheduleReq;
var sortingOrder;
var recordSize;

var isPaginationExtendEnable = false;
var lengthMenu = isPaginationExtendEnable ? [10, 20, 50, 100, 200, 500, 1000]  : [10, 20, 50, 100, 200];

/* Build Data Tabel */
function buildDataTableStructure(jsonData, pageLength, columns, sortingIndex)
{
	buildDataTableStructureImproved(jsonData, pageLength, columns, sortingIndex, &quot;CommonDataTableId&quot;)
}

/* Build Data Tabel by passing Dynamic Table Id*/
function buildDataTableStructureImproved(jsonData, pageLength, columns, sortingIndex, dynTableId)
{
	var isReqSorting = true;
	if(sortingIndex &lt; 0)
	{
		isReqSorting  = false;
		sortingIndex = 0;
	}
	var table = $(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; +dynTableId+&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;).DataTable( {
        data: jsonData,
        deferRender:    true,
        &quot;pageLength&quot;: pageLength,
        &quot;ordering&quot; : isReqSorting,
        &quot;autoWidth&quot;:false,
        &quot;scrollX&quot;: true,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px&quot; , &quot;'&quot; , &quot;> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100&quot; , &quot;'&quot; , &quot;>[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
                &quot;previous&quot;:   &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;
            },
        },
         columns: columns,
         &quot;order&quot;: [[ sortingIndex, &quot; , &quot;'&quot; , &quot;asc&quot; , &quot;'&quot; , &quot; ]],
         dom: &quot; , &quot;'&quot; , &quot;Blfrtip&quot; , &quot;'&quot; , &quot;,
         buttons: [
            //Empty for no exports
         ]
    } );
}

/* Build Data Tabel With Default Data Table Export */
function buildDataTableStructureWithExports(jsonData, pageLength, columns, exportColumns, titleHeaderName, fileName, pdfPageSize, sortingIndex)
{
	var isReqSorting = true;
	if(sortingIndex &lt; 0)
	{
		isReqSorting  = false;
		sortingIndex = 0;
	}
	var excelReportType = $(&quot;#excelReportType&quot;).val();
	excelReportType = &quot;.&quot; + excelReportType.toLowerCase();
	var disclaimerText = $(&quot;#disclaimerText&quot;).val();
	titleHeaderName = replaceBackXMLEntities(titleHeaderName);
	fileName = replaceBackXMLEntities(fileName);
	var table = $(&quot; , &quot;'&quot; , &quot;#CommonDataTableId&quot; , &quot;'&quot; , &quot;).DataTable( {
        data: 	jsonData,
        deferRender:    true,
        &quot;pageLength&quot;: pageLength,
        &quot;ordering&quot; : isReqSorting,
        &quot;scrollX&quot;: true,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	 &quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
             &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px&quot; , &quot;'&quot; , &quot;> Displaying _START_ to _END_ &lt;label>&quot;,
             &quot;infoEmpty&quot;: &quot;No records to display&quot;,
             &quot;infoFiltered&quot;: &quot;&lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100&quot; , &quot;'&quot; , &quot; >[Filtered from _MAX_ records] &lt;/label>&quot;,
             &quot;search&quot;:         &quot;Search&quot;,
             &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
             &quot;paginate&quot;: { 
                &quot;next&quot;:       &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
                &quot;previous&quot;:   &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;
            },
        },
         columns: columns,
         &quot;order&quot;: [[ sortingIndex, &quot; , &quot;'&quot; , &quot;asc&quot; , &quot;'&quot; , &quot; ]],
         dom: &quot; , &quot;'&quot; , &quot;Blfrtip&quot; , &quot;'&quot; , &quot;,
         buttons: [
        	 {
        		 text:   &quot; , &quot;'&quot; , &quot;&lt;label class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot; class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>|&amp;nbsp;&amp;nbsp; Export:&amp;nbsp;&lt;/label>&quot; , &quot;'&quot; , &quot;,
        		 autoClose: &quot; , &quot;'&quot; , &quot;false&quot; , &quot;'&quot; , &quot;,
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;
        	 },
             {
                 extend: &quot; , &quot;'&quot; , &quot;excelHtml5&quot; , &quot;'&quot; , &quot;,
                 exportOptions: {
                 	columns: exportColumns,
                 	orthogonal: &quot; , &quot;'&quot; , &quot;export&quot; , &quot;'&quot; , &quot;,
                 	 stripHtml: true
                 },
                 filename: fileName,
                 text:   &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/excel.png&quot; title=&quot;Export to Excel&quot; align=&quot;middle&quot; class=&quot;ph-wo-vertical-align-middle ph-wo-cursor-pointer&quot;>&quot; , &quot;'&quot; , &quot;,
                 autoClose: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;,
                 extension: excelReportType,
                 createEmptyCells: true,
                 title : titleHeaderName,
                 messageBottom: function () {
                     return disclaimerText;
                 	},
                     customize: function ( xlsx ) {
						var sheet = xlsx.xl.worksheets[&quot; , &quot;'&quot; , &quot;sheet1.xml&quot; , &quot;'&quot; , &quot;];
						$(&quot; , &quot;'&quot; , &quot;row c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;25&quot; , &quot;'&quot; , &quot; );
                        $(&quot; , &quot;'&quot; , &quot;c[r=A1] t&quot; , &quot;'&quot; , &quot;, sheet).text( titleHeaderName );
                        $(&quot; , &quot;'&quot; , &quot;row:first c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;27&quot; , &quot;'&quot; , &quot; ); 
                        $(&quot; , &quot;'&quot; , &quot;row:eq(1) c&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;27&quot; , &quot;'&quot; , &quot;);
                        
                        if(disclaimerText != null &amp;&amp; disclaimerText != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
                        {
                        	$(&quot; , &quot;'&quot; , &quot;row:last c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;55&quot; , &quot;'&quot; , &quot; );
	                        $(&quot; , &quot;'&quot; , &quot;row:last&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;customHeight&quot; , &quot;'&quot; , &quot;, 1);
	                        $(&quot; , &quot;'&quot; , &quot;row:last&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;ht&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;150&quot; , &quot;'&quot; , &quot;);
                        }
                      },
                      
                 button: [&quot; , &quot;'&quot; , &quot;excel&quot; , &quot;'&quot; , &quot;]

             },
             {
                 extend: &quot; , &quot;'&quot; , &quot;pdfHtml5&quot; , &quot;'&quot; , &quot;,
                 orientation: &quot; , &quot;'&quot; , &quot;landscape&quot; , &quot;'&quot; , &quot;,
                 exportOptions: {
                 	columns: exportColumns,
                 	orthogonal: &quot; , &quot;'&quot; , &quot;export&quot; , &quot;'&quot; , &quot;,
                 	 stripHtml: true
                 },
                 text:      &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/pdf.png&quot; title=&quot;Export to PDF&quot; align=&quot;middle&quot; class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>&quot; , &quot;'&quot; , &quot;,
                 filename: fileName,
                 autoClose: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 footer:&quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 messageBottom: function () {
                     return disclaimerText;
                  	},
               	 customize: function ( doc ) {
               		if(exportColumns.length &lt;= 4 || pdfPageSize == &quot; , &quot;'&quot; , &quot;A4&quot; , &quot;'&quot; , &quot;)
               		{
	               		doc.content[1].table.widths = Array(doc.content[1].table.body[0].length + 1).join(&quot; , &quot;'&quot; , &quot;*&quot; , &quot;'&quot; , &quot;).split(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
               		}
               		
               	   var cols = [];
               	   cols[0] = {text: &quot; , &quot;'&quot; , &quot;© JaMocha Tech Pvt. Ltd. 2009-2026&quot; , &quot;'&quot; , &quot;, alignment: &quot; , &quot;'&quot; , &quot;center&quot; , &quot;'&quot; , &quot;, fontSize:&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;, margin:[0, 0, 0, 0] };
               	   var objFooter = {};
               	   objFooter[&quot; , &quot;'&quot; , &quot;columns&quot; , &quot;'&quot; , &quot;] = cols;
               	   doc[&quot; , &quot;'&quot; , &quot;footer&quot; , &quot;'&quot; , &quot;]=objFooter;
               	   doc.styles.tableHeader.fillColor = &quot; , &quot;'&quot; , &quot;#006bb7&quot; , &quot;'&quot; , &quot;;
               	   doc.styles.tableHeader.alignment=&quot;left&quot;;
               	 	
               	   doc[&quot; , &quot;'&quot; , &quot;header&quot; , &quot;'&quot; , &quot;]=(function(page, pages) {
							return {
								columns: [
									{
										alignment: &quot; , &quot;'&quot; , &quot;left&quot; , &quot;'&quot; , &quot;,
										text: &quot; , &quot;'&quot; , &quot;ProHance&quot; , &quot;'&quot; , &quot;,
										fontSize: 12,
									},
									{
										alignment: &quot; , &quot;'&quot; , &quot;right&quot; , &quot;'&quot; , &quot;,
										fontSize: 8,
										text: [&quot; , &quot;'&quot; , &quot;Page &quot; , &quot;'&quot; , &quot;, { text: page.toString() }]
									}
								],
								margin: 20
							}
						});
               	doc.content.forEach(function(item) {
	               		if (item.table)
	               		{
	               			item.layout = {
	               					hLineColor: function(i, node) { return &quot; , &quot;'&quot; , &quot;#000000&quot; , &quot;'&quot; , &quot;; },
	               					vLineColor: function(i, node) { return &quot; , &quot;'&quot; , &quot;#000000&quot; , &quot;'&quot; , &quot;; }
	               			};
	                    }
	               });
		           doc.styles.tableHeader = {
		        		   fontSize		: 11,
		        		   bold			: true,
		        		   color		: &quot; , &quot;'&quot; , &quot;#ffffff&quot; , &quot;'&quot; , &quot;,
		        		   fillColor	: &quot; , &quot;'&quot; , &quot;#006bb7&quot; , &quot;'&quot; , &quot;
			       };
               	 },
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;,
                 pageSize : pdfPageSize,
                 titleAttr : &quot; , &quot;'&quot; , &quot;PDF&quot; , &quot;'&quot; , &quot;,
                 title : titleHeaderName,
                 button: [ &quot; , &quot;'&quot; , &quot;pdf&quot; , &quot;'&quot; , &quot; ]
             }
         ],
         drawCallback: function() {
          	  var hasRows = this.api().rows({ filter: &quot; , &quot;'&quot; , &quot;applied&quot; , &quot;'&quot; , &quot; }).data().length > 0;
          	 $(&quot; , &quot;'&quot; , &quot;.buttons-excel&quot; , &quot;'&quot; , &quot;)[0].style.visibility = hasRows ? &quot; , &quot;'&quot; , &quot;visible&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;;
          	 $(&quot; , &quot;'&quot; , &quot;.buttons-pdf&quot; , &quot;'&quot; , &quot;)[0].style.visibility = hasRows ? &quot; , &quot;'&quot; , &quot;visible&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;;
          	}
    } );
	
}

/* Build Data Tabel With Custom Export (Custom Excel And PDF doExport) along With Email Report Schedular */
function buildDataTableStructureWithReportScheduler(jsonData, columns, jsonObjArr)
{
	fnUpdateProperties(jsonObjArr);
	var isReqSorting = true;
	if(dynSortingIndex &lt; 0)
	{
		isReqSorting  = false;
		dynSortingIndex = 0;
	}
	
	var exportExcel = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	var exportPdf = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	var emailSpan = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	if(reportScheduleReq == &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;)
	{
		emailSpan = &quot; , &quot;'&quot; , &quot;&lt;span id=&quot;emailSpan&quot; class=&quot;ph-wo-cursor-pointer&quot; data-onclick=&quot;showSchedulerDialogOnClick&quot;>&lt;img src=&quot;/phworkoutput/images/email.png&quot; title=&quot;Email&quot; align=&quot;middle&quot; class=&quot;ph-wo-vertical-align-middle&quot;>&lt;/span>&quot; , &quot;'&quot; , &quot;;
		if($(&quot;#isAllowEmailExport&quot;).val() == &quot; , &quot;'&quot; , &quot;false&quot; , &quot;'&quot; , &quot;)
		{
			emailSpan = &quot; , &quot;'&quot; , &quot;&lt;span id=&quot;emailSpan&quot; class=&quot;ph-wo-cursor-pointer&quot; title=&quot;You do not have permission to download this content&quot;>&lt;img src=&quot;/phworkoutput/images/email.png&quot; title=&quot;Email&quot; align=&quot;middle&quot; class=&quot;ph-wo-vertical-align-middle&quot;>&lt;/span>&quot; , &quot;'&quot; , &quot;;
		}
	}
	if(isMultiOrgAdmin == &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;)
	{
		exportExcel = &quot; , &quot;'&quot; , &quot;excelHtml5&quot; , &quot;'&quot; , &quot;;
		exportPdf = &quot; , &quot;'&quot; , &quot;pdfHtml5&quot; , &quot;'&quot; , &quot;;
	}
	var excelIcon = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	var pdfIcon = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	if($(&quot;#isAllowExcelExport&quot;).val() == &quot; , &quot;'&quot; , &quot;false&quot; , &quot;'&quot; , &quot;)
	{
		excelIcon = &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/excel.png&quot; title=&quot;You do not have permission to download this content&quot; align=&quot;middle&quot; class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>&quot; , &quot;'&quot; , &quot;;
		exportExcel = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	}
	else
	{
		excelIcon = &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/excel.png&quot; title=&quot;Export to Excel&quot; data-onclick=&quot;fnDataTableExportOnClick&quot; data-etype=&quot;excel&quot; align=&quot;middle&quot; class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>&quot; , &quot;'&quot; , &quot;;
	}
	if($(&quot;#isAllowPdfExport&quot;).val() == &quot; , &quot;'&quot; , &quot;false&quot; , &quot;'&quot; , &quot;)
	{
		pdfIcon = &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/pdf.png&quot; title=&quot;You do not have permission to download this content&quot; align=&quot;middle&quot; class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>&quot; , &quot;'&quot; , &quot;;
		exportPdf = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	}
	else if(recordSize > 10000)
	{
		pdfIcon = &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/pdf.png&quot; title=&quot;PDF file cannot be downloaded, as number of records more then 10000&quot; align=&quot;middle&quot; class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>&quot; , &quot;'&quot; , &quot;;
		exportPdf = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	}	
	else
	{
		pdfIcon = &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/pdf.png&quot; title=&quot;Export to PDF&quot; data-onclick=&quot;fnDataTableExportOnClick&quot; data-etype=&quot;pdf&quot; align=&quot;middle&quot; class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>&quot; , &quot;'&quot; , &quot;;
	}
	
	var excelReportType = $(&quot;#excelReportType&quot;).val();
	excelReportType = &quot;.&quot; + excelReportType.toLowerCase();
	titleHeaderName = replaceBackXMLEntities(dynTitleHeaderName);
	fileName = replaceBackXMLEntities(dynFileName);
	var disclaimerText = $(&quot;#disclaimerText&quot;).val();
	var table = $(&quot; , &quot;'&quot; , &quot;#CommonDataTableId&quot; , &quot;'&quot; , &quot;).DataTable( {
		 &quot;fnDrawCallback&quot;: function() {
	            // after table is redrawndo something here
	        	$(&quot;#thirdViewSortOrder&quot;).val($(&quot;#CommonDataTableId&quot;).dataTable().fnSettings().aaSorting);
	        },
        data: 	jsonData,
        deferRender:    true,
        &quot;pageLength&quot;: dynPageLength,
        &quot;scrollX&quot;: true,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;ordering&quot; : isReqSorting,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	 &quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
             &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px&quot; , &quot;'&quot; , &quot;> Displaying _START_ to _END_ &lt;label>&quot;,
             &quot;infoEmpty&quot;: &quot;No records to display&quot;,
             &quot;infoFiltered&quot;: &quot;&lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100&quot; , &quot;'&quot; , &quot; >[Filtered from _MAX_ records] &lt;/label>&quot;,
             &quot;search&quot;:         &quot;Search&quot;,
             &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
                &quot;previous&quot;:   &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;
            },
        },
         columns: columns,
         &quot;order&quot;: [[ dynSortingIndex, sortingOrder ]],
         dom: &quot; , &quot;'&quot; , &quot;Blfrtip&quot; , &quot;'&quot; , &quot;,
         buttons: [
        	 {
        		 text:   &quot; , &quot;'&quot; , &quot;&lt;label class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>|&amp;nbsp;&amp;nbsp; Export:&amp;nbsp;&lt;/label>&quot; , &quot;'&quot; , &quot;,
        		 autoClose: &quot; , &quot;'&quot; , &quot;false&quot; , &quot;'&quot; , &quot;,
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;
        	 },
        	 {
        		 text:   emailSpan,
        		 autoClose: &quot; , &quot;'&quot; , &quot;false&quot; , &quot;'&quot; , &quot;,
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;
        	 },
        	 {
                 extend: exportExcel,
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: &quot; , &quot;'&quot; , &quot;export&quot; , &quot;'&quot; , &quot;,
                 	 stripHtml: true
                 },
                 filename: fileName,
                 text:   excelIcon,
                 autoClose: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;,
                 extension: excelReportType,
                 createEmptyCells: true,
                 title : titleHeaderName,
                 messageBottom: function () {
                     return disclaimerText;
                 	},
                     customize: function ( xlsx ) {
						var sheet = xlsx.xl.worksheets[&quot; , &quot;'&quot; , &quot;sheet1.xml&quot; , &quot;'&quot; , &quot;];
						$(&quot; , &quot;'&quot; , &quot;row c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;25&quot; , &quot;'&quot; , &quot; );
                        $(&quot; , &quot;'&quot; , &quot;c[r=A1] t&quot; , &quot;'&quot; , &quot;, sheet).text( titleHeaderName );
                        $(&quot; , &quot;'&quot; , &quot;row:first c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;27&quot; , &quot;'&quot; , &quot; ); 
                        $(&quot; , &quot;'&quot; , &quot;row:eq(1) c&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;27&quot; , &quot;'&quot; , &quot;);
                        
                        if(disclaimerText != null &amp;&amp; disclaimerText != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
                        {
                        	$(&quot; , &quot;'&quot; , &quot;row:last c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;55&quot; , &quot;'&quot; , &quot; );
	                        $(&quot; , &quot;'&quot; , &quot;row:last&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;customHeight&quot; , &quot;'&quot; , &quot;, 1);
	                        $(&quot; , &quot;'&quot; , &quot;row:last&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;ht&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;150&quot; , &quot;'&quot; , &quot;);
                        }
                        
                      },
                 button: [&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;]
             },
             {
                 extend: exportPdf,
                 orientation: &quot; , &quot;'&quot; , &quot;landscape&quot; , &quot;'&quot; , &quot;,
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: &quot; , &quot;'&quot; , &quot;export&quot; , &quot;'&quot; , &quot;,
                 	 stripHtml: true
                 },
                 text:    pdfIcon ,
                 filename: fileName,
                 autoClose: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 footer:&quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 messageBottom: function () {
                     return disclaimerText;
                  	},
               	 customize: function ( doc ) {
               		
               	   var cols = [];
               	   cols[0] = {text: &quot; , &quot;'&quot; , &quot;© JaMocha Tech Pvt. Ltd. 2009-2026&quot; , &quot;'&quot; , &quot;, alignment: &quot; , &quot;'&quot; , &quot;center&quot; , &quot;'&quot; , &quot;, fontSize:&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;, margin:[0, 0, 0, 0] };
               	   var objFooter = {};
               	   objFooter[&quot; , &quot;'&quot; , &quot;columns&quot; , &quot;'&quot; , &quot;] = cols;
               	   doc[&quot; , &quot;'&quot; , &quot;footer&quot; , &quot;'&quot; , &quot;]=objFooter;
               	   doc.styles.tableHeader.fillColor = &quot; , &quot;'&quot; , &quot;#006bb7&quot; , &quot;'&quot; , &quot;;
               	   doc.styles.tableHeader.alignment=&quot;left&quot;;
               	 	
               	   doc[&quot; , &quot;'&quot; , &quot;header&quot; , &quot;'&quot; , &quot;]=(function(page, pages) {
							return {
								columns: [
									{
										alignment: &quot; , &quot;'&quot; , &quot;left&quot; , &quot;'&quot; , &quot;,
										text: &quot; , &quot;'&quot; , &quot;ProHance&quot; , &quot;'&quot; , &quot;,
										fontSize: 12,
									},
									{
										alignment: &quot; , &quot;'&quot; , &quot;right&quot; , &quot;'&quot; , &quot;,
										fontSize: 8,
										text: [&quot; , &quot;'&quot; , &quot;Page &quot; , &quot;'&quot; , &quot;, { text: page.toString() }]
									}
								],
								margin: 20
							}
						});
               	   
	               	if(dynNumericAlignColArr.length > 0)
	               	{
	               		var rowCount = doc.content[1].table.body.length;
		               	for (i = 0; i &lt; rowCount; i++) 
		               	{
		               		dynNumericAlignColArr.forEach(function(number) {
		                   		doc.content[1].table.body[i][number].alignment = &quot; , &quot;'&quot; , &quot;right&quot; , &quot;'&quot; , &quot;;
		                   	  });
		               	}
               		}
	               	
	               	doc.content[1].table.widths = dynWidthArr;
	               	doc.content.forEach(function(item) {
	               		if (item.table)
	               		{
	               			item.layout = {
	               					hLineColor: function(i, node) { return &quot; , &quot;'&quot; , &quot;#000000&quot; , &quot;'&quot; , &quot;; },
	               					vLineColor: function(i, node) { return &quot; , &quot;'&quot; , &quot;#000000&quot; , &quot;'&quot; , &quot;; }
	               			};
	                    }
	               });
		           doc.styles.tableHeader = {
		        		   fontSize		: 11,
		        		   bold			: true,
		        		   color		: &quot; , &quot;'&quot; , &quot;#ffffff&quot; , &quot;'&quot; , &quot;,
		        		   fillColor	: &quot; , &quot;'&quot; , &quot;#006bb7&quot; , &quot;'&quot; , &quot;
			       };
               	 },
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;,
                 pageSize : &quot; , &quot;'&quot; , &quot;LEGAL&quot; , &quot;'&quot; , &quot;,
                 titleAttr : &quot; , &quot;'&quot; , &quot;PDF&quot; , &quot;'&quot; , &quot;,
                 title : titleHeaderName,
                 button: [ &quot; , &quot;'&quot; , &quot;pdf&quot; , &quot;'&quot; , &quot; ]
             }
         ]
    } );
}

/* Build Data Tabel With Row Grouping and Default Data Table Export */
function buildDataTableStructureRowsGroups(jsonData, pageLength, columns, exportColumns, titleHeaderName, fileName, pdfPageSize, sortingIndex, rowsGroup, jsonObjArr)
{
	//fnUpdateProperties(jsonObjArr);
	var isReqSorting = true;
	if(sortingIndex &lt; 0)
	{
		isReqSorting  = false;
		sortingIndex = 0;
	}
	var excelReportType = $(&quot;#excelReportType&quot;).val();
	excelReportType = &quot;.&quot; + excelReportType.toLowerCase();  
	var disclaimerText = $(&quot;#disclaimerText&quot;).val();
    titleHeaderName = replaceBackXMLEntities(titleHeaderName);
    fileName = replaceBackXMLEntities(fileName);
	var table = $(&quot; , &quot;'&quot; , &quot;#CommonDataTableId&quot; , &quot;'&quot; , &quot;).DataTable( {
        data: 	jsonData,
        deferRender:    false,
        &quot;pageLength&quot;: pageLength,
        &quot;ordering&quot; : isReqSorting,
        &quot;scrollX&quot;: false,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	 &quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
             &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px&quot; , &quot;'&quot; , &quot;> Displaying _START_ to _END_ &lt;label>&quot;,
             &quot;infoEmpty&quot;: &quot;No records to display&quot;,
             &quot;infoFiltered&quot;: &quot;&lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100&quot; , &quot;'&quot; , &quot; >[Filtered from _MAX_ records] &lt;/label>&quot;,
             &quot;search&quot;:         &quot;Search&quot;,
             &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
                &quot;previous&quot;:   &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;
            },
        },
         columns: columns,
         columnDefs: [{
        	    &quot;defaultContent&quot;: &quot;&quot;,
        	    &quot;targets&quot;: &quot;_all&quot;
        	  }],    
         &quot;order&quot;: [[ sortingIndex, &quot; , &quot;'&quot; , &quot;asc&quot; , &quot;'&quot; , &quot; ]],
         rowGroup: rowsGroup,      
         dom: &quot; , &quot;'&quot; , &quot;Blfrtip&quot; , &quot;'&quot; , &quot;,
         buttons: [
             //Empty for no exports
          ]
    });
	
}

function fnUpdateProperties(jsonObjArr) 
{
	dynPageLength = jsonObjArr[&quot;pageLength&quot;];
	dynExportColumns = jsonObjArr[&quot;exportColumns&quot;];
	dynTitleHeaderName = jsonObjArr[&quot;titleHeaderName&quot;];
	dynFileName = jsonObjArr[&quot;fileName&quot;];
	dynPdfPageSize = jsonObjArr[&quot;pdfPageSize&quot;];
	dynSortingIndex = jsonObjArr[&quot;sortingIndex&quot;];
	dynNumericAlignColArr = jsonObjArr[&quot;rightAlignColArr&quot;];
	dynWidthArr	=	jsonObjArr[&quot;dynWidthArr&quot;];
	isMultiOrgAdmin	=	jsonObjArr[&quot;isMultiOrgAdmin&quot;];
	reportScheduleReq	=	jsonObjArr[&quot;reportScheduleReq&quot;];
	if(jsonObjArr[&quot;order&quot;] != null &amp;&amp; jsonObjArr[&quot;order&quot;] != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
	{
		sortingOrder	=	jsonObjArr[&quot;order&quot;];
	}
	else 
	{
		sortingOrder = &quot;asc&quot;;
	}
	recordSize	=	jsonObjArr[&quot;recordSize&quot;];
}

/* Build Data Tabel with Default Data Table Export along with JSONArray param*/
function buildDataTableStructureWithExportsArr(jsonData, columns, jsonObjArr)
{
	fnUpdateProperties(jsonObjArr);
	var isReqSorting = true;
	if(dynSortingIndex &lt; 0)
	{
		isReqSorting  = false;
		dynSortingIndex = 0;
	}
	var excelReportType = $(&quot;#excelReportType&quot;).val();
	excelReportType = &quot;.&quot; + excelReportType.toLowerCase();
	var disclaimerText = $(&quot;#disclaimerText&quot;).val();
	titleHeaderName = replaceBackXMLEntities(dynTitleHeaderName);
	fileName = replaceBackXMLEntities(dynFileName);
	var table = $(&quot; , &quot;'&quot; , &quot;#CommonDataTableId&quot; , &quot;'&quot; , &quot;).DataTable( {
        data: 	jsonData,
        deferRender:    true,
        &quot;pageLength&quot;: dynPageLength,
        &quot;bDestroy&quot;: true,
        &quot;scrollX&quot;: true,
        &quot;ordering&quot; : isReqSorting,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px&quot; , &quot;'&quot; , &quot;> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100&quot; , &quot;'&quot; , &quot; >[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
                &quot;previous&quot;:   &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;
            },
        },
         columns: columns,
         &quot;order&quot;: [[ dynSortingIndex, &quot; , &quot;'&quot; , &quot;asc&quot; , &quot;'&quot; , &quot; ]],
         rowGroup: {
             dataSrc: 88
         },
         dom: &quot; , &quot;'&quot; , &quot;Blfrtip&quot; , &quot;'&quot; , &quot;,
         buttons: [
        	 {
				text:   &quot; , &quot;'&quot; , &quot;&lt;label class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>|&amp;nbsp;&amp;nbsp; Export:&amp;nbsp;&lt;/label>&quot; , &quot;'&quot; , &quot;,
        		 autoClose: &quot; , &quot;'&quot; , &quot;false&quot; , &quot;'&quot; , &quot;,
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;
        	 },
             {
                 extend: &quot; , &quot;'&quot; , &quot;excelHtml5&quot; , &quot;'&quot; , &quot;,
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: &quot; , &quot;'&quot; , &quot;export&quot; , &quot;'&quot; , &quot;,
                 	 stripHtml: true
                 },
                 destroy:true,
                 filename: fileName,
				 text:   &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/excel.png&quot; title=&quot;Export to Excel&quot; align=&quot;middle&quot; class=&quot;ph-wo-vertical-align-middle ph-wo-cursor-pointer&quot;>&quot; , &quot;'&quot; , &quot;,
                 autoClose: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;,
                 extension: excelReportType,
                 createEmptyCells: true,
                 title : titleHeaderName,
                 messageBottom: function () {
                     return disclaimerText;
                 	},
                     customize: function ( xlsx ) {
						 var sheet = xlsx.xl.worksheets[&quot; , &quot;'&quot; , &quot;sheet1.xml&quot; , &quot;'&quot; , &quot;];
						$(&quot; , &quot;'&quot; , &quot;row c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;25&quot; , &quot;'&quot; , &quot; );
                        $(&quot; , &quot;'&quot; , &quot;c[r=A1] t&quot; , &quot;'&quot; , &quot;, sheet).text( titleHeaderName );
                        $(&quot; , &quot;'&quot; , &quot;row:first c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;27&quot; , &quot;'&quot; , &quot; ); 
                        $(&quot; , &quot;'&quot; , &quot;row:eq(1) c&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;27&quot; , &quot;'&quot; , &quot;);
                        
                        if(disclaimerText != null &amp;&amp; disclaimerText != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
                        {
                        	$(&quot; , &quot;'&quot; , &quot;row:last c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;55&quot; , &quot;'&quot; , &quot; );
	                        $(&quot; , &quot;'&quot; , &quot;row:last&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;customHeight&quot; , &quot;'&quot; , &quot;, 1);
	                        $(&quot; , &quot;'&quot; , &quot;row:last&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;ht&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;150&quot; , &quot;'&quot; , &quot;);
                        }
                      },
                      
                 button: [&quot; , &quot;'&quot; , &quot;excel&quot; , &quot;'&quot; , &quot;]

             },
             {
                 extend: &quot; , &quot;'&quot; , &quot;pdfHtml5&quot; , &quot;'&quot; , &quot;,
                 orientation: &quot; , &quot;'&quot; , &quot;landscape&quot; , &quot;'&quot; , &quot;,
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: &quot; , &quot;'&quot; , &quot;export&quot; , &quot;'&quot; , &quot;,
                 	 stripHtml: true
                 },
				 text:      &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/pdf.png&quot; title=&quot;Export to PDF&quot; align=&quot;middle&quot; class=&quot;ph-wo-vertical-align-middle ph-wo-cursor-pointer&quot;>&quot; , &quot;'&quot; , &quot;,
                 filename: fileName,
                 autoClose: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 footer:&quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 destroy:true,
                 messageBottom: function () {
                     return disclaimerText;
                  	},
               	 customize: function ( doc ) {
               		
               	   var cols = [];
               	   cols[0] = {text: &quot; , &quot;'&quot; , &quot;© JaMocha Tech Pvt. Ltd. 2009-2026&quot; , &quot;'&quot; , &quot;, alignment: &quot; , &quot;'&quot; , &quot;center&quot; , &quot;'&quot; , &quot;, fontSize:&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;, margin:[0, 0, 0, 0] };
               	   var objFooter = {};
               	   objFooter[&quot; , &quot;'&quot; , &quot;columns&quot; , &quot;'&quot; , &quot;] = cols;
               	   doc[&quot; , &quot;'&quot; , &quot;footer&quot; , &quot;'&quot; , &quot;]=objFooter;
               	   doc.styles.tableHeader.fillColor = &quot; , &quot;'&quot; , &quot;#006bb7&quot; , &quot;'&quot; , &quot;;
               	   doc.styles.tableHeader.alignment=&quot;left&quot;;
               	 	
               	   doc[&quot; , &quot;'&quot; , &quot;header&quot; , &quot;'&quot; , &quot;]=(function(page, pages) {
							return {
								columns: [
									{
										alignment: &quot; , &quot;'&quot; , &quot;left&quot; , &quot;'&quot; , &quot;,
										text: &quot; , &quot;'&quot; , &quot;ProHance&quot; , &quot;'&quot; , &quot;,
										fontSize: 12,
									},
									{
										alignment: &quot; , &quot;'&quot; , &quot;right&quot; , &quot;'&quot; , &quot;,
										fontSize: 8,
										text: [&quot; , &quot;'&quot; , &quot;Page &quot; , &quot;'&quot; , &quot;, { text: page.toString() }]
									}
								],
								margin: 20
							}
						});
	               	if(dynNumericAlignColArr.length > 0)
	               	{
	               		var rowCount = doc.content[1].table.body.length;
		               	for (i = 0; i &lt; rowCount; i++) 
		               	{
		               		dynNumericAlignColArr.forEach(function(number) {
		                   		doc.content[1].table.body[i][number].alignment = &quot; , &quot;'&quot; , &quot;right&quot; , &quot;'&quot; , &quot;;
		                   	  });
		               	}
	               	}
               	
	               doc.content[1].table.widths=	dynWidthArr;
               	
               		doc.content.forEach(function(item) {
	               		if (item.table)
	               		{
	               			item.layout = {
	               					hLineColor: function(i, node) { return &quot; , &quot;'&quot; , &quot;#000000&quot; , &quot;'&quot; , &quot;; },
	               					vLineColor: function(i, node) { return &quot; , &quot;'&quot; , &quot;#000000&quot; , &quot;'&quot; , &quot;; }
	               			};
	                    }
	               });
		           doc.styles.tableHeader = {
		        		   fontSize		: 11,
		        		   bold			: true,
		        		   color		: &quot; , &quot;'&quot; , &quot;#ffffff&quot; , &quot;'&quot; , &quot;,
		        		   fillColor	: &quot; , &quot;'&quot; , &quot;#006bb7&quot; , &quot;'&quot; , &quot;
			       };
               		doc.content.forEach(function(item) {
	               		if (item.table)
	               		{
	               			item.layout = {
	               					hLineColor: function(i, node) { return &quot; , &quot;'&quot; , &quot;#000000&quot; , &quot;'&quot; , &quot;; },
	               					vLineColor: function(i, node) { return &quot; , &quot;'&quot; , &quot;#000000&quot; , &quot;'&quot; , &quot;; }
	               			};
	                    }
	               });
		           doc.styles.tableHeader = {
		        		   fontSize		: 11,
		        		   bold			: true,
		        		   color		: &quot; , &quot;'&quot; , &quot;#ffffff&quot; , &quot;'&quot; , &quot;,
		        		   fillColor	: &quot; , &quot;'&quot; , &quot;#006bb7&quot; , &quot;'&quot; , &quot;
			       };
               	 },
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;,
                 pageSize : &quot; , &quot;'&quot; , &quot;LEGAL&quot; , &quot;'&quot; , &quot;,
                 titleAttr : &quot; , &quot;'&quot; , &quot;PDF&quot; , &quot;'&quot; , &quot;,
                 title : titleHeaderName,
                 button: [ &quot; , &quot;'&quot; , &quot;pdf&quot; , &quot;'&quot; , &quot; ]
             }
         ],
         drawCallback: function() {
       	  var hasRows = this.api().rows({ filter: &quot; , &quot;'&quot; , &quot;applied&quot; , &quot;'&quot; , &quot; }).data().length > 0;
       	 $(&quot; , &quot;'&quot; , &quot;.buttons-excel&quot; , &quot;'&quot; , &quot;)[0].style.visibility = hasRows ? &quot; , &quot;'&quot; , &quot;visible&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;;
       	 $(&quot; , &quot;'&quot; , &quot;.buttons-pdf&quot; , &quot;'&quot; , &quot;)[0].style.visibility = hasRows ? &quot; , &quot;'&quot; , &quot;visible&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;;
       	}
    } );
	
}

/* Build Data table for WO Work Type Attributes*/
function buildDataTableStructureForWorkTypeAttributesBulkUpload(jsonData, columns, jsonObjArr)
{
	fnUpdateProperties(jsonObjArr);
	var isReqSorting = true;
	if(dynSortingIndex &lt; 0)
	{
		isReqSorting  = false;
		dynSortingIndex = 0;
	}
	var excelReportType = $(&quot;#excelReportType&quot;).val();
	excelReportType = &quot;.&quot; + excelReportType.toLowerCase();
	var disclaimerText = $(&quot;#disclaimerText&quot;).val();
	titleHeaderName = replaceBackXMLEntities(dynTitleHeaderName);
	fileName = replaceBackXMLEntities(dynFileName);
	var table = $(&quot; , &quot;'&quot; , &quot;#CommonDataTableId&quot; , &quot;'&quot; , &quot;).DataTable( {
        data: 	jsonData,
        deferRender:    true,
        &quot;pageLength&quot;: dynPageLength,
        &quot;bDestroy&quot;: true,
        &quot;scrollX&quot;: true,
        &quot;ordering&quot; : isReqSorting,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px&quot; , &quot;'&quot; , &quot;> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100&quot; , &quot;'&quot; , &quot; >[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
                &quot;previous&quot;:   &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;
            },
        },
         columns: columns,
         &quot;order&quot;: [[ dynSortingIndex, sortingOrder ]],
         rowGroup: {
             dataSrc: 88
         },
         dom: &quot; , &quot;'&quot; , &quot;Blfrtip&quot; , &quot;'&quot; , &quot;,
         buttons: [
        	 {
        		 text:   &quot; , &quot;'&quot; , &quot;&lt;label class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>|&amp;nbsp;&amp;nbsp; Export:&amp;nbsp;&lt;/label>&quot; , &quot;'&quot; , &quot;,
        		 autoClose: &quot; , &quot;'&quot; , &quot;false&quot; , &quot;'&quot; , &quot;,
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;
        	 },
             {
                 extend: &quot; , &quot;'&quot; , &quot;excelHtml5&quot; , &quot;'&quot; , &quot;,
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: &quot; , &quot;'&quot; , &quot;export&quot; , &quot;'&quot; , &quot;,
                 	 stripHtml: true
                 },
                 destroy:true,
                 filename: fileName,
                 text:   &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/excel.png&quot; title=&quot;Export to Excel&quot; align=&quot;middle&quot; class=&quot;ph-wo-vertical-align-middle ph-wo-cursor-pointer&quot;>&quot; , &quot;'&quot; , &quot;,
                 autoClose: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;,
                 extension: excelReportType,
                 createEmptyCells: true,
                 title : titleHeaderName,
                 messageBottom: function () {
                     return disclaimerText;
                 	},
                     customize: function ( xlsx ) {
						 var sheet = xlsx.xl.worksheets[&quot; , &quot;'&quot; , &quot;sheet1.xml&quot; , &quot;'&quot; , &quot;];
						$(&quot; , &quot;'&quot; , &quot;row c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;25&quot; , &quot;'&quot; , &quot; );
                        $(&quot; , &quot;'&quot; , &quot;c[r=A1] t&quot; , &quot;'&quot; , &quot;, sheet).text( titleHeaderName );
                        $(&quot; , &quot;'&quot; , &quot;row:first c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;27&quot; , &quot;'&quot; , &quot; ); 
                        $(&quot; , &quot;'&quot; , &quot;row:eq(1) c&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;27&quot; , &quot;'&quot; , &quot;);
                        
                        if(disclaimerText != null &amp;&amp; disclaimerText != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
                        {
                        	$(&quot; , &quot;'&quot; , &quot;row:last c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;55&quot; , &quot;'&quot; , &quot; );
	                        $(&quot; , &quot;'&quot; , &quot;row:last&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;customHeight&quot; , &quot;'&quot; , &quot;, 1);
	                        $(&quot; , &quot;'&quot; , &quot;row:last&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;ht&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;150&quot; , &quot;'&quot; , &quot;);
                        }
                         
                      },
                      
                 button: [&quot; , &quot;'&quot; , &quot;excel&quot; , &quot;'&quot; , &quot;]

             },
             {
                 extend: &quot; , &quot;'&quot; , &quot;pdfHtml5&quot; , &quot;'&quot; , &quot;,
                 orientation: &quot; , &quot;'&quot; , &quot;landscape&quot; , &quot;'&quot; , &quot;,
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: &quot; , &quot;'&quot; , &quot;export&quot; , &quot;'&quot; , &quot;,
                 	 stripHtml: true
                 },
                 text:      &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/pdf.png&quot; title=&quot;Export to PDF&quot; align=&quot;middle&quot; class=&quot;ph-wo-vertical-align-middle ph-wo-cursor-pointer&quot;>&quot; , &quot;'&quot; , &quot;,
                 filename: fileName,
                 autoClose: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 footer:&quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 destroy:true,
                 messageBottom: function () {
                     return disclaimerText;
                  	},
               	 customize: function ( doc ) {
               		
               	   var cols = [];
               	   cols[0] = {text: &quot; , &quot;'&quot; , &quot;© JaMocha Tech Pvt. Ltd. 2009-2026&quot; , &quot;'&quot; , &quot;, alignment: &quot; , &quot;'&quot; , &quot;center&quot; , &quot;'&quot; , &quot;, fontSize:&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;, margin:[0, 0, 0, 0] };
               	   var objFooter = {};
               	   objFooter[&quot; , &quot;'&quot; , &quot;columns&quot; , &quot;'&quot; , &quot;] = cols;
               	   doc[&quot; , &quot;'&quot; , &quot;footer&quot; , &quot;'&quot; , &quot;]=objFooter;
               	   doc.styles.tableHeader.fillColor = &quot; , &quot;'&quot; , &quot;#006bb7&quot; , &quot;'&quot; , &quot;;
               	   doc.styles.tableHeader.alignment=&quot;left&quot;;
               	 	
               	   doc[&quot; , &quot;'&quot; , &quot;header&quot; , &quot;'&quot; , &quot;]=(function(page, pages) {
							return {
								columns: [
									{
										alignment: &quot; , &quot;'&quot; , &quot;left&quot; , &quot;'&quot; , &quot;,
										text: &quot; , &quot;'&quot; , &quot;ProHance&quot; , &quot;'&quot; , &quot;,
										fontSize: 12,
									},
									{
										alignment: &quot; , &quot;'&quot; , &quot;right&quot; , &quot;'&quot; , &quot;,
										fontSize: 8,
										text: [&quot; , &quot;'&quot; , &quot;Page &quot; , &quot;'&quot; , &quot;, { text: page.toString() }]
									}
								],
								margin: 20
							}
						});
	               	if(dynNumericAlignColArr.length > 0)
	               	{
	               		var rowCount = doc.content[1].table.body.length;
		               	for (i = 0; i &lt; rowCount; i++) 
		               	{
		               		dynNumericAlignColArr.forEach(function(number) {
		                   		doc.content[1].table.body[i][number].alignment = &quot; , &quot;'&quot; , &quot;right&quot; , &quot;'&quot; , &quot;;
		                   	  });
		               	}
	               	}
               	
	               doc.content[1].table.widths=	dynWidthArr;
	               doc.content.forEach(function(item) {
	               		if (item.table)
	               		{
	               			item.layout = {
	               					hLineColor: function(i, node) { return &quot; , &quot;'&quot; , &quot;#000000&quot; , &quot;'&quot; , &quot;; },
	               					vLineColor: function(i, node) { return &quot; , &quot;'&quot; , &quot;#000000&quot; , &quot;'&quot; , &quot;; }
	               			};
	                    }
	               });
		           doc.styles.tableHeader = {
		        		   fontSize		: 11,
		        		   bold			: true,
		        		   color		: &quot; , &quot;'&quot; , &quot;#ffffff&quot; , &quot;'&quot; , &quot;,
		        		   fillColor	: &quot; , &quot;'&quot; , &quot;#006bb7&quot; , &quot;'&quot; , &quot;
			       };
               	
               	 },
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;,
                 pageSize : &quot; , &quot;'&quot; , &quot;LEGAL&quot; , &quot;'&quot; , &quot;,
                 titleAttr : &quot; , &quot;'&quot; , &quot;PDF&quot; , &quot;'&quot; , &quot;,
                 title : titleHeaderName,
                 button: [ &quot; , &quot;'&quot; , &quot;pdf&quot; , &quot;'&quot; , &quot; ]
             }
         ],
         drawCallback: function() {
       	  var hasRows = this.api().rows({ filter: &quot; , &quot;'&quot; , &quot;applied&quot; , &quot;'&quot; , &quot; }).data().length > 0;
       	 $(&quot; , &quot;'&quot; , &quot;.buttons-excel&quot; , &quot;'&quot; , &quot;)[0].style.visibility = hasRows ? &quot; , &quot;'&quot; , &quot;visible&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;;
       	 $(&quot; , &quot;'&quot; , &quot;.buttons-pdf&quot; , &quot;'&quot; , &quot;)[0].style.visibility = hasRows ? &quot; , &quot;'&quot; , &quot;visible&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;;
       	}
    } );
	
}

/* Build Data Tabel along with SX - scrollX param */
function buildScrollXDataTableStructure(jsonData, pageLength, columns, sortingIndex, isScrollX)
{
	var isReqSorting = true;
	if(sortingIndex &lt; 0)
	{
		isReqSorting  = false;
		sortingIndex = 0;
	}
	var table = $(&quot; , &quot;'&quot; , &quot;#CommonDataTableId&quot; , &quot;'&quot; , &quot;).DataTable( {
        data: jsonData,
        deferRender:    true,
        &quot;pageLength&quot;: pageLength,
        &quot;ordering&quot; : isReqSorting,
        &quot;scrollX&quot;: isScrollX,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px&quot; , &quot;'&quot; , &quot;> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100&quot; , &quot;'&quot; , &quot; >[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
                &quot;previous&quot;:   &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;
            },
        },
         columns: columns,
         &quot;order&quot;: [[ sortingIndex, &quot; , &quot;'&quot; , &quot;asc&quot; , &quot;'&quot; , &quot; ]],
         dom: &quot; , &quot;'&quot; , &quot;Blfrtip&quot; , &quot;'&quot; , &quot;,
         buttons: [
            //Empty for no exports
         ]
    } );
}

/* Build Data Tabel with Defalt Data Table Export along with SX - scrollX param */
function buildDataTableStructureWithExportsArrScrollX(jsonData, columns, jsonObjArr, isScrollX)
{
	fnUpdateProperties(jsonObjArr);
	var isReqSorting = true;
	if(dynSortingIndex &lt; 0)
	{
		isReqSorting  = false;
		dynSortingIndex = 0;
	}
	var excelReportType = $(&quot;#excelReportType&quot;).val();
	excelReportType = &quot;.&quot; + excelReportType.toLowerCase();
	var disclaimerText = $(&quot;#disclaimerText&quot;).val();
	titleHeaderName = replaceBackXMLEntities(dynTitleHeaderName);
	fileName = replaceBackXMLEntities(dynFileName);
	var table = $(&quot; , &quot;'&quot; , &quot;#CommonDataTableId&quot; , &quot;'&quot; , &quot;).DataTable( {
        data: 	jsonData,
        deferRender:    true,
        &quot;pageLength&quot;: dynPageLength,
        &quot;bDestroy&quot;: true,
        &quot;scrollX&quot;: isScrollX,
        &quot;ordering&quot; : isReqSorting,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px&quot; , &quot;'&quot; , &quot;> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100&quot; , &quot;'&quot; , &quot; >[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
                &quot;previous&quot;:   &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;
            },
        },
         columns: columns,
         &quot;order&quot;: [[ dynSortingIndex, &quot; , &quot;'&quot; , &quot;asc&quot; , &quot;'&quot; , &quot; ]],
         rowGroup: {
             dataSrc: 88
         },
         dom: &quot; , &quot;'&quot; , &quot;Blfrtip&quot; , &quot;'&quot; , &quot;,
         buttons: [
        	 {
        		 text:   &quot; , &quot;'&quot; , &quot;&lt;label class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>|&amp;nbsp;&amp;nbsp; Export:&amp;nbsp;&lt;/label>&quot; , &quot;'&quot; , &quot;,
        		 autoClose: &quot; , &quot;'&quot; , &quot;false&quot; , &quot;'&quot; , &quot;,
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;
        	 },
             {
                 extend: &quot; , &quot;'&quot; , &quot;excelHtml5&quot; , &quot;'&quot; , &quot;,
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: &quot; , &quot;'&quot; , &quot;export&quot; , &quot;'&quot; , &quot;,
                 	 stripHtml: true,
                 	 format: {
                        header: function (data, columnIdx) {
                            if ((columnIdx === 6 &amp;&amp; columns.length == 8) || (columnIdx === 5 &amp;&amp; columns.length === 7))
                            {   
                                return &quot; , &quot;'&quot; , &quot;Part of Filter&quot; , &quot;'&quot; , &quot;;
                            }
                            return data;
                        }
                    }
                 },
                 destroy:true,
                 filename: fileName,
                 text:   &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/excel.png&quot; title=&quot;Export to Excel&quot; align=&quot;middle&quot; class=&quot;ph-wo-vertical-align-middle ph-wo-cursor-pointer&quot;>&quot; , &quot;'&quot; , &quot;,
                 autoClose: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;,
                 extension: excelReportType,
                 createEmptyCells: true,
                 title : titleHeaderName,
                 messageBottom: function () {
                     return disclaimerText;
                 	},
                     customize: function ( xlsx ) {
						 var sheet = xlsx.xl.worksheets[&quot; , &quot;'&quot; , &quot;sheet1.xml&quot; , &quot;'&quot; , &quot;];
						$(&quot; , &quot;'&quot; , &quot;row c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;25&quot; , &quot;'&quot; , &quot; );
                        $(&quot; , &quot;'&quot; , &quot;c[r=A1] t&quot; , &quot;'&quot; , &quot;, sheet).text( titleHeaderName );
                        $(&quot; , &quot;'&quot; , &quot;row:first c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;27&quot; , &quot;'&quot; , &quot; ); 
                        $(&quot; , &quot;'&quot; , &quot;row:eq(1) c&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;27&quot; , &quot;'&quot; , &quot;);
                        
                        if(disclaimerText != null &amp;&amp; disclaimerText != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
                        {
                        	$(&quot; , &quot;'&quot; , &quot;row:last c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;55&quot; , &quot;'&quot; , &quot; );
	                        $(&quot; , &quot;'&quot; , &quot;row:last&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;customHeight&quot; , &quot;'&quot; , &quot;, 1);
	                        $(&quot; , &quot;'&quot; , &quot;row:last&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;ht&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;150&quot; , &quot;'&quot; , &quot;);
                        }
                         
                      },
                      
                 button: [&quot; , &quot;'&quot; , &quot;excel&quot; , &quot;'&quot; , &quot;]

             },
             {
                 extend: &quot; , &quot;'&quot; , &quot;pdfHtml5&quot; , &quot;'&quot; , &quot;,
                 orientation: &quot; , &quot;'&quot; , &quot;landscape&quot; , &quot;'&quot; , &quot;,
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: &quot; , &quot;'&quot; , &quot;export&quot; , &quot;'&quot; , &quot;,
                 	stripHtml: true,
                 	format: {
                        header: function (data, columnIdx) {
                            if ((columnIdx === 6 &amp;&amp; columns.length == 8) || (columnIdx === 5 &amp;&amp; columns.length === 7))
                            {   
                                return &quot; , &quot;'&quot; , &quot;Part of Filter&quot; , &quot;'&quot; , &quot;;
                            }
                            return data;
                        }
                    }
                 },
                 text:      &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/pdf.png&quot; title=&quot;Export to PDF&quot; align=&quot;middle&quot; class=&quot;ph-wo-vertical-align-middle ph-wo-cursor-pointer&quot;>&quot; , &quot;'&quot; , &quot;,
                 filename: fileName,
                 autoClose: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 footer:&quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 destroy:true,
                 messageBottom: function () {
                     return disclaimerText;
                  	},
               	 customize: function ( doc ) {
               		
               	   var cols = [];
               	   cols[0] = {text: &quot; , &quot;'&quot; , &quot;© JaMocha Tech Pvt. Ltd. 2009-2026&quot; , &quot;'&quot; , &quot;, alignment: &quot; , &quot;'&quot; , &quot;center&quot; , &quot;'&quot; , &quot;, fontSize:&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;, margin:[0, 0, 0, 0] };
               	   var objFooter = {};
               	   objFooter[&quot; , &quot;'&quot; , &quot;columns&quot; , &quot;'&quot; , &quot;] = cols;
               	   doc[&quot; , &quot;'&quot; , &quot;footer&quot; , &quot;'&quot; , &quot;]=objFooter;
               	   doc.styles.tableHeader.fillColor = &quot; , &quot;'&quot; , &quot;#006bb7&quot; , &quot;'&quot; , &quot;;
               	   doc.styles.tableHeader.alignment=&quot;left&quot;;
               	 	
               	   doc[&quot; , &quot;'&quot; , &quot;header&quot; , &quot;'&quot; , &quot;]=(function(page, pages) {
							return {
								columns: [
									{
										alignment: &quot; , &quot;'&quot; , &quot;left&quot; , &quot;'&quot; , &quot;,
										text: &quot; , &quot;'&quot; , &quot;ProHance&quot; , &quot;'&quot; , &quot;,
										fontSize: 12,
									},
									{
										alignment: &quot; , &quot;'&quot; , &quot;right&quot; , &quot;'&quot; , &quot;,
										fontSize: 8,
										text: [&quot; , &quot;'&quot; , &quot;Page &quot; , &quot;'&quot; , &quot;, { text: page.toString() }]
									}
								],
								margin: 20
							}
						});
	               	if(dynNumericAlignColArr.length > 0)
	               	{
	               		var rowCount = doc.content[1].table.body.length;
		               	for (i = 0; i &lt; rowCount; i++) 
		               	{
		               		dynNumericAlignColArr.forEach(function(number) {
		                   		doc.content[1].table.body[i][number].alignment = &quot; , &quot;'&quot; , &quot;right&quot; , &quot;'&quot; , &quot;;
		                   	  });
		               	}
	               	}
               	
	               doc.content[1].table.widths=	dynWidthArr;
	               doc.content.forEach(function(item) {
	               		if (item.table)
	               		{
	               			item.layout = {
	               					hLineColor: function(i, node) { return &quot; , &quot;'&quot; , &quot;#000000&quot; , &quot;'&quot; , &quot;; },
	               					vLineColor: function(i, node) { return &quot; , &quot;'&quot; , &quot;#000000&quot; , &quot;'&quot; , &quot;; }
	               			};
	                    }
	               });
		           doc.styles.tableHeader = {
		        		   fontSize		: 11,
		        		   bold			: true,
		        		   color		: &quot; , &quot;'&quot; , &quot;#ffffff&quot; , &quot;'&quot; , &quot;,
		        		   fillColor	: &quot; , &quot;'&quot; , &quot;#006bb7&quot; , &quot;'&quot; , &quot;
			       };
               	
               	 },
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;,
                 pageSize : &quot; , &quot;'&quot; , &quot;LEGAL&quot; , &quot;'&quot; , &quot;,
                 titleAttr : &quot; , &quot;'&quot; , &quot;PDF&quot; , &quot;'&quot; , &quot;,
                 title : titleHeaderName,
                 button: [ &quot; , &quot;'&quot; , &quot;pdf&quot; , &quot;'&quot; , &quot; ]
             }
         ],
         drawCallback: function() {
        	 var api = this.api();
       	  	 var hasRows = api.rows({ filter: &quot; , &quot;'&quot; , &quot;applied&quot; , &quot;'&quot; , &quot; }).data().length > 0;
       	  	 $(&quot; , &quot;'&quot; , &quot;.buttons-excel&quot; , &quot;'&quot; , &quot;)[0].style.visibility = hasRows ? &quot; , &quot;'&quot; , &quot;visible&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;;
       	  	 $(&quot; , &quot;'&quot; , &quot;.buttons-pdf&quot; , &quot;'&quot; , &quot;)[0].style.visibility = hasRows ? &quot; , &quot;'&quot; , &quot;visible&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;;
       	  	 api.rows({ page: &quot; , &quot;'&quot; , &quot;current&quot; , &quot;'&quot; , &quot; }).every(function () {
	             var data = this.data();
	             if (data &amp;&amp; data.options) 
	             {
	                initSlimScroll(data.options.length, data.id);
	             }
          	 });
       	}
    } );
}

/* Build Data Tabel with Row Grouping, along with SX - scrollX param */
function buildDTRowGroupingWithoutExportSX(jsonData, pageLength, columns, sortingIndex, isScrollX, rowsGroup, ascnding)
{
	var isReqSorting = true;
	if(sortingIndex &lt; 0)
	{
		isReqSorting  = false;
		sortingIndex = 0;
	}
	var table = $(&quot; , &quot;'&quot; , &quot;#CommonDataTableId&quot; , &quot;'&quot; , &quot;).DataTable({
        data: jsonData,
        deferRender:    true,
        &quot;pageLength&quot;: pageLength,
        &quot;ordering&quot; : isReqSorting,
        &quot;scrollX&quot;: isScrollX,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px&quot; , &quot;'&quot; , &quot;> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100&quot; , &quot;'&quot; , &quot; >[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
                &quot;previous&quot;:   &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;
            },
        },
         columns: columns,
         columnDefs: [{
       	    &quot;defaultContent&quot;: &quot;&quot;,
       	    &quot;targets&quot;: &quot;_all&quot;
       	  }],
         &quot;order&quot;: [[ sortingIndex, ascnding ]],
         rowGroup: rowsGroup,
         dom: &quot; , &quot;'&quot; , &quot;Blfrtip&quot; , &quot;'&quot; , &quot;,
         &quot;bDestroy&quot;: true,
         buttons: [
            //Empty for no exports
         ]
    });
}

/* Build Data Tabel with Defalt Data Table, Export and Report Schedular Email along With SX - scrollX param */
function buildDTWithDTExportsWithRSEmailSX(jsonData, columns, jsonObjArr, isScrollX)
{
	fnUpdateProperties(jsonObjArr);
	var isReqSorting = true;
	if(dynSortingIndex &lt; 0)
	{
		isReqSorting  = false;
		dynSortingIndex = 0;
	}
	
	if(reportScheduleReq == &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;)
	{
		emailSpan = &quot; , &quot;'&quot; , &quot;&lt;span id=&quot;emailSpan&quot; class=&quot;ph-wo-cursor-pointer&quot; data-onclick=&quot;showSchedulerDialogOnClick&quot;>&lt;img src=&quot;/phworkoutput/images/email.png&quot; title=&quot;Email&quot; align=&quot;middle&quot; class=&quot;ph-wo-vertical-align-middle&quot;>&lt;/span>&quot; , &quot;'&quot; , &quot;;
		if($(&quot;#isAllowEmailExport&quot;).val() == &quot; , &quot;'&quot; , &quot;false&quot; , &quot;'&quot; , &quot;)
		{
			emailSpan = &quot; , &quot;'&quot; , &quot;&lt;span id=&quot;emailSpan&quot; class=&quot;ph-wo-cursor-pointer&quot; title=&quot;You do not have permission to download this content&quot;>&lt;img src=&quot;/phworkoutput/images/email.png&quot; title=&quot;Email&quot; align=&quot;middle&quot; class=&quot;ph-wo-vertical-align-middle&quot;>&lt;/span>&quot; , &quot;'&quot; , &quot;;
		}
	}
	
	var excelReportType = $(&quot;#excelReportType&quot;).val();
	excelReportType = &quot;.&quot; + excelReportType.toLowerCase();
	var disclaimerText = $(&quot;#disclaimerText&quot;).val();
	titleHeaderName = replaceBackXMLEntities(dynTitleHeaderName);
	fileName = replaceBackXMLEntities(dynFileName);
	var table = $(&quot; , &quot;'&quot; , &quot;#CommonDataTableId&quot; , &quot;'&quot; , &quot;).DataTable( {
        data: 	jsonData,
        deferRender:    true,
        &quot;pageLength&quot;: dynPageLength,
        &quot;bDestroy&quot;: true,
        &quot;scrollX&quot;: isScrollX,
        &quot;ordering&quot; : isReqSorting,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px&quot; , &quot;'&quot; , &quot;> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100&quot; , &quot;'&quot; , &quot; >[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
                &quot;previous&quot;:   &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;
            },
        },
         columns: columns,
         &quot;order&quot;: [[ dynSortingIndex, &quot; , &quot;'&quot; , &quot;asc&quot; , &quot;'&quot; , &quot; ]],
         rowGroup: {
             dataSrc: 88
         },
         dom: &quot; , &quot;'&quot; , &quot;Blfrtip&quot; , &quot;'&quot; , &quot;,
         buttons: [
        	 {
        		 text:   &quot; , &quot;'&quot; , &quot;&lt;label class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>|&amp;nbsp;&amp;nbsp; Export:&amp;nbsp;&lt;/label>&quot; , &quot;'&quot; , &quot;,
        		 autoClose: &quot; , &quot;'&quot; , &quot;false&quot; , &quot;'&quot; , &quot;,
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;
        	 },
        	 {
        		 text:   emailSpan,
        		 autoClose: &quot; , &quot;'&quot; , &quot;false&quot; , &quot;'&quot; , &quot;,
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;
        	 },
             {
                 extend: &quot; , &quot;'&quot; , &quot;excelHtml5&quot; , &quot;'&quot; , &quot;,
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: &quot; , &quot;'&quot; , &quot;export&quot; , &quot;'&quot; , &quot;,
                 	 stripHtml: true
                 },
                 destroy:true,
                 filename: fileName,
                 text:   &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/excel.png&quot; title=&quot;Export to Excel&quot; align=&quot;middle&quot; class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>&quot; , &quot;'&quot; , &quot;,
                 autoClose: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;,
                 extension: excelReportType,
                 createEmptyCells: true,
                 title : titleHeaderName,
                 messageBottom: function () {
                     return disclaimerText;
                 	},
                     customize: function ( xlsx ) {
						 var sheet = xlsx.xl.worksheets[&quot; , &quot;'&quot; , &quot;sheet1.xml&quot; , &quot;'&quot; , &quot;];
						$(&quot; , &quot;'&quot; , &quot;row c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;25&quot; , &quot;'&quot; , &quot; );
                        $(&quot; , &quot;'&quot; , &quot;c[r=A1] t&quot; , &quot;'&quot; , &quot;, sheet).text( titleHeaderName );
                        $(&quot; , &quot;'&quot; , &quot;row:first c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;27&quot; , &quot;'&quot; , &quot; ); 
                        $(&quot; , &quot;'&quot; , &quot;row:eq(1) c&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;27&quot; , &quot;'&quot; , &quot;);
                        
                        if(disclaimerText != null &amp;&amp; disclaimerText != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
                        {
                        	$(&quot; , &quot;'&quot; , &quot;row:last c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;55&quot; , &quot;'&quot; , &quot; );
	                        $(&quot; , &quot;'&quot; , &quot;row:last&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;customHeight&quot; , &quot;'&quot; , &quot;, 1);
	                        $(&quot; , &quot;'&quot; , &quot;row:last&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;ht&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;150&quot; , &quot;'&quot; , &quot;);
                        }
                      },
                      
                 button: [&quot; , &quot;'&quot; , &quot;excel&quot; , &quot;'&quot; , &quot;]
             },
             {
                 extend: &quot; , &quot;'&quot; , &quot;pdfHtml5&quot; , &quot;'&quot; , &quot;,
                 orientation: &quot; , &quot;'&quot; , &quot;landscape&quot; , &quot;'&quot; , &quot;,
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: &quot; , &quot;'&quot; , &quot;export&quot; , &quot;'&quot; , &quot;,
                 	 stripHtml: true
                 },
                 text:      &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/pdf.png&quot; title=&quot;Export to PDF&quot; align=&quot;middle&quot; class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>&quot; , &quot;'&quot; , &quot;,
                 filename: fileName,
                 autoClose: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 footer:&quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 destroy:true,
                 messageBottom: function () {
                     return disclaimerText;
                  	},
               	 customize: function ( doc ) {
               		
               	   var cols = [];
               	   cols[0] = {text: &quot; , &quot;'&quot; , &quot;© JaMocha Tech Pvt. Ltd. 2009-2026&quot; , &quot;'&quot; , &quot;, alignment: &quot; , &quot;'&quot; , &quot;center&quot; , &quot;'&quot; , &quot;, fontSize:&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;, margin:[0, 0, 0, 0] };
               	   var objFooter = {};
               	   objFooter[&quot; , &quot;'&quot; , &quot;columns&quot; , &quot;'&quot; , &quot;] = cols;
               	   doc[&quot; , &quot;'&quot; , &quot;footer&quot; , &quot;'&quot; , &quot;]=objFooter;
               	   doc.styles.tableHeader.fillColor = &quot; , &quot;'&quot; , &quot;#006bb7&quot; , &quot;'&quot; , &quot;;
               	   doc.styles.tableHeader.alignment=&quot;left&quot;;
               	 	
               	   doc[&quot; , &quot;'&quot; , &quot;header&quot; , &quot;'&quot; , &quot;]=(function(page, pages) {
							return {
								columns: [
									{
										alignment: &quot; , &quot;'&quot; , &quot;left&quot; , &quot;'&quot; , &quot;,
										text: &quot; , &quot;'&quot; , &quot;ProHance&quot; , &quot;'&quot; , &quot;,
										fontSize: 12,
									},
									{
										alignment: &quot; , &quot;'&quot; , &quot;right&quot; , &quot;'&quot; , &quot;,
										fontSize: 8,
										text: [&quot; , &quot;'&quot; , &quot;Page &quot; , &quot;'&quot; , &quot;, { text: page.toString() }]
									}
								],
								margin: 20
							}
						});
	               	if(dynNumericAlignColArr.length > 0)
	               	{
	               		var rowCount = doc.content[1].table.body.length;
		               	for (i = 0; i &lt; rowCount; i++) 
		               	{
		               		dynNumericAlignColArr.forEach(function(number) {
		                   		doc.content[1].table.body[i][number].alignment = &quot; , &quot;'&quot; , &quot;right&quot; , &quot;'&quot; , &quot;;
		                   	  });
		               	}
	               	}
               	
	               doc.content[1].table.widths=	dynWidthArr;
	               doc.content.forEach(function(item) {
	               		if (item.table)
	               		{
	               			item.layout = {
	               					hLineColor: function(i, node) { return &quot; , &quot;'&quot; , &quot;#000000&quot; , &quot;'&quot; , &quot;; },
	               					vLineColor: function(i, node) { return &quot; , &quot;'&quot; , &quot;#000000&quot; , &quot;'&quot; , &quot;; }
	               			};
	                    }
	               });
		           doc.styles.tableHeader = {
		        		   fontSize		: 11,
		        		   bold			: true,
		        		   color		: &quot; , &quot;'&quot; , &quot;#ffffff&quot; , &quot;'&quot; , &quot;,
		        		   fillColor	: &quot; , &quot;'&quot; , &quot;#006bb7&quot; , &quot;'&quot; , &quot;
			       };
               	
               	 },
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;,
                 pageSize : &quot; , &quot;'&quot; , &quot;LEGAL&quot; , &quot;'&quot; , &quot;,
                 titleAttr : &quot; , &quot;'&quot; , &quot;PDF&quot; , &quot;'&quot; , &quot;,
                 title : titleHeaderName,
                 button: [ &quot; , &quot;'&quot; , &quot;pdf&quot; , &quot;'&quot; , &quot; ]
             }
         ],
         drawCallback: function() {
       	  var hasRows = this.api().rows({ filter: &quot; , &quot;'&quot; , &quot;applied&quot; , &quot;'&quot; , &quot; }).data().length > 0;
       	 $(&quot; , &quot;'&quot; , &quot;.buttons-excel&quot; , &quot;'&quot; , &quot;)[0].style.visibility = hasRows ? &quot; , &quot;'&quot; , &quot;visible&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;;
       	 $(&quot; , &quot;'&quot; , &quot;.buttons-pdf&quot; , &quot;'&quot; , &quot;)[0].style.visibility = hasRows ? &quot; , &quot;'&quot; , &quot;visible&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;;
       	}
    } );
}

$(document).off(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;showSchedulerDialogOnClick&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;showSchedulerDialogOnClick&quot;]&quot; , &quot;'&quot; , &quot;, function() {
    showSchedulerDialog();
});

$(document).off(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnDataTableExportOnClick&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnDataTableExportOnClick&quot;]&quot; , &quot;'&quot; , &quot;, function() {
    var etype = $(this).data(&quot; , &quot;'&quot; , &quot;etype&quot; , &quot;'&quot; , &quot;);
    fnDataTableExport(etype);
});

/* Build Data Tabel With Custom Export (Custom Excel And PDF doExport) For Work Type Definition Screen */
function buildDataTableStructureWithExportsForWorkType(jsonData, columns, jsonObjArr)
{
	fnUpdateProperties(jsonObjArr);
	var isReqSorting = true;
	if(dynSortingIndex &lt; 0)
	{
		isReqSorting  = false;
		dynSortingIndex = 0;
	}
	
	var exportExcel = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	var exportPdf = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	var emailSpan = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	var excelIcon = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	var pdfIcon = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	
	excelIcon = &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/excel.png&quot; title=&quot;Export to Excel&quot; data-onclick=&quot;fnDataTableExportexpdf&quot; data-etype=&quot;excel&quot; align=&quot;middle&quot; class=&quot;ph-wo-vertical-align-middle ph-wo-cursor-pointer&quot;>&quot; , &quot;'&quot; , &quot;;
	
	if(recordSize > 10000)
	{
		pdfIcon = &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/pdf.png&quot; title=&quot;PDF file cannot be downloaded, as number of records more then 10000&quot; align=&quot;middle&quot; class=&quot;ph-wo-vertical-align-middle ph-wo-cursor-pointer&quot;>&quot; , &quot;'&quot; , &quot;;
		exportPdf = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	}	
	else
	{
		pdfIcon = &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/pdf.png&quot; title=&quot;Export to PDF&quot; data-onclick=&quot;fnDataTableExportexpdf&quot; data-etype=&quot;pdf&quot; class=&quot;ph-wo-vertical-align-middle ph-wo-cursor-pointer&quot;>&quot; , &quot;'&quot; , &quot;;
	}
	
	var excelReportType = $(&quot;#excelReportType&quot;).val();
	excelReportType = &quot;.&quot; + excelReportType.toLowerCase();
	
	titleHeaderName = replaceBackXMLEntities(dynTitleHeaderName);
	fileName = replaceBackXMLEntities(dynFileName);
	
	var disclaimerText = $(&quot;#disclaimerText&quot;).val();
	var table = $(&quot; , &quot;'&quot; , &quot;#CommonDataTableId&quot; , &quot;'&quot; , &quot;).DataTable( {
        data: 	jsonData,
        deferRender:    true,
        &quot;pageLength&quot;: dynPageLength,
        &quot;scrollX&quot;: true,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;ordering&quot; : isReqSorting,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
            &quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px&quot; , &quot;'&quot; , &quot;> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class=&quot; , &quot;'&quot; , &quot;dataTables_ExportsInfoFilter&quot; , &quot;'&quot; , &quot; >[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
            &quot;next&quot;:       &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
            &quot;previous&quot;:   &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;
            },
        },
         columns: columns,
         columnDefs: [{
      	    &quot;defaultContent&quot;: &quot;&quot;,
      	    &quot;targets&quot;: &quot;_all&quot;
      	  }],
         &quot;order&quot;: [[ dynSortingIndex, &quot; , &quot;'&quot; , &quot;asc&quot; , &quot;'&quot; , &quot; ]],
         dom: &quot; , &quot;'&quot; , &quot;Blfrtip&quot; , &quot;'&quot; , &quot;,
         buttons: [
        	 {
        	 	 text:   &quot; , &quot;'&quot; , &quot;&lt;label class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>|&amp;nbsp;&amp;nbsp; Export:&amp;nbsp;&lt;/label>&quot; , &quot;'&quot; , &quot;,
        		 autoClose: &quot; , &quot;'&quot; , &quot;false&quot; , &quot;'&quot; , &quot;,
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;
        	 },
        	 {
        		 text:   emailSpan,
        		 autoClose: &quot; , &quot;'&quot; , &quot;false&quot; , &quot;'&quot; , &quot;,
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;
        	 },
        	 {
                 extend: exportExcel,
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: &quot; , &quot;'&quot; , &quot;export&quot; , &quot;'&quot; , &quot;,
                 	 stripHtml: true
                 },
                 filename: fileName,
                 text:   excelIcon,
                 autoClose: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;,
                 extension: excelReportType,
                 createEmptyCells: true,
                 title : titleHeaderName,
                 messageBottom: function () {
                     return disclaimerText;
                 	},
                     customize: function ( xlsx ) {
						var sheet = xlsx.xl.worksheets[&quot; , &quot;'&quot; , &quot;sheet1.xml&quot; , &quot;'&quot; , &quot;];
						$(&quot; , &quot;'&quot; , &quot;row c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;25&quot; , &quot;'&quot; , &quot; );
                        $(&quot; , &quot;'&quot; , &quot;c[r=A1] t&quot; , &quot;'&quot; , &quot;, sheet).text( titleHeaderName );
                        $(&quot; , &quot;'&quot; , &quot;row:first c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;27&quot; , &quot;'&quot; , &quot; ); 
                        $(&quot; , &quot;'&quot; , &quot;row:eq(1) c&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;27&quot; , &quot;'&quot; , &quot;);
                        
                        if(disclaimerText != null &amp;&amp; disclaimerText != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
                        {
                        	$(&quot; , &quot;'&quot; , &quot;row:last c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;55&quot; , &quot;'&quot; , &quot; );
	                        $(&quot; , &quot;'&quot; , &quot;row:last&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;customHeight&quot; , &quot;'&quot; , &quot;, 1);
	                        $(&quot; , &quot;'&quot; , &quot;row:last&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;ht&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;150&quot; , &quot;'&quot; , &quot;);
                        }
                        
                      },
                 button: [&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;]
             },
             {
                 extend: exportPdf,
                 orientation: &quot; , &quot;'&quot; , &quot;landscape&quot; , &quot;'&quot; , &quot;,
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: &quot; , &quot;'&quot; , &quot;export&quot; , &quot;'&quot; , &quot;,
                 	 stripHtml: true
                 },
                 text:    pdfIcon ,
                 filename: fileName,
                 autoClose: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 footer:&quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 messageBottom: function () {
                     return disclaimerText;
                  	},
               	 customize: function ( doc ) {
               		
               	   var cols = [];
               	   cols[0] = {text: &quot; , &quot;'&quot; , &quot;© JaMocha Tech Pvt. Ltd. 2009-2026&quot; , &quot;'&quot; , &quot;, alignment: &quot; , &quot;'&quot; , &quot;center&quot; , &quot;'&quot; , &quot;, fontSize:&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;, margin:[0, 0, 0, 0] };
               	   var objFooter = {};
               	   objFooter[&quot; , &quot;'&quot; , &quot;columns&quot; , &quot;'&quot; , &quot;] = cols;
               	   doc[&quot; , &quot;'&quot; , &quot;footer&quot; , &quot;'&quot; , &quot;]=objFooter;
               	   doc.styles.tableHeader.alignment=&quot;left&quot;;
               	 	
               	   doc[&quot; , &quot;'&quot; , &quot;header&quot; , &quot;'&quot; , &quot;]=(function(page, pages) {
							return {
								columns: [
									{
										alignment: &quot; , &quot;'&quot; , &quot;left&quot; , &quot;'&quot; , &quot;,
										text: &quot; , &quot;'&quot; , &quot;ProHance&quot; , &quot;'&quot; , &quot;,
										fontSize: 12,
									},
									{
										alignment: &quot; , &quot;'&quot; , &quot;right&quot; , &quot;'&quot; , &quot;,
										fontSize: 8,
										text: [&quot; , &quot;'&quot; , &quot;Page &quot; , &quot;'&quot; , &quot;, { text: page.toString() }]
									}
								],
								margin: 20
							}
						});
               	   
	               	if(dynNumericAlignColArr.length > 0)
	               	{
	               		var rowCount = doc.content[1].table.body.length;
		               	for (i = 0; i &lt; rowCount; i++) 
		               	{
		               		dynNumericAlignColArr.forEach(function(number) {
		                   		doc.content[1].table.body[i][number].alignment = &quot; , &quot;'&quot; , &quot;right&quot; , &quot;'&quot; , &quot;;
		                   	  });
		               	}
               		}
	               	
	               	doc.content[1].table.widths = dynWidthArr;
	                doc.content.forEach(function(item) {
	                       if (item.table) {
	                           item.layout = {
	                               hLineColor: function(i, node) { return &quot; , &quot;'&quot; , &quot;#000000&quot; , &quot;'&quot; , &quot;; }, // Set the border color here
	                               vLineColor: function(i, node) { return &quot; , &quot;'&quot; , &quot;#000000&quot; , &quot;'&quot; , &quot;; }, // Set the border color here
	                           };
	                       }
	                   });
	                doc.styles.tableHeader = {
	            		   fontSize: 11,
	                       bold: true,
	                       color: &quot; , &quot;'&quot; , &quot;#ffffff&quot; , &quot;'&quot; , &quot;,  // Set font color
	                       fillColor: &quot; , &quot;'&quot; , &quot;#006bb7&quot; , &quot;'&quot; , &quot;
		               };
               	 },
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;,
                 pageSize : &quot; , &quot;'&quot; , &quot;LEGAL&quot; , &quot;'&quot; , &quot;,
                 titleAttr : &quot; , &quot;'&quot; , &quot;PDF&quot; , &quot;'&quot; , &quot;,
                 title : titleHeaderName,
                 button: [ &quot; , &quot;'&quot; , &quot;pdf&quot; , &quot;'&quot; , &quot; ]
             }
         ]
    } );
	setTimeout(function() {
		table.columns.adjust().draw();
	}, 20); 
}

function fnDataTableExportexpdf(data)
{
	var param = data.data(&quot; , &quot;'&quot; , &quot;param&quot; , &quot;'&quot; , &quot;);
	fnDataTableExport(param);
}

function buildDataTableStructureForServerSideWorkTypes(pageLength, columnArr, sortingIndex, dynTableId, dataCollectionId, menuItemId, userGroupNameIds)
{
	$.fn.DataTable.ext.pager.numbers_length = 5;
	$.fn.DataTable.ext.pager.customPager = function (page, pages) {
	    const buttons = [];
	    const startPage = Math.max(page - 1, 1);
	    const endPage = Math.min(startPage + 1, pages - 2);

	    buttons.push(0);
	    if (startPage > 2)
	    {
	        buttons.push(&quot; , &quot;'&quot; , &quot;ellipsis&quot; , &quot;'&quot; , &quot;);
	    }

	    for (let i = startPage; i &lt;= endPage; i++)
	    {
	        buttons.push(i);
	    }

	    if (endPage &lt; pages - 1)
	    {
	        buttons.push(&quot; , &quot;'&quot; , &quot;ellipsis&quot; , &quot;'&quot; , &quot;);
	    }
	    buttons.push(pages - 1);
	    return buttons;
	};
	
	var table = $(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot;+dynTableId).DataTable( {
	    &quot;processing&quot;: false,
        &quot;serverSide&quot;: true,
        &quot;ajax&quot;: {
        	&quot;async&quot;: true,
        	headers: {
      	        &quot; , &quot;'&quot; , &quot;X-Content-Security-Policy-Nonce&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;xZrLNGLeef26xpNnnF1igA==&quot; , &quot;'&quot; , &quot;
      	      },
            &quot;url&quot;: &quot;/phworkoutput/getWOWorkType.htm&quot;,
            &quot;type&quot;: &quot;POST&quot;,
            &quot;data&quot;: function (d) {
            	 d.groupName = userGroupNameIds;
                 d.paramGroupId = dataCollectionId;
                 d.menuItemId = menuItemId;
                return d;
            },
            &quot;beforeSend&quot;: function() {
            	fnRemoveClassWithSameStyle(&quot;#wtDiv&quot;, &quot; , &quot;'&quot; , &quot;ph-wo-height&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            	$(&quot; , &quot;'&quot; , &quot;#wtDiv&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;ph-wo-display-none&quot; , &quot;'&quot; , &quot;);
            	fnRemoveClassWithSameStyle(&quot;#loaderDiv&quot;, &quot; , &quot;'&quot; , &quot;ph-wo-height&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            	$(&quot; , &quot;'&quot; , &quot;#loaderDiv&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;ph-wo-display-none&quot; , &quot;'&quot; , &quot;);
            },
            &quot;complete&quot;: function() {
            	fnRemoveClassWithSameStyle(&quot;#wtDiv&quot;, &quot; , &quot;'&quot; , &quot;ph-wo-height&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            	$(&quot; , &quot;'&quot; , &quot;#wtDiv&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;ph-wo-display-none&quot; , &quot;'&quot; , &quot;);
            	fnRemoveClassWithSameStyle(&quot;#loaderDiv&quot;, &quot; , &quot;'&quot; , &quot;ph-wo-height&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            	$(&quot; , &quot;'&quot; , &quot;#loaderDiv&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;ph-wo-display-none&quot; , &quot;'&quot; , &quot;);
            	table.columns.adjust();
            	var totalRows = $(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot;+dynTableId +&quot; tbody tr&quot;).length;
            	// Remove the existing scroll and add it based on a condition
            	if ($(&quot; , &quot;'&quot; , &quot;.customScrollWT&quot; , &quot;'&quot; , &quot;).parent().hasClass(&quot; , &quot;'&quot; , &quot;slimScrollDiv&quot; , &quot;'&quot; , &quot;)) 
				{
            	    $(&quot; , &quot;'&quot; , &quot;.customScrollWT&quot; , &quot;'&quot; , &quot;).slimScroll({ destroy: true });
            	    $(&quot; , &quot;'&quot; , &quot;.customScrollWT&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;);
            	    $(&quot; , &quot;'&quot; , &quot;.customScrollWT&quot; , &quot;'&quot; , &quot;).siblings(&quot; , &quot;'&quot; , &quot;.slimScrollBar, .slimScrollRail&quot; , &quot;'&quot; , &quot;).remove();
            	}
            	if ( totalRows > 6 || $(&quot; , &quot;'&quot; , &quot;#wtDiv&quot; , &quot;'&quot; , &quot;).height() > 330)
            	{
            		initSlimScroll();
            	}
            	setSelectedValue();
            	fnCheckSelectAllParameter();
            }
        },
        &quot;pageLength&quot;: pageLength,
        &quot;scrollX&quot;: true,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;ordering&quot; : true,
        &quot;lengthMenu&quot;: lengthMenu,
        initComplete : function() {
        	var self = this.api();
			var $filter = $(&quot; , &quot;'&quot; , &quot;#CommonDataTableWorkTypeId_filter&quot; , &quot;'&quot; , &quot;);
			var $input = $filter.find(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).unbind();
            var $searchButton = $(&quot; , &quot;'&quot; , &quot;&lt;span title=&quot;Search&quot; class=&quot;custom-datatable-btn&quot;>&lt;i class=&quot;fa fa-search&quot;/>&lt;/span>&quot; , &quot;'&quot; , &quot;)
                .click(function(event) {
                	event.preventDefault();
                    self.search($input.val()).draw();
                });
            var $clearButton = $(&quot; , &quot;'&quot; , &quot;&lt;span title=&quot;Clear&quot; class=&quot;custom-datatable-btn&quot;>&lt;i class=&quot;fa fa-times&quot;/>&lt;/span>&quot; , &quot;'&quot; , &quot;)
                .click(function(event) {
                	event.preventDefault();
                    self.search(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;).draw();
                });
            $filter.append($searchButton, $clearButton);
        },
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px&quot; , &quot;'&quot; , &quot;> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100&quot; , &quot;'&quot; , &quot; >[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
            	&quot;previous&quot;:   &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;
            },
            renderer: &quot; , &quot;'&quot; , &quot;customPager&quot; , &quot;'&quot; , &quot;,
        },
        columns: columnArr.map((column, index) => {
            if (index === 0) {
                column.orderable = false;
            }
            return column;
        }),
        order: [[1, &quot; , &quot;'&quot; , &quot;asc&quot; , &quot;'&quot; , &quot;]],
        dom: &quot; , &quot;'&quot; , &quot;Blfrtip&quot; , &quot;'&quot; , &quot;,
        buttons: [
        	 ]
    } );
}

function buildDataTableStructureForDynamicTable(jsonData, pageLength, columnArr, sortingIndex, dynTableId)
{
	$.fn.DataTable.ext.pager.numbers_length = 5;
	$.fn.DataTable.ext.pager.customPager = function (page, pages) {
	    const buttons = [];
	    const startPage = Math.max(page - 1, 1);
	    const endPage = Math.min(startPage + 1, pages - 2);

	    buttons.push(0);
	    if (startPage > 2)
	    {
	        buttons.push(&quot; , &quot;'&quot; , &quot;ellipsis&quot; , &quot;'&quot; , &quot;);
	    }

	    for (let i = startPage; i &lt;= endPage; i++)
	    {
	        buttons.push(i);
	    }

	    if (endPage &lt; pages - 1)
	    {
	        buttons.push(&quot; , &quot;'&quot; , &quot;ellipsis&quot; , &quot;'&quot; , &quot;);
	    }
	    buttons.push(pages - 1);
	    return buttons;
	};
	
	var table = $(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot;+dynTableId).DataTable( {
		&quot;data&quot;: 	jsonData,
        &quot;deferRender&quot;:    true,
	    &quot;processing&quot;: false,
        &quot;pageLength&quot;: pageLength,
        &quot;scrollX&quot;: true,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;ordering&quot; : true,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px&quot; , &quot;'&quot; , &quot;> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100&quot; , &quot;'&quot; , &quot;>[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
                &quot;previous&quot;:   &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;
            },
            renderer: &quot; , &quot;'&quot; , &quot;customPager&quot; , &quot;'&quot; , &quot;,
        },
        &quot;drawCallback&quot;: function() {
        	var totalRows = $(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot;+dynTableId +&quot; tbody tr&quot;).length;
        	if ( totalRows > 5 )
        	{
        		initSlimScroll();
        	}
        },
        columns: columnArr.map((column, index) => {
            if (index === 0) {
                column.orderable = false;
            }
            return column;
        }),
        order: [[1, &quot; , &quot;'&quot; , &quot;asc&quot; , &quot;'&quot; , &quot;]],
        dom: &quot; , &quot;'&quot; , &quot;Blfrtip&quot; , &quot;'&quot; , &quot;,
        buttons: []
        
    });
}

function buildDataTableStructureWithReportDataForWOWorkType(columns, jsonObjArr, menuItemId, persmissionObject)
{
	fnUpdateProperties(jsonObjArr);
	
	const dataKeys = columns.map(column => column.data);
	const dataKeysString = dataKeys.join(&quot;, &quot;);

	var isReqSorting = true;
	if(dynSortingIndex &lt; 0)
	{
		isReqSorting  = false;
		dynSortingIndex = 0;
	}
	var exportExcel = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	var exportPdf = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	var emailSpan = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	var excelIcon = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	var pdfIcon = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	
	
	excelIcon = &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/excel.png&quot; title=&quot;Export to Excel&quot; data-onclick=&quot;fnDataTableExportexpdf&quot; data-etype=&quot;excel&quot; align=&quot;middle&quot; class=&quot;ph-wo-vertical-align-middle ph-wo-cursor-pointer&quot;>&quot; , &quot;'&quot; , &quot;;
	
	if(recordSize > 10000)
	{
		pdfIcon = &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/pdf.png&quot; title=&quot;PDF file cannot be downloaded, as number of records more then 10000&quot; align=&quot;middle&quot; class=&quot;ph-wo-vertical-align-middle ph-wo-cursor-pointer&quot;>&quot; , &quot;'&quot; , &quot;;
		exportPdf = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	}	
	else
	{
		pdfIcon = &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/pdf.png&quot; title=&quot;Export to PDF&quot; data-onclick=&quot;fnDataTableExportexpdf&quot; data-etype=&quot;pdf&quot; class=&quot;ph-wo-vertical-align-middle ph-wo-cursor-pointer&quot;>&quot; , &quot;'&quot; , &quot;;
	}
	
	var excelReportType = $(&quot;#excelReportType&quot;).val();
	excelReportType = &quot;.&quot; + excelReportType.toLowerCase();
	
	titleHeaderName = replaceBackXMLEntities(dynTitleHeaderName);
	fileName = replaceBackXMLEntities(dynFileName);
	var counter = 1;
	var disclaimerText = $(&quot;#disclaimerText&quot;).val();
	var table = $(&quot; , &quot;'&quot; , &quot;#CommonDataTableId&quot; , &quot;'&quot; , &quot;).DataTable( {
	    &quot;processing&quot;: false,
        &quot;serverSide&quot;: true,
        &quot;ajax&quot;: {
        	&quot;async&quot;: true,
        	
            &quot;url&quot;: &quot;/phworkoutput/WOWorkTypeDetailTableAjax.htm&quot;,
            &quot;type&quot;: &quot;POST&quot;,
            headers	: {&quot; , &quot;'&quot; , &quot;X-Content-Security-Policy-Nonce&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;xZrLNGLeef26xpNnnF1igA==&quot; , &quot;'&quot; , &quot;},
             &quot;data&quot;: function (d) 
             {
            	 d.menuItemId = menuItemId;
                 d.persmissionObject = persmissionObject;
            	 d.selectedColumns = dataKeysString;
                return d;
            }
        },
        &quot;pageLength&quot;: dynPageLength,
        &quot;scrollX&quot;: true,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;ordering&quot; : isReqSorting,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px&quot; , &quot;'&quot; , &quot;> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100&quot; , &quot;'&quot; , &quot; >[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
            	&quot;previous&quot;:   &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;
            },
        },
         columns: columns,
         columnDefs: [{
     	    &quot;defaultContent&quot;: &quot;&quot;,
     	    &quot;targets&quot;: &quot;_all&quot;
     	  }],
         &quot;order&quot;: [[ dynSortingIndex, sortingOrder ]],
         dom: &quot; , &quot;'&quot; , &quot;Blfrtip&quot; , &quot;'&quot; , &quot;,
         buttons: [
        	 {
        		 text:   &quot; , &quot;'&quot; , &quot;&lt;label class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>|&amp;nbsp;&amp;nbsp; Export:&amp;nbsp;&lt;/label>&quot; , &quot;'&quot; , &quot;,
        		 autoClose: &quot; , &quot;'&quot; , &quot;false&quot; , &quot;'&quot; , &quot;,
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;
        	 },
        	 {
        		 text:   emailSpan,
        		 autoClose: &quot; , &quot;'&quot; , &quot;false&quot; , &quot;'&quot; , &quot;,
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;
        	 },
        	 {
                 extend: exportExcel,
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: &quot; , &quot;'&quot; , &quot;export&quot; , &quot;'&quot; , &quot;,
                 	 stripHtml: true
                 },
                 filename: fileName,
                 text:   excelIcon,
                 autoClose: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;,
                 extension: excelReportType,
                 createEmptyCells: true,
                 title : titleHeaderName,
                 messageBottom: function () {
                     return disclaimerText;
                 	},
                     customize: function ( xlsx ) {
						var sheet = xlsx.xl.worksheets[&quot; , &quot;'&quot; , &quot;sheet1.xml&quot; , &quot;'&quot; , &quot;];
						$(&quot; , &quot;'&quot; , &quot;row c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;25&quot; , &quot;'&quot; , &quot; );
                        $(&quot; , &quot;'&quot; , &quot;c[r=A1] t&quot; , &quot;'&quot; , &quot;, sheet).text( titleHeaderName );
                        $(&quot; , &quot;'&quot; , &quot;row:first c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;27&quot; , &quot;'&quot; , &quot; ); 
                        $(&quot; , &quot;'&quot; , &quot;row:eq(1) c&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;27&quot; , &quot;'&quot; , &quot;);
                        
                        if(disclaimerText != null &amp;&amp; disclaimerText != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
                        {
                        	$(&quot; , &quot;'&quot; , &quot;row:last c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;55&quot; , &quot;'&quot; , &quot; );
	                        $(&quot; , &quot;'&quot; , &quot;row:last&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;customHeight&quot; , &quot;'&quot; , &quot;, 1);
	                        $(&quot; , &quot;'&quot; , &quot;row:last&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;ht&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;150&quot; , &quot;'&quot; , &quot;);
                        }
                        
                      },
                 button: [&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;]
             },
             {
                 extend: exportPdf,
                 orientation: &quot; , &quot;'&quot; , &quot;landscape&quot; , &quot;'&quot; , &quot;,
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: &quot; , &quot;'&quot; , &quot;export&quot; , &quot;'&quot; , &quot;,
                 	 stripHtml: true
                 },
                 text:    pdfIcon ,
                 filename: fileName,
                 autoClose: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 footer:&quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 messageBottom: function () {
                     return disclaimerText;
                  	},
               	 customize: function ( doc ) {
               		
               	   var cols = [];
               	   cols[0] = {text: &quot; , &quot;'&quot; , &quot;© JaMocha Tech Pvt. Ltd. 2009-2026&quot; , &quot;'&quot; , &quot;, alignment: &quot; , &quot;'&quot; , &quot;center&quot; , &quot;'&quot; , &quot;, fontSize:&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;, margin:[0, 0, 0, 0] };
               	   var objFooter = {};
               	   objFooter[&quot; , &quot;'&quot; , &quot;columns&quot; , &quot;'&quot; , &quot;] = cols;
               	   doc[&quot; , &quot;'&quot; , &quot;footer&quot; , &quot;'&quot; , &quot;]=objFooter;
               	   doc.styles.tableHeader.alignment=&quot;left&quot;;
               	 	
               	   doc[&quot; , &quot;'&quot; , &quot;header&quot; , &quot;'&quot; , &quot;]=(function(page, pages) {
							return {
								columns: [
									{
										alignment: &quot; , &quot;'&quot; , &quot;left&quot; , &quot;'&quot; , &quot;,
										text: &quot; , &quot;'&quot; , &quot;ProHance&quot; , &quot;'&quot; , &quot;,
										fontSize: 12,
									},
									{
										alignment: &quot; , &quot;'&quot; , &quot;right&quot; , &quot;'&quot; , &quot;,
										fontSize: 8,
										text: [&quot; , &quot;'&quot; , &quot;Page &quot; , &quot;'&quot; , &quot;, { text: page.toString() }]
									}
								],
								margin: 20
							}
						});
               	   
	               	if(dynNumericAlignColArr.length > 0)
	               	{
	               		var rowCount = doc.content[1].table.body.length;
		               	for (i = 0; i &lt; rowCount; i++) 
		               	{
		               		dynNumericAlignColArr.forEach(function(number) {
		                   		doc.content[1].table.body[i][number].alignment = &quot; , &quot;'&quot; , &quot;right&quot; , &quot;'&quot; , &quot;;
		                   	  });
		               	}
               		}
	               	
	               	doc.content[1].table.widths = dynWidthArr;
	                doc.content.forEach(function(item) {
	                       if (item.table) {
	                           item.layout = {
	                               hLineColor: function(i, node) { return &quot; , &quot;'&quot; , &quot;#000000&quot; , &quot;'&quot; , &quot;; }, // Set the border color here
	                               vLineColor: function(i, node) { return &quot; , &quot;'&quot; , &quot;#000000&quot; , &quot;'&quot; , &quot;; }, // Set the border color here
	                           };
	                       }
	                   });
	                doc.styles.tableHeader = {
	            		   fontSize: 11,
	                       bold: true,
	                       color: &quot; , &quot;'&quot; , &quot;#ffffff&quot; , &quot;'&quot; , &quot;,  // Set font color
	                       fillColor: &quot; , &quot;'&quot; , &quot;#006bb7&quot; , &quot;'&quot; , &quot;
		               };
               	 },
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;,
                 pageSize : &quot; , &quot;'&quot; , &quot;LEGAL&quot; , &quot;'&quot; , &quot;,
                 titleAttr : &quot; , &quot;'&quot; , &quot;PDF&quot; , &quot;'&quot; , &quot;,
                 title : titleHeaderName,
                 button: [ &quot; , &quot;'&quot; , &quot;pdf&quot; , &quot;'&quot; , &quot; ]
             }
         ]
    } );
	setTimeout(function() {
		table.columns.adjust().draw();
	}, 20); 
}

function buildDataTableStructureForDashboardWorkType(pageLength, columnArr, sortingIndex, dynTableId, dataParam, urlStr)
{
	if ($.fn.dataTable.isDataTable(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot;+dynTableId))
	{
		var existingTable = $(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot;+dynTableId);
		existingTable.DataTable().clear().destroy();
		existingTable.off().empty();
	}
	$.fn.DataTable.ext.pager.numbers_length = 5;
	$.fn.DataTable.ext.pager.customPager = function (page, pages) {
	    const buttons = [];
	    const startPage = Math.max(page - 1, 1);
	    const endPage = Math.min(startPage + 1, pages - 2);

	    buttons.push(0);
	    if (startPage > 2)
	    {
	        buttons.push(&quot; , &quot;'&quot; , &quot;ellipsis&quot; , &quot;'&quot; , &quot;);
	    }

	    for (let i = startPage; i &lt;= endPage; i++)
	    {
	        buttons.push(i);
	    }

	    if (endPage &lt; pages - 1)
	    {
	        buttons.push(&quot; , &quot;'&quot; , &quot;ellipsis&quot; , &quot;'&quot; , &quot;);
	    }
	    buttons.push(pages - 1);
	    return buttons;
	};
	
	var table = $(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot;+dynTableId).DataTable( {
	    &quot;processing&quot;: false,
        &quot;serverSide&quot;: true,
        &quot;ajax&quot;: {
        	&quot;async&quot;: true,
            &quot;url&quot;: urlStr,
            &quot;type&quot;: &quot;POST&quot;,
            headers	: {&quot; , &quot;'&quot; , &quot;X-Content-Security-Policy-Nonce&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;xZrLNGLeef26xpNnnF1igA==&quot; , &quot;'&quot; , &quot;},
            &quot;data&quot;: dataParam,
            &quot;beforeSend&quot;: function() {
            	fnRemoveClassWithSameStyle(&quot;.content-loading&quot;, &quot; , &quot;'&quot; , &quot;ph-wo-display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            	$(&quot;.content-loading&quot;).removeClass(&quot;ph-wo-display-none&quot;);
            	fnRemoveClassWithSameStyle(&quot;#specificWorktypeContent&quot;, &quot; , &quot;'&quot; , &quot;ph-wo-display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
				$(&quot;#specificWorktypeContent&quot;).addClass(&quot;ph-wo-display-none&quot;);
            },
            &quot;complete&quot;: function() {
            	setSelectedValue();
            	fnRemoveClassWithSameStyle(&quot;.content-loading&quot;, &quot; , &quot;'&quot; , &quot;ph-wo-display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            	$(&quot;.content-loading&quot;).addClass(&quot;ph-wo-display-none&quot;);
            	fnRemoveClassWithSameStyle(&quot;#specificWorktypeContent&quot;, &quot; , &quot;'&quot; , &quot;ph-wo-display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
				$(&quot;#specificWorktypeContent&quot;).removeClass(&quot;ph-wo-display-none&quot;);
			  	table.columns.adjust();
			  	initSlimScroll();
            }
        },
        initComplete : function() {
        	$(&quot; , &quot;'&quot; , &quot;.dataTables_filter&quot; , &quot;'&quot; , &quot;).removeClass(&quot;category-search&quot;);
		  	$(&quot; , &quot;'&quot; , &quot;.dataTables_filter&quot; , &quot;'&quot; , &quot;).addClass(&quot;workType-search&quot;);
        	var self = this.api();
            var $filter = $(&quot; , &quot;'&quot; , &quot;.workType-search&quot; , &quot;'&quot; , &quot;);
            var $input = $filter.find(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).unbind();
            var $searchButton = $(&quot; , &quot;'&quot; , &quot;&lt;span title=&quot;Search&quot; class=&quot;custom-datatable-btn&quot;>&lt;i class=&quot;fa fa-search&quot;/>&lt;/span>&quot; , &quot;'&quot; , &quot;)
                .click(function(event) {
                	event.preventDefault();
                	if($input.val().trim() != null &amp;&amp; $input.val().trim() != &quot;&quot;)
                	{
                    	self.search($input.val()).draw();
                    	initSlimScroll();
                	}
                });
            var $clearButton = $(&quot; , &quot;'&quot; , &quot;&lt;span title=&quot;Clear&quot; class=&quot;custom-datatable-btn&quot;>&lt;i class=&quot;fa fa-times&quot;/>&lt;/span>&quot; , &quot;'&quot; , &quot;)
                .click(function(event) {
                	event.preventDefault();
                    self.search(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;).draw();
                });
            if (!$filter.has($searchButton).length) {
            	  $filter.append($searchButton);
            	}

            	if (!$filter.has($clearButton).length) {
            	  $filter.append($clearButton);
            	}
        },
        &quot;pageLength&quot;: pageLength,
        &quot;scrollX&quot;: true,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;scrollCollapse&quot;: true,
        &quot;ordering&quot; : true,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px&quot; , &quot;'&quot; , &quot;> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100&quot; , &quot;'&quot; , &quot; >[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
            	&quot;previous&quot;:   &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;
            },
            renderer: &quot; , &quot;'&quot; , &quot;customPager&quot; , &quot;'&quot; , &quot;,
        },
		columns: columnArr.map((column, index) => {
			column.orderable = index !== 0;
            return column;
        }),
        order: [[1, &quot; , &quot;'&quot; , &quot;asc&quot; , &quot;'&quot; , &quot;]],
        dom: &quot; , &quot;'&quot; , &quot;Blfrtip&quot; , &quot;'&quot; , &quot;,
        buttons: [],
    });
}

function buildDataTableStructureForWorkTypeMapping(columns, jsonObjArr, menuItemId, persmissionObject)
{
	fnUpdateProperties(jsonObjArr);
	
	const dataKeys = columns.map(column => column.data);
	const dataKeysString = dataKeys.join(&quot;, &quot;);

	var isReqSorting = true;
	if(dynSortingIndex &lt; 0)
	{
		isReqSorting  = false;
		dynSortingIndex = 0;
	}
	titleHeaderName = replaceBackXMLEntities(dynTitleHeaderName);
	fileName = replaceBackXMLEntities(dynFileName);
	var counter = 1;
	var disclaimerText = $(&quot;#disclaimerText&quot;).val();
	var table = $(&quot; , &quot;'&quot; , &quot;#CommonDataTableId&quot; , &quot;'&quot; , &quot;).DataTable( {
	    &quot;processing&quot;: false,
        &quot;serverSide&quot;: true,
        &quot;ajax&quot;: {
        	&quot;async&quot;: true,
        	
            &quot;url&quot;: &quot;/phworkoutput/WorkTypeMappingDetailTableAjax.htm&quot;,
            &quot;type&quot;: &quot;POST&quot;,
            headers	: {&quot; , &quot;'&quot; , &quot;X-Content-Security-Policy-Nonce&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;xZrLNGLeef26xpNnnF1igA==&quot; , &quot;'&quot; , &quot;},
             &quot;data&quot;: function (d) 
             {
            	 d.menuItemId = menuItemId;
                 d.persmissionObject = persmissionObject;
            	 d.selectedColumns = dataKeysString;
                return d;
            }
        },
        &quot;pageLength&quot;: dynPageLength,
        &quot;scrollX&quot;: true,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;ordering&quot; : isReqSorting,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px&quot; , &quot;'&quot; , &quot;> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100&quot; , &quot;'&quot; , &quot; >[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
            	&quot;previous&quot;:   &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;
            },
        },
         columns: columns,
         columnDefs: [{
     	    &quot;defaultContent&quot;: &quot;&quot;,
     	    &quot;targets&quot;: &quot;_all&quot;
     	  }],
         &quot;order&quot;: [[ dynSortingIndex, sortingOrder ]],
         dom: &quot; , &quot;'&quot; , &quot;Blfrtip&quot; , &quot;'&quot; , &quot;,
         buttons: [
        	 ],
    });
	setTimeout(function() {
		table.columns.adjust().draw();
	}, 20); 
}

function buildDataTableStructureForWOCategories(pageLength, columnArr, sortingIndex, dynTableId, dataParam)
{
	$.fn.DataTable.ext.pager.numbers_length = 5;
	$.fn.DataTable.ext.pager.customPager = function (page, pages) {
	    const buttons = [];
	    const startPage = Math.max(page - 1, 1);
	    const endPage = Math.min(startPage + 1, pages - 2);

	    buttons.push(0);
	    if (startPage > 2)
	    {
	        buttons.push(&quot; , &quot;'&quot; , &quot;ellipsis&quot; , &quot;'&quot; , &quot;);
	    }

	    for (let i = startPage; i &lt;= endPage; i++)
	    {
	        buttons.push(i);
	    }

	    if (endPage &lt; pages - 1)
	    {
	        buttons.push(&quot; , &quot;'&quot; , &quot;ellipsis&quot; , &quot;'&quot; , &quot;);
	    }
	    buttons.push(pages - 1);
	    return buttons;
	};
	
	var table = $(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot;+dynTableId).DataTable( {
	    &quot;processing&quot;: false,
        &quot;serverSide&quot;: true,
        &quot;ajax&quot;: {
        	&quot;async&quot;: true,
            &quot;url&quot;: &quot;/phworkoutput/getWOCategories.htm&quot;,
            &quot;type&quot;: &quot;POST&quot;,
            headers	: {&quot; , &quot;'&quot; , &quot;X-Content-Security-Policy-Nonce&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;xZrLNGLeef26xpNnnF1igA==&quot; , &quot;'&quot; , &quot;},
            &quot;data&quot;: dataParam,
            &quot;beforeSend&quot;: function() {
            	fnRemoveClassWithSameStyle(&quot;.content-loading&quot;, &quot; , &quot;'&quot; , &quot;ph-wo-display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            	$(&quot;.content-loading&quot;).removeClass(&quot;ph-wo-display-none&quot;);
            	$(&quot;#specificCategoryContent&quot;).removeClass(&quot;ph-wo-display-none&quot;);
            	fnRemoveClassWithSameStyle(&quot;.specificCategoryContent&quot;, &quot; , &quot;'&quot; , &quot;ph-wo-display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            	$(&quot;.specificCategoryContent&quot;).addClass(&quot;ph-wo-display-none&quot;);
				$(&quot;#specificCategoryContent&quot;).removeClass(&quot;ph-wo-display-none&quot;);
				fnRemoveClassWithSameStyle(&quot;.specificCategoryContent&quot;, &quot; , &quot;'&quot; , &quot;ph-wo-display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
				$(&quot;#specificCategoryContent&quot;).addClass(&quot;ph-wo-display-none&quot;);
            },
            &quot;complete&quot;: function() {
            	setSelectedCategoryValue();
            	fnRemoveClassWithSameStyle(&quot;.content-loading&quot;, &quot; , &quot;'&quot; , &quot;ph-wo-display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            	$(&quot;.content-loading&quot;).addClass(&quot;ph-wo-display-none&quot;);
            	fnRemoveClassWithSameStyle(&quot;#specificCategoryContent&quot;, &quot; , &quot;'&quot; , &quot;ph-wo-display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
				$(&quot;#specificCategoryContent&quot;).removeClass(&quot;ph-wo-display-none&quot;);
				fnRemoveClassWithSameStyle(&quot;.specificCategoryContent&quot;, &quot; , &quot;'&quot; , &quot;ph-wo-display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
			  	table.columns.adjust();
			  	initCategorySlimScroll();
            }
        },
        initComplete : function() {
        	$(&quot; , &quot;'&quot; , &quot;.dataTables_filter&quot; , &quot;'&quot; , &quot;).removeClass(&quot;workType-search&quot;);
		  	$(&quot; , &quot;'&quot; , &quot;.dataTables_filter&quot; , &quot;'&quot; , &quot;).addClass(&quot;category-search&quot;);
        	var self = this.api();
            var $filter = $(&quot; , &quot;'&quot; , &quot;.category-search&quot; , &quot;'&quot; , &quot;);
            var $input = $filter.find(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).unbind();
            var $searchButton = $(&quot; , &quot;'&quot; , &quot;&lt;span title=&quot;Search&quot; class=&quot;custom-datatable-btn&quot;>&lt;i class=&quot;fa fa-search category&quot;/>&lt;/span>&quot; , &quot;'&quot; , &quot;)
                .click(function(event) {
                	event.preventDefault();
                	if($input.val().trim() != null &amp;&amp; $input.val().trim() != &quot;&quot;)
                	{
                    	self.search($input.val()).draw();
                	}
                });
            var $clearButton = $(&quot; , &quot;'&quot; , &quot;&lt;span title=&quot;Clear&quot; class=&quot;custom-datatable-btn&quot;>&lt;i class=&quot;fa fa-times category&quot;/>&lt;/span>&quot; , &quot;'&quot; , &quot;)
                .click(function(event) {
                	event.preventDefault();
                    self.search(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;).draw();
                });
            if (!$filter.has($searchButton).length) 
            {
            	  $filter.append($searchButton);
            }

            if (!$filter.has($clearButton).length) 
            {
            	  $filter.append($clearButton);
            }
        },
        &quot;pageLength&quot;: pageLength,
        &quot;scrollX&quot;: true,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;scrollCollapse&quot;: true,
        &quot;ordering&quot; : true,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px&quot; , &quot;'&quot; , &quot;> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100&quot; , &quot;'&quot; , &quot; >[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
            	&quot;previous&quot;:   &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;
            },
            renderer: &quot; , &quot;'&quot; , &quot;customPager&quot; , &quot;'&quot; , &quot;,
        },
		columns: columnArr.map((column, index) => {
			column.orderable = index !== 0;
            return column;
        }),
        order: [[1, &quot; , &quot;'&quot; , &quot;asc&quot; , &quot;'&quot; , &quot;]],
        dom: &quot; , &quot;'&quot; , &quot;Blfrtip&quot; , &quot;'&quot; , &quot;,
        buttons: [],
    });
}

/* Build Data Tabel with Default Data Table Export along with JSONArray param*/
function buildDataTableStructureForWOCategoryArr(columns, jsonObjArr, menuItemId, persmissionObject)
{
	
	fnUpdateProperties(jsonObjArr);
	const dataKeys = columns.map(column => column.data);
	const dataKeysString = dataKeys.join(&quot;, &quot;);
	var isReqSorting = true;
	if(dynSortingIndex &lt; 0)
	{
		isReqSorting  = false;
		dynSortingIndex = 0;
	}
	
	var exportExcel = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	var exportPdf = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	var emailSpan = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	var excelIcon = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	var pdfIcon = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	
	excelIcon = &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/excel.png&quot; title=&quot;Export to Excel&quot; data-onclick=&quot;fnDataTableExportexpdf&quot; data-etype = &quot;excel&quot; class=&quot;ph-wo-vertical-align-middle ph-dash-cursor-pointer&quot;>&quot; , &quot;'&quot; , &quot;;
	if(recordSize > 10000)
	{
		pdfIcon = &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/pdf.png&quot; title=&quot;PDF file cannot be downloaded, as number of records more then 10000&quot; class=&quot;ph-wo-vertical-align-middle ph-dash-cursor-pointer&quot;>&quot; , &quot;'&quot; , &quot;;
		exportPdf = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	}	
	else
	{
		pdfIcon = &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/pdf.png&quot; title=&quot;Export to PDF&quot; data-onclick=&quot;fnDataTableExportexpdf&quot; data-etype=&quot;pdf&quot; class=&quot;ph-wo-vertical-align-middle ph-dash-cursor-pointer&quot;>&quot; , &quot;'&quot; , &quot;;
	}
	
	var excelReportType = $(&quot;#excelReportType&quot;).val();
	excelReportType = &quot;.&quot; + excelReportType.toLowerCase();
	var disclaimerText = $(&quot;#disclaimerText&quot;).val();
	titleHeaderName = replaceBackXMLEntities(dynTitleHeaderName);
	fileName = replaceBackXMLEntities(dynFileName);
	var table = $(&quot; , &quot;'&quot; , &quot;#CommonDataTableId&quot; , &quot;'&quot; , &quot;).DataTable( {
        deferRender:    true,
        &quot;processing&quot;: false,
        &quot;serverSide&quot;: true,
        &quot;ajax&quot;: {
        	&quot;async&quot;: true,
        	
            &quot;url&quot;: &quot;/phworkoutput/WOCategoryDetailTableAjax.htm&quot;,
            &quot;type&quot;: &quot;POST&quot;,
            headers	: {&quot; , &quot;'&quot; , &quot;X-Content-Security-Policy-Nonce&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;xZrLNGLeef26xpNnnF1igA==&quot; , &quot;'&quot; , &quot;},
             &quot;data&quot;: function (d) 
             {
            	 d.menuItemId = menuItemId;
                 d.persmissionObject = persmissionObject;
            	 d.selectedColumns = dataKeysString;
                return d;
            }
        },
        &quot;pageLength&quot;: dynPageLength,
        &quot;bDestroy&quot;: true,
        &quot;scrollX&quot;: true,
        &quot;ordering&quot; : isReqSorting,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px&quot; , &quot;'&quot; , &quot;> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100&quot; , &quot;'&quot; , &quot; >[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
            	&quot;previous&quot;:   &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;
            },
        },
         columns: columns,
         &quot;order&quot;: [[ dynSortingIndex, &quot; , &quot;'&quot; , &quot;asc&quot; , &quot;'&quot; , &quot; ]],
         rowGroup: {
             dataSrc: 88
         },
         dom: &quot; , &quot;'&quot; , &quot;Blfrtip&quot; , &quot;'&quot; , &quot;,
         buttons: [
        	 {
        		 text:   &quot; , &quot;'&quot; , &quot;&lt;label class=&quot;ph-wo-font-weight-300 ph-wo-margin-left-10px ph-wo-margin-top-10px&quot;>|&amp;nbsp;&amp;nbsp; Export:&amp;nbsp;&lt;/label>&quot; , &quot;'&quot; , &quot;,
        		 autoClose: &quot; , &quot;'&quot; , &quot;false&quot; , &quot;'&quot; , &quot;,
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;
        	 },
             {
                 extend: exportExcel,
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: &quot; , &quot;'&quot; , &quot;export&quot; , &quot;'&quot; , &quot;,
                 	 stripHtml: true
                 },
                 destroy:true,
                 filename: fileName,
                 text:  excelIcon,
                 autoClose: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;,
                 extension: excelReportType,
                 createEmptyCells: true,
                 title : titleHeaderName,
                 messageBottom: function () {
                     return disclaimerText;
                 	},
                     customize: function ( xlsx ) {
						 var sheet = xlsx.xl.worksheets[&quot; , &quot;'&quot; , &quot;sheet1.xml&quot; , &quot;'&quot; , &quot;];
						$(&quot; , &quot;'&quot; , &quot;row c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;25&quot; , &quot;'&quot; , &quot; );
                        $(&quot; , &quot;'&quot; , &quot;c[r=A1] t&quot; , &quot;'&quot; , &quot;, sheet).text( titleHeaderName );
                        $(&quot; , &quot;'&quot; , &quot;row:first c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;27&quot; , &quot;'&quot; , &quot; ); 
                        $(&quot; , &quot;'&quot; , &quot;row:eq(1) c&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;27&quot; , &quot;'&quot; , &quot;);
                        
                        if(disclaimerText != null &amp;&amp; disclaimerText != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
                        {
                        	$(&quot; , &quot;'&quot; , &quot;row:last c&quot; , &quot;'&quot; , &quot;, sheet).attr( &quot; , &quot;'&quot; , &quot;s&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;55&quot; , &quot;'&quot; , &quot; );
	                        $(&quot; , &quot;'&quot; , &quot;row:last&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;customHeight&quot; , &quot;'&quot; , &quot;, 1);
	                        $(&quot; , &quot;'&quot; , &quot;row:last&quot; , &quot;'&quot; , &quot;, sheet).attr(&quot; , &quot;'&quot; , &quot;ht&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;150&quot; , &quot;'&quot; , &quot;);
                        }
                         
                      },
                      
                 button: [&quot; , &quot;'&quot; , &quot;excel&quot; , &quot;'&quot; , &quot;]

             },
             {
                 extend: exportPdf,
                 orientation: &quot; , &quot;'&quot; , &quot;landscape&quot; , &quot;'&quot; , &quot;,
                 exportOptions: {
                 	columns: dynExportColumns,
                 	orthogonal: &quot; , &quot;'&quot; , &quot;export&quot; , &quot;'&quot; , &quot;,
                 	 stripHtml: true
                 },
                 text: pdfIcon,
                 filename: fileName,
                 autoClose: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 footer:&quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                 destroy:true,
                 messageBottom: function () {
                     return disclaimerText;
                  	},
               	 customize: function ( doc ) {
               		
               	   var cols = [];
               	   cols[0] = {text: &quot; , &quot;'&quot; , &quot;© JaMocha Tech Pvt. Ltd. 2009-2026&quot; , &quot;'&quot; , &quot;, alignment: &quot; , &quot;'&quot; , &quot;center&quot; , &quot;'&quot; , &quot;, fontSize:&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;, margin:[0, 0, 0, 0] };
               	   var objFooter = {};
               	   objFooter[&quot; , &quot;'&quot; , &quot;columns&quot; , &quot;'&quot; , &quot;] = cols;
               	   doc[&quot; , &quot;'&quot; , &quot;footer&quot; , &quot;'&quot; , &quot;]=objFooter;
               	   doc.styles.tableHeader.fillColor = &quot; , &quot;'&quot; , &quot;#006bb7&quot; , &quot;'&quot; , &quot;;
               	   doc.styles.tableHeader.alignment=&quot;left&quot;;
               	 	
               	   doc[&quot; , &quot;'&quot; , &quot;header&quot; , &quot;'&quot; , &quot;]=(function(page, pages) {
							return {
								columns: [
									{
										alignment: &quot; , &quot;'&quot; , &quot;left&quot; , &quot;'&quot; , &quot;,
										text: &quot; , &quot;'&quot; , &quot;ProHance&quot; , &quot;'&quot; , &quot;,
										fontSize: 12,
									},
									{
										alignment: &quot; , &quot;'&quot; , &quot;right&quot; , &quot;'&quot; , &quot;,
										fontSize: 8,
										text: [&quot; , &quot;'&quot; , &quot;Page &quot; , &quot;'&quot; , &quot;, { text: page.toString() }]
									}
								],
								margin: 20
							}
						});
	               	if(dynNumericAlignColArr.length > 0)
	               	{
	               		var rowCount = doc.content[1].table.body.length;
		               	for (i = 0; i &lt; rowCount; i++) 
		               	{
		               		dynNumericAlignColArr.forEach(function(number) {
		                   		doc.content[1].table.body[i][number].alignment = &quot; , &quot;'&quot; , &quot;right&quot; , &quot;'&quot; , &quot;;
		                   	  });
		               	}
	               	}
               	
	               doc.content[1].table.widths=	dynWidthArr;
               	
               	 },
                 tag: &quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;,
                 pageSize : &quot; , &quot;'&quot; , &quot;LEGAL&quot; , &quot;'&quot; , &quot;,
                 titleAttr : &quot; , &quot;'&quot; , &quot;PDF&quot; , &quot;'&quot; , &quot;,
                 title : titleHeaderName,
                 button: [ &quot; , &quot;'&quot; , &quot;pdf&quot; , &quot;'&quot; , &quot; ]
             }
         ],
         drawCallback: function() {
       	  var hasRows = this.api().rows({ filter: &quot; , &quot;'&quot; , &quot;applied&quot; , &quot;'&quot; , &quot; }).data().length > 0;
       	 $(&quot; , &quot;'&quot; , &quot;.buttons-excel&quot; , &quot;'&quot; , &quot;)[0].style.visibility = hasRows ? &quot; , &quot;'&quot; , &quot;visible&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;;
       	 $(&quot; , &quot;'&quot; , &quot;.buttons-pdf&quot; , &quot;'&quot; , &quot;)[0].style.visibility = hasRows ? &quot; , &quot;'&quot; , &quot;visible&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;;
       	}
    } );
	
	setTimeout(function() {
		table.columns.adjust().draw();
	}, 20); 	
}


function buildScrollXDataTableStructure(jsonData, pageLength, columns, sortingIndex, isScrollX)
{
	var isReqSorting = true;
	if(sortingIndex &lt; 0)
	{
		isReqSorting  = false;
		sortingIndex = 0;
	}
	var table = $(&quot; , &quot;'&quot; , &quot;#CommonDataTableId&quot; , &quot;'&quot; , &quot;).DataTable( {
        data: jsonData,
        deferRender:    true,
        &quot;pageLength&quot;: pageLength,
        &quot;ordering&quot; : isReqSorting,
        &quot;autoWidth&quot;:false,
        &quot;scrollX&quot;: false,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px&quot; , &quot;'&quot; , &quot;> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100&quot; , &quot;'&quot; , &quot;>[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
                &quot;previous&quot;:   &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;
            },
        },
         columns: columns,
         &quot;order&quot;: [[ sortingIndex, &quot; , &quot;'&quot; , &quot;asc&quot; , &quot;'&quot; , &quot; ]],
         dom: &quot; , &quot;'&quot; , &quot;Blfrtip&quot; , &quot;'&quot; , &quot;,
         buttons: []
    } );
}


function buildDataTableStructureForWorkType(jsonData, pageLength, columns, sortingIndex, isScrollX)
{
	$.fn.DataTable.ext.pager.numbers_length = 5;
	$.fn.DataTable.ext.pager.customPager = function (page, pages) {
	    const buttons = [];
	    const startPage = Math.max(page - 1, 1);
	    const endPage = Math.min(startPage + 1, pages - 2);
	    buttons.push(0);
	    if (startPage > 2) buttons.push(&quot; , &quot;'&quot; , &quot;ellipsis&quot; , &quot;'&quot; , &quot;);
	    for (let i = startPage; i &lt;= endPage; i++) buttons.push(i);
	    if (endPage &lt; pages - 1) buttons.push(&quot; , &quot;'&quot; , &quot;ellipsis&quot; , &quot;'&quot; , &quot;);
	    buttons.push(pages - 1);
	    return buttons;
	};
	var table = $(&quot; , &quot;'&quot; , &quot;#CommonDataTableWorkTypeId&quot; , &quot;'&quot; , &quot;).DataTable( {
	    data: jsonData,
	    deferRender: true,
	    &quot;processing&quot;: false,
	    &quot;bDestroy&quot;: true,
	    &quot;pageLength&quot;: pageLength,
	    &quot;scrollX&quot;: true,
	    &quot;sScrollXInner&quot;: &quot;100%&quot;,
	    &quot;scrollCollapse&quot;: true,
	    &quot;ordering&quot;: true,
	    &quot;lengthMenu&quot;: lengthMenu,
	    &quot;language&quot;: {
	        &quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
	        &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px&quot; , &quot;'&quot; , &quot;> Displaying _START_ to _END_ &lt;label>&quot;,
	        &quot;infoEmpty&quot;: &quot;No records to display&quot;,
	        &quot;infoFiltered&quot;: &quot;&lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100&quot; , &quot;'&quot; , &quot; >[Filtered from _MAX_ records] &lt;/label>&quot;,
	        &quot;search&quot;: &quot;Search&quot;,
	        &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
	        &quot;paginate&quot;: {
	            &quot;next&quot;: &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
	            &quot;previous&quot;: &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;
	        },
	        renderer: &quot; , &quot;'&quot; , &quot;customPager&quot; , &quot;'&quot; , &quot;,
	    },
	    columns: columns.map((column, index) => {
	        column.orderable = index !== 0;
	        return column;
	    }),
	    order: [[1, &quot; , &quot;'&quot; , &quot;asc&quot; , &quot;'&quot; , &quot;]],
	    dom: &quot; , &quot;'&quot; , &quot;Blfrtip&quot; , &quot;'&quot; , &quot;,
	    buttons: [],
	} );
}

function buildDataTableStructureForWTAttributeOptions(pageLength, columnArr, sortingIndex, dynTableId, dataParam)
{
	$.fn.DataTable.ext.pager.numbers_length = 5;
	$.fn.DataTable.ext.pager.customPager = function (page, pages) {
	    const buttons = [];
	    const startPage = Math.max(page - 1, 1);
	    const endPage = Math.min(startPage + 1, pages - 2);

	    buttons.push(0);
	    if (startPage > 2)
	    {
	        buttons.push(&quot; , &quot;'&quot; , &quot;ellipsis&quot; , &quot;'&quot; , &quot;);
	    }

	    for (let i = startPage; i &lt;= endPage; i++)
	    {
	        buttons.push(i);
	    }

	    if (endPage &lt; pages - 1)
	    {
	        buttons.push(&quot; , &quot;'&quot; , &quot;ellipsis&quot; , &quot;'&quot; , &quot;);
	    }
	    buttons.push(pages - 1);
	    return buttons;
	};
	
	var table = $(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot;+dynTableId).DataTable( {
	    &quot;processing&quot;: false,
        &quot;serverSide&quot;: true,
        &quot;ajax&quot;: {
        	&quot;async&quot;: true,
            &quot;url&quot;: &quot;/phworkoutput/getSpecificWorktypeAttrOptions.htm&quot;,
            &quot;type&quot;: &quot;POST&quot;,
            headers	: {&quot; , &quot;'&quot; , &quot;X-Content-Security-Policy-Nonce&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;xZrLNGLeef26xpNnnF1igA==&quot; , &quot;'&quot; , &quot;},
            &quot;data&quot;: dataParam,
            &quot;dataSrc&quot;: function(json) {
            	parameterJson = json.data;
                return json.data;
            },
            &quot;beforeSend&quot;: function() {
            	$(&quot;#loading_attr_options&quot;).removeClass(&quot;ph-wo-display-none&quot;);
				fnRemoveClassWithSameStyle(&quot;.content-loading&quot;, &quot; , &quot;'&quot; , &quot;ph-wo-display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            	$(&quot;.content-loading&quot;).removeClass(&quot;ph-wo-display-none&quot;);
            	$(&quot;#specificAttrOptionContent&quot;).removeClass(&quot;ph-wo-display-none&quot;);
            	fnRemoveClassWithSameStyle(&quot;.specificAttrOptionContent&quot;, &quot; , &quot;'&quot; , &quot;ph-wo-display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            	$(&quot;.specificAttrOptionContent&quot;).addClass(&quot;ph-wo-display-none&quot;);
				$(&quot;#specificAttrOptionContent&quot;).removeClass(&quot;ph-wo-display-none&quot;);
				fnRemoveClassWithSameStyle(&quot;.specificAttrOptionContent&quot;, &quot; , &quot;'&quot; , &quot;ph-wo-display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
				$(&quot;#specificAttrOptionContent&quot;).addClass(&quot;ph-wo-display-none&quot;);
            },
            &quot;complete&quot;: function() {
            	$(&quot;#loading_attr_options&quot;).addClass(&quot;ph-wo-display-none&quot;);
            	fnRemoveClassWithSameStyle(&quot;.content-loading&quot;, &quot; , &quot;'&quot; , &quot;ph-wo-display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            	$(&quot;.content-loading&quot;).addClass(&quot;ph-wo-display-none&quot;);
            	fnRemoveClassWithSameStyle(&quot;#specificAttrOptionContent&quot;, &quot; , &quot;'&quot; , &quot;ph-wo-display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
				$(&quot;#specificAttrOptionContent&quot;).removeClass(&quot;ph-wo-display-none&quot;);
				fnRemoveClassWithSameStyle(&quot;.specificAttrOptionContent&quot;, &quot; , &quot;'&quot; , &quot;ph-wo-display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
			  	table.columns.adjust();
            }
        },
        initComplete : function() {
		  	$(&quot; , &quot;'&quot; , &quot;.dataTables_filter&quot; , &quot;'&quot; , &quot;).addClass(&quot;attrOption-search&quot;);
        	var self = this.api();
            var $filter = $(&quot; , &quot;'&quot; , &quot;.attrOption-search&quot; , &quot;'&quot; , &quot;);
            var $input = $filter.find(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).unbind();
            var $searchButton = $(&quot; , &quot;'&quot; , &quot;&lt;span title=&quot;Search&quot; class=&quot;custom-datatable-btn&quot;>&lt;i class=&quot;fa fa-search attrOption&quot;/>&lt;/span>&quot; , &quot;'&quot; , &quot;)
                .click(function(event) {
                	event.preventDefault();
                	if($input.val().trim() != null &amp;&amp; $input.val().trim() != &quot;&quot;)
                	{
                		$(&quot;#loading_attr_options&quot;).removeClass(&quot;ph-wo-display-none&quot;);
                    	self.search($input.val()).draw();
                	}
                });
            var $clearButton = $(&quot; , &quot;'&quot; , &quot;&lt;span title=&quot;Clear&quot; class=&quot;custom-datatable-btn&quot;>&lt;i class=&quot;fa fa-times attrOption&quot;/>&lt;/span>&quot; , &quot;'&quot; , &quot;)
                .click(function(event) {
                	event.preventDefault();
                	$(&quot;#loading_attr_options&quot;).removeClass(&quot;ph-wo-display-none&quot;);
                	self.search(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;).draw();
                });
            if (!$filter.has($searchButton).length) 
            {
            	  $filter.append($searchButton);
            }

            if (!$filter.has($clearButton).length) 
            {
            	  $filter.append($clearButton);
            }
        },
        &quot;pageLength&quot;: pageLength,
        &quot;scrollX&quot;: true,
        &quot;sScrollXInner&quot;: &quot;100%&quot;,
        &quot;scrollCollapse&quot;: true,
        &quot;ordering&quot; : true,
        &quot;lengthMenu&quot;: lengthMenu,
        &quot;language&quot;: {
        	&quot;lengthMenu&quot;: &quot;Records to display _MENU_&quot;,
            &quot;info&quot;: &quot;Total Records: _MAX_   &lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-font-weight-100 ph-wo-margin-left-300px ph-wo-margin-bottom-min-5px&quot; , &quot;'&quot; , &quot;> Displaying _START_ to _END_ &lt;label>&quot;,
            &quot;infoEmpty&quot;: &quot;No records to display&quot;,
            &quot;infoFiltered&quot;: &quot;&lt;label class=&quot; , &quot;'&quot; , &quot;ph-wo-margin-left-2px ph-wo-margin-bottom-0px ph-wo-font-weight-100&quot; , &quot;'&quot; , &quot; >[Filtered from _MAX_ records] &lt;/label>&quot;,
            &quot;search&quot;:         &quot;Search&quot;,
            &quot;sZeroRecords&quot; : &quot;No matching records found&quot;,
            &quot;paginate&quot;: { 
                &quot;next&quot;:       &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-right ph-wo-padding-3px ph-wo-border-left-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
            	&quot;previous&quot;:   &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-angle-left ph-wo-padding-3px ph-wo-border-right-hidden&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;
            },
            renderer: &quot; , &quot;'&quot; , &quot;customPager&quot; , &quot;'&quot; , &quot;,
        },
		columns: columnArr.map((column, index) => {
			column.orderable = index !== 0;
            return column;
        }),
        order: [[1, &quot; , &quot;'&quot; , &quot;asc&quot; , &quot;'&quot; , &quot;]],
        dom: &quot; , &quot;'&quot; , &quot;Blfrtip&quot; , &quot;'&quot; , &quot;,
        buttons: [],
    });
}

$(document).off(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnDataTableExportexpdf&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnDataTableExportexpdf&quot;]&quot; , &quot;'&quot; , &quot;, function() {
    var etype = $(this).data(&quot; , &quot;'&quot; , &quot;etype&quot; , &quot;'&quot; , &quot;);
    fnDataTableExport(etype);
});
 










 
	

    
			













	
		×
		
	



	
	
	
	


	
		
			
				
					
						
							
							
							
								
							
							
						
						
							
								
										
						
					
					
			
			
			
			
		
	



 $( document ).ready(function() {
	fnFadeoutEvent(&quot;#dialogsuccessspan&quot;, 20000, &quot;ph-wo-display-none&quot;);
	fnFadeoutEvent(&quot;#dialogfailurespan&quot;, 20000, &quot;ph-wo-display-none&quot;);
	fnFadeoutEvent(&quot;#dialogdependencyspan&quot;, 20000, &quot;ph-wo-display-none&quot;);
	
	if ($(&quot;#dialogsuccessspan&quot;).is(&quot; , &quot;'&quot; , &quot;:hidden&quot; , &quot;'&quot; , &quot;)) 
	{
		fnRemoveClassWithSameStyle(&quot;#dialogsuccessspan&quot;, &quot;ph-wo-height&quot;, &quot;&quot;, false)
	 	$(&quot;#dialogsuccessspan&quot;).addClass(&quot;ph-wo-height-0px&quot;);
	}
	if ($(&quot;#dialogfailurespan&quot;).is(&quot; , &quot;'&quot; , &quot;:hidden&quot; , &quot;'&quot; , &quot;)) 
	{
		fnRemoveClassWithSameStyle(&quot;#dialogfailurespan&quot;, &quot;ph-wo-height&quot;, &quot;&quot;, false)
	 	$(&quot;#dialogfailurespan&quot;).addClass(&quot;ph-wo-height-0px&quot;);
	}
});

function fnSucessClose()
{
	fnRemoveClassWithSameStyle(&quot;#dialogsuccessspan&quot;, &quot;ph-wo-height&quot;, &quot;&quot;, false)
 	$(&quot;#dialogsuccessspan&quot;).addClass(&quot;ph-wo-height-0px&quot;);
}

function fnFailureClose()
{
	fnRemoveClassWithSameStyle(&quot;#dialogfailurespan&quot;, &quot;ph-wo-height&quot;, &quot;&quot;, false)
 	$(&quot;#dialogfailurespan&quot;).addClass(&quot;ph-wo-height-0px&quot;);
}

function fnShowDependency()
{
	$(&quot; , &quot;'&quot; , &quot;.modal&quot; , &quot;'&quot; , &quot;).draggable({
	    handle: &quot;.modal-header&quot;
	});
	
	var form;
	if (&quot; , &quot;'&quot; , &quot;WorkTypeForm&quot; , &quot;'&quot; , &quot; == &quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;)
	{
		//
	}
	else
	{
		form = document.WorkTypeForm;
	}
	var url = &quot;/phworkoutput/getDataDependency/getDependencyList.htm?deleteRecordId=&quot;+-1;
	windowTitle = fnGetWindowName();
	var winObj =  window.open(&quot;&quot;, windowTitle, &quot;status=1, toolbar=0, scrollbars=1, resizable=1, width=1330, height=600, left=20, top=20&quot;);
	winObj.document.body.appendChild(fnGetDependencyPopUpLoader());
	window.parent.parent.fnAddToChildWindows(winObj);
	tempAction = form.action;
    tempTarget = form.target;
    form.action = url;
    form.target = windowTitle;
    form.submit();
    winObj.focus();
    form.action = tempAction;
    form.target = tempTarget;
}

function fnGetDependencyPopUpLoader()
{
	var loaderDiv = window.parent.document.createElement(&quot; , &quot;'&quot; , &quot;div&quot; , &quot;'&quot; , &quot;);
    loaderDiv.style.textAlign = &quot; , &quot;'&quot; , &quot;center&quot; , &quot;'&quot; , &quot;;
    loaderDiv.style.width = &quot; , &quot;'&quot; , &quot;100%&quot; , &quot;'&quot; , &quot;;
    loaderDiv.style.position = &quot; , &quot;'&quot; , &quot;absolute&quot; , &quot;'&quot; , &quot;;
    loaderDiv.style.top = &quot; , &quot;'&quot; , &quot;48%&quot; , &quot;'&quot; , &quot;;
    loaderDiv.setAttribute(&quot; , &quot;'&quot; , &quot;nonce&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;xZrLNGLeef26xpNnnF1igA==&quot; , &quot;'&quot; , &quot;);
 
    var loaderImg =  window.parent.document.createElement(&quot; , &quot;'&quot; , &quot;img&quot; , &quot;'&quot; , &quot;);
    loaderImg.src = &quot;/phworkoutput/images/loader-trans.gif&quot;;
    loaderImg.setAttribute(&quot; , &quot;'&quot; , &quot;nonce&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;xZrLNGLeef26xpNnnF1igA==&quot; , &quot;'&quot; , &quot;);
    loaderDiv.appendChild(loaderImg);
    return loaderDiv;
}

/* function fnDeleteDependency()
{
	var primaryKeyId = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	var moduleName = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	var menuItemId = &quot; , &quot;'&quot; , &quot;302&quot; , &quot;'&quot; , &quot;;
	var formDefinitionId  = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	
	var actionName = &quot;/phworkoutput/getDataDependency/deleteDataDependency.htm?primaryKeyId=&quot;+primaryKeyId+&quot;&amp;moduleName=&quot;+moduleName+&quot;&amp;menuItemId=&quot;+menuItemId+&quot;&amp;formDefinitionId=&quot;+formDefinitionId;
	doAjaxCall(actionName, false, false);
} */


$(document).off(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;hideMsg&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;hideMsg&quot;]&quot; , &quot;'&quot; , &quot;, function() {
	
	fnRemoveClassWithSameStyle(&quot;.internalmessagefailure&quot;, &quot;ph-wo-display&quot;, &quot;&quot;, false);
	$(&quot;.internalmessagefailure&quot;).addClass(&quot;ph-wo-display-none&quot;);
	fnRemoveClassWithSameStyle(&quot;#displayErrorMessage&quot;, &quot;ph-wo-display&quot;, &quot;&quot;, false);
	$(&quot;#displayErrorMessage&quot;).addClass(&quot;ph-wo-display-none&quot;);
});

$(document).off(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnSucessClose&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnSucessClose&quot;]&quot; , &quot;'&quot; , &quot;, function() {
	fnSucessClose();
});

$(document).off(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnFailureClose&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnFailureClose&quot;]&quot; , &quot;'&quot; , &quot;, function() {
	fnFailureClose();
});

$(document).off(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnShowDependency&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[data-onclick=&quot;fnShowDependency&quot;]&quot; , &quot;'&quot; , &quot;, function() {
	fnShowDependency();
});


	 



	 
        
			
				 
			        
						
						
							
								
									
										ADD WORK TYPES
									
								
							
							
								
									
										
											
												Records to display 102050100200Search
											        
											        Work TypeDescription#User GroupsStatus#dipankar_processWO Scoredcard 2.01Active#Process_028WO Scoredcard 2.01Active#Process_0288 clonedWO Scoredcard 2.01Active#Process_05WO Scoredcard 2.01Active#Process_055WO Scoredcard 2.01Active123zc bz b0Active1_WT0Active2_WTWT10ActiveabWT1231Activeact1230Active
												Total Records: 32    Displaying 1 to 10 1234
											
											
											
									
								
							
						
					
				
			
		
	





$(document).off(&quot;click&quot;, &quot;.fnAddWorkTypes&quot;).on(&quot;click&quot;, &quot;.fnAddWorkTypes&quot;, function(){
	fnAddWorkTypes(this.form);
});

$(document).off(&quot;submit&quot;, &quot;#categoryForm&quot;).on(&quot;submit&quot;, &quot;#categoryForm&quot;, function(){
	return false;
});

form = document.CategoryForm;

var userGroupHashedMap = new Array();

function fnAddWorkTypes(form)
{
	
	mappedWorkTypes = &quot; , &quot;'&quot; , &quot;67,68,69,71,73,138,74,139,140,142,143,144,145,146,147,149,90,94,32,98,99,102,104,105,106,107,108,111,117,119,57,123&quot; , &quot;'&quot; , &quot;;
	$(&quot;#mappedWorkTypes&quot;).val(mappedWorkTypes);
	windowTitle = fnGetWindowName();
	var url = &quot;/phworkoutput/WOCategoryDetails/popupWorkTypeSummary.htm?id=1&amp;menuItemId=302&quot;;
	console.log(url)
	var winObj =  window.open(&quot;&quot;, windowTitle, &quot;status=1, toolbar=0, scrollbars=1, resizable=1, width=1330, height=600, left=20, top=20&quot;);
	winObj.document.body.appendChild(fnGetPopUpLoader(winObj));
	window.parent.parent.fnAddToChildWindows(winObj);
	tempAction = form.action;
    tempTarget = form.target;
    form.action = url;
    form.target = windowTitle;
    form.submit();
    winObj.focus();
    form.action = tempAction;
    form.target = tempTarget;
}

function fnAddWorkTypeMapping(workTypIds)
{
	$(&quot;#workTypeIdStr&quot;).val(workTypIds);
	
	var url = &quot;/phworkoutput/WOCategoryDetails/addWorkTypeMapping.htm?menuItemId=302&quot;;
	$.ajax({
		url: url,
		headers	: {&quot; , &quot;'&quot; , &quot;X-Content-Security-Policy-Nonce&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;xZrLNGLeef26xpNnnF1igA==&quot; , &quot;'&quot; , &quot;},
		global: false,
		type: &quot;POST&quot;,
		data: ({ 			
				&quot;categoryId&quot; 	: $(&quot;#categoryId&quot;).val(),
				&quot;workTypeIdStr&quot;		: workTypIds
 	          }),
	         success: function(resp)
			 {
	        	var json = JSON.parse(resp);
	        	if(json[&quot;message&quot;] == &quot; , &quot;'&quot; , &quot;SUCCESS&quot; , &quot;'&quot; , &quot;)
		        {
	                $(&quot;#categotyWorkTypesMsg&quot;).val(&quot; , &quot;'&quot; , &quot;SUCCESS&quot; , &quot;'&quot; , &quot;);
		        }
		        else
		        {
	                $(&quot;#categotyWorkTypesMsg&quot;).val(&quot; , &quot;'&quot; , &quot;Failed&quot; , &quot;'&quot; , &quot;);
		        }
	        	
	        	 fnShowWorkTypeScreen(true);
			 },
		     error :  function(msg,arg1,arg2)
		     {
		    	 processErrorStatus(msg.status, &quot;/phworkoutput&quot;);
		    	 return false;
			 }
	});
}

if(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; == &quot;fromGroupDelete&quot;)
{
	form.mappedUserGroupIds.value = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
}

$(document).ready(function() 
{
	fnPrepareDataTable();
});

function fnPrepareDataTable()
{
	var Val = &quot; , &quot;'&quot; , &quot;[{\&quot;workTypeName\&quot;:\&quot;Test Cases Written\&quot;,\&quot;worktypeId\&quot;:32,\&quot;userGroupCount\&quot;:6,\&quot;description\&quot;:\&quot;Test Cases\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;WO Bugs Dev Verified\&quot;,\&quot;worktypeId\&quot;:57,\&quot;userGroupCount\&quot;:8,\&quot;description\&quot;:\&quot;Bugs Dev Verified [From Workflow jobs using Database Import]\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;Design\&quot;,\&quot;worktypeId\&quot;:67,\&quot;userGroupCount\&quot;:5,\&quot;description\&quot;:\&quot;WO Scoredcard 2.0\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;Dev. Support\&quot;,\&quot;worktypeId\&quot;:68,\&quot;userGroupCount\&quot;:2,\&quot;description\&quot;:\&quot;WO Scoredcard 2.0\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;Development\&quot;,\&quot;worktypeId\&quot;:69,\&quot;userGroupCount\&quot;:7,\&quot;description\&quot;:\&quot;WO Scoredcard 2.0\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;R&amp;amp;I Developement\&quot;,\&quot;worktypeId\&quot;:71,\&quot;userGroupCount\&quot;:1,\&quot;description\&quot;:\&quot;WO Scoredcard 2.0\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;Testing\&quot;,\&quot;worktypeId\&quot;:73,\&quot;userGroupCount\&quot;:8,\&quot;description\&quot;:\&quot;WO Scoredcard 2.0\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;workflowimport\&quot;,\&quot;worktypeId\&quot;:74,\&quot;userGroupCount\&quot;:15,\&quot;description\&quot;:\&quot;WO Scoredcard 2.0\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;ukS)g\&quot;,\&quot;worktypeId\&quot;:90,\&quot;userGroupCount\&quot;:0,\&quot;description\&quot;:\&quot;\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;x0-tN\&quot;,\&quot;worktypeId\&quot;:94,\&quot;userGroupCount\&quot;:0,\&quot;description\&quot;:\&quot;\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;jO_)d\&quot;,\&quot;worktypeId\&quot;:98,\&quot;userGroupCount\&quot;:0,\&quot;description\&quot;:\&quot;\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;#dipankar_process\&quot;,\&quot;worktypeId\&quot;:99,\&quot;userGroupCount\&quot;:1,\&quot;description\&quot;:\&quot;WO Scoredcard 2.0\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;#Process_05\&quot;,\&quot;worktypeId\&quot;:102,\&quot;userGroupCount\&quot;:1,\&quot;description\&quot;:\&quot;WO Scoredcard 2.0\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;Bugs Resolved\&quot;,\&quot;worktypeId\&quot;:104,\&quot;userGroupCount\&quot;:6,\&quot;description\&quot;:\&quot;WO Scoredcard 2.0\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;#Process_028\&quot;,\&quot;worktypeId\&quot;:105,\&quot;userGroupCount\&quot;:1,\&quot;description\&quot;:\&quot;WO Scoredcard 2.0\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;#Process_0288 cloned\&quot;,\&quot;worktypeId\&quot;:106,\&quot;userGroupCount\&quot;:1,\&quot;description\&quot;:\&quot;WO Scoredcard 2.0\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;Test_Claim Provider insurance\&quot;,\&quot;worktypeId\&quot;:107,\&quot;userGroupCount\&quot;:1,\&quot;description\&quot;:\&quot;WO Scoredcard 2.0\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;rkq3x\&quot;,\&quot;worktypeId\&quot;:108,\&quot;userGroupCount\&quot;:0,\&quot;description\&quot;:\&quot;\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;Fy5_n\&quot;,\&quot;worktypeId\&quot;:111,\&quot;userGroupCount\&quot;:0,\&quot;description\&quot;:\&quot;\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;zc bz b\&quot;,\&quot;worktypeId\&quot;:117,\&quot;userGroupCount\&quot;:1,\&quot;description\&quot;:\&quot;\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;New WT01\&quot;,\&quot;worktypeId\&quot;:119,\&quot;userGroupCount\&quot;:1,\&quot;description\&quot;:\&quot;New WT01\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;zc bz b123\&quot;,\&quot;worktypeId\&quot;:123,\&quot;userGroupCount\&quot;:0,\&quot;description\&quot;:\&quot;\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;Bugs Dev Fixed\&quot;,\&quot;worktypeId\&quot;:138,\&quot;userGroupCount\&quot;:0,\&quot;description\&quot;:\&quot;\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;soumya\&quot;,\&quot;worktypeId\&quot;:139,\&quot;userGroupCount\&quot;:1,\&quot;description\&quot;:\&quot;\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;scc,gvgbmj\&quot;,\&quot;worktypeId\&quot;:140,\&quot;userGroupCount\&quot;:2,\&quot;description\&quot;:\&quot;WO Scoredcard 2.0\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;WT1\&quot;,\&quot;worktypeId\&quot;:142,\&quot;userGroupCount\&quot;:0,\&quot;description\&quot;:\&quot;WT1\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;abWT123\&quot;,\&quot;worktypeId\&quot;:143,\&quot;userGroupCount\&quot;:1,\&quot;description\&quot;:\&quot;\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;act123\&quot;,\&quot;worktypeId\&quot;:144,\&quot;userGroupCount\&quot;:0,\&quot;description\&quot;:\&quot;\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;123zc bz b\&quot;,\&quot;worktypeId\&quot;:145,\&quot;userGroupCount\&quot;:0,\&quot;description\&quot;:\&quot;\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;1_WT\&quot;,\&quot;worktypeId\&quot;:146,\&quot;userGroupCount\&quot;:0,\&quot;description\&quot;:\&quot;\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;2_WT\&quot;,\&quot;worktypeId\&quot;:147,\&quot;userGroupCount\&quot;:0,\&quot;description\&quot;:\&quot;WT1\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true},{\&quot;workTypeName\&quot;:\&quot;#Process_055\&quot;,\&quot;worktypeId\&quot;:149,\&quot;userGroupCount\&quot;:1,\&quot;description\&quot;:\&quot;WO Scoredcard 2.0\&quot;,\&quot;id\&quot;:0,\&quot;category\&quot;:\&quot;\&quot;,\&quot;categoryId\&quot;:1,\&quot;status\&quot;:true}]&quot; , &quot;'&quot; , &quot;;
	var json = JSON.parse(Val);
	var columnArr = [];
	
	columnArr.push(
		{
			&quot;title&quot;		: &quot;Work Type&quot;, 
		 	&quot;data&quot;		: &quot;workTypeName&quot;, 
		 	&quot;width&quot; 	: &quot;45%&quot;,
		 	&quot;className&quot;	: &quot;text&quot;,
		 	&quot;render&quot;	: function (workTypeName, type, full, meta) 
		 	{
		 		var actions = &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;ph-wo-word-break-break-all&quot;>&quot; , &quot;'&quot; , &quot;+workTypeName+&quot; , &quot;'&quot; , &quot;&lt;/div>&quot; , &quot;'&quot; , &quot;;
				return actions;
			}
	 	});
	
	columnArr.push(
		{
			&quot;title&quot;		: &quot;Description&quot;, 
		 	&quot;data&quot;		: &quot;description&quot;, 
		 	&quot;width&quot; 	: &quot;40%&quot;,
		 	&quot;className&quot;	: &quot;text&quot;,
		 	&quot;render&quot;	: function (description, type, full, meta) 
		 	{
		 		var actions = &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;ph-wo-word-break-break-all&quot;>&quot; , &quot;'&quot; , &quot;+description+&quot; , &quot;'&quot; , &quot;&lt;/div>&quot; , &quot;'&quot; , &quot;;
				return actions;
			}
	 	});
	
	columnArr.push({&quot;title&quot;: &quot;#User Groups&quot;, &quot;data&quot;: &quot;userGroupCount&quot;, &quot;width&quot; : &quot;10%&quot;, &quot;class&quot;: &quot;numeric&quot;});
	
	columnArr.push({ 
   	 &quot;title&quot;	 : &quot;Status&quot;, 
     	 &quot;data&quot;		 : &quot;status&quot;,
     	 &quot;searchable&quot;: false,
     	 &quot;sortable&quot;	 : true,
     	&quot;width&quot; 	 : &quot;5%&quot;,
     	 &quot;className&quot; : &quot;text-center&quot;,
     	 &quot;render&quot;	 : function (json, type, full, meta) 
     	 {
          	var actions = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
          	if(full.status == true)
      		{
      			actions = &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-circle circle-active&quot; title=&quot;Active&quot;>&lt;span class=&quot;ph-wo-display-none&quot;>Active&lt;/span>&lt;/i>&quot; , &quot;'&quot; , &quot;;
      		}
      		else if(full.status == false)
      		{
      			actions = &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-circle circle-inactive&quot; title=&quot;Inactive&quot;>&lt;span class=&quot;ph-wo-display-none&quot;>Inactive&lt;/span>&lt;/i>&quot; , &quot;'&quot; , &quot;;
      		}
			return actions;
 		}
    });
	
	
	var pageSize = 10;
	var sortingIndex = 0;
	
	if(json != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
	{
		buildScrollXDataTableStructure(json, pageSize, columnArr, sortingIndex, false);
	}
	var table = $(&quot; , &quot;'&quot; , &quot;#CommonDataTableId&quot; , &quot;'&quot; , &quot;).DataTable();
	table.columns.adjust().draw();
}




								
							
						
					
				
				
					
						
							
								PREVIOUS
							
						
						
							CANCEL
						
						
							
								NEXT
							
						
						
							
								EXIT
							
						
					
				
			
		
	


	


$(document).off(&quot;click&quot;, &quot;.fnShowBasicScreen1&quot;).on(&quot;click&quot;, &quot;.fnShowBasicScreen1&quot;, function(){
	fnShowBasicScreen1();
});

$(document).off(&quot;click&quot;, &quot;.fnShowWorkTypeScreen&quot;).on(&quot;click&quot;, &quot;.fnShowWorkTypeScreen&quot;, function(){
	var flag = $(this).data(&quot;flag&quot;);
	fnShowWorkTypeScreen(flag);
});

$(document).off(&quot;click&quot;, &quot;.fnShowPreviousScreen&quot;).on(&quot;click&quot;, &quot;.fnShowPreviousScreen&quot;, function(){
	fnShowPreviousScreen();
});

$(document).off(&quot;click&quot;, &quot;.fnShowNextScreen&quot;).on(&quot;click&quot;, &quot;.fnShowNextScreen&quot;, function(){
	fnShowNextScreen();
});

$(document).off(&quot;click&quot;, &quot;.fnBackToCategorySummary&quot;).on(&quot;click&quot;, &quot;.fnBackToCategorySummary&quot;, function(){
	fnBackToCategorySummary();
});

form = document.CategoryForm;

previousScreenName = &quot;&quot;;
currentScreenName = &quot;&quot;;
nextScreenName =&quot;&quot;;


function fnShowPreviousScreen()
{
	fnShowScreen(previousScreenName)
}

function fnShowNextScreen()
{
	fnShowScreen(nextScreenName)
}

function fnShowScreen(screenName)
{
	if(screenName == &quot;BASIC_SCREEN&quot;)
	{
		fnShowBasicScreen1();
	}
	else if (screenName == &quot;WORKTYPEMAPPING_SCREEN&quot;)
	{
		fnShowWorkTypeScreen(false);
	}
	else
	{
		alert(&quot; , &quot;'&quot; , &quot;Unknown screen&quot; , &quot;'&quot; , &quot;);
	}
}
function fnHighLightMenu(screenName)
{
	$(&quot;#parameterGroupTD&quot;).addClass(&quot; , &quot;'&quot; , &quot;wizardtdnohighlight&quot; , &quot;'&quot; , &quot;);
	$(&quot;#basicsTD&quot;).addClass(&quot;wizardtdnohighlight&quot;);
	$(&quot;#heatCodeTD&quot;).addClass(&quot;wizardtdnohighlight&quot;);
	
	if(screenName == &quot;BASIC_SCREEN&quot;)
	{
		fnRemoveClassWithSameStyle(&quot;#ExitButton&quot;,&quot;ph-wo-display&quot;,&quot;&quot;, false);
		$(&quot;#ExitButton&quot;).addClass(&quot;ph-wo-display-none&quot;);
		previousScreenName = &quot;&quot;;
		nextScreenName =&quot;WORKTYPEMAPPING_SCREEN&quot;;
		fnRemoveClassWithSameStyle(&quot;#previousButton&quot;,&quot;ph-wo-display&quot;,&quot;&quot;, false);
		$(&quot;#previousButton&quot;).addClass(&quot;ph-wo-display-none&quot;);
		fnRemoveClassWithSameStyle(&quot;#ExitButton&quot;,&quot;ph-wo-display&quot;,&quot;&quot;, false);
		$(&quot;#ExitButton&quot;).addClass(&quot;ph-wo-display-none&quot;);
		fnRemoveClassWithSameStyle(&quot;#nextButton&quot;,&quot;ph-wo-display&quot;,&quot;&quot;, false);
		$(&quot;#nextButton&quot;).addClass(&quot;ph-wo-display-empty&quot;);
		
		$(&quot;#basicsTD&quot;).removeClass();
		$(&quot;#basicsTD&quot;).addClass(&quot;fnShowBasicScreen1&quot;);
		$(&quot;#parameterGroupTD&quot;).removeClass(&quot; , &quot;'&quot; , &quot;wizardtdhighlight&quot; , &quot;'&quot; , &quot;);
		$(&quot;#heatCodeTD&quot;).removeClass(&quot; , &quot;'&quot; , &quot;wizardtdhighlight&quot; , &quot;'&quot; , &quot;);
		$(&quot;#basicsTD&quot;).addClass(&quot; , &quot;'&quot; , &quot;wizardtdhighlight&quot; , &quot;'&quot; , &quot;);
	}
	else if (screenName == &quot;WORKTYPEMAPPING_SCREEN&quot;)
	{
		fnRemoveClassWithSameStyle(&quot;#ExitButton&quot;,&quot;ph-wo-display&quot;,&quot;&quot;, false);
		$(&quot;#ExitButton&quot;).addClass(&quot;ph-wo-display-empty&quot;);
		previousScreenName = &quot;BASIC_SCREEN&quot;;
		nextScreenName =&quot;&quot;;
		fnRemoveClassWithSameStyle(&quot;#ExitButton&quot;,&quot;ph-wo-display&quot;,&quot;&quot;, false);
		$(&quot;#ExitButton&quot;).addClass(&quot;ph-wo-display-empty&quot;);
		fnRemoveClassWithSameStyle(&quot;#previousButton&quot;,&quot;ph-wo-display&quot;,&quot;&quot;, false);
		$(&quot;#previousButton&quot;).addClass(&quot;ph-wo-display-empty&quot;);
		fnRemoveClassWithSameStyle(&quot;#nextButton&quot;,&quot;ph-wo-display&quot;,&quot;&quot;, false);
		$(&quot;#nextButton&quot;).addClass(&quot;ph-wo-display-none&quot;);
		
		$(&quot;#basicsTD&quot;).removeClass(&quot; , &quot;'&quot; , &quot;wizardtdhighlight&quot; , &quot;'&quot; , &quot;);
		$(&quot;#heatCodeTD&quot;).removeClass(&quot; , &quot;'&quot; , &quot;wizardtdhighlight&quot; , &quot;'&quot; , &quot;);
		$(&quot;#parameterGroupTD&quot;).removeClass();
		$(&quot;#parameterGroupTD&quot;).addClass(&quot;fnShowWorkTypeScreen&quot;);
		$(&quot;#parameterGroupTD&quot;).addClass(&quot; , &quot;'&quot; , &quot;wizardtdhighlight&quot; , &quot;'&quot; , &quot;);
		$(&quot;#right_table&quot;).height(185);
	}
	else
	{
		alert(&quot; , &quot;'&quot; , &quot;Unknown screen&quot; , &quot;'&quot; , &quot;);
	}
	fnRemoveClassWithSameStyle(&quot;#contentTD&quot;,&quot;ph-wo-display&quot;,&quot;&quot;, false);
	$(&quot;#contentTD&quot;).addClass(&quot;ph-wo-display-empty&quot;);
}


function fnRemoveClass()
{
	$(&quot;#basicsTD&quot;).removeClass();
	$(&quot;#basicsTD&quot;).addClass(&quot;fnShowBasicScreen1&quot;);
	$(&quot;#parameterGroupTD&quot;).removeClass();
}


function fnShowBasicScreen1()
{
	fnHighLightMenu(&quot; , &quot;'&quot; , &quot;BASIC_SCREEN&quot; , &quot;'&quot; , &quot;);
	actionType = &quot;add&quot;;
	if($(&quot;#categoryId&quot;).val() > 0)
	{
		showIFrameLoading();
		actionType = &quot;modify&quot;;
		form.action = &quot;/phworkoutput/WOCategoryDetails/modify.htm?menuItemId=302&amp;subActionType=&quot;+actionType+&quot;&amp;categoryId=&quot;+$(&quot;#categoryId&quot;).val();
		form.submit();
	}
}

function fnShowWorkTypeScreen(isAddedWorkTypes)
{
	fnHighLightMenu(&quot; , &quot;'&quot; , &quot;WORKTYPEMAPPING_SCREEN&quot; , &quot;'&quot; , &quot;);
	var url = &quot;/phworkoutput/WOCategoryDetails/workTypeMappingScreen.htm?menuItemId=302&amp;categoryId=&quot;+$(&quot;#categoryId&quot;).val();
	doAjaxCall(url, null, $(&quot;#mappedParameterIds&quot;).val(), isAddedWorkTypes);
}

function fnAddWorkTypeMapping(str)
{
	//Empty fn required for adding workType Ids in second wizard screen
}


function doAjaxCall(url, selectedIds, mappedParameterIds, isAddedWorkTypes)
{
	$(&quot;#contentTD&quot;).html(fnGetAjaxLoader());
	$.ajax({
		url: url,
		headers	: {&quot; , &quot;'&quot; , &quot;X-Content-Security-Policy-Nonce&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;xZrLNGLeef26xpNnnF1igA==&quot; , &quot;'&quot; , &quot;},
		global: false,
		type: &quot;POST&quot;,
		data: ({ 			
				&quot;menuItemId&quot;	: &quot; , &quot;'&quot; , &quot;302&quot; , &quot;'&quot; , &quot;,
				&quot;isAjaxCall&quot; 	: true,
				&quot;mappedUserGroupIds&quot;: selectedIds,
				&quot;mappedParameterIds&quot;: mappedParameterIds
 	          }),
	         success: function(resp)
			 {

	        	$(&quot;#contentTD&quot;).html(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		        $(&quot;#contentTD&quot;).html(resp);
		        fnRemoveClassWithSameStyle(&quot;#pagingTD2&quot;,&quot;ph-wo-display&quot;,&quot;&quot;, false);
		        $(&quot;#pagingTD2&quot;).addClass(&quot;ph-wo-display-none&quot;);
		        fnSetEmptyTdHeight($(&quot;#right_table&quot;).height(), false);
		        if(isAddedWorkTypes)
		        {
		        	var successMsg = &quot;Work Types added successfully.&quot;;
					var failureMsg = &quot;Work Types can not be added, due to Error.&quot;;
				    
		        	if($(&quot;#categotyWorkTypesMsg&quot;).val() == &quot; , &quot;'&quot; , &quot;SUCCESS&quot; , &quot;'&quot; , &quot;)
			        {
		                var html = &quot; , &quot;'&quot; , &quot;&lt;span class=&quot;alert alert-success config-alert ph-wo-width-50percentage ph-wo-margin-left-min-150px&quot; id=&quot;successspanSubHead&quot;>&lt;a href=&quot;&quot; class=&quot;close&quot; data-dismiss=&quot;alert&quot; aria-label=&quot;close&quot;>&amp;times;&lt;/a>&quot; , &quot;'&quot; , &quot;;
		                        html += &quot; , &quot;'&quot; , &quot;&lt;span id=&quot;successdivSubHead&quot;>&lt;/span>&lt;/span>&quot; , &quot;'&quot; , &quot;;
		                $(&quot;#successspanSubHead&quot;).html(html);
		                $(&quot;#successdivSubHead&quot;).html(successMsg);
		                fnRemoveClassWithSameStyle(&quot;#successspanSubHead&quot;,&quot;ph-wo-display&quot;,&quot;&quot;, false);
		                $(&quot;#successspanSubHead&quot;).addClass(&quot;ph-wo-display-empty&quot;);
		               setTimeout(function() { $(&quot; , &quot;'&quot; , &quot;#successspanSubHead&quot; , &quot;'&quot; , &quot;).html(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;); }, 10000); 
		                $(&quot;#categotyWorkTypesMsg&quot;).val(&quot; , &quot;'&quot; , &quot;SUCCESS&quot; , &quot;'&quot; , &quot;);
			        }
			        else
			        {
		                var html = &quot; , &quot;'&quot; , &quot;&lt;span class=&quot;alert alert-danger config-alert ph-wo-width-50percentage ph-wo-margin-left-min-150px&quot; id=&quot;failurespanSubHead&quot;>&lt;a href=&quot;&quot; class=&quot;close&quot; data-dismiss=&quot;alert&quot; aria-label=&quot;close&quot;>&amp;times;&lt;/a>&quot; , &quot;'&quot; , &quot;;
		                        html +=  &quot; , &quot;'&quot; , &quot;&lt;span id=&quot;failuredivSubHead&quot; >&lt;/span>&lt;/span>&quot; , &quot;'&quot; , &quot;;
		                $(&quot;#failurespanSubHead&quot;).html(html);
		                  $(&quot;#failuredivSubHead&quot;).html(failureMsg);
		                fnRemoveClassWithSameStyle(&quot;#failurespanSubHead&quot;,&quot;ph-wo-display&quot;,&quot;&quot;, false);
		                $(&quot;#failurespanSubHead&quot;).addClass(&quot;ph-wo-display-empty&quot;);
		               setTimeout(function() { $(&quot; , &quot;'&quot; , &quot;#failurespanSubHead&quot; , &quot;'&quot; , &quot;).html(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;); }, 10000); 
		                $(&quot;#categotyWorkTypesMsg&quot;).val(&quot; , &quot;'&quot; , &quot;Failed&quot; , &quot;'&quot; , &quot;);
			        }
		        }
		        if($.browser.chrome){
		        	fnRemoveClassWithSameStyle(&quot;#right_table&quot;,&quot;ph-wo-border-bottom&quot;,&quot;&quot;, false);
		    		$(&quot;#right_table&quot;).addClass(&quot;ph-wo-border-bottom-none&quot;);
		        }

			 },
		     error :  function(msg,arg1,arg2)
		     {
		    	 processErrorStatus(msg.status, &quot;/phworkoutput&quot;);
		    	 return false;
			 }
	});
}

fnRemoveClassWithSameStyle(&quot;#adminBodyContent&quot;,&quot;ph-wo-margin-top&quot;,&quot;&quot;, false);
$(&quot;#adminBodyContent&quot;).addClass(&quot; , &quot;'&quot; , &quot;ph-wo-margin-top-min-17px&quot; , &quot;'&quot; , &quot;);

$(document).ready(function() 
{
	fnHighLightMenu(&quot;BASIC_SCREEN&quot;);
	if(&quot; , &quot;'&quot; , &quot;modify&quot; , &quot;'&quot; , &quot; == &quot; , &quot;'&quot; , &quot;modify&quot; , &quot;'&quot; , &quot; || $(&quot;#workTypeId&quot;).val() > 0)
	{
		fnRemoveClassWithSameStyle(&quot;#naviButtons&quot;,&quot;ph-wo-display&quot;,&quot;&quot;, false);
		$(&quot;#naviButtons&quot;).addClass(&quot;ph-wo-display-empty&quot;);
	}
	else
	{
		fnRemoveClassWithSameStyle(&quot;#naviButtons&quot;,&quot;ph-wo-display&quot;,&quot;&quot;, false);
		$(&quot;#naviButtons&quot;).addClass(&quot;ph-wo-display-none&quot;);
	}
	fnSetEmptyTdHeight($(&quot;#right_table&quot;).outerHeight(), true);
	if(&quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot; == &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;)
	{
		fnShowWorkTypeScreen(false);
	}
});

function fnSetEmptyTdHeight(rightTabHeight, onload)
{
	var subactionType = &quot; , &quot;'&quot; , &quot;modify&quot; , &quot;'&quot; , &quot;;
	var trHeight = ($(&quot;#tab_table&quot;).find(&quot;tr:visible&quot;).length - 1) * 22;
	var borderHeight = ($(&quot;#tab_table&quot;).outerHeight()) - (trHeight + $(&quot;#empty_td&quot;).height());
	if(borderHeight &lt; 0)
    	borderHeight = 0;
	var objAgent = navigator.userAgent;
	var objbrowserName  = navigator.appName;
	var objfullVersion  = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;+parseFloat(navigator.appVersion); 
	if ((objOffsetVersion=objAgent.indexOf(&quot;Firefox&quot;))!=-1) 
	{
		 objbrowserName = &quot;Firefox&quot;;
	 }
	 if (objbrowserName == &quot; , &quot;'&quot; , &quot;Firefox&quot; , &quot;'&quot; , &quot;) 
	 {
		 if(subactionType == &quot;modify&quot;)
		{
			 remHt = rightTabHeight  - (trHeight+50);
		}
		 else if(subactionType == &quot;add&quot;)
		 {
			 remHt = rightTabHeight  - (trHeight+24);
		 }
		
	 }
	 else
	 {
		 remHt = rightTabHeight - (trHeight + borderHeight);
	 }  
	if((remHt + trHeight + borderHeight) == rightTabHeight)
		remHt = remHt - 1;
	remHt = getEmptyTdHeightInSafari(remHt);
	$(&quot;#empty_td&quot;).height(remHt);
}



	



	.disclaimerDiv
	{
		padding: 10px 50px 30px 50px;
		font-size: 10px;
	}



	
	The information made available through this web portal is intended solely for authorized users and for general informational purposes. While we strive to ensure that the data and reports are accurate and up to date, we make no warranties or representations-express or implied-regarding the completeness, accuracy, reliability, or fitness for a particular purpose of the information presented. Unauthorized access, use, distribution, copying, or modification of any content or data from this platform is strictly prohibited and may be unlawful. If you are not an intended or authorized user, please exit immediately and notify the administrator. We take precautions to safeguard this platform against viruses and malicious code; however, users are responsible for performing their own virus scans before downloading any files. We do not accept any liability for loss or damage arising from the use of this platform or the information accessed through it. By continuing to use this system, you agree to these terms and acknowledge your responsibility for any actions taken based on the information provided herein.



$(document).ready(function () {
    placeDisclaimer();
});

$(window).resize(placeDisclaimer);

function placeDisclaimer() 
{
    var contentHeight = $(&quot;.table-responsive&quot;).outerHeight(true);
    var windowHeight = $(window).height();
    var disclaimerHeight = $(&quot;.disclaimerDiv&quot;).outerHeight(true);

    fnRemoveClassWithSameStyle(&quot;#contentFrame&quot;, &quot;ph-wo-min-height&quot;, &quot;&quot;, false);
	
    if (contentHeight + disclaimerHeight &lt; windowHeight) 
    {
        var dynamicMargin = windowHeight - (contentHeight + disclaimerHeight) - 80;
		var className = (&quot;ph-wo-margin-top-&quot; +dynamicMargin + &quot;px&quot;).replace(&quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;);
		addDynamicCSS(&quot;xZrLNGLeef26xpNnnF1igA==&quot;, className , { &quot;margin-top&quot;:  dynamicMargin +&quot; , &quot;'&quot; , &quot;px !important&quot; , &quot;'&quot; , &quot;});
        $(&quot;.disclaimerDiv&quot;).addClass(className);
    } 
}



	
		
			
				
					
						Help
						
							
								
							
						
					
					
			
      		
		
	

		

    



	
		
			
				×
				Warning
      		
      		
      			 
      			
      		
      		
        		OK
      		
		
	



	
		
			
				×
				Confirmation
      		
      		
      			 
      			
      		
      		
      			NO
        		YES
      		
		
	









function fnHelp()
{
	
	 
		alert(&quot;Help cannot be shown as it is not been configured.&quot;);
	 
}
  

document.onclick = window.parent.fnHideSideBar;
gloabalDeviceWidth = window.parent.gloabalDeviceWidth;

function adjustDataTableWidth()
{
	if(gloabalDeviceWidth &lt;= 1024)
	{
		var style=document.createElement(&quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;);
		style.type=&quot;text/css&quot;;
		style.setAttribute(&quot; , &quot;'&quot; , &quot;nonce&quot; , &quot;'&quot; , &quot;, &quot;xZrLNGLeef26xpNnnF1igA==&quot;);
		var css=&quot; , &quot;'&quot; , &quot;.admin-responsivetable{max-width:&quot; , &quot;'&quot; , &quot;+gloabalDeviceWidth - 75+&quot; , &quot;'&quot; , &quot;px !important};&quot; , &quot;'&quot; , &quot;
		style.appendChild(document.createTextNode(css));
		document.head.appendChild(style);
	}
}

function adjustContentOverflow(elementId)
{
	if(gloabalDeviceWidth &lt;= 1024)
	{
		var style=document.createElement(&quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;);
		style.type=&quot;text/css&quot;;
		style.setAttribute(&quot; , &quot;'&quot; , &quot;nonce&quot; , &quot;'&quot; , &quot;, &quot;xZrLNGLeef26xpNnnF1igA==&quot;);
		var css=&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot;+elementId+&quot; , &quot;'&quot; , &quot;{max-width:&quot; , &quot;'&quot; , &quot;+gloabalDeviceWidth - 75+&quot; , &quot;'&quot; , &quot;!important};&quot; , &quot;'&quot; , &quot;
		style.appendChild(document.createTextNode(css));
		document.head.appendChild(style);
	}
}

$(document).ready(function() {
	window.parent.hideWindowScroll();
	window.parent.hideBottomScroll();
	window.parent.fnSetHelpPath(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
	hideIFrameLoading();
	
	// forcing slimscroll without mouse movement
	$(&quot; , &quot;'&quot; , &quot;#adminBodyContent&quot; , &quot;'&quot; , &quot;).mouseover();
	$(&quot; , &quot;'&quot; , &quot;#adminBodyContent&quot; , &quot;'&quot; , &quot;).focus(); 

	setAlertHideTimer();
	fnRemoveClassWithSameStyle(&quot;.disclaimerDiv&quot;, &quot;ph-wo-display&quot;, &quot;&quot;, false);
	if(isIE8 != -1)
	{
		$(&quot;.input-right-content-textarea&quot;).addClass(&quot;input-right-content-textarea_ie&quot;);
	}
});
	
$(function() {
    var e = 300,
        a = 500;
    navigator.userAgent.match(/iPhone|iPad|iPod/i) ? $(&quot;#adminBodyContent&quot;).bind(&quot;touchend touchcancel touchleave&quot;, function(t) {
        $(this).scrollTop() > e ? $(&quot;#scroll-to-top-admin&quot;).fadeIn(a) : $(&quot;#scroll-to-top-admin&quot;).fadeOut(a)
    }) : $(&quot;#adminBodyContent&quot;).scroll(function() {
    	checkAndToggleBottomScroll();
        $(this).scrollTop() > e ? $(&quot;#scroll-to-top-admin&quot;).fadeIn(a) : $(&quot;#scroll-to-top-admin&quot;).fadeOut(a)
    }), $(&quot;#scroll-to-top-admin&quot;).click(function(e) {
    	$(&quot; , &quot;'&quot; , &quot;#adminBodyContent&quot; , &quot;'&quot; , &quot;).focus(); 
        return e.preventDefault(), $(&quot;#adminBodyContent&quot;).animate({
            scrollTop: 0
        }, a), !1
    })
}); 

function isSessionExpired(response)
{
	return window.parent.isSessionExpired(response);
}

var alertHideInterval;
function setAlertHideTimer()
{
	alertHideInterval = window.setTimeout(function() {
		hideAlertMessage();
	}, 20000);
	
	$(&quot;.config-alert > a&quot;).click(function() {
		hideAlertMessage();
		return false;
	});
}

function hideAlertMessage() {
	$(&quot;.config-alert&quot;).fadeTo(500, 0).slideUp(500, function(){
        $(this).remove();
    });
	
	clearTimeout(alertHideInterval);
}

function fnShowDialogWithNote(label, key, note)
{
	var url = &quot;/phworkoutput/HelpFramework.htm?subActionType=helpDialog&quot;;

	$.ajax({
			url: url,
			global: false,
			headers	: {&quot; , &quot;'&quot; , &quot;X-Content-Security-Policy-Nonce&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;xZrLNGLeef26xpNnnF1igA==&quot; , &quot;'&quot; , &quot;},
			type: &quot;POST&quot;,
			data: ({
					&quot;menuItemId&quot;: form.menuItemId.value,
					&quot;label&quot; : replaceBackJSParamXMLEntities(label),
					&quot;key&quot; : key,
					&quot;note&quot; : note
				}),
			success: function(resp)
			{
				if(!isSessionExpired(resp))
				{
					$(&quot;#showData&quot;).html(resp);
					fnRemoveClassWithSameStyle(&quot;#helpDialog&quot;, &quot;ph-wo-display&quot;, &quot;&quot;, false);
					$(&quot;#helpDialog&quot;).modal(&quot;show&quot;);
		    	}	
			},
			error :  function(msg,arg1,arg2)
			{
				processErrorStatus(msg.status, &quot;/phworkoutput&quot;);
				return false;
			}
	});
}

function fnShowDialog(label, key)
{
	var url = &quot;/phworkoutput/HelpFramework.htm?subActionType=helpDialog&quot;;

	$.ajax({
			url: url,
			global: false,
			headers	: {&quot; , &quot;'&quot; , &quot;X-Content-Security-Policy-Nonce&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;xZrLNGLeef26xpNnnF1igA==&quot; , &quot;'&quot; , &quot;},
			type: &quot;POST&quot;,
			data: ({
					&quot;menuItemId&quot;: &quot;302&quot;,
					&quot;label&quot; : replaceBackJSParamXMLEntities(label),
					&quot;key&quot; : key
				}),
			success: function(resp)
			{
				if(!isSessionExpired(resp))
				{
					$(&quot;#showData&quot;).html(resp);
					fnRemoveClassWithSameStyle(&quot;#helpDialog&quot;, &quot;ph-wo-display&quot;, &quot;&quot;, false);
					$(&quot;#helpDialog&quot;).modal(&quot;show&quot;);
		    	}	
			},
			error :  function(msg,arg1,arg2)
			{
				processErrorStatus(msg.status, &quot;/phworkoutput&quot;);
			    return false;
			}
	});
}

function fnShowPopup(xmlHttp, label, key)
{
	if (xmlHttp.readyState == 4)
	{
		
	}
}

$(document).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;.dropdown-menu&quot; , &quot;'&quot; , &quot;, function(e) {
	e.stopPropagation();
});

function showIFrameLoading()
{
	window.parent.showLoadingModel();
}

function hideIFrameLoading()
{
	window.parent.hideLoadingModel();
}

$(&quot;.adminform&quot;).submit(function(e) {
	if(!isAlertModalVisible)
	{
		showIFrameLoading();
	}
});

function doSubmit(form) {
	showIFrameLoading();
	form.submit();
}

function fnGetAjaxLoader()
{
	divBuffer = &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;ph-wo-text-align-center ph-wo-width-100percentage&quot;>&quot; , &quot;'&quot; , &quot;;
	divBuffer += &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/ajax-loader.gif&quot;>&quot; , &quot;'&quot; , &quot;;
	divBuffer +=&quot; , &quot;'&quot; , &quot;&lt;/div>&quot; , &quot;'&quot; , &quot;;
	  
	return divBuffer;
}

function fnGetPopUpLoader()
{
	divBuffer = &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;ph-wo-text-align-center ph-wo-width-100percentage ph-wo-position-absolute ph-wo-top-48px&quot;>&quot; , &quot;'&quot; , &quot;;
	divBuffer += &quot; , &quot;'&quot; , &quot;&lt;img src=&quot;/phworkoutput/images/loader-trans.gif&quot;>&quot; , &quot;'&quot; , &quot;;
	divBuffer +=&quot; , &quot;'&quot; , &quot;&lt;/div>&quot; , &quot;'&quot; , &quot;;
	  
	return divBuffer;
} 

function fnGetPopUpLoader(winObj)
{
	var loaderDiv = winObj.document.createElement(&quot; , &quot;'&quot; , &quot;div&quot; , &quot;'&quot; , &quot;);
    loaderDiv.style.textAlign = &quot; , &quot;'&quot; , &quot;center&quot; , &quot;'&quot; , &quot;;
    loaderDiv.style.width = &quot; , &quot;'&quot; , &quot;100%&quot; , &quot;'&quot; , &quot;;
    loaderDiv.style.position = &quot; , &quot;'&quot; , &quot;absolute&quot; , &quot;'&quot; , &quot;;
    loaderDiv.style.top = &quot; , &quot;'&quot; , &quot;48%&quot; , &quot;'&quot; , &quot;;
    loaderDiv.setAttribute(&quot; , &quot;'&quot; , &quot;nonce&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;xZrLNGLeef26xpNnnF1igA==&quot; , &quot;'&quot; , &quot;);
 
    var loaderImg = winObj.document.createElement(&quot; , &quot;'&quot; , &quot;img&quot; , &quot;'&quot; , &quot;);
    loaderImg.src = &quot;/phworkoutput/images/loader-trans.gif&quot;;
    loaderImg.setAttribute(&quot; , &quot;'&quot; , &quot;nonce&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;xZrLNGLeef26xpNnnF1igA==&quot; , &quot;'&quot; , &quot;);
    loaderDiv.appendChild(loaderImg);
    return loaderDiv;
}

function checkAndToggleBottomScroll()
{
	if($(&quot;.pagebanner&quot;).is(&quot;:visible&quot;))
    {
		var scrollPos = $(&quot;#adminBodyContent&quot;).scrollTop() + $(window).height();
		var remHeight = ($(&quot; , &quot;'&quot; , &quot;.table-responsive&quot; , &quot;'&quot; , &quot;).position().top + $(&quot; , &quot;'&quot; , &quot;#adminBodyContent&quot; , &quot;'&quot; , &quot;).scrollTop()) + $(&quot; , &quot;'&quot; , &quot;.table-responsive&quot; , &quot;'&quot; , &quot;).outerHeight(true);
    	if(scrollPos >= remHeight)
      	{
        	hideBottomScroll();
      	}
        else
      	{
    		showBottomScroll();
      	} 
    }
	else
	{
		hideBottomScroll();
	} 
}


function hideBottomScroll()
{
	fnRemoveClassWithSameStyle(&quot;.scroll-bottom&quot;, &quot;ph-wo-display&quot;, &quot;&quot;, false);
	$(&quot;.scroll-bottom&quot;).addClass(&quot;ph-wo-display&quot;);
}

function showBottomScroll()
{
	if($(&quot;#scrollInnerDiv&quot;).width() > 0)
	{
		fnRemoveClassWithSameStyle(&quot;.scroll-bottom&quot;, &quot;ph-wo-display&quot;, &quot;&quot;, false);
	}
	if($(&quot; , &quot;'&quot; , &quot;.pagebanner&quot; , &quot;'&quot; , &quot;).is(&quot;:visible&quot;))
	{
		$(&quot;.scroll-bottom&quot;).scrollLeft($(&quot;.table-responsive:visible&quot;).scrollLeft());
	}
}

function getEmptyTdHeightInSafari(tdHeight)
{
	if($.browser.safari){
        tdHeight = tdHeight - 12;
        return tdHeight;
    }
	else
	{
		return tdHeight;
	}
}

function removeSlimScroll(objectId) 
{
    if($(&quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot;+objectId).parent().prop(&quot; , &quot;'&quot; , &quot;className&quot; , &quot;'&quot; , &quot;) == &quot; , &quot;'&quot; , &quot;slimScrollDiv&quot; , &quot;'&quot; , &quot;)
    {
		$(&quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot;+objectId).slimScroll().unbind(&quot; , &quot;'&quot; , &quot;slimscroll&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot;+objectId).parent().replaceWith($(&quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot;+objectId));
		fnRemoveClassWithSameStyle(&quot;.&quot;+objectId, &quot;ph-wo-overflow&quot;, &quot;&quot;, false);
		fnRemoveClassWithSameStyle(&quot;.&quot;+objectId, &quot;ph-wo-height&quot;, &quot;&quot;, false);
    }
}

function addScrollToApplicationBody()
{
	var frameHeight = parseInt($(window).height());
	var style=document.createElement(&quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;);
	style.type=&quot;text/css&quot;;
	style.setAttribute(&quot; , &quot;'&quot; , &quot;nonce&quot; , &quot;'&quot; , &quot;, &quot;xZrLNGLeef26xpNnnF1igA==&quot;);
	var css=&quot; , &quot;'&quot; , &quot;#bodyContent{min-height:&quot; , &quot;'&quot; , &quot;+frameHeight+&quot; , &quot;'&quot; , &quot; !important};&quot; , &quot;'&quot; , &quot;
	style.appendChild(document.createTextNode(css));
	document.head.appendChild(style);
}  

&quot;))]</value>
      <webElementGuid>e2af2b7e-b85d-4ce2-807b-d4fe6db0f894</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
