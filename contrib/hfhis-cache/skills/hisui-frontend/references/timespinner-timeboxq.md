# 时间框(timeboxq)

- 官方页面标题：快速时间组件
- 分类：极速系列
- 来源：http://hisui.cn/api/timespinner/timeboxq.html
- 官网更新时间：Tue, 12 Mar 2024 03:51:34 GMT
- 提取时间：2026-07-25T13:18:46+00:00

> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。

## 时间框(timeboxq) 2020-10-14

### timeboxq继承自validatebox。简洁时间框,支持数字输入及n输入,当光标离开时自动转换成要求格式。如11转成 11:00:00 ,118转成 11:08:00 ,n+15转成15分钟后 时间,n-15表示15分钟前 时间

### 一.日期格式设置( timeFormat:"HMS" )

### 二.日期范围设置( minTime:"8:30",maxTime:"17:30" )

```javascript
$.extend($.fn.validatebox.defaults.rules, {
				TimeG:{
					validator:function(value,param){
						console.log(value);
						console.log(param);
						return true;
					}
				}
			});
			$("#setTimeDisBtn").click(function(){
				var opt = $("#tb2").timeboxq('options');
				var target = document.getElementById('tb2');
				opt.minTime = opt.formatter.call(target,opt.parser.call(target,"9:00"));
				opt.maxTime = opt.formatter.call(target,opt.parser.call(target,"21:00"));
				$.messager.popover({msg: '设置可选范围成功',type:'success',timeout: 1000});
			});
```

```javascript
$.parser.onComplete = function(ctx){
				var opt = $('#exeTime').timeboxq('options'); 
				var target = document.getElementById('exeTime');
				$("#af15").html(opt.formatter.call(target,opt.parser.call(target,"n+15")));
				$("#bf15").html(opt.formatter.call(target,opt.parser.call(target,"n-15")));
				$("#exeTime").timeboxq({
					onChange:function(newValue,oldValue){
						console.log("newValue="+newValue+", oldValue="+oldValue);
					},
					validType:["TimeG['endtb']"]
				});

			};
```

| 属性 | 说明 | 默认值 |  |
| --- | --- | --- | --- |
| timeFormat | 默认值为HMS。表示日期显示成HH-MM-SS | HMS | 可以设置成HM,如果需求实现其它方式格式可重写formatter |
| minTime | 最小时间。如:8:30 | 00:00:00 |  |
| maxTime | 最大时间。如:17:30 | 23:59:59 |  |
| 事件 | 说明 | 入参 |  |
| onChange | 值变化后调用 | newTime,oldTime |  |
