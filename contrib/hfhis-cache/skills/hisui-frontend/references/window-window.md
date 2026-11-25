# 窗口(window)

- 官方页面标题：窗口
- 分类：窗口
- 来源：http://hisui.cn/api/window/window.html
- 官网更新时间：Mon, 21 Jul 2025 03:00:16 GMT
- 提取时间：2026-07-25T13:18:46+00:00

> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。

## 窗口

### 说明:

窗口（window）默认有四个工具：collapsible、minimizable、maximizable和closable。

### 如：

```html
<div class="hisui-window" title="完成接诊" style="width:400px;height:200px;top:10px;left:500px;padding:10px"	
data-options="iconCls:'icon-w-save',resizable:true,modal:false,isTopZindex:true">
		
	我是一个Window窗口
	<code>hisui-window</code>
</div>
```

| 属性 | 说明 | 默认值 |  |
| --- | --- | --- | --- |
| isTopZindex | 是否覆盖控件界面 | false | 默认情况下会被病历控件遮盖，true时则可显示到最上层,但dom会复杂些。 |
| constrain | 限制在容器内拖动 | Boolean类型, 默认false | true时拖动范围不超出容器, false允许超出 20250721 |
