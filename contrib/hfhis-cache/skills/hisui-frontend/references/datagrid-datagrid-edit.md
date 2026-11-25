# 表格编辑(edit)

- 官方页面标题：编辑列表示例
- 分类：表格及树
- 来源：http://hisui.cn/api/datagrid/datagrid.edit.html
- 官网更新时间：Sat, 03 Aug 2024 07:28:30 GMT
- 提取时间：2026-07-25T13:18:46+00:00

> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。

## 本页目录

- 数据网格-点击修改
  - 说明:
  - 一、数据列表编辑-基于已有编辑类型
  - 二、数据列表编辑-实现自己的编辑器( 20180830 )

## 数据网格-点击修改

### 说明:

编辑列表中 text , textarea , checkbox , numberbox , validatebox , datebox , datetimebox , combobox , combotree , combogrid , icheckbox , switchbox , linkbutton , lookup timespinner , timeboxq 组件的使用示例

### 一、数据列表编辑-基于已有编辑类型

```javascript
Mock.mock("data.csp",{
		"rows|10":[{
			"patid|+1":100,
			"feetypeid":1,
			"feetypename":"自费",
			"feeprice":1.9,
			"feetotal":10,
			"note":"此处写入说明",
			"active":/[YN]/,
			"active1":/[YN]/,
			"active2":/禁用|启用/,
			"op":"结束编辑",
			"dbq":'2020-07-15',
			"timesp":"8:30",
			timebq:"9:30"
		}],
		"total":10
	})
	Mock.mock("feetype.csp",
		[{"feetypeid":1,'feetypename':'自费'},{"feetypeid":2,'feetypename':'职工'},{"feetypeid":3,'feetypename':'医保'}]
	);
```

```html
<div class="use-prettyprint" prettyprintfor="#mydg1codehtml">
<table class="hisui-datagrid" data-options="toolbar:[],singleSelect: true,
		headerCls:'panel-header-gray',onClickRow: onClickRow,iconCls:'icon-paper',url: 'data.csp'" id="dg" style="width:1200px;height:300px" title="点击编辑列表">
<thead>
<tr>
<th data-options="field:'patid',width:60">病人Id</th>
<th data-options="field:'feetypeid',width:100,
					formatter:function(value,row){
						return row.feetypename;
					},
					editor:{
						type:'combobox',
						options:{
							valueField:'feetypeid',
							textField:'feetypename',
							method:'get',
							url:'feetype.csp',
							required:true,
							blurValidValue:true
						}
					}">费用类型combobox</th>
<th data-options="field:'feeprice',width:80,align:'right',editor:{type:'numberbox',options:{precision:1}}">价格(numberbox)</th>
<th data-options="field:'feetotal',width:80,align:'right',editor:'numberbox'">数量numberbox</th>
<th data-options="field:'note',width:200,editor:{type:'textarea',options:{required:true,height:'60px'}}">说明(text)</th>
<th data-options="field:'active',width:60,align:'center',editor:{type:'checkbox',options:{on:'Y',off:'N'}}">激活checkbox</th>
<th data-options="field:'active1',width:60,align:'center',editor:{type:'icheckbox',options:{on:'Y',off:'N'}}">激活icheckbox</th>
<th data-options="field:'active2',width:60,align:'center',editor:{type:'switchbox',options:{onClass:'primary',offClass:'gray',onText:'启用',offText:'禁用'}}">开关switchbox</th>
<th data-options="field:'op',width:140,align:'center',editor:{type:'linkbutton',options:{iconCls:'icon-w-edit',handler:function(){endEditing();}}}">按钮linkbutton</th>
<th data-options="field:'dbq',width:120,align:'left',editor:{type:'dateboxq'}">日期q</th>
<th data-options="field:'timesp',width:80,align:'left',editor:{type:'timespinner'}">时间</th>
<th data-options="field:'timebq',width:80,align:'left',editor:{type:'timeboxq'}">时间q</th>
</tr>
</thead>
</table>
</div>
```

```javascript
var editIndex=undefined;
	var modifyBeforeRow = {};
	var modifyAfterRow = {};
	function endEditing(){
			if (editIndex == undefined){return true}
			if ($('#dg').datagrid('validateRow', editIndex)){
				debugger
				//列表中下拉框实现，修改后把回写feetypename，因为formatter显示的是feetypename字段
				var ed = $('#dg').datagrid('getEditor', {index:editIndex,field:'feetypeid'});
				var feetypename = $(ed.target).combobox('getText');
				$('#dg').datagrid('getRows')[editIndex]['feetypename'] = feetypename;
				$('#dg').datagrid('endEdit', editIndex);
				modifyAfterRow = $('#dg').datagrid('getRows')[editIndex];
				var aStr = JSON.stringify(modifyAfterRow);
				var bStr = JSON.stringify(modifyBeforeRow);
				if(aStr!=bStr){
					console.log('修改前：');
					console.dir(modifyBeforeRow);
					console.log('修改后：');
					console.dir(modifyAfterRow);
				}
				editIndex = undefined;
				return true;
			} else {
				return false;
			}
		}
	function onClickRow(index){
		if (editIndex!=index) {
			if (endEditing()){
				$('#dg').datagrid('selectRow', index)
						.datagrid('beginEdit', index);
				editIndex = index;
				modifyBeforeRow = $.extend({},$('#dg').datagrid('getRows')[editIndex]);
			} else {
				$('#dg').datagrid('selectRow', editIndex);
			}
		}
	}
```

