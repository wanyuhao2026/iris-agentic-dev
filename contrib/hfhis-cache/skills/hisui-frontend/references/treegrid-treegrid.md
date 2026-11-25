# 树形表格(treegrid)

- 官方页面标题：树形网格
- 分类：表格及树
- 来源：http://hisui.cn/api/treegrid/treegrid.html
- 官网更新时间：Tue, 12 Mar 2024 03:51:35 GMT
- 提取时间：2026-07-25T13:18:46+00:00

> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。

## 本页目录

- 树形网格
  - 说明:
  - 如：
  - 基本使用
  - 报表 合并列
  - 1.3.6树组件扩展的相关配置、事件、方法

## 树形网格

### 说明:

树形网格（TreeGrid）从数据网格（DataGrid）继承，但是允许在行之间存在父/子节点关系。许多属性继承至数据网格（DataGrid），可以用在树形网格（TreeGrid）中。为了使用树形网格（TreeGrid），用户必须定义 'treeField' 属性，指明哪个字段作为树节点。

### 如：

### 基本使用

```javascript
function log(o){
			$('#logspan').html("").html(JSON.stringify(o));
		}
```

```html
<table title="菜单管理" class='hisui-treegrid' id="treegrid2"
	data-options="fit:true,idField:'id',headerCls:'panel-header-gray',iconCls:'icon-paper',treeField:'menuName',rownumbers:true,autoSizeColumn:false,checkbox:true,toolbar:[{
		iconCls: 'icon-paper-table',text:'获得所有选中',
		handler: function(){
			debugger;
			var o = $('#treegrid2').treegrid('getCheckedNodes','checked');
			log(o);
		}
	},{
		iconCls:'icon-all-unselect',
		text:'取消选中【医生站】',
		handler:function(){
			log($('#treegrid2').treegrid('uncheckNode',2));
		}
	},{
		iconCls:'icon-all-select',
		text:'选中【医生站】',
		handler:function(){
			log($('#treegrid2').treegrid('checkNode',2));
		}
	}]">
	<thead>
		<tr>
			<th field="menuName" width="160" align="left">菜单名<th>
			<th field="menuLink" width="100" align="right">链接<th>
			<th field="menuCode" width="160" align="right">代码<th>
			<th field="menuSort" width="60" align="right">顺序<th>
			<th field="menuEx" width="160" align="right">表达式<th>
		</tr>
	<thead>
</table>
```

```javascript
function log(o){
	$('#logspan').html("");
	for (var i=0,len=arguments.length;i<len;i++){
		if(i>0) $('#logspan').append('\n,');
		$('#logspan').append(JSON.stringify(arguments[i],null,4));
	}
}
$(function(){
	var treegridObj = $HUI.treegrid("#treegrid2",{
		onCheckNode:function(row,checked){
			log(row,"checked="+checked);
			
		},
		onBeforeCheckNode:function(row,checked){
			log(row,"checked="+checked);
			
		}
	});
	treegridObj.loadData({"total":19,"rows":[
		{"id":1,"menuName":"配置管理","menuLink":"cfgmgr.csp","menuCode":"cfgManager","menuSort":"","menuEx":""},
		{"id":11,"menuName":"菜单管理","menuLink":"menumgr.csp","menuCode":"menuManager","menuSort":1,"menuEx":"&name=1","_parentId":1},
		{"id":12,"menuName":"用户管理","menuLink":"usermgr.csp","menuCode":"userManager","menuSort":2,"menuEx":"&name=1","_parentId":1},
		{"id":13,"menuName":"安全管理","menuLink":"groupmgr.csp","menuCode":"groupManager","menuSort":3,"menuEx":"&name=1","_parentId":1},
		
		{"id":2,"menuName":"医生站","menuLink":"docmgr.csp","menuCode":"docManager","menuSort":"","menuEx":""},
		{"id":14,"menuName":"便民门诊","menuLink":"regmgr.csp","menuCode":"regManager","menuSort":1,"menuEx":"&name=1","_parentId":2},
		{"id":15,"menuName":"诊断录入","menuLink":"diagmgr.csp","menuCode":"diagManager","menuSort":2,"menuEx":"&name=1","_parentId":2},
		{"id":16,"menuName":"医嘱录入","menuLink":"ordermgr.csp","menuCode":"orderManager","menuSort":3,"menuEx":"&name=1","_parentId":2},
		{"id":17,"menuName":"病历书写","menuLink":"eprmgr.csp","menuCode":"eprManager","menuSort":4,"menuEx":"&name=1","_parentId":2},
		{"id":18,"menuName":"病人列表","menuLink":"patlist.csp","menuCode":"patlistManager","menuSort":5,"menuEx":"&name=1","_parentId":2},
		{"id":19,"menuName":"历次就诊","menuLink":"admlist.csp","menuCode":"admlistManager","menuSort":6,"menuEx":"&name=1","_parentId":2}
	]});
});
```

