# 表格(datagrid)

- 官方页面标题：数据列表示例
- 分类：表格及树
- 来源：http://hisui.cn/api/datagrid/datagrid.html
- 官网更新时间：Fri, 31 May 2024 03:56:13 GMT
- 提取时间：2026-07-25T13:18:46+00:00

> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。

## 本页目录

- 数据网格
  - 说明:
    - 查看表格数据格式：
  - 如：默认表格(hisui-datagrid)
  - 一、编辑灰色表格( headerCls:'panel-header-gray' )
  - 二、自定义表格工具栏( toolbar:'#custtb' )
  - 三、通过JS代码定义表格(表格中tip实现 20180827 )及Shift进行连勾选
  - 四、定制翻页条
  - 五、用在M中特有属性 2019-08-31
  - 六、列头高度自定义实现 2020-05-14
  - 七、行高适应示例(nowrap为false实现, false时性能下降)
  - 八、表格与列的fontSize,lineHeight属性演示
  - 九、生成查询工具条(showFilterToolbar 20220817 )
  - 三、勾选列 (headerCheckbox)20230710

## 数据网格

### 说明:

数据网格（datagrid）的列信息是定义在 <thead>标记中，数据是定义在 <tbody>标记中，确保为所有的数据列设置 field 名称。

#### 查看表格数据格式：

```javascript
// 表格数据格式
{
	{
  rows:[
    {col1:value,col2:value,...},
    {col1:value,col2:value,...},
    ...  // 一页显示的数据
  ],
  footer:[{col1:value,col2:value,...}],       // 脚行数据，一般用于总计,footer为可选值
  total:10					// 总行数
}
```

### 如：默认表格(hisui-datagrid)

```javascript
headerCls:'panel-header-gray' //配置项使表格变成灰色
toolbar: [] //配置项toolbar为空时,会在标题与列头产生间距",
iconCls:'icon-paper' //默认小图标
```

### 一、编辑灰色表格( headerCls:'panel-header-gray' )

```html
<table class="hisui-datagrid" title="病人列表" style="width:600px;height:400px" 
			data-options="autoSizeColumn:false,fitColumns:true,headerCls:'panel-header-gray',iconCls:'icon-paper',pagination:true,toolbar: [{
			iconCls: 'icon-write-order',
			text:'编辑',
			handler: function(){var jqobj = $('#mytb');var curInd =jqobj.datagrid('getRowIndex',jqobj.datagrid('getSelected')) ;
				$('#mytb').datagrid('beginEdit',curInd);}
		},{
			iconCls: 'icon-save',
			text:'保存',
			handler: function(){var jqobj = $('#mytb');var curInd =jqobj.datagrid('getRowIndex',jqobj.datagrid('getSelected')) ;
				$('#mytb').datagrid('endEdit',curInd)}
		},{
			iconCls: 'icon-cancel',
			text:'删除',
			disabled:true,
			handler: function(){var jqobj = $('#mytb');var curInd =jqobj.datagrid('getRowIndex',jqobj.datagrid('getSelected')) ;
				$('#mytb').datagrid('deleteRow',curInd)}
		},{
			iconCls:'icon-set-col',
			text:'修改列标题',
			handler:function(){var jqobj = $('#mytb');jqobj.datagrid('setColumnTitle',{'datetime':'时间(新)','code':'代码(新)'})}
		},{
			iconCls: 'icon-help',
			text:'帮助',
			handler: function(){alert('你点击了帮助按钮')}
		}]
	">   
		<thead>   
			<tr>
				<th data-options="field:'code',width:40">编码</th>   
				<th data-options="field:'name',width:40,editor:{type:'text'}">姓名</th>   
				<th data-options="field:'price',width:50">就诊科室</th>
				<th data-options="field:'num',width:40">诊断</th>
				<th data-options="field:'note',width:50,editor:{type:'validatebox',options:{required:true}}">名称</th>
				<th data-options="field:'re',width:70,editor:{type:'datebox'}">日期</th>   
				<th data-options="field:'datetime',width:100,editor:{type:'datetimebox'}">日期与时间</th>   
			</tr>   
		</thead>   
		<tbody>   
			<tr>   
				<td>001</td><td>王小二</td><td>内三科</td><td>感觉</td><td>name1</td><td>2323</td>   
			</tr>
			......
		</tbody>   
	</table>
```

### 二、自定义表格工具栏( toolbar:'#custtb' )

```html
<table id='mytb2' class="hisui-datagrid" title="病人列表" style="width:600px;height:400px" 
		data-options="headerCls:'panel-header-gray',iconCls:'icon-paper',pagination:true,toolbar:'#custtb',...">   
	...
</table> 
<div id="custtb" >
	<a href="javascript:void(0)" class="hisui-linkbutton" data-options="iconCls:'icon-search',plain:true">查询</a>
	<a href="javascript:void(0)" class="hisui-linkbutton" data-options="iconCls:'icon-cancel',plain:true">删除</a>
	<a href="javascript:void(0)" id="menubtn-toolbar" class="hisui-menubutton menubutton-toolbar" 
		data-options="menu:'#mm-toolbar',iconCls:'icon-add-note'">菜单</a>
	<div id="mm-toolbar" style="width:100px;">   
		<div onclick="console.log('停止');">停止</div>   
		<div onclick="console.log('作废');">作废</div>     
	</div>
</div>
```

