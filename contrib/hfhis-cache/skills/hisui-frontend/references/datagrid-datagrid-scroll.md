# 表格滚动加载(scroll)

- 官方页面标题：滚动列表示例
- 分类：表格及树
- 来源：http://hisui.cn/api/datagrid/datagrid.scroll.html
- 官网更新时间：Tue, 12 Mar 2024 03:51:26 GMT
- 提取时间：2026-07-25T13:18:46+00:00

> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。

## 本页目录

- 数据网格-滚动加载
  - 说明:
  - 本地数据。一次加载所有数据，拉动时渲染要显示的行数据
  - 远程数据。拉动时一次加载一页数据，渲染要显示的行数据

## 数据网格-滚动加载

### 说明:

当用户不希望使用翻页条时,且加载数据记录大量条数时，可以使用些插件

```html
//引入hisui后再引入插件
<script type="text/javascript" src="../../dist/plugin/datagrid-scrollview.js"></script>
```

### 本地数据。一次加载所有数据，拉动时渲染要显示的行数据

```html
<div class="demo-exp-code entry-content use-prettyprint" prettyprintfor="#mydg1codehtml">
<table class="hisui-datagrid" data-options="url:null,toolbar:[],idField:'no',fitColumns:true,headerCls:'panel-header-gray',iconCls:'icon-paper',view:scrollview,rownumbers:true,singleSelect:false,autoRowHeight:false,pageSize:50,pagination:true" id="dg" style="width:800px;height:300px" title="病人列表">
<thead>
<tr>
<th data-options="field:'checkOrd',checkbox:'true',align:'center',width:30,auto:false">选择</th>
<th data-options="field:'no',width:20">编码</th>
<th data-options="field:'name',width:20">姓名</th>
<th data-options="field:'loc',width:20">科室</th>
<th data-options="field:'diag',width:20">诊断</th>
<th data-options="field:'code',width:20">代码</th>
<th data-options="field:'stdate',width:20">日期</th>
</tr>
</thead>
</table>
</div>
```

```javascript
$.parser.onComplete = function(){
			
			var rows = [];
			for(var i=1; i<=8002; i++){
				var amount = Math.floor(Math.random()*1000);
				var price = Math.floor(Math.random()*1000);
				rows.push({
					no: 'NO '+i,
					name: '张 '+i,
					loc: '内'+i+'科',
					diag:'diag '+i,
					code: i,
					stdate: $.fn.datebox.defaults.formatter(new Date())
				});
			}
			/*$('#dg').datagrid({
				detailFormatter:function(rowIndex, rowData){
					return '<table><tr>' +
						'<td style="border:0;padding-right:10px">' +
						'<p>姓名: ' + rowData.name + '</p>' +
						'<p>科室: ' + rowData.loc + '</p>' +
						'</td>' +
						'<td style="border:0">' +
						'<p>日期: ' + rowData.stdate + '</p>' +
						'<p>序号: ' + rowData.no + '</p>' +
						'</td>' +
						'</tr></table>';
				}	
			})*/
			$("#dg").datagrid('loadData', rows);
		};
```

### 远程数据。拉动时一次加载一页数据，渲染要显示的行数据

```html
<div class="demo-exp-code entry-content use-prettyprint" prettyprintfor="#mydg2codehtml"> <!--,view:scrollview-->
<table class="hisui-datagrid" data-options="url:'getpatdata.jsp',toolbar:[],view:scrollview,fitColumns:true,headerCls:'panel-header-gray',iconCls:'icon-paper',rownumbers:true,singleSelect:true,autoRowHeight:false,pageSize:50,pagination:false" id="dg1" style="width:800px;height:300px" title="病人列表">
<thead>
<tr>
<th data-options="field:'no',width:20">编码</th>
<th data-options="field:'name',width:20">姓名</th>
<th data-options="field:'loc',width:20">科室</th>
<th data-options="field:'diag',width:20">诊断</th>
<th data-options="field:'code',width:20">代码</th>
<th data-options="field:'stdate',width:20">日期</th>
</tr>
</thead>
</table>
</div>
```

```javascript
$(function(){
			$('#dg1').datagrid({
				detailFormatter:function(rowIndex, rowData){
					return '<table><tr>' +
						'<td style="border:0;padding-right:10px">' +
						'<p>姓名: ' + rowData.name + '</p>' +
						'<p>科室: ' + rowData.loc + '</p>' +
						'</td>' +
						'<td style="border:0">' +
						'<p>就诊日期: ' + rowData.stdate + '</p>' +
						'<p>序号: ' + rowData.no + '</p>' +
						'</td>' +
						'</tr></table>';
				}	
			})
		});
		Mock.setup({
			timeout: '100-300' //后台时间
		})
		Mock.mock('getpatdata.jsp',{
			'rows|50':[{  //重复50次
				'no|+1':1, //从1开始,每次加1
				'name':/[赵钱孙李吴郑王张周汪韩苏辛][志三亮会强子祥云四土秀士二章隆财明勇艳][志三亮会强子秀祥云四土士二艳书轩章财明勇]/,
				'loc':/内[一二三]科/,
				'diag':'diag '+'@no',
				'code':/[A-Z][A-Z][A-Z][1-9][1-9][1-9]/, //占位符
				'stdate':/(2016|2017|2018)-(1[0-2]|0[1-9])-(2[0-9]|1[0-9]|0[1-9])/
			}],"total":8000
		});
```

| 属性 | 说明 | 默认值 |  |
| --- | --- | --- | --- |
| detailFormatter | 定义格式显示行数据函数。function(rowIndex, rowData){return html;} | null |  |
