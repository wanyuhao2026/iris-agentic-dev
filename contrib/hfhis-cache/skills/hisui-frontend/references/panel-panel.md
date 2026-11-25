# 面板(panel)

- 官方页面标题：面板
- 分类：布局
- 来源：http://hisui.cn/api/panel/panel.html
- 官网更新时间：Tue, 12 Mar 2024 03:51:31 GMT
- 提取时间：2026-07-25T13:18:46+00:00

> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。

## 面板

### 说明:

面板是其他组件或元素的容器

### 一、卡片式面板

```html
<div class="use-prettyprint" prettyprintfor="#mypanelpp1">
<div class="hisui-panel" data-options="headerCls:'panel-header-card'" style="height:150px;width:400px;padding:20px" title="登记（不常用）">
				卡片面板
				<code>headerCls:'panel-header-card'</code>
</div>
</div>
```

```html
<div class="use-prettyprint" prettyprintfor="#mypanelpp2">
<div class="hisui-panel" data-options="headerCls:'panel-header-card-gray'" style="height:150px;width:400px;padding:20px" title="登记（不常用）">
				灰色卡片面板
				<code>"headerCls:'panel-header-card-gray'"</code>
</div>
</div>
```

### 二、灰色系面板

```html
<div class="hisui-panel" title="病人信息" style="width:400px;padding:20px" data-options="headerCls:'panel-header-gray',iconCls:'icon-paper',closable:true,collapsible:true,minimizable:true,maximizable:true"></div>
```

```html
<div class="hisui-panel panel-header-gray" title="" data-options="..."></div>
```

```html
<div data-options="headerCls:'panel-header-big panel-header-gray',..."></div>
```

```html
<div id="accPanel" title="病人信息" style="width:400px;padding:20px;">还可通过js生成</div>
```

```javascript
var init = function () {
			var valbox = $HUI.panel("#accPanel", {
				iconCls: 'icon-paper',
				closable: true,
				onBeforeClose: function () {
					alert("你点击了关闭!");
					return false
				},
				collapsible: true,
				onBeforeCollapse: function () {
					alert("你点击了收起!");
					return false
				},
				minimizable: true,
				onMinimize: function () {
					alert("你点击了最小化!");
					return false
				},
				maximizable: true,
				onMaximize: function () {
					alert("你点击了最大化!");
					return false
				},
				headerCls: 'panel-header-gray'
			});
		}
		$(init);
```

| 属性 | 说明 | 默认值 |  |
| --- | --- | --- | --- |
| headerCls | 面板头样式类 | null,表示蓝色 | 可选值： 'panel-header-gray' , 'panel-header-blue' , 'panel-header-acc' , 'panel-header-white' , 'panel-header-card' , 'panel-header-card-gray' , 'panel-header-big' |
| bodyCls | 面板内容样式类 | null | 可选值： 'panel-body-gray' |
| notTrans | 不自动翻译与否 | false | 默认自动翻译。true表示不翻译 20230801 |
