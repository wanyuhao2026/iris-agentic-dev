# 日期框(dateboxq)

- 官方页面标题：快速日期
- 分类：极速系列
- 来源：http://hisui.cn/api/datebox/dateboxq.html
- 官网更新时间：Tue, 12 Mar 2024 03:51:26 GMT
- 提取时间：2026-07-25T13:18:46+00:00

> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。

## 快速日期框

### 说明:

快速日期框（dateboxq）在初始化时不初始化日历，按钮实现方式不再一样，在IE8或IE11下速度大大提高。

支持快捷t方式输入,t+2表示二天后,t-2表示二天前的日期。

### 如：

```javascript
function onSelect(date){
				$('#result').text(date)
			}
			$.parser.onComplete = function(){
				$('#db').dateboxq('calendar').calendar({
					validator:function(date){
						var now = new Date();
						return date<=now;
					}
				});
				$("#samthBtn").click(function(){
					var val = $("#samth").dateboxq('getValue');
					$.messager.alert( "提示",val);
				});
				$("#setDateDisBtn").click(function(){
					var now = new Date();
					var now6 = new Date();
					now6.setTime(now.getTime()+(6*24*60*60*1000));
					var opt = $("#db").dateboxq('options');
					opt.minDate = opt.formatter(now); //'2019-11-24';
					opt.maxDate = opt.formatter(now6); //'2019-11-30';
					$.messager.popover({msg: '设置可选日期范围成功',type:'success',timeout: 1000});
				});
				$("#dbBtn").click(function(){
					var val = $("#db").dateboxq('getValue');
					$.messager.alert( "提示",val);
				});
				$("#self1dbBtn").click(function(){
					var val = $("#self1db").dateboxq('getValue');
					$.messager.alert( "提示",val);
				});
				$("#self3dbBtn").click(function(){
					var val = $("#self3db").dateboxq('getValue');
					if (val){
						//取到Date日期对象
						var valDateObj = $("#self3db").dateboxq('options').parser(val);
						console.log(valDateObj.getTime());
					}
					$.messager.alert( "提示",val);
				});
				$("#self3dbSetBtn").click(function(){
					var date = new Date();
					date.setDate(1);
					var jobj = $("#self3db");
					var dateText = jobj.dateboxq('options').formatter(date);
					jobj.dateboxq('setValue',dateText);
				});
				$("#self4dbBtn").click(function(){
					var val = $("#self4db").dateboxq('getValue');
					$.messager.alert( "提示",val);
				});
			};
```

| 属性 | 说明 | 默认值 |  |
| --- | --- | --- | --- |
| validParams | 默认值为YMD。表示日期显示成YYYY-MM-DD | YMD | 如果是其它格式则不强制检查. 如配置成YM再实现formatter可实现例1效果 |
| minDate | 可选最小日期。如:2019-11-25 | 默认null |  |
| maxDate | 可选最大日期。如:2019-11-25 | 默认null |  |
| format | 日期格式(yyyy-MM-dd,dd/MM/yyyy,yyyy年MM月dd日) | '' | 默认空时走formatter,不为空时强制覆盖formatter方法，实现对应格式 2020-12-31 |
| options | 获得配置项目 | 默认null |  |
| 方法名 | 说明 | 入参 | 返回值 |
| setValue | 设置日期框值。(如："yyyy-MM-dd") | value |  |
| getValue | 获得日期值 |  | 格式化字符串日期 |
| setDisabled | 禁用日期框. false表示启用日期框，true表示禁用日期框 | value |  |
| disable | 禁用日期框 | 无 |  |
| enable | 启用日期框 | 无 |  |
| 事件 | 说明 | 默认值 |  |
| onSelect | date |  |  |
| onChange | newVal,oldValue |  |  |