### 三、通过JS代码定义表格(表格中tip实现 20180827 )及Shift进行连勾选

```javascript
$(function(){
				$HUI.datagrid('#mytable3',{
					onBeforeUnselect:function(rowIndex,rowData){
						console.log("onBeforeUnselect  arg1 = "+rowIndex + ",  arg2: " + JSON.stringify(rowData));
					},
					onBeforeSelect:function(rowIndex,rowData){
						console.log("onBeforeSelect  arg1 = "+rowIndex + ",  arg2: " + JSON.stringify(rowData));
						//return false; //返回false则结束选中动作
					},
					onSelect:function(rowIndex,rowData){
						console.log("onSelect  arg1 = "+rowIndex + ",  arg2: " + JSON.stringify(rowData));
					},
					onBeforeCheck:function(ind,row){
						console.log('onBeforeCheck',ind,row)
						if(ind==0){
							$.messager.popover({msg:'第一行不能选中',type:'error'})
							return false;
						} 
					},
					onBeforeUncheck:function(ind,row){
						console.log('onBeforeUncheck',ind,row)
						if(ind==1) {
							$.messager.popover({msg:'第二行不能取消选中',type:'error'})
							return false;
						} 
					},
					onLoadSuccess:function(data){
						//$(this).datagrid('columnMoving');
						/*$('.mytooltip').tooltip({
							trackMouse:true,
							onShow:function(e){
								$(this).tooltip('tip').css({
									width:250 ,top:e.pageY+20,left:e.pageX-(250/2)
								});
						}});*/
					},
					//nowrap:false,
					//autoSizeColumn:false,
					//fitColumns:true,
					showFooter:true,
					loadFilter:function(data){ 
						if(data.footer && data.footer.length>0){
							data.footer[0]["code"] = "合计";
						}
						return data;
					},
					frozenColumns:[[
						{field:'ck',title:'ckbox',checkbox:true},	
						{field:'id',title:'ID',width:40},
						{field:'code',title:'代码',width:80,showTip:true,tipWidth:100,tipPosition:"top"},
					]],
					columns:[[
						
						{field:'name',title:'描述',width:60,tipWidth:150,tipPosition:"right",showTipFormatter:function(row,rowIndex){
							return "<span style='display:block;'>代码："+row['code']+"</span>"
							+"<span style='display:block;'>描述："+row['name']+"</span>"
							+"<span style='display:block;'>价格："+row['price']+"</span>";
						}},
						{field:'price',title:'价格',width:60,align:'right'},
						{field:'desc',title:'说明',width:330,showTip:true,tipWidth:450,tipTrackMouse:true} /*,formatter:function(value,row,index){
							if (value) return '<span class="mytooltip" title="'+value+'">'+value+'</span>'
						}*/
					]],
					data:{
						rows:[
						{id:'1',code:'Banana',name:'香蕉',price:'2.50',desc:'香蕉（学名：Musa nana Lour.）芭蕉科芭蕉属植物，又指其果实，热带地区广泛种植。香蕉味香、富含营养，植株为大型草本，从根状茎发出，由叶鞘下部形成高3～6公尺(10～20尺)的假杆；叶长圆形至椭圆形，有的长达3～3.5公尺(10～11.5尺)，宽65公分(26寸)，10～20枚簇生茎顶。穗状花序下垂 [1]  ，由假杆顶端抽出，花多数，淡黄色；果序弯垂，结果10～20串，约50～150个。植株结果后枯死，由根状茎长出的吸根继续繁殖，每一根株可活多年。原产亚洲东南部，台湾、海南、广东、广西等均有栽培。'},
						{id:'2',code:'Apple',name:'苹果',price:'2.50',desc:'苹果（学名：Malus pumila）是水果的一种，是蔷薇科苹果亚科苹果属植物，其树为落叶乔木。苹果的果实富含矿物质和维生素，是人们经常食用的水果之一。苹果是一种低热量食物，每100克只产生60千卡热量。苹果中营养成分可溶性大，易被人体吸收，故有“活水”之称。其有利于溶解硫元素，使皮肤润滑柔嫩 [1]  。据说“每天一苹果，医生远离我”。'},
						{id:'3',code:'Bear',name:'梨',price:'5.50',desc:'梨，通常品种是一种落叶乔木或灌木，极少数品种为常绿，属于被子植物门双子叶植物纲蔷薇科苹果亚科。叶片多呈卵形，大小因品种不同而各异。花为白色，或略带黄色、粉红色，有五瓣。果实形状有圆形的，也有基部较细尾部较粗的，即俗称的“梨形”；不同品种的果皮颜色大相径庭，有黄色、绿色、黄中带绿、绿中带黄、黄褐色、绿褐色、红褐色、褐色，个别品种亦有紫红色；野生梨的果径较小，在1到4厘米之间，而人工培植的品种果径可达8厘米，长度可达18厘米'},
						{id:'4',code:'orange',name:'橙子',price:'3.50',desc:'橙子（学名：Citrus sinensis 英语：orange），是芸香科柑橘属植物橙树的果实，亦称为黄果、柑子、金环、柳丁。橙子是一种柑果，有很高的食用，药用价值。'},
						{id:'5',code:'tangerine',name:'橘子',price:'4.50',desc:'橘（英语：Mandarin orange；学名：Citrus reticulata）是芸香科柑橘属的一种水果。“橘”（jú）和“桔”（jú）都是现代汉语规范字，然“桔”作橘子一义时，为“橘”的俗写。在广东的一些方言中二字同音，“桔”也曾做过“橘”的二简字。闽南语称橘为柑仔。西南官话区的各方言中呼为“柑子”或“柑儿”。'},
						{id:'6',code:'Mango',name:'芒果',price:'4.60',desc:'芒果是杧果 [1]  （中国植物志）的通俗名（拉丁学名：Mangifera indica L.），芒果是一种原产印度的漆树科常绿大乔木，叶革质，互生；花小，杂性，黄色或淡黄色，成顶生的圆锥花序。核果大，压扁，长5-10厘米，宽3-4.5厘米，成熟时黄色，味甜，果核坚硬。'},
						{id:'7',code:'Watermelon',name:'西瓜',price:'2.40',desc:'西瓜（学名：Citrullus lanatus (Thunb.) Matsum. et Nakai）一年生蔓生藤本；茎、枝粗壮，具明显的棱。卷须较粗壮，具短柔毛，叶柄粗，密被柔毛；叶片纸质，轮廓三角状卵形，带白绿色，两面具短硬毛，叶片基部心形。雌雄同株。雌、雄花均单生于叶腋。雄花花梗长3-4厘米，密被黄褐色长柔毛；花萼筒宽钟形；花冠淡黄色；雄蕊近离生，花丝短，药室折曲。雌花：花萼和花冠与雄花同；子房卵形，柱头肾形。果实大型，近于球形或椭圆形，肉质，多汁，果皮光滑，色泽及纹饰各式。种子多数，卵形，黑色、红色，两面平滑，基部钝圆，通常边缘稍拱起，花果期夏季。'},
						{id:'8',code:'litchi',name:'荔枝',price:'2.50',desc:'荔枝（学名：Litchi chinensis Sonn.）无患子科，荔枝属常绿乔木，高约10米。果皮有鳞斑状突起，鲜红，紫红。成熟时至鲜红色；种子全部被肉质假种皮包裹。花期春季，果期夏季。果肉产鲜时半透明凝脂状，味香美，但不耐储藏。分布于中国的西南部、南部和东南部，广东和福建南部栽培最盛。亚洲东南部也有栽培，非洲、美洲和大洋洲有引种的记录。 荔枝与香蕉、菠萝、龙眼一同号称“南国四大果品”。'},
						{id:'9',code:'Pineapple',name:'凤梨',price:'2.50',desc:'凤梨（学名：Ananas comosus (Linn.) Merr.），茎短。叶多数，莲座式排列，剑形，顶端渐尖，全缘或有锐齿，腹面绿色，背面粉绿色，边缘和顶端常带褐红色，生于花序顶部的叶变小，常呈红色。花序于叶丛中抽出，状如松球；苞片基部绿色，上半部淡红色，三角状卵形；萼片宽卵形，肉质，顶端带红色；花瓣长椭圆形，端尖，上部紫红色，下部白色。聚花果肉质。花期夏季至冬季。'},
						{id:'10',code:'Coconut',name:'椰子',price:'2.50',desc:'椰子（学名：Cocos nucifera L. ）棕榈科椰子属植物，植株高大，乔木状，高15-30米，茎粗壮，有环状叶痕，基部增粗，常有簇生小根。叶柄粗壮，花序腋生，果卵球状或近球形，果腔含有胚乳（即“果肉”或种仁），胚和汁液（椰子水），花果期主要在秋季。'}
						],
						footer:[{price:'33元'}],
						total:10
					},
					idField:'id',
					headerCls:'panel-header-gray',
					iconCls:'icon-paper',
					rownumbers:true,
					singleSelect:false,
					width:650,
					height:300,
					title:'水果单价表',
					toolbar:[],
					shiftCheck:true
				})
			})
```

