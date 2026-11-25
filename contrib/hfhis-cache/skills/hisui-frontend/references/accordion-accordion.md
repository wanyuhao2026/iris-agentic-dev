# 手风琴(accordion)

- 官方页面标题：手风琴2223
- 分类：布局
- 来源：http://hisui.cn/api/accordion/accordion.html
- 官网更新时间：Tue, 12 Mar 2024 03:51:23 GMT
- 提取时间：2026-07-25T13:18:46+00:00

> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。

## 手风琴

### 说明:

折叠面板（Accordion）俗称手风琴，其包含一系列的面板（panel）。 所有面板（panel）的头部（header）都是可见的，但是一次仅仅显示一个面板（panel）的 body 内容。 当用户点击面板（panel）的头部（header）时，该面板（panel）的 body 内容将可见，同时其他面板（panel）的 body 内容将隐藏不可见。

### 如：

### Accordion for jQuery

```html
<!--蓝色手风琴-->
<div class="hisui-accordion" style="width:200px;height:450px;float:left;" data-options="border:false">  
    <div title="等待区" data-options="iconCls:'icon-w-update'" style="overflow:auto;padding:10px;background-color:#EBEBEB;">   
        <h3 style="color:#0099FF;">Accordion for jQuery</h3>   
        <p>Accordion is a part of easyui framework for jQuery.     
        It lets you define your accordion component on web page more easily.</p>   
    </div>   
    <div title="住院区" data-options="iconCls:'icon-w-import',selected:true" style="padding:10px;background-color:#EBEBEB;">   
        content2    
    </div>   
    <div title="出院区" data-options="iconCls:'icon-w-export" style="background-color:#EBEBEB;">   
        content3    
    </div>   
</div>
<!--灰色手风琴-->
<div class="hisui-accordion accordion-gray" style="width:200px;height:450px;float:left;"
    data-options="border:false">   
    <div title="等待区" data-options="iconCls:'icon-paper'" style="overflow:auto;padding:10px;border: 1px #ccc solid;">   
        content1
    </div>   
    <div title="住院区" data-options="iconCls:'icon-bed',selected:true" style="padding:10px;border: 1px #ccc solid;">   
        <code>class="hisui-accordion accordion-gray"</code>
        <code>border:1px #cccccc solid;</code>
    </div>   
    <div title="出院区" data-options="iconCls:'icon-change-loc'" style="padding:10px;border: 1px #ccc solid;">   
        content3    
    </div>   
</div>
<!--绿色手风琴-->
<div id="acc" class="hisui-accordion accordion-green" style="width:200px;height:450px;float:left;" data-options="border:false">   
    <div title="等待区" data-options="iconCls:'icon-w-update'" style="overflow:auto;padding:10px;">   
        <h3 style="color:#0099FF;">Accordion for jQuery</h3>   
        <p>Accordion is a part of easyui framework for jQuery.     
        It lets you define your accordion component on web page more easily.</p>   
    </div>   
    <div title="住院区" data-options="iconCls:'icon-w-import',selected:true" style="padding:10px;">   
        此手风琴可以进行增加和删除操作   
    </div>   
    <div title="出院区" data-options="iconCls:'icon-w-export">   
        content3    
    </div>   
</div>  
<div style="height:300px;width:120px;float:left">
     <a id="addBtn" href="#" >增加</a>
</div>
<div style="height:300px;width:120px;float:left">
     <a id="removeBtn" href="#" class="hisui-linkbutton">删除当前</a>
</div>
```

```javascript
<script type="text/javascript">
    $(function(){
        var accObj = $HUI.accordion("#acc");
        $HUI.linkbutton("#addBtn",{
            onClick:function(){
                accObj.add({
                    title:'新标题',
                    iconCls:'icon-w-add',
                    content:'你点击按钮了，我就有了。',
                    selected:true //,
                    //style:{backgroundColor:'red',borderWidth:2}
                });
            }
        });
        $HUI.linkbutton("#removeBtn",{
            onClick:function(){
                // 删除当前选中的
                var p = accObj.getSelected();
                if (p){
                    var index = accObj.getPanelIndex(p);
                    accObj.remove(index);
                }
            }
        })
    });
</script>
```
