# 触发框(triggerbox)

- 官方页面标题：自定义触发框
- 分类：表单
- 来源：http://hisui.cn/api/triggerbox/triggerbox.html
- 官网更新时间：Tue, 12 Mar 2024 03:51:35 GMT
- 提取时间：2026-07-25T13:18:46+00:00

> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。

## 事件触发框

### 说明:

触发框（triggerbox）用于自定义功能，图标可以自定义。 2019-01-03

### 如：

```html
<div class="hisui-panel" title="触发框示例" style="width:500px;padding:10px 30px 20px 30px"
	data-options="headerCls:'panel-header-gray',iconCls:'icon-paper-info'">
	<table>
		<tr>
			<td class='r-label'>默认触发框</td>
			<td>
				<input name="default" class="hisui-triggerbox textbox" data-options="handler:msg" />
			</td>
			<td></td>
		</tr>
		<tr>
			<td class='r-label'>图片触发框</td>
			<td>
				<input id="img" name="img" class="hisui-triggerbox textbox"
					data-options="icon:'icon-img-blue',handler:msg,plain:true" />
			</td>
			<td></td>
		</tr>
		<tr>
			<td class='r-label'>卡触发框</td>
			<td>
				<input id="card" name="card" class="hisui-triggerbox textbox"
					data-options="icon:'icon-card',handler:msg,plain:true" />
			</td>
			<td></td>
		</tr>
		<tr>
			<td class='r-label'>复制触发框</td>
			<td>
				<input id="copy" name="copy" class="hisui-triggerbox textbox"
					data-options="icon:'icon-copy-blue',handler:msg,plain:true" />
			</td>
			<td style="padding-left: 10px;font-size: 0;">
				<a class="hisui-linkbutton" id="btn-disable">禁用</a>
				<a class="hisui-linkbutton" id="btn-enable" style="margin-left: 10px;">启用</a>
			</td>
		</tr>
	</table>
</div>
```

```javascript
function msg(value, name) {
	var title = "点击了name=" + name + "触发组件的按钮,value=" + value;
	$.messager.popover({
		type: "info",
		msg: title
	});
	if (name == "copy") {
		var v = $("#card").triggerbox('getValue');
		$("#copy").triggerbox('setValue', v);
	}
}
$(function () {
	$('#btn-disable').click(function () {
		$('#copy').triggerbox('disable')
	})
	$('#btn-enable').click(function () {
		$('#copy').triggerbox('enable')
	})
})
```

| 属性 | 说明 | 默认值 |  |
| --- | --- | --- | --- |
| icon | 可配置按钮显示的图标 | 'icon-w-trigger-box' | 见【基础】-【图标(icon)】界面 |
| prompt | 为空时的提示值 | '' |  |
| value | 初值 | '' |  |
| disabled | 是否禁用 | false |  |
| plain 2019-12-23 图标背景是否透明 false 当未指定icon且未指定plain时。自动修改配置项 icon:'icon-trigger-box',plain:true |  |  |  |
| 方法 | 说明 | 入参 | 返回值 |
| getValue | 获得组件的值 | 无 | 组件值 |
| setValue | 给组件赋值 | val | 当前jq对象 |
| disable | 禁用组件 | 无 | 当前jq对象 |
| enable | 启用组件 | 无 | 当前jq对象 |
| options | 获取组件配置项 | 无 | 配置项 |
| textbox | 获取组件输入框jq对象 | 无 | 输入框jq对象 |
| clear | 清空值 | 无 | 当前jq对象 |
| reset | 还原初值 | 无 | 当前jq对象 |
| getName | 获取Name属性 | 无 | Name属性 |
| destroy | 销毁组件 | 无 | 当前jq对象 |
| resize | 调整宽度 | width | 当前jq对象 |
| 事件 | 说明 | 入参 |  |
| handler | 点击按钮时触发事件 | value,name |  |