### 二、数据列表编辑-实现自己的编辑器( 20180830 )

```javascript
Mock.mock("data2.csp",{
		"rows|10":[{
			"grantBtnGroup":"id1^保存^1#id2^删除^0#id3^打印^0#id4^新建^1" //|(复制^0#新建^0)/
		}],
		"total":10
	})
```

```html
<div class="use-prettyprint" prettyprintfor="#mydg2codehtml">
<table class="hisui-datagrid" data-options="headerCls:'panel-header-gray',iconCls:'icon-paper',singleSelect: true,url:'data2.csp',
		
		method: 'get',rownumbers:true,onClickRow: onClickRow2,toolbar:[]" id="dg2" style="width:300px;height:300px" title="点击编辑列表">
<thead>
<tr>
<th data-options="field:'grantBtnGroup',width:250,editor:{type:'mybtns',options:{on:'1',off:'0'}},formatter:function(value,row){
				var valArr = value.split('#');
				var rtnArr = [];
				$.each(valArr,function(i,item){
					var itemArr = item.split('^');
					if (itemArr[2]==1) rtnArr.push(itemArr[1]);
				});
				return rtnArr.join();
			},">授权</th>
</tr>
</thead>
</table>
</div>
```

```javascript
$.extend($.fn.datagrid.defaults.editors, {
		mybtns: {
			// "grantBtnGroup":"id1^保存^1#id2^删除^0#id3^打印^0#id4^新建^1"
			init: function (_662, _663) {
				var opt = $.extend({on:'on',off:'off'},_663);
				var _664 = $("<div></div>").appendTo(_662);
				//_664.checkbox(opt);
				_664.data("opt",opt);
				return _664;
			}, getValue: function (initRtnObj) {
				var rtnArr = [];
				initRtnObj.find("input[type='checkbox']").each(function(){
					var lbl = $(this).attr('label');
					var id = $(this).attr('id');
					var chck = $(this).next('label').hasClass('checked');
					chck = chck?1:0;
					rtnArr.push(id+"^"+lbl+"^"+chck);
				});
				return rtnArr.join("#");
			}, setValue: function (initRtnObj, val) {
				var btnArr = val.split("#");
				var opt = initRtnObj.data("opt");
				$.each(btnArr,function(i,item){
					var itemArr = item.split("^");
					$("<input type=\"checkbox\" id="+itemArr[0]+" label="+itemArr[1]+">").appendTo(initRtnObj).checkbox({checked:itemArr[2]==1?true:false});
				});
			}
		}
	});
	var editIndex2=undefined;
	var modifyBeforeRow2 = {};
	var modifyAfterRow2 = {};
	function endEditing2(){
		if (editIndex2 == undefined){return true}
		if ($('#dg2').datagrid('validateRow', editIndex2)){
			$('#dg2').datagrid('endEdit', editIndex2);
			modifyAfterRow2 = $('#dg2').datagrid('getRows')[editIndex2];
			var aStr = JSON.stringify(modifyAfterRow2);
			var bStr = JSON.stringify(modifyBeforeRow2);
			if(aStr!=bStr){
				console.log('表格二修改前：');
				console.dir(modifyBeforeRow2);
				console.log('表格二修改后：');
				console.dir(modifyAfterRow2);
				$.messager.popover({msg:'保存成功',type:'success'});
			}
			//去除授权列修改标志
			$("#dg2").datagrid("getPanel").find("tr.datagrid-row[datagrid-row-index="+editIndex2+"]>td[field='grantBtnGroup']").removeClass("datagrid-value-changed");
			editIndex2 = undefined;
			return true;
		} else {
			return false;
		}
	}
	function onClickRow2(index){
		if (editIndex2!=index) {
			if (endEditing2()){
				$('#dg2').datagrid('selectRow', index)
						.datagrid('beginEdit', index);
				editIndex2 = index;
				modifyBeforeRow2 = $.extend({},$('#dg2').datagrid('getRows')[editIndex2]);
			} else {
				$('#dg2').datagrid('selectRow', editIndex2);
			}
		}
	}
```

| 增加列编辑类型 |  |  |  |
| --- | --- | --- | --- |
| 属性 | 说明 | 默认值 |  |
| text | 解析成原生input |  |  |
| textarea | 解析成原生textarea |  |  |
| checkbox | 解析成原生checkbox |  |  |
| numberbox | 解析成numberbox |  |  |
| validatebox | 解析成validatebox |  |  |
| datebox | 解析成datebox组件 |  |  |
| datetimebox | 解析成datetimebox |  |  |
| combobox | 解析成combobox |  |  |
| combotree | 解析成combotree |  |  |
| combogrid | 解析成combogrid |  |  |
| icheckbox | 解析成checkbox组件，非原生checkbox | 显示显示'on'\|'off' | 返回on\|off |
| linkbutton | 解析成linkbutton组件 |  |  |
| switchbox | 解析成switchbox组件 | 默认显示'开'\|'关'，定义项见switchbox组件 | 返回onText\|offText |
| timespinner | 解析成timespinner组件 |  |  |