### 报表 合并列

```html
<table title="报表" class="hisui-treegrid" id="treegrid1" style="width:880px;height:400px"
data-options="method: 'get',
	showFooter: true,
	idField: 'id',
	treeField: 'region',
	iconCls:'icon-paper',
	headerCls:'panel-header-gray'
">
	<thead frozen="true">
		<tr>
			<th field="region" width="200">地区</th>
		</tr>
	</thead>
	<thead>
		<tr>
			<th colspan="4">2009</th>
			<th colspan="4">2010</th>
		</tr>
		<tr>
			<th field="f1" width="80" align="right">第一季度</th>
			<th field="f2" width="80" align="right">第二季度</th>
			<th field="f3" width="80" align="right">第三季度</th>
			<th field="f4" width="80" align="right">第四季度</th>
			<th field="f5" width="80" align="right">第一季度</th>
			<th field="f6" width="80" align="right">第二季度</th>
			<th field="f7" width="80" align="right">第三季度</th>
			<th field="f8" width="80" align="right">第四季度</th>
		</tr>
	</thead>
</table>
```

```javascript
$(function(){
	var treegridObj = $HUI.treegrid("#treegrid1",{});
	treegridObj.loadData({"total":9,
		"rows":[
			{"id":1,"region":"Wyoming"},
			{"id":11,"region":"Albin","f1":2000,"f2":1800,"f3":1903,"f4":2183,"f5":2133,"f6":1923,"f7":2018,"f8":1838,"_parentId":1},
			{"id":12,"region":"Canon","f1":2000,"f2":1800,"f3":1903,"f4":2183,"f5":2133,"f6":1923,"f7":2018,"f8":1838,"_parentId":1},
			{"id":13,"region":"Egbert","f1":2000,"f2":1800,"f3":1903,"f4":2183,"f5":2133,"f6":1923,"f7":2018,"f8":1838,"_parentId":1},
			{"id":2,"region":"Washington"},
			{"id":21,"region":"Bellingham","f1":2000,"f2":1800,"f3":1903,"f4":2183,"f5":2133,"f6":1923,"f7":2018,"f8":1838,"_parentId":2},
			{"id":22,"region":"Chehalis","f1":2000,"f2":1800,"f3":1903,"f4":2183,"f5":2133,"f6":1923,"f7":2018,"f8":1838,"_parentId":2},
			{"id":23,"region":"Ellensburg","f1":2000,"f2":1800,"f3":1903,"f4":2183,"f5":2133,"f6":1923,"f7":2018,"f8":1838,"_parentId":2},
			{"id":24,"region":"Monroe","f1":2000,"f2":1800,"f3":1903,"f4":2183,"f5":2133,"f6":1923,"f7":2018,"f8":1838,"_parentId":2}
		],
		"footer":[
			{"region":"Total","f1":14000,"f2":12600,"f3":13321,"f4":15281,"f5":14931,"f6":13461,"f7":14126,"f8":12866}
		]
	});
});
```

### 1.3.6树组件扩展的相关配置、事件、方法

| 属性 | 说明 | 默认值 |  |
| --- | --- | --- | --- |
| checkbox | 可勾选树 | false | 从1.5代码修改 |
| 方法名 | 说明 | 默认值 |  |
| getCheckedNodes | 获得所有选中 | checked |  |
| uncheckNode | 取消选中 | id |  |
| checkNode | 选中 | id |  |
| ... | 可查看1.5的treegrid |  |  |
| 事件名 | 说明 | 默认值 |  |
| onCheckNode | 勾选后 | row,checked |  |
| onBeforeCheckNode | 勾选前，返回false可取消动作 | row,checked |  |
| ... | 可查看1.5的treegrid |  |  |

```javascript
$(function(){
            $('.demo-exp-code.entry-content').prev('h3').each(function(ind){
                $(this).text('一二三四五六七八九十'.charAt(ind)+'、'+ $(this).text());
            })
        })
		$.parser.onComplete = function(){
			tree1OnComplete();
			tree2OnComplete();
		}
```
