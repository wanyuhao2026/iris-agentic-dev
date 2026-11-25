# 菜单树(menutree)

- 官方页面标题：菜单树
- 分类：表格及树
- 来源：http://hisui.cn/api/menutree/menutree.html
- 官网更新时间：Fri, 21 Feb 2025 02:27:07 GMT
- 提取时间：2026-07-25T13:18:46+00:00

> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。

## 本页目录

- 菜单树
  - 说明:
  - 如：

## 菜单树

### 说明:

基于tree组件实现的菜单树组件menutree

### 如：

```javascript
var hasLevel4=false; //是否有更多层级菜单

			var myMenuData=[];
            for(var i=1;i<4;i++){
                var item={id:i+'',text:'一级菜单'+i,state:'open',iconCls:'icon-outInstc-mgr',children:[],attributes:{
					engText:'Level1 Menu '+i
				}}
                for (var j=1;j<(7-i);j++){
                    item.children.push({
                        id:i+'-'+j,
                        text:'二级菜单'+i+'-'+j,
						iconCls:'icon-outInstc-mgr',
                        state:'open',
                        attributes:{
                            url:'./layout-query.html',
                            count:(i==1&&j<=2)?j:0
							,engText:'Level2 Menu '+i+'-'+j,
                        }
                    })

                }
                myMenuData.push(item);
                
            }
			var myMenuData2=[];
            for(var i=1;i<4;i++){
                var item={id:i+'',text:'一级菜单'+i,state:'open',iconCls:'icon-outInstc-mgr',children:[],attributes:{
					engText:'Level1 Menu '+i
				}}
                for (var j=1;j<(7-i);j++){
					var item2={
                        id:i+'-'+j,
                        text:'二级菜单'+i+'-'+j,
                        state:'open',
						iconCls:'icon-outInstc-mgr',
                        attributes:{
                            url:'./layout-query.html',
                            count:(i==1&&j<=2)?j:0
							,engText:'Level2 Menu '+i+'-'+j,
                        }
                    }
					if (j==4||j==5) {
						item2.attributes.url='';
						item2.children=[];
						for(var k=1;k<4;k++) {
							var item3={
								id:i+'-'+j+'-'+k,
								text:'三级菜单'+i+'-'+j+'-'+k,
								state:'open',
								attributes:{
									url:'./layout-query.html',
									count:4
								}
							}

							if (hasLevel4 && (k==2 ||k==3)) { //四级菜单
								item3.attributes.url='';
								item3.children=[];
								for(var o=1;o<4;o++) {
									var item4={
										id:i+'-'+j+'-'+k+'-'+o,
										text:'四级菜单'+i+'-'+j+'-'+k+'-'+o,
										state:'open',
										attributes:{
											url:'./layout-query.html',
											count:4
										}
									}
									if (o==2 ||o==3) { //wu级菜单
										item4.attributes.url='';
										item4.children=[];
										for(var p=1;p<4;p++) {
											var item5={
												id:i+'-'+j+'-'+k+'-'+o+'-'+p,
												text:'五级菜单'+i+'-'+j+'-'+k+o+'-'+p,
												state:'open',
												attributes:{
													url:'./layout-query.html',
													count:4
												}
											}
											item4.children.push(item5)
										}
										

									}
									item3.children.push(item4)
								}
								

							}



							item2.children.push(item3)
						} 
						

					}

                    item.children.push(item2)

                }
                myMenuData2.push(item);
                
            }
            Mock.mock("getMenuData", function( options ){
                var obj=myMenuData
                return obj;

            })
            Mock.mock("getMenuData2", function( options ){
                var obj=myMenuData2
                return obj;

            })
```

```javascript
$(function () {
				///根节点全展开（一般用于只有两级菜单的）  不可向左收缩
				$('#mm').menutree({
					url:'getMenuData',
					height:600,
					prompt:'检索菜单',
					rootCollapsible:false,
					collapsible:false
					,onMenuClick:function(node){
						$.messager.popover({msg:'您点击了菜单'+node.text,type:'info'})
					}
					,searchFields:'engText'

				})
				///根节点全展开（一般用于只有两级菜单的）  不可向左收缩
				$('#mm1').menutree({
					url:'getMenuData',
					height:600,
					prompt:'检索菜单',
					rootCollapsible:true,
					collapsible:false
					,onMenuClick:function(node){
						$.messager.popover({msg:'您点击了菜单'+node.text,type:'info'})
					}
					,searchable:false

				})
				/// 根节点全展开（一般用于只有两级菜单的）  可向左收缩 （炫彩默认给带上标题）
				$('#mm2').menutree({
					url:'getMenuData',
					height:600,
					prompt:'检索菜单',
					rootCollapsible:false,
					collapsible:true
					,onMenuClick:function(node){
						$.messager.popover({msg:'您点击了菜单'+node.text,type:'info'})
					}
					,searchFields:'engText'
				})

				///根节点不全展开（一般用于有3级等更多菜单的） 一般需要配置标题  不可向左收缩
				$('#mm3').menutree({
					url:'getMenuData2',
					title:'菜单',
					height:600,
					prompt:'检索菜单',
					rootCollapsible:true,
					collapsible:false
					,onMenuClick:function(node){
						$.messager.popover({msg:'您点击了菜单'+node.text,type:'info'})
					}
					,searchFields:'engText'
				})
				///根节点不全展开（一般用于有3级等更多菜单的） 一般需要配置标题  可向左收缩
				$('#mm4').menutree({
					url:'getMenuData2',
					title:'菜单',
					height:600,
					prompt:'检索菜单',
					rootCollapsible:true,
					collapsible:true
					,onMenuClick:function(node){
						$.messager.popover({msg:'您点击了菜单'+node.text,type:'info'})
					}
					,searchFields:'engText'
					,onPanelCollapse:function(width){
						$.messager.popover({msg:'您折叠了菜单树,宽度为'+width,type:'info'})
					},onPanelExpand:function(width){
						$.messager.popover({msg:'您展开了菜单树,宽度为'+width,type:'info'})
					}
				})
			})
```

| 属性 | 说明 | 默认值 |  |
| --- | --- | --- | --- |
| url | 获取远程数据的URL |  |  |
| collapsible | 是否可向左折叠 | false |  |
| rootCollapsible | 根节点是否可折叠 | true |  |
| fit | 自适应父容器大小 | false |  |
| width | 宽度 |  |  |
| height | 高度 | 'auto' |  |
| title | 标题 | '' | 仅炫彩风格会显示 |
| onlyOneExpanded | 同级菜单是否只能展开一个 | true |  |
| searchable | 是否显示搜索框 | true |  |
| searchFields | 除text字段外用于查询匹配的字段 | '' |  |
| animate | 动画 | true |  |
| 方法名 | 说明 | 入参 | 返回值 |
| selectById | 根据id字段选择节点 | id |  |
| getTree | 获取树的jq对象 |  | jq对象 |
| findNode | 根据id获取树节点 | id | node节点对象 |
| 事件名 | 说明 | 入参 | 类型 |
| onMenuClick | 菜单点击事件 | node | 树的node节点对象 |
| onMenuGroupClick | 菜单组点击事件 | node | 树的node节点对象 |
| onPanelCollapse | 向左折叠事件 | width | 折叠后的宽度 |
| onPanelExpand | 向右展开事件 | width | 展开后的宽度 |
