# 验证框(validatebox)

- 官方页面标题：验证框
- 分类：表单
- 来源：http://hisui.cn/api/validatebox/validatebox.html
- 官网更新时间：Fri, 13 Mar 2026 07:06:14 GMT
- 提取时间：2026-07-25T13:18:46+00:00

> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。

```javascript
$.fn.datebox.defaults.formatter = function(date){
			var y = date.getFullYear();
			var m = date.getMonth()+1;
			var d = date.getDate();
			return y+"-"+m+'-'+d;
		}
```

## 基础验证框

### 说明:

基础验证框（validatebox）用来验证表单，在输入框上加入简单的验证 。

### 如：

```javascript
var init = function () {
				var valbox = $HUI.validatebox("#patno", {
					required: true,
					placeholder:"序号必填"
				});
				$("#btn1").click(function(){
					$("#patno").validatebox("setValue",'12345678');
					//$("#patno").val("12345678").validatebox("validate");
				});
				$("#btn").click(function () {
					//$("#patno").data("validatebox").validating=true;
					//$("#patno").validatebox("validate");
					// valbox.jdata <=> $("#patno").data("validatebox")
					//下面二句 可以让tooltip主动显示出来
					valbox.jdata.validating = true;
					valbox.validate();
					// 获得验证结果
					var patnoHasValue = valbox.isValid();
					if (patnoHasValue) {
						alert("序号验证通过");
					} else {
						alert("序号为空");
					}
				});
				$("#btn2").click(function(){
					var box = $("#urlVx");
					var value = box.prop("disabled");
					if(value == false){
						box.validatebox("setDisabled",true);
						$("#btn2 > span > span").text("启用URL");
						box.validatebox("validate");
					}
					else{
						box.validatebox("setDisabled",false);
						$("#btn2 > span > span").text("禁用URL");
						box.validatebox("validate");
					}
				});
	
			}
			$(init);
```

| 属性 | 说明 | 默认值 |  |
| --- | --- | --- | --- |
| placeholder | 默认值"" | 当不为空时，在输入框显示placeholder, IE9+ , chrome 支持 。2018-10-18 |  |
| disabled | 默认值"false" | 当为true时，输入框被禁止使用。 2019-9-2 |  |
| validType | 格式校验 | 新增加身份证号校验'idcare',手机号检验'mobilephone'。 2020-01-19 |  |
| 方法名 | 说明 | 入参 | 返回值 |
| setDisabled | 改变disabled属性的值 | false/true | jquery对象 |
| setValue | 赋值且验证 |  | jquery对象 2026-03-13 |