```html
<style>
td[field="desc"] div.datagrid-cell, .datagrid-cell-group{
	text-overflow: ellipsis; /*2018-12-4 showTip 列数据太多显示...*/
	padding: 0 8px;
}
</style>
<table id="mytable3"></table>
<script type="text/javascript">
	$(function(){
		$HUI.datagrid('#mytable3',{
			onBeforeUnselect:function(rowIndex,rowData){
				console.log("onBeforeUnselect  arg1 = "+rowIndex + ",  arg2: " + JSON.stringify(rowData));
			},
			onBeforeSelect:function(rowIndex,rowData){
				console.log("onBeforeSelect  arg1 = "+rowIndex + ",  arg2: " + JSON.stringify(rowData));
				//return false; //返回false则结束选中动作
			},
			onSelect:function(rowIndex,rowData){
				console.log("onSelect  arg1 = "+rowIndex + ",  arg2: " + JSON.stringify(rowData));
			},
			onBeforeCheck:function(ind,row){
				console.log('onBeforeCheck',ind,row)
				if(ind==0){
					$.messager.popover({msg:'第一行不能选中',type:'error'})
					return false;
				} 
			},
			onBeforeUncheck:function(ind,row){
				console.log('onBeforeUncheck',ind,row)
				if(ind==1) {
					$.messager.popover({msg:'第二行不能取消选中',type:'error'})
					return false;
				} 
			},
			autoSizeColumn:false,
			fitColumns:true,
			showFooter:true,
			loadFilter:function(data){ //拿到数据后，为合计行赋值
				if(data.footer && data.footer.length>0){
					data.footer[0]["code"] = "合计";
				}
				return data;
			},
			columns:[[
				{field:'name',title:'描述',width:60,tipWidth:150,tipPosition:"right",showTipFormatter:function(row,rowIndex){
					return "<span style='display:block;'>代码："+row['code']+"</span>"
					+"<span style='display:block;'>描述："+row['name']+"</span>"
					+"<span style='display:block;'>价格："+row['price']+"</span>";
				}},
				{field:'price',title:'价格',width:60,align:'right'},
				{field:'desc',title:'说明',width:300,showTip:true,tipWidth:450,tipTrackMouse:true}
			]],
			data:{
				rows:[
				{id:'1',code:'banana',name:'香蕉',price:'2.50',desc:'...'},
				{id:'2',code:'apple',name:'苹果',price:'2.50',desc:'...'},...
				],
				footer:[{price:'33元'}],
				total:10
			},
			idField:'id',
			headerCls:'panel-header-gray',
			iconCls:'icon-paper',
			rownumbers:true,
			singleSelect:false,
			width:500,
			title:'水果单价表',
			headerCls:'panel-header-gray',
			iconCls:'icon-paper'
		})
	})		
</script>
```

