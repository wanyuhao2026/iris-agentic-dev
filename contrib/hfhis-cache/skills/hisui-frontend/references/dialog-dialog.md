# 模态窗(dialog)

- 官方页面标题：对话框
- 分类：窗口
- 来源：http://hisui.cn/api/dialog/dialog.html
- 官网更新时间：Mon, 21 Jul 2025 03:00:22 GMT
- 提取时间：2026-07-25T13:18:46+00:00

> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。

## 模态窗口

### 说明:

对话框（Dialog）是一个特殊的窗口（window），可以包含在顶部的工具栏和在底部的按钮。默认情况下，对话框（Dialog）不能改变大小，但是用户可以设置 resizable 属性为 true，使其可以改变大小。

### 如：

```javascript
function opennew(){
			$('#dd3').dialog({
				title:'d3',
				width:400,
				height:200,
				top:150,
				left:220,
				closeKeyCode:113,
				content:'dd3'
			});
		}
```

```html
<div id="dd2" class="hisui-dialog" title="完成接诊" style="width:400px;height:350px;top:250px;left:500px;padding:10px;"　
data-options="iconCls:'icon-save',resizable:true,modal:true,toolbar:[{
		text:'修改',
		iconCls:'icon-write-order',
		handler:function(){alert('edit')}
	},{
		text:'帮助',
		iconCls:'icon-help',
		handler:function(){alert('help')}
	}],buttons:[{
		text:'保存',
		handler:function(){}
	},{
		text:'关闭',
		handler:function(){$HUI.dialog('#dd2').close();}
	}]">   
	这是一个模态操作框<code>hisui-dialog</code>
</div>
```

| 属性 | 说明 | 默认值 |  |
| --- | --- | --- | --- |
| isTopZindex | 是否覆盖控件界面 | false | 默认情况下会被病历控件遮盖，true时则可显示到最上层,但dom会复杂些。 |
| closeKeyCode | 弹出窗口后关闭快捷 | Number类型，默认不响应事件 | 如：27表示Esc，113表示F2。 20230728 |
