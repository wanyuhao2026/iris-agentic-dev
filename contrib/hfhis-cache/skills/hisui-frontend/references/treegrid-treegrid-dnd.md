# 拖拽树形表格(treegrid.dnd)

- 官方页面标题：树形网格
- 分类：测试
- 来源：http://hisui.cn/api/treegrid/treegrid.dnd.html
- 官网更新时间：Tue, 12 Mar 2024 03:51:35 GMT
- 提取时间：2026-07-25T13:18:46+00:00

> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。

## 本页目录

- 树形网格
  - 说明:
  - 如：
  - 1.3.6树组件扩展的相关配置、事件、方法

## 树形网格

### 说明:

树形网格（TreeGrid）从数据网格（DataGrid）继承，但是允许在行之间存在父/子节点关系。许多属性继承至数据网格（DataGrid），可以用在树形网格（TreeGrid）中。为了使用树形网格（TreeGrid），用户必须定义 'treeField' 属性，指明哪个字段作为树节点。

### 如：

### 1.3.6树组件扩展的相关配置、事件、方法

```javascript
var data = [{
			"id":1,
			"name":"C",
			"size":"",
			"date":"02/19/2010",
			"children":[{
				"id":2,
				"name":"Program Files",
				"size":"120 MB",
				"date":"03/20/2010",
				"children":[{
					"id":21,
					"name":"Java",
					"size":"",
					"date":"01/13/2010",
					"state":"closed",
					"children":[{
						"id":211,
						"name":"java.exe",
						"size":"142 KB",
						"date":"01/13/2010"
					},{
						"id":212,
						"name":"jawt.dll",
						"size":"5 KB",
						"date":"01/13/2010"
					}]
				},{
					"id":22,
					"name":"MySQL",
					"size":"",
					"date":"01/13/2010",
					"state":"closed",
					"children":[{
						"id":221,
						"name":"my.ini",
						"size":"10 KB",
						"date":"02/26/2009"
					},{
						"id":222,
						"name":"my-huge.ini",
						"size":"5 KB",
						"date":"02/26/2009"
					},{
						"id":223,
						"name":"my-large.ini",
						"size":"5 KB",
						"date":"02/26/2009"
					}]
				}]
			},{
				"id":3,
				"name":"eclipse",
				"size":"",
				"date":"01/20/2010",
				"children":[{
					"id":31,
					"name":"eclipse.exe",
					"size":"56 KB",
					"date":"05/19/2009"
				},{
					"id":32,
					"name":"eclipse.ini",
					"size":"1 KB",
					"date":"04/20/2010"
				},{
					"id":33,
					"name":"notice.html",
					"size":"7 KB",
					"date":"03/17/2005"
				}]
			}]
		}]
		function onDropHandler(t,s,p){

			if(p=='append'){
				$(this).treegrid('enableDnd', t?t.id:null);
			}else{
				var par=$('#treegrid-dnd').treegrid('getParent',t.id)
				if (par && par.id) {
					$(this).treegrid('enableDnd', par?par.id:null);
				}
							
			}
		}
		$(function(){
			$('#toggleDnd').click(function(){
				if ( $(this).text()=="禁止拖拽" ){
					$('#treegrid-dnd').treegrid("disableDnd");
					$(this).find('.l-btn-text').text("开启拖拽");
				}else{
					$('#treegrid-dnd').treegrid("enableDnd");
					$(this).find('.l-btn-text').text("禁止拖拽");
				}
			})
		})
```