### 四、定制翻页条

```html
<table ... data-options="headerCls:'panel-header-gray',
			showPageList:false, showRefresh:false,
			afterPageText:'页,共{pages}页', beforePageText:'第', displayMsg:'显示{from}到{to}条，共{total}条',
			pagination:true"> </table>
```

### 五、用在M中特有属性 2019-08-31

```javascript
$("#querydg").datagrid({
	//不用定义columns属性, 支持自定义列头
	className:"web.Test",
	queryName:"LookUp", 
	url:$URL+"?ClassName=web.Test&QueryName=LookUp&...",
	// 20220818 增加 方便实现默认定义列宽, 定义编辑功能
	// 如果有【界面列定义】,定义列信息会部分覆盖默认信息, 如果顺序,列宽。
	// 为可选项
	defaultsColumns:[
		{field:"TID",hidden:true},
		{field:'TIPAddr',width:150,editor:{type:'text'}},
		{field:'TMacAddr',width:150,editor:{type:'text'}},
		{field:'TCompName',width:150,editor:{type:'text'}},
		{field:'TNote',width:150,editor:{type:'text'}},
		{field:'TDisableLogon',width:60,formatter:function(val,row,index){
					if (val==1) return "禁用";
					else {return "可用"}
				},editor:{type:'checkbox',options:{on:1,off:0}}
		},
		{field:'TDisableLogonDate',width:150,editor:{type:'dateboxq'}},
		{field:'TDisableLogonTime',width:150,editor:{type:'timeboxq'}}
	],
	...
});
```

```javascript
$("#querydg").datagrid({
	className:"web.Test",
	queryName:"LookUp", 
	onColumnsLoad:function(cm){
		for (var i=0;i<cm.length;i++){
			if(cm[i]['field']=="TStDate"){
				cm[i].formatter = function(val,row,index){
					return "日期"+val;
				}
				cm[i].styler=function(val,row,index){
					return 'color:red;';
				}
			}
		}
	},
	url:$URL+"?ClassName=web.Test&QueryName=LookUp&...",
	...
});
```

### 六、列头高度自定义实现 2020-05-14

```html
<!--使用样式类 [.gridId-header-row行号] 来定义列头高度．如下-->
<style> .dg6-header-row0{ height: 20px;} .dg6-header-row1{ height: 40px;} .dg6-header-row2{ height: 60px;} </style>
<table id="dg6"></table>
```

