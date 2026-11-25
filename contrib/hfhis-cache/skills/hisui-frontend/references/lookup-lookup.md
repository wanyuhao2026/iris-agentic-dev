# 放大镜(lookup)

- 官方页面标题：放大镜
- 分类：极速系列
- 来源：http://hisui.cn/api/lookup/lookup.html
- 官网更新时间：Tue, 12 Mar 2024 03:51:30 GMT
- 提取时间：2026-07-25T13:18:46+00:00

> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。

## 本页目录

- 放大镜
  - 说明:
  - 基础使用
  - 当输入3个字符以上开始查询
  - 禁用启用，是否只读切换示例
  - onBeforeShowPanel事件
  - selectRowRender方法
  - 放大镜(lookup)组件，属性、事件、方法

## 放大镜

### 说明:

继承于comobq与datagrid，按钮实现方式不再一样，在IE8或IE11下速度大大提高。在首次初始化时不初始化列表，无论界面中有多少个放大镜，界面只存在一个列表。即只保留当前激活下拉框列表数据。

### 基础使用

```javascript
$(function(){
				$("#group").lookup({
					width:200,
					
					panelWidth:746,
					
					url:'getGroup',
					mode:'remote',
					idField:'HIDDEN',
					textField:'Description',
					columns:[[  
						{field:'Description',title:'安全组名称',width:200},  
						{field:'HIDDEN',title:'安全组ID',width:100} ,
						{field:'Other',title:'其他信息',width:400,formatter:function(val,row){
							return row['Description']+'：分配人数'+Math.floor(Math.random()*500)
						}} 
					]],
					pagination:true,
					onSelect:function(index,rowData){
						console.log("index="+index+",rowData=",rowData);
						//$('#group').lookup('setText',rowData.HIDDEN+"-"+rowData.Description);  //textField虽是Description，在onSelect这可以自己改显示的值 但此种操作需要自己处理好查询条件间的关系
					}
				});
			})
```

### 当输入3个字符以上开始查询

```javascript
$(function(){
				$("#loc").lookup({
					// disabled:true,
					// readOnly:true,
					// hasDownArrow:false,
					width:200,
					
					panelWidth:446,
					
					url:'getLoc',
					mode:'remote',
					idField:'HIDDEN',
					textField:'Description',
					columns:[[  
						{field:'Code',title:'科室代码',width:150},  
						{field:'Description',title:'科室描述',width:200},  
						{field:'HIDDEN',title:'科室ID',width:50}  
					]],
					pagination:true,
					onSelect:function(index,rowData){
						console.log("index="+index+",rowData=",rowData);
					},
					isCombo:true,
					enableNumberEvent:true,
					minQueryLen:1
				});
			})
```

### 禁用启用，是否只读切换示例

```javascript
$(function(){
				$("#user").lookup({
					url:'getUser',
					mode:'remote',
					idField:'HIDDEN',
					textField:'Description',
					columnsLoader:function(){
						return [[  
							{field:'Code',title:'工号',width:150},  
							{field:'Description',title:'姓名',width:150},  
							{field:'HIDDEN',title:'用户ID',width:100}  
						]];
					},
					pagination:true,
					
					panelWidth:446,
					
				});
				$('.op-btn').linkbutton({
					onClick:function(){
						if($(this).text()=="禁用") $("#user").lookup("disable");
						if($(this).text()=="启用") $("#user").lookup("enable");
						if($(this).text()=="只读") $("#user").lookup("readonly",true);
						if($(this).text()=="取消只读") $("#user").lookup("readonly",false);
					
					}
				})
			});
```

### onBeforeShowPanel事件

```javascript
$(function(){
				$("#group2").lookup({
					width:200,
					
					panelWidth:446,
					
					url:'getGroup',
					mode:'remote',
					idField:'HIDDEN',
					textField:'Description',
					columns:[[  
						{field:'Description',title:'安全组名称',width:300},  
						{field:'HIDDEN',title:'安全组ID',width:100}  
					]],
					pagination:true,
					onSelect:function(index,rowData){
						console.log("index="+index+",rowData=",rowData);
					},
					onBeforeShowPanel:function(){
						if ($(this).val()=="ad") return false;
					}
				});
			})
```

### selectRowRender方法

