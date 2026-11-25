# 菜单按钮(menubutton)

- 官方页面标题：菜单按钮
- 分类：表单
- 来源：http://hisui.cn/api/menubutton/menubutton.html
- 官网更新时间：Tue, 12 Mar 2024 03:51:31 GMT
- 提取时间：2026-07-25T13:18:46+00:00

> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。

## 菜单按钮

### 说明:

菜单按钮（menubutton）是下拉菜单的一部分。它与链接按钮（linkbutton）及菜单（menu）有关。显示链接按钮（linkbutton），隐藏菜单（menu）。当用户点击或移动鼠标到链接按钮（linkbutton）上时，将显示菜单（menu）以允许用户点击菜单。

### 一、多级菜单

```html
<a href="javascript:void(0)" id="mbedit" class="hisui-menubutton" data-options="menu:'#mmedit',iconCls:'icon-write-order'">修改</a>
    <div id="mmedit" style="width:150px;">
        <div data-options="iconCls:'icon-undo'">撤销</div>
        <div data-options="iconCls:'icon-redo'">撤回</div>
        <div class="menu-sep"></div>
        <div>剪切</div>
        <div>复制</div>
        <div>
            <span>粘贴</span>
            <div class='menu-no-icon'>
                <div>粘贴成文本</div>
                <div>粘贴成网页</div>
                <div>粘贴成表格</div>
            </div>
        </div>
        <div class="menu-sep"></div>
        <div data-options="iconCls:'icon-remove'">删除</div>
        <div>选择</div>
    </div>
```

### 二、无图标菜单( menu-no-icon ) 2018-09-17

```html
<<a href="javascript:void(0)" class="hisui-menubutton" data-options="menu:'#mm3',iconCls:'icon-paper-info'">操作</a>
            <div id="mm3" style="width:100px;" class='menu-no-icon'>   <!--menu-no-icon让菜单更紧凑-->
                <div onclick="console.log('停止');">停止</div>   
                <div onclick="console.log('作废');">作废</div>     
            </div>
```

### 三、蓝色的MenuButton如：

```html
<a href="javascript:void(0)" id="menubtn-blue" class="hisui-menubutton menubutton-blue" style="width:130px;" data-options="menu:'#mm-blue'">操作</a>
<div id="mm-blue" style="width:100px;">   
    <div onclick="console.log('停止');">停止</div>   
    <div onclick="console.log('作废');">作废</div>     
    <!-- data-options="iconCls:'icon-cancel'" -->
</div>

<a href="javascript:void(0)" id="mbedit-blue" class="hisui-menubutton" style="width:200px;" data-options="menu:'#mmedit-blue',iconCls:'icon-edit',otherCls:'menubutton-blue'">Edit</a>
<div id="mmedit-blue" style="width:150px;">
    <div data-options="iconCls:'icon-undo'">Undo</div>
    <div data-options="iconCls:'icon-redo'">Redo</div>
    <div class="menu-sep"></div>
    <div>Cut</div>
    <div>Copy</div>
    <div>Paste</div>
    <div class="menu-sep"></div>
    <div data-options="iconCls:'icon-remove'">Delete</div>
    <div>Select All</div>
</div>
```

### 四、工具栏的MenuButton如：

```html
<a href="javascript:void(0)" id="menubtn-toolbar" class="hisui-menubutton menubutton-toolbar"  data-options="menu:'#mm-toolbar',iconCls:'icon-paper-info'">操作</a>
<div id="mm-toolbar" style="width:100px;">   
    <div onclick="console.log('停止');">停止</div>   
    <div onclick="console.log('作废');">作废</div>     
    <!-- data-options="iconCls:'icon-cancel'" -->
</div>

<a href="javascript:void(0)" id="mbedit-toolbar" class="hisui-menubutton"  data-options="menu:'#mmedit-toolbar',iconCls:'icon-write-order',otherCls:'menubutton-toolbar'">修改</a>
<div id="mmedit-toolbar" style="width:150px;">
    <div data-options="iconCls:'icon-undo'">撤销</div>
    <div data-options="iconCls:'icon-redo'">撤回</div>
    <div class="menu-sep"></div>
    <div>剪切</div>
    <div>复制</div>
    <div>粘贴</div>
    <div class="menu-sep"></div>
    <div data-options="iconCls:'icon-remove'">删除</div>
    <div>查询</div>
</div>
```

### 表格中使用按钮

| 属性 | 说明 | 默认值 |  |
| --- | --- | --- | --- |
| isTopZindex | 是否覆盖在病历编辑器这上 | false | 可选值： true\|false |