```javascript
$(function(){
			$('#dg6').datagrid({
				title:'地区',
				height:'200',
				columns:[
					[
						{title:'行序号',align:'left',halign:'center',rowspan:3}	
						,{title:'地区',align:'left',halign:'center',colspan:9}
					],[
						{title:'北京',field:'bj', align:'left',halign:'center',colspan:3},
						{title:'上海',field:'sh', align:'left',halign:'center',colspan:3},
						{title:'南昌',field:'nc', align:'left',halign:'center',colspan:3}
					],[
						{title:'朝阳区',field:'bjzy', align:'left',halign:'center',sortable:true,order:'asc',width:82},
						{title:'海淀区',field:'bjhd', align:'left',halign:'center',sortable:true,order:'asc',width:82},
						{title:'昌平区',field:'bjzp', align:'left',halign:'center',sortable:true,order:'asc',width:82},
						{title:'虹口区',field:'bjhk', align:'left',halign:'center',sortable:true,order:'asc',width:82},
						{title:'徐汇区',field:'bjxf', align:'left',halign:'center',sortable:true,order:'asc',width:82},
						{title:'宝山区',field:'bjbs', align:'left',halign:'center',sortable:true,order:'asc',width:82},
						{title:'西湖区',field:'bjsf', align:'left',halign:'center',sortable:true,order:'asc',width:82},
						{title:'红谷滩新区',field:'bjhgt', align:'left',halign:'center',sortable:true,order:'asc',width:82},
						{title:'高新区',field:'bjgx', align:'left',halign:'center',sortable:true,order:'asc',width:82}
					]
				]
			});
		})
```

### 七、行高适应示例(nowrap为false实现, false时性能下降)

```javascript
$(function(){
				$HUI.datagrid('#mytable7',{
					autoSizeColumn:false,
					fitColumns:true,idField:'id',
					headerCls:'panel-header-gray',iconCls:'icon-paper',
					rownumbers:true,width:650,height:200,
					title:'属性列表',
					toolbar:[],
					nowrap:false,  /*此处为false*/
					columns:[[
						{field:'ck',title:'sel',checkbox:true},
						{field:'code',title:'工号',width:100},
						{field:'name',title:'属性名',width:100,wordBreak:"break-all"},
						{field:'type',title:'类型',width:100,align:'right'},
						{field:'desc',title:'说明',width:200,wordBreak:"break-all"}  /*break-all 说明内容任意换行*/
					]],
					data:{
						rows:[
						{id:'1',code:'10',name:'checkOnSelect',type:'boolean',desc:'If true, the checkbox is checked/unchecked when the user clicks on a row. If false, the checkbox is only checked/unchecked when the user clicks exactly on the checkbox.This property is available since version 1.3.'},
						{id:'2',code:'11',name:'nowrap',type:'boolean',desc:'True to display data in one line. Set to true can improve loading performance.'},
						{id:'3',code:'12',name:'fitColumns',type:'boolean',desc:'True to auto expand/contract the size of the columns to fit the grid width and prevent horizontal scrolling.'},
						{id:'4',code:'13',name:'resizeHandle',type:'boolean',desc:'Resizing column position, Available value are: "left","right","both". When "right", users can resize columns by dragging the right edge of column headers, etc.This property is available since version 1.3.2.'}
						],
						total:10
					}
				})
			})
```

### 八、表格与列的fontSize,lineHeight属性演示

```javascript
$(function(){
				$HUI.datagrid('#mytable8',{
					autoSizeColumn:false,
					fitColumns:true,idField:'id',
					headerCls:'panel-header-gray',iconCls:'icon-paper',
					rownumbers:true,width:650,height:200,
					title:'属性列表',titleNoWrap:false,/*表头自动折行*/
					toolbar:[],
					nowrap:false,
					fontSize:12, /*表格上设置字体大小*/
					lineHeight:12,
					columns:[[
						{field:'ck',title:'sel',checkbox:true},
						{field:'code',title:'属性序号',width:40},
						{field:'name',title:'属性名',width:100},
						{field:'type',title:'类型',width:100,align:'right'},
						{field:'desc',title:'说明',width:200,fontSize:10,lineHeight:14}  /*列上设置字体大小*/
					]],
					data:{
						rows:[
						{id:'1',code:'10',name:'checkOnSelect',type:'boolean',desc:'If true, the checkbox is checked/unchecked when the user clicks on a row. If false, the checkbox is only checked/unchecked when the user clicks exactly on the checkbox.This property is available since version 1.3.'},
						{id:'2',code:'11',name:'nowrap',type:'boolean',desc:'True to display data in one line. Set to true can improve loading performance.'},
						{id:'3',code:'12',name:'fitColumns',type:'boolean',desc:'True to auto expand/contract the size of the columns to fit the grid width and prevent horizontal scrolling.'},
						{id:'4',code:'13',name:'resizeHandle',type:'boolean',desc:'Resizing column position, Available value are: "left","right","both". When "right", users can resize columns by dragging the right edge of column headers, etc.This property is available since version 1.3.2.'}
						],
						total:10
					}
				})
			})
```

