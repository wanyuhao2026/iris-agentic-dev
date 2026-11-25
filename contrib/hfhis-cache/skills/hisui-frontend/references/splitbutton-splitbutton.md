# 分割按钮(splitbutton)

- 官方页面标题：分割按钮
- 分类：表单
- 来源：http://hisui.cn/api/splitbutton/splitbutton.html
- 官网更新时间：Tue, 12 Mar 2024 03:51:32 GMT
- 提取时间：2026-07-25T13:18:46+00:00

> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。

## 分割按钮

### 说明:

分割按钮（Split Button）包含一个链接按钮（Link Button）和一个菜单（Menu）。当用户点击或者鼠标悬停在向下箭头区域，将会显示一个对应的菜单。

### 如：

```html
<a href="javascript:void(0)" id="sb1" class="hisui-splitbutton" data-options="menu:'#mm2',iconCls:'icon-abort-order',onClick:function(){
    $.messager.popover({msg:'按钮自己',type:'info'});
}">作废医嘱</a>   
<div id="mm2" style="width:100px;" class="menu-no-icon">   
    <div onclick="$.messager.popover({msg:'停止',type:'info'});">停止</div>   
    <div onclick="$.messager.popover({msg:'作废',type:'info'});">作废</div>   
</div>
```