```javascript
$(function(){
				$("#group3").keydown(function(e){
					if (e.keyCode ==13){
						setTimeout(function(){$("#group9").focus();},1200);
					}
				});
				$("#group3").lookup({
					width:200,
					
					panelWidth:446,
					
					url:'getGroup',
					mode:'remote',
					idField:'HIDDEN',
					textField:'Description',
					columns:[[  
						{field:'Description',title:'安全组名称',width:300},  
						{field:'HIDDEN',title:'安全组ID',width:100}  
					]],
					pagination:true,
					onSelect:function(index,rowData){
						console.log("index="+index+",rowData=",rowData);
					},
					selectRowRender:function(row){
						console.log('selectRowRender',row);
						if (!row)  return '<p style="margin:0;padding:10px;">无提示信息</p>';
						if (row.Description.length>12){ //安全组名字长于10 显示提示信息
							return '<p style="margin:0;padding:10px;"><span style="font-weight:bold;"></sapn>安全组<span style="color:red;">'+row.Description+',此安全组字符超过12字符，这里可以显示当前行记录的说明信息。此处也测试超长时滚动条问题。,此安全组字符超过12字符，这里可以显示当前行记录的说明信息。此处也测试超长时滚动条问题。,此安全组字符超过12字符，这里可以显示当前行记录的说明信息。此处也测试超长时滚动条问题。</span>';
						}if (row.Description.length==5){ //安全组名字长于10 显示提示信息
							return '<p style="margin:0;padding:10px;display:block;height:40px;overflow:auto;"><span style="font-weight:bold;"></sapn>安全组<span style="color:red;">'+row.Description+',此安全组字符为5字符，这里可以显示当前行记录的说明信息。此处也测试超长时滚动条问题。,此安全组字符为5字符，这里可以显示当前行记录的说明信息。此处也测试超长时滚动条问题。,此安全组字符超过为5字符，这里可以显示当前行记录的说明信息。此处也测试超长时滚动条问题。</span>';
						}else if (row.Description.length>10){ //安全组名字长于10 显示提示信息
							return '<p style="margin:0;padding:10px;"><span style="font-weight:bold;"></sapn>安全组<span style="color:red;">'+row.Description+',此安全组字符超过10字符，这里可以显示当前行记录的说明信息。此处也测试超长时滚动条问题</span>';
						}else if(row.Description.length>8){
							return '<p style="margin:0;padding:10px;"><span style="font-weight:bold;"></sapn>安全组<span style="color:red;">'+row.Description+'</span>';
						}else{
							return '<p style="margin:0;padding:10px;">无提示信息</p>';
						}
						
					}
				});
				$("#group9").lookup({
					width:200,panelWidth:460,
					url:'getGroup',
					mode:'remote',
					idField:'HIDDEN',
					textField:'Description',
					columns:[[  
						{field:'Description',title:'安全组名称',width:200},  
						{field:'HIDDEN',title:'安全组ID',width:50}  
					]],
					pagination:true,
					onSelect:function(index,rowData){
						console.log("index="+index+",rowData=",rowData);
					},
					selectRowRender:function(row){
						console.log('selectRowRender',row);
						if (!row) return '<p style="margin:0;padding:10px;">无提示信息</p>';
						if (row.Description.length>12){ //安全组名字长于10 显示提示信息
							return '<p style="margin:0;padding:10px;"><span style="font-weight:bold;"></sapn>安全组<span style="color:red;">'+row.Description+',此安全组字符超过12字符，这里可以显示当前行记录的说明信息。此处也测试超长时滚动条问题。,此安全组字符超过12字符，这里可以显示当前行记录的说明信息。此处也测试超长时滚动条问题。,此安全组字符超过12字符，这里可以显示当前行记录的说明信息。此处也测试超长时滚动条问题。</span>';
						}if (row.Description.length==5){ //安全组名字长于10 显示提示信息
							return '<p style="margin:0;padding:10px;display:block;height:40px;overflow:auto;"><span style="font-weight:bold;"></sapn>安全组<span style="color:red;">'+row.Description+',此安全组字符为5字符，这里可以显示当前行记录的说明信息。此处也测试超长时滚动条问题。,此安全组字符为5字符，这里可以显示当前行记录的说明信息。此处也测试超长时滚动条问题。,此安全组字符超过为5字符，这里可以显示当前行记录的说明信息。此处也测试超长时滚动条问题。</span>';
						}else if (row.Description.length>10){ //安全组名字长于10 显示提示信息
							return '<p style="margin:0;padding:10px;"><span style="font-weight:bold;"></sapn>安全组<span style="color:red;">'+row.Description+',此安全组字符超过10字符，这里可以显示当前行记录的说明信息。此处也测试超长时滚动条问题</span>';
						}else if(row.Description.length>8){
							return '<p style="margin:0;padding:10px;"><span style="font-weight:bold;"></sapn>安全组<span style="color:red;">'+row.Description+'</span>';
						}else{
							return '<p style="margin:0;padding:10px;">无提示信息</p>'
						}
						
					}
				});
			})
```