### 九、生成查询工具条(showFilterToolbar 20220817 )

```javascript
$(function(){
				// $HUI.datagrid('#mytable9',{
				// 	autoSizeColumn:false,
				// 	fitColumns:true,idField:'id',
				// 	headerCls:'panel-header-gray',iconCls:'icon-paper',
				// 	rownumbers:true,width:650,height:200,
				// 	title:'属性列表',titleNoWrap:false,/*表头自动折行*/
				// 	toolbar:[],
				// 	nowrap:false,
				// 	fontSize:12, /*表格上设置字体大小*/
				// 	lineHeight:12,
				// 	showFilterToolbar:true, /*显示过滤工具栏*/
				// 	columns:[[
				// 		{field:'ck',title:'sel',checkbox:true},
				// 		{field:'code',title:'属性序号',width:40},
				// 		{field:'name',title:'属性名',width:100},
				// 		{field:'type',title:'类型',width:100,align:'right'},
				// 		{field:'desc',title:'说明',width:200,fontSize:10,lineHeight:14}  /*列上设置字体大小*/
				// 	]],
				// 	data:{
				// 		rows:[
				// 		{id:'1',code:'10',name:'checkOnSelect',type:'boolean',desc:'If true, the checkbox is checked/unchecked when the user clicks on a row. If false, the checkbox is only checked/unchecked when the user clicks exactly on the checkbox.This property is available since version 1.3.'},
				// 		{id:'2',code:'11',name:'nowrap',type:'boolean',desc:'True to display data in one line. Set to true can improve loading performance.'},
				// 		{id:'3',code:'12',name:'fitColumns',type:'boolean',desc:'True to auto expand/contract the size of the columns to fit the grid width and prevent horizontal scrolling.'},
				// 		{id:'4',code:'13',name:'resizeHandle',type:'boolean',desc:'Resizing column position, Available value are: "left","right","both". When "right", users can resize columns by dragging the right edge of column headers, etc.This property is available since version 1.3.2.'}
				// 		],
				// 		total:10
				// 	}
				// });
				$HUI.datagrid('#mytable9',{
					columns:[[
						// {field:'ck',title:'ckbox',checkbox:true},
						{field:'id',title:'ID',width:40},
						{field:'code',title:'代码',width:80,showTip:true,tipWidth:100,tipPosition:"top"},
						{field:'name',title:'描述',width:60},
						{field:'price',title:'价格',width:60,align:'right'}
					]],
					data:{
						rows:[
						{id:'1',code:'Banana',name:'香蕉',price:'2.50'},
						{id:'2',code:'Apple',name:'苹果',price:'2.50'},
						{id:'3',code:'Bear',name:'梨',price:'5.50'},
						{id:'4',code:'orange',name:'橙子',price:'3.50'},
						{id:'5',code:'tangerine',name:'橘子',price:'4.50'},
						{id:'6',code:'Mango',name:'芒果',price:'4.60'},
						{id:'7',code:'Watermelon',name:'西瓜',price:'2.40'},
						{id:'8',code:'litchi',name:'荔枝',price:'2.50'},
						{id:'9',code:'Pineapple',name:'凤梨',price:'2.50'},
						{id:'10',code:'Coconut',name:'椰子',price:'2.50'}
						],
						total:10
					},
					idField:'id',
					autoSizeColumn:false,
					headerCls:'panel-header-gray',
					iconCls:'icon-paper',
					rownumbers:true,
					singleSelect:false,
					width:650,
					height:300,
					title:'水果单价表',
					toolbar:[],fitColumns:true,
					showFilterToolbar:true /*显示过滤工具栏*/
				})
			})
```

### 三、勾选列 (headerCheckbox)20230710

```html
<div class="demo-exp-code entry-content use-prettyprint" prettyprintfor="#mydg10codehtml"><table id="mytable10"></table></div>
```

