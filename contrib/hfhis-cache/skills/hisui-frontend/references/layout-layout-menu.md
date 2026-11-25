# 左侧菜单布局

- 官方页面标题：布局菜单
- 分类：测试
- 来源：http://hisui.cn/api/layout/layout-menu.html
- 官网更新时间：Tue, 12 Mar 2024 03:51:29 GMT
- 提取时间：2026-07-25T13:18:46+00:00

> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。

```javascript
$(function(){
            var myMenuData=[];
            for(var i=1;i<4;i++){
                var item={id:i+'',text:'一级菜单'+i,state:'open',children:[]}
                for (var j=1;j<(7-i);j++){
                    item.children.push({
                        id:i+'-'+j,
                        text:'二级菜单'+i+'-'+j,
                        state:'open',
                        attributes:{
                            url:'./layout-query-inside.html',
                            count:(i==1&&j<=2)?j:0
                        }
                    })

                }
                myMenuData.push(item);
                
            }
            Mock.mock("getMenuData", function( options ){
                console.log("getMenuData请求开始了",options);
                var obj=myMenuData
                console.log("getMenuData请求结束了，结果",obj);
                return obj;

            })

            var addOrOpenTab=function(tabOpts){
                var tab=$('#menu-tabs').tabs('getTab',tabOpts.title)
                if (tab){
                    $('#menu-tabs').tabs('select',tabOpts.title)
                }else{
                    var myTabOpts={
                        id:'mmt-'+tabOpts.code,
                        title:tabOpts.title,
                        content:'<iframe name="menu-tabs-frame-'+tabOpts.code+'" src="'+tabOpts.url+'" style="display:block;width: 100%;height: 100%; margin:0; border: 0;" scroll="auto"></iframe>',
                        closable:true
                    }
                    $('#menu-tabs').tabs('add',myTabOpts)

                }
            }
            
            $('#menu-tabs').tabs('options').onSelect=function(title,ind){
                var tab=$('#menu-tabs').tabs('getTab',ind);
                if(tab && tab.attr('id') && tab.attr('id').indexOf('mmt-')==0) {
                    var menuid=tab.attr('id').slice(4);
                    $('#mm').menutree('selectById',menuid);
                }

            }
            
            $('#mm').menutree({
                url:'getMenuData',
                prompt:'检索菜单',
                fit:true,
				rootCollapsible:false,
                collapsible:location.href.indexOf('collapsible')>-1,
                onMenuClick: function (node) {
                    console.log(node)
                    if(node && node.attributes &&node.attributes.url) {
                        addOrOpenTab($.extend({
                            code:node.id,
                            title:node.text
                        },node.attributes))
                    }
                },onPanelCollapse:function(width){
						//$.messager.popover({msg:'您折叠了菜单树,宽度为'+width,type:'info'})

                        $('body').layout('panel','west').panel('resize',{width:width+10});
                        $('body').layout('resize');
                },onPanelExpand:function(width){
                    //$.messager.popover({msg:'您展开了菜单树,宽度为'+width,type:'info'})
                    $('body').layout('panel','west').panel('resize',{width:width+10});
                        $('body').layout('resize');
                }
            })
            
            


        })
```