### 放大镜(lookup)组件，属性、事件、方法

| 属性 | 说明 | 默认值 |  |
| --- | --- | --- | --- |
| loadMsg | 当数据网格（datagrid）加载远程数据时显示的消息。 | null |  |
| idField | id 的字段名。 | null |  |
| textField | 显示在文本框中的 text 字段名。 | null |  |
| mode | 定义当文本改变时如何加载数据网格（datagrid）数据。如果组合网格（combogrid）从服务器加载就设置为 'remote'。当设置为 'remote' 模式时，用户输入的值将会被作为名为 'q' 的 http 请求参数发送到服务器，以获取新的数据。 | 'remote' |  |
| filter | function(q, row) 定义当 'mode' 设置为 'local' 时如何选择本地数据。返回 true 则选择该行。 |  |  |
| width | 组件的宽度。 | 'auto' |  |
| panelWidth | 下拉面板宽度 | null |  |
| panelHeight | 下拉面板高度 | 200 |  |
| selectOnNavigation | 定义当通过键盘导航项目时是否选择项目 | false |  |
| editable | 定义用户是否可以往文本域中直接输入文字 | true |  |
| disabled | 是否禁用 | false |  |
| readonly | 是否只读 | false |  |
| hasDownArrow | 是否有下拉按钮 | true |  |
| delay | 从最后一个键的输入事件起，延迟进行搜索。 | 200 |  |
| isCombo | 是否输入字符即触发事件，进行搜索 | false |  |
| minQueryLen | isCombo为true时，可以搜索要求的字符最小长度 | 0 |  |
| queryOnSameQueryString | 在查询条件和上一次一致时，回车或点击下拉按钮是否重新搜索 | true |  |
| columnsLoader | 函数类型,返回colums。见示例三 | null |  |
| enableNumberEvent | 是否开启数字选行功能,在isCombo为true时生效。 2020-02-14 见示例二 | false |  |
| forceFocus | 加载数据后光标是否到放大镜输入框中。 2020-06-22 ,于便使用toolbar | true |  |
| singleRequest | 是否只保留最后一个请求。true时,只保留最后一个请求 2021-11-10 | true |  |
| panelHeightFix | 是否自适应高度。在可见区域内最大化显示放大镜高度。 | false | 配置了selectRowRender时,值自动为true |
| panelMaxHeight | 自适应高度时,最大高度 | 500 |  |
| panelMinHeight | 自适应高度时,最小高度 | 160 |  |
| rowSummaryHeight | 行提示区域高度 | 0 | 默认为0, 为0时区域高度为第一次提示信息区域的高度 20230705 |
| 事件名 | 说明 | 入参 |  |
| onBeforeShowPanel | 下拉面板展开前触发，若返回值是false,则阻止展开 | null |  |
| onShowPanel | 当下拉面板显示的时候触发. | null |  |
| onHidePanel | 隐藏时触发 | null |  |
| selectRowRender | 当高亮一行时调用此function，入参为当前行数据row,返回提示内容的html 复现原extjs的lookup | row |  |
| 方法名 | 说明 | 入参 | 返回值 |
| options | 拿到配置项对象 |  | 配置对象 |
| grid | 返回数据网格（datagrid）对象 此方法虽存在，但是获取的grid未必对应此lookup 因为所有的lookup都使用一个grid |  | jquery对象 |
| setText | 设置文本框的值 | text | jquery对象 |
| getText | 获取输入框的文本 |  |  |
| setValue | 设置放大镜的值,只设置值,不校验值。选中行后组件会通过此方法把idField对应的值记录起来。 2020-02-12 | text |  |
| getValue | 获得放大镜的值。 2020-02-12 | text |  |
| clear | 清空组件值 |  | jquery对象 |
| reset | 重置组件的值 |  | jquery对象 |
| resize | 调整组件宽度 | width | jquery对象 |
| showPanel | 显示面板 |  | jquery对象 |
| hidePanel | 隐藏面板 |  | jquery对象 |
| disable | 禁用组件 |  | jquery对象 |
| enable | 启用组件 |  | jquery对象 |
| readonly | 启用/禁用只读模式。 | mode(true\|false) | jquery对象 |
| isValid | 返回验证结果 |  | true\|false |
| fixPanelTLWH | 重计算放大镜弹出框位置及大小，方法继承自comboq。 2020-02-12 | 无 | 无 |

```javascript
$(function(){
			$('.demo-exp-code.entry-content').prev('h3').each(function(ind){
				$(this).text('一二三四五六七八九十'.charAt(ind)+'、'+ $(this).text());
			})
		})
```