```javascript
$(function(){
			$HUI.datagrid('#mytable10',{
				columns:[[
					// {field:'ck',title:'ckbox',checkbox:true},
					{field:'id',title:'ID',width:20},
					{field:'caption',title:'已写病历授权范围',width:200,showTip:true,tipWidth:100,tipPosition:"top",align:'left',formatter:function(value,row){
							return '<input class="hisui-checkbox" type="checkbox"/>'+value;
						},
						headerCheckbox:true /*true时列头显示勾选框,且勾选时，把当前列数据全勾或全取消勾*/},
					{field:'update',title:'修改',width:60,align:'center',formatter:function(value,row){
							if (value=='Y'){
								return '<input class="hisui-checkbox" type="checkbox" checked/>';
							}
							return '<input class="hisui-checkbox" type="checkbox"/>';
						},
						headerCheckbox:true /*true时列头显示勾选框,且勾选时，把当前列数据全勾或全取消勾*/},
					{field:'delete',title:'删除',width:60,align:'center',formatter:function(value,row){
							if (value=='Y') return '<input class="hisui-checkbox" type="checkbox" checked/>';
							return '<input class="hisui-checkbox" type="checkbox"/>';
						},
						headerCheckbox:true /*true时列头显示勾选框,且勾选时，把当前列数据全勾或全取消勾*/},
					{field:'print',title:'打印',width:60,align:'center',formatter:function(value,row){
							var val = '';	
							if (value=='Y') val="checked";
							if (row.id < 3){
								return '<input class="hisui-checkbox" type="checkbox" disabled '+ val +'/>';
							}else{
								return '<input class="hisui-checkbox" type="checkbox" '+val+'/>';
							}
						},
						headerCheckbox:true /*true时列头显示勾选框,且勾选时，把当前列数据全勾或全取消勾*/
					}
				]],
				data:{
					rows:[
					{id:'1',caption:'入院记录(糖尿病) 2023-02-10 09:09:10 医生1',update:'Y',delete:'N',print:'N'},
					{id:'2',caption:'首次病程记录  2023-02-10 17:10:21 医生1',update:'Y',delete:'Y',print:'N'},
					{id:'3',caption:'日常病程记录 2023-03-11 11:39:01 医生1',update:'Y',delete:'N',print:'Y'},
					{id:'4',caption:'交班记录  2023-03-12 10:39:31 医生1',update:'Y',delete:'N',print:'N'},
					{id:'5',caption:'日常病程记录 2023-03-13 11:39:01 医生1',update:'Y',delete:'N',print:'N'},
					{id:'6',caption:'日常病程记录 2023-03-14 11:39:01 医生1',update:'Y',delete:'N',print:'N'}
					],
					total:10
				},
				toolbar:[{iconCls: 'icon-search',text:"获得[打印]列选中值",handler:function(){
					var row = $('#mytable10').datagrid('getCheckboxRows','print');
					console.log(row);
				}}],
				idField:'id',
				autoSizeColumn:false,
				headerCls:'panel-header-gray',
				iconCls:'icon-paper',
				onLoadSuccess:function(data){
					$(".hisui-checkbox").checkbox();  // 使用这句代码可以让行勾选框变成hisui-checkbox, 也可以注释此行
				},
				rownumbers:true,
				singleSelect:false,
				width:850,
				height:310,
				title:'目录权限',
				fitColumns:true,
				checkOnSelect: false, selectOnCheck: false
			})
		})
```

