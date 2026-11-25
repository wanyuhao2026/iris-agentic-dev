# 日期时间(datetimeboxq)

- 官方页面标题：日期时间
- 分类：极速系列
- 来源：http://hisui.cn/api/datebox/datetimeboxq.html
- 官网更新时间：Tue, 12 Mar 2024 03:51:26 GMT
- 提取时间：2026-07-25T13:18:46+00:00

> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。

## 快速日期时间框

### 说明:

快速日期时间框（datetimeboxq）继承自dateboxq组件，在初始化时不初始化日历，按钮实现方式不再一样，在IE8或IE11下速度大大提高。

### 如：

```javascript
function onSelect(date){
				$('#result').text(date)
			}
			$.parser.onComplete = function(){
				$('#dtbq1').datetimeboxq('calendar').calendar({
					validator:function(date){
						var now = new Date();
						return date<=now;
					}
				});
				
				var min = $.fn.datebox.defaults.formatter($.fn.datebox.defaults.parser('t'));
				var max = $.fn.datebox.defaults.formatter($.fn.datebox.defaults.parser('t+7'));
				
				$('#dtbq2').datetimeboxq({
					onSelect:onSelect,
					// value:min+' 11:11:11',
					minDate:min,
					maxDate:max
				});
				$('#dtbq2').datetimeboxq('setValue',"2023-11-16 09:10:10");
			};
```

| 属性 | 说明 | 默认值 |  |
| --- | --- | --- | --- |
| showSeconds | 是否显示到秒 | true |  |
| minDate | 可选最小日期。如:2019-11-25 | 默认null |  |
| maxDate | 可选最大日期。如:2019-11-25 | 默认null |  |
| options | 获得配置项目 | 默认null |  |
| 方法 | 说明 | 默认值 |  |
| setValue | 设置日期时间 |  |  |
| getValue | 获得日期时间 |  |  |
| setDisabled | 禁用日期框.false表示启用，true表示禁用 |  |  |
| 事件 | 说明 | 默认值 |  |
| onHidePanel | 隐藏面板时 |  |  |
| onSelect | 选中日期时 | date |  |
