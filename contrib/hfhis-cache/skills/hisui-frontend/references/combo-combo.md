# 组合(combo)

- 官方页面标题：下拉框
- 分类：表单
- 来源：http://hisui.cn/api/combo/combo.html
- 官网更新时间：Tue, 12 Mar 2024 03:51:24 GMT
- 提取时间：2026-07-25T13:18:46+00:00

> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。

## 组合框

### 说明:

组合（combo）是在 html 页面上显示一个可编辑的文本框和下拉面板。它是用于创建其他复杂的组合组件（比如：组合框 combobox、组合树 combotree、组合网格 combogrid，等等）的基础组件。

组合（combo）可以使用 javascript 从 <select> 或 <input> 元素进行创建。请注意， 从标记创建组合（combo）是无效的 。

从标记创建组合（combo）是无效的

### 如：

```javascript
$(function(){
					var ccObj=$HUI.combo('#cc',{});
				});
```

```javascript
$(function(){
					var cc2Obj=$HUI.combo('#cc2',{editable:false,placeholder:'自定义下拉面板'});
					$('#sp').appendTo(cc2Obj.panel());
					$('#sp label').click(function(){
						var l = $(this).html();
						var v = $(this).attr('value');
						cc2Obj.setValue(v);
						cc2Obj.setText(l);
						cc2Obj.hidePanel();
						console.log(cc2Obj.getValue());
					});
				});
```

| 属性 | 说明 | 默认值 |  |
| --- | --- | --- | --- |
| enterNullValueClear | 默认值true。当为false时，在输入框内回车，没有匹配值不清空输入框。 | true |  |
| placeholder | 默认值"" | 当不为空时，在输入框显示placeholder, IE9+ , chrome 支持。 |  |