| 属性 | 说明 | 默认值 |  |
| --- | --- | --- | --- |
| fitColumns | 列自动适应 | none | 为true时， 一定要表格的每一列都给上width值 ，以便自动按比例缩放。
如果某一列不给宽度，会以width:auto宽度来自动展开，其它指定宽度的列再按等比缩放宽度。（但此方式在某些情况下会出现列头不能对齐问题） |
| autoSizeColumn | 是否自动计算列属性 | true | 如果不用请设置成false。严重影响效率 |
| showRefresh | 是否显示翻页条上的刷新按钮 | true |  |
| showPageList | 是否显示翻页条上的下拉行列表 | true |  |
| afterPageText | 翻页条上输入框后-显示信息 | 页,共{pages}页 | 值为''，则不显示信息 |
| beforePageText | 翻页条上输入框前-显示信息 | 第 | 值为''，则不显示信息 |
| displayMsg | 显示信息 | 显示 {from} 到 {to} ,从 {total} 条记录 | 值为''，则不显示信息 |
| fixRowNumber | 让行号列自适应宽度。 rownumbers 为 true 时有效. | false | 值为true时，行号列自适应宽度。 (20180822) |
| showTip | 列属性，当为true时,鼠标移入时弹出提示层，见例3 | false | 默认为false |
| tipWidth | 列属性，定义鼠标移入时弹出提示层的宽度，见例3 | 300 | 默认为300px |
| tipPosition | 列属性，定义鼠标移入时弹出提示层的位置，见例3 | 'bottom' | 'top','bottom','left','right' (20210105) |
| tipTrackMouse | 列属性，定义鼠标移入时弹出的提示层是否跟随鼠标，见例3 | false | true或false,默认为false (20210105) |
| btoolbar | 底部工具栏。与toolbar对应,只是位置变到列表的底部 | null | 默认不显示。 (20190510) |
| className | m中类名 | "" | 默认为空。非常规属性。 (20190829) |
| queryName | m中query名 | "" | 默认为空。非常规属性。通过className与queryName生成columns，且能自定义列属性。 (20190829) |
| defaultsColumns | 默认列定义。可选项 | null | 非常规属性。与通过className与queryName生成`cm`合并成新的columns。 (20220818) |
| fontSize | 设计表格内容字体大小 | "" | 默认为空。如fontSize:10则表格内容字段大小为10px。在nowrap:false时字体设置后表格行高会变化 (20200824) |
| lineHeight | 设计表格内容字体行高 | "" | 默认为空。如lineHeight:10则表格内容行高为10px (20201126) |
| titleNoWrap | 表头不折行 | true | 默认不折行。值为false时自动折行 20200824 |
| clicksToEdit | 编辑行点击动作配置 | 0 | 0表示默认无动作，1单击编辑，2双击编辑 20200907 |
| shiftCheck | shift连选功能配置 | false | true时，按中[Shift+鼠标左键]连选多行 20210923 |
| singleRequest | 是否只保留最后一个请求 | false | true时,只保留最后一个请求 2021-11-10 |
| loadBeforeClearSelect | 加载数据前是否清空选中，去除全选勾 | false | true时,在加载数据前清空所有选中 20220126 |
| columnsUrl | 获得表格列定义数据路径,返回的json格式为:{"cm":[{"field":"Code","title":"代码"},{...}],"pageSize":15,"originPage":"xx.csp"}。 会把cm重写到columns及defaultColumns中 | null | 可选,优先于columns属性 (20221101) |
| editColumnsPage | 编辑列定义界面路径 | null | 可选,定义后双击列头会弹出此界面 (20221101) |
| editColumnsGrantUrl | 判断是否有权力编辑列定义路径,返回1表示有权限 | null | 可选,定义后在双击列头时,会通过此链接判断是否有权限弹出editColumnsPage界面 (20221101) |
| clearSelectionsOnload | 数据加载后清除选中信息 | false | 可选项,防止全选勾及选中信息带入下一页 20230824 |
| refLinkButton | 关联按钮的jQuery选择器 | null | 如配置'#Find'，配置后会联动Id为Find的按钮禁用状态 20231211 |
| 方法名 | 说明 | 入参 |  |
| setColumnTitle | 重设置列头标题 | colOpt | 如:{'datatime':'时间列标题','code':'代码列标题'} (20181009) |
| getEditingIndex | 获得表格当前编辑行索引 | undefined | 返回编辑行，从0开始 (20200907) |
| uncheckAll | 取消表格当前页的所有勾选 | isAllPage | 入参为true可以取消所有页的勾选 (20230208) |
| unselectAll | 取消表格当前页的所有选中 | isAllPage | 入参为true可以取消所有页的选中 (20230208) |
| getCheckboxRows | 获得某列的所有勾选的记录 | fieldName | 返回rows (20230711) |
| 事件名 | 说明 | 入参 |  |
| onDblClickHeader | 双击列头事件 | e,field |  |
| onBeforeSelect | 选中行之前触发，返回false时，取消动作。 | rowIndex,rowData | rowIndex为行索引，从0开始 |
| onBeforeUnselect | 取消选中行之前触发，返回false时，取消动作。 | rowIndex,rowData | rowIndex为行索引，从0开始 |
| onBeforeCheck | 用户勾选一行之前触发，返回false时，取消动作。 | rowIndex,rowData | rowIndex为行索引，从0开始 |
| onBeforeUncheck | 当用户取消勾选一行之前触发，返回false时，取消动作。 | rowIndex,rowData | rowIndex为行索引，从0开始 |
| onHighlightRow | 高亮行事件 | rowIndex,rowData |  |
| onColumnsLoad | 获得columns后触发事件 | cm | 配置className与queryName后有效 2019-09-31 |
| onInitBefore | 用于在表格生成前修改上下文 | options | 当前this指向datagrid (20221101) |
| 列配置项 | 说明 | 默认值 | 描述 |
| wordBreak | 在nowrap为false时，列内容换行规则 | null | 任意截断：'break-all',完整单词截断：'break-word' 2020-08-19 |
| fontSize | 当前列内容字体大小 | "" | 默认为空。如fontSize:10则表格内容字段大小为10px。在nowrap:false时字体设置后表格行高会变化 (20200824) |
| lineHeight | 当前列内容字体行高 | "" | 默认为空。如lineHeight:10则表格内容行高为10px (20201126) |
| showTip | 列属性,当为true时,鼠标移入某单元格时,弹出提示层,内容为当前单元格内容。见例3-说明列 | false | 默认为false |
| showTipFormatter | 列属性,当配置为函数时,会在鼠标移入单元格时,弹出提示层, 提示层内容为当前函数的返回值。见例3-描述列 20220706 | null | 函数入参function(row,rowIndex){},返回文本或html片段 |
| tipWidth | 列属性，定义鼠标移入时弹出提示层的宽度，见例3 | 300 | 默认为300px |
| tipPosition | 列属性，定义鼠标移入时弹出提示层的位置，见例3 | 'bottom' | 'top','bottom','left','right' (20210105) |
| tipTrackMouse | 列属性，定义鼠标移入时弹出的提示层是否跟随鼠标，见例3 | false | true或false,默认为false (20210105) |
| headerCheckbox | 列头上显示勾选框。勾选时会把当前列中所有checkbox勾中。 | false | true时显示 (20230711) |
| columnHeaderTitle | 列头上显示说明提示 | undefined | 有值时才显示说明提示，且列头变色 (20230711) |
