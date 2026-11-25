# 日期框(datebox)

- 官方页面标题：日期
- 分类：表单
- 来源：http://hisui.cn/api/datebox/datebox.html
- 官网更新时间：Tue, 12 Mar 2024 03:51:26 GMT
- 提取时间：2026-07-25T13:18:46+00:00

> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。

## 日期框

### 说明:

日期框（datebox）把可编辑的文本框和下拉日历面板结合起来，用户可以从下拉日历面板中选择日期，在文本框中输入的字符串可转换为有效日期。支持快捷t方式输入,t+2表示二天后,t-2表示二天前的日期。

### 如：

```javascript
function onSelect(date){
				$('#result').text(date)
			}
			$(function(){
				$("#samthBtn").click(function(){
					var val = $("#samth").datebox('getValue');
					$.messager.alert( "提示",val);
				});
				$("#setDateDisBtn").click(function(){
					var now = new Date();
					var now6 = new Date();
					now6.setTime(now.getTime()+(6*24*60*60*1000));
					var opt = $("#db").datebox('options');
					opt.minDate = opt.formatter(now); //'2019-11-24';
					opt.maxDate = opt.formatter(now6); //'2019-11-30';
					$.messager.popover({msg: '设置可选日期范围成功',type:'success',timeout: 1000});
				});
				$("#dbBtn").click(function(){
					var val = $("#db").datebox('getValue');
					$.messager.alert( "提示",val);
				});
			})
```

```javascript
// 	var init = function () {
// 		$("#btn1").click(function(){
// 			var valbox = $HUI.datebox("#db");
// 			var val = valbox.getValue();
// 			alert(val);
// 		});

// 	}	
//  	$(init);
```

| 属性 | 说明 | 默认值 | 查看EasyUI1.3.2的对应配置 |
| --- | --- | --- | --- |
| validParams | 默认值为YMD。表示日期显示成YYYY-MM-DD | YMD | 如果是其它格式则不强制检查. 如配置成YM再实现formatter可实现例1效果 2019-07-05 |
| minDate | 可选最小日期。如:2019-11-25 | 默认null | 2019-11-26 |
| maxDate | 可选最大日期。如:2019-11-25 | 默认null | 2019-11-26 |
