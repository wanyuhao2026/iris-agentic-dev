# 数字框(numberbox)

- 官方页面标题：数字验证框
- 分类：表单
- 来源：http://hisui.cn/api/numberbox/numberbox.html
- 官网更新时间：Tue, 12 Mar 2024 03:51:31 GMT
- 提取时间：2026-07-25T13:18:46+00:00

> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。

## 数字验证框

### 说明:

数字验证框（numberbox）用来验证数字表单，在输入框上加入简单的验证 。

### 如：

```javascript
$(function(){
				$("#nb").on('keyup',function(e){
					console.log("numberbox value = "+$(this).numberbox('getValue'));
					if (e.keyCode==13){
						console.log("numberbox value enterkey ="+$(this).numberbox('getValue'));
					}
				});
			})
```

| 属性 | 默认值 | 说明 |  |
| --- | --- | --- | --- |
| isKeyupChange | 默认值false | 值为false时blur时改变组件的值。为true时则keyup时改变组件的值 2019-4-18 |  |
| keyupChangeDelay | 默认值0 | 当isKeyupChange为true时，keyup改变组件值延迟时间 2023-08-24 用于解决当值范围为5-10000时，想要输入123却变成523问题 |  |
| fix | 默认值true | true时强制转换值为min或max，值为false时只给出提示 2020-1-21 |  |
| forcePrecisionZoer | 默认值true | true强制按有效数字配置补齐,false时不强制补0 2022-5-22 |  |
