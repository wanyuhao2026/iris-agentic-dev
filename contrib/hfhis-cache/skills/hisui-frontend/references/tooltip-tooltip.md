# 提示（tooltip）

- 官方页面标题：提示
- 分类：基础
- 来源：http://hisui.cn/api/tooltip/tooltip.html
- 官网更新时间：Tue, 12 Mar 2024 03:51:34 GMT
- 提取时间：2026-07-25T13:18:46+00:00

> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。

## 提示

### 说明:

当用户点击或者鼠标悬停在此区域，将会显示提示。

### 如：

```html
<a id="tt1" href="#" title="这是提示信息" class="hisui-tooltip" data-options="position:'right'">鼠标移动到这(点击试试)</a>
```

```javascript
$(function(){
	$("#btn").click(function(){
		$HUI.tooltip("#tt1",{position:'bottom'}).show();
		//等价 
		//$("#tt1").tooltip({position:'bottom'}).tooltip('show');
	});
});
```

```javascript
$(function(){
			/*var tt = $HUI.tooltip('#tt',{
				content:'<span style="color:#fff;">这是通过js提示的</span>',
				position: 'bottom'
				,deltaX:50
			});*/
			$("#btn").click(function(){
				$HUI.tooltip("#tt1",{position:'bottom'}).show();
				//$("#tt1").tooltip({position:'bottom'}).tooltip('show');

			});
		});
```

| 属性 | 说明 | 默认值 |  |
| --- | --- | --- | --- |
| position | 提示层的位置 | bottom | 可选值：'top' , 'right', 'bottom', 'left' |
| tipWidth | 提示层的宽度 | undefined | 默认为undefined,宽度自动 (20210105) |
