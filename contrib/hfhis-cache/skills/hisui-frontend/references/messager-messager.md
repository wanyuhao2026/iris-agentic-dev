# 消息框(messager)

- 官方页面标题：消息提示框
- 分类：窗口
- 来源：http://hisui.cn/api/messager/messager.html
- 官网更新时间：Thu, 29 Aug 2024 14:53:11 GMT
- 提取时间：2026-07-25T13:18:46+00:00

> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。

## 本页目录

- 消息框
  - 说明:
  - 如：
  - 基于messager的简单提示方法( $.messager.popover )

## 消息框

### 说明:

### 如：

```javascript
$(function () {
					$('#alert1').click(function () {
						$.messager.alert("简单提示", "请求超出预计时间，可能网络异常,请检查网络或联系管理员。", 'info');
					});
					$('#alert2').click(function () {
						$.messager.alert("简单提示", "数据请求失败，请根据以下提示来完成该操作。<p>检查网线是否与电脑正常连接</p><p>请进入网络和Internet界面检查电脑IP地址与网关是否设置正确</p><p>不解决联系管理员</p>", 'error');
					});
					$('#alert3').click(function () {
						$.messager.alert("简单提示", "支付成功", 'success');
					});
					$('#alert4').click(function(){
						$.messager.alert({
							width:520,
							icon:'info',
							title:'合理用药提示',
							msg:'<div style="height:300px;overflow:auto;">'
							+'阿司匹林肠溶片(5)本患者（门急诊）同时开具多个阿司匹林肠溶片医嘱。避免重复给药。阿司匹林肠溶片(5)阿司匹林肠溶片与阿司匹林肠溶片均含有阿司匹林。'
							+'避免重复用药。 阿司匹林肠溶片(4)本品用于预防一过性脑缺血发作、心肌梗死、心房动、人工心脏膜、动静脉或其他手术后的血栓形成。也可用于治疗不稳定型心绞痛，'
							+'以及脑梗、心梗的二级预防。不建议超适应症使用。阿司匹林肠溶片(5)阿司匹林肠溶片与阿司匹林肠溶片均含有阿司匹林。避免重复用药，阿司匹林肠溶片(4)本品用于预防'
							+'一过性脑缺血发作、心肌梗死、心房颤动、人工心脏瓣膜、动静脉或其他手术后的血栓形成。也可用于治疗不稳定型心绞痛，以及脑梗、心梗的二级预防。不建议超适应症使用。'
							+'阿司匹林肠溶片(5)本患者（门急诊）同时开具多个阿司匹林肠溶片医嘱。避免重复给药。阿司匹林肠溶片(5)阿司匹林肠溶片与阿司匹林肠溶片均含有阿司匹林。'
							+'避免重复用药。 阿司匹林肠溶片(4)本品用于预防一过性脑缺血发作、心肌梗死、心房动、人工心脏膜、动静脉或其他手术后的血栓形成。也可用于治疗不稳定型心绞痛，'
							+'以及脑梗、心梗的二级预防。不建议超适应症使用。阿司匹林肠溶片(5)阿司匹林肠溶片与阿司匹林肠溶片均含有阿司匹林。避免重复用药，阿司匹林肠溶片(4)本品用于预防'
							+'一过性脑缺血发作、心肌梗死、心房颤动、人工心脏瓣膜、动静脉或其他手术后的血栓形成。也可用于治疗不稳定型心绞痛，以及脑梗、心梗的二级预防。不建议超适应症使用。'
							+'</div>'
							+'<div style="color:red;">是否继续审核?</div>',
							fn:function(){
								$.messager.popover({ msg: "你已点击确定" });
							}
						});
					});
					$('#confirm1').click(function () {
						var oldOk = $.messager.defaults.ok;
						var oldCancel = $.messager.defaults.cancel;
						$.messager.defaults.ok = "同意";
						$.messager.defaults.cancel = "拒绝";
						$.messager.confirm("删除", "确定提交并保存?", function (r) {
							if (r) {
								$.messager.popover({ msg: "点击了确定", type: 'info' });
							} else {
								$.messager.popover({ msg: "点击了取消" });
							}
							/*要写在回调方法内,否则在旧版下可能不能回调方法*/
							$.messager.defaults.ok = oldOk;
							$.messager.defaults.cancel = oldCancel;
						});
					});
					$('#confirm2').click(function () {
						var btns = $.messager.confirm("保存", "确定提交并保存?", function (r) {
							if (r) {
								$.messager.popover({ msg: "点击了确定", type: 'info' });
							} else {
								$.messager.popover({ msg: "点击了取消" });
							}
						}).children("div.messager-button");
						btns.children("a:eq(1)").focus();
						btns.children("a:eq(0)").addClass('green'); //修改成green2943195
					});
					$('#confirm3').click(function () {
						var oldOk = $.messager.defaults.ok;
						var oldNo = $.messager.defaults.no;
						$.messager.defaults.ok = "保存并签名";
						$.messager.defaults.no = "不保存";
						var btns = $.messager.confirm3("提示", "病历内容有修改,你确定保存吗?", function (r) {
							if (true===r) {
								$.messager.popover({ msg: "保存病历", type: 'info' });
							} else if (false === r){
								$.messager.popover({ msg: "不保存病历", type: 'info' });
							} else {
								$.messager.popover({ msg: "取消" });
							}
							/*要写在回调方法内,否则在旧版下可能不能回调方法*/
							$.messager.defaults.ok = oldOk;
							$.messager.defaults.no = oldNo;
						}).children("div.messager-button");
						btns.children("a:eq(0)").addClass('green');
						btns.children("a:eq(2)").focus();
						
					});
					$('#prompt1').click(function () {
						var btns = $.messager.prompt("提示", "修改前请签名：", function (r) {
							if (r) {
								$.messager.popover({ msg: "你的签名是：" + r, type: 'info' });
							} else {
								$.messager.popover({ msg: "点击了【取消】或输入为空" });
							}
						}).children("div.messager-button");
						btns.children("a:eq(0)").addClass('green');
					})
					$('#progress1').click(function () {
						$.messager.progress({
							title: "提示",
							msg: '正在导入数据',
							text: '导入中....'
						});
						setTimeout('$.messager.progress("close");', 4 * 1000);
					})

					$('#show1').click(function () {
						$.messager.show({
							title: '标题',
							msg: '3秒后会自动关闭',
							timeout: 3000,
							showType: 'slide'/*,
						style:{
							right:'',
							top:document.body.scrollTop+document.documentElement.scrollTop,
							bottom:''
						}*/
						});
					});
				})
```

### 基于messager的简单提示方法( $.messager.popover )

```javascript
$(function () {
					$('#succ').click(function () {
						$.messager.popover({
							msg: '保存成功！',
							type: 'success',
							timeout: 2000, 		//0不自动关闭。3000s
							showType: 'slide'  //show,fade,slide
						});
					});
					$('#info').click(function () {
						$.messager.popover({
							msg: '简单提示！',
							type: 'info',
							timeout: 2000, 		//0不自动关闭。3000s
							showSpeed: 'slow', //fast,slow,normal,1500
							showType: 'fade'  //show,fade,slide
						});
					});
					$('#alert').click(function () {
						$.messager.popover({
							msg: '简单警示！',
							type: 'alert'
						});
					});
					$('#error').click(function () {
						$.messager.popover({
							msg: '保存失败！',
							type: 'error'
						});
					});
				})
```

| $.messager.confirm3 | 三选提示说明 | 2022-02-23 增加 |  |
| --- | --- | --- | --- |
| title | 第一个入参：提示标题 |  |  |
| msg | 第二个入参：提示内容 |  |  |
| fun | 第三个入参：用户点击按钮后的回调函数 | function(r){
// 第一个按钮：ok,返回true,
// 第二个按钮：no,返回false,
// 第三个按钮：cancel,返回undefined
// 示例见[confirm三选]
} |  |
| $.messager.popover | 说明 | 默认值 |  |
| msg | 提示内容 | '' |  |
| type | 提示类型 | 'error' | 决定显示的提示样式，可选属性,'success','info','alert','error' |
| timeout | 显示时间长度(毫秒)/td> 3000 0表示一直显示不消失。 |  |  |
| showSpeed | 显示速度 | 'fast' | 可选属性,'fast','slow','normal',数字(毫秒数) |
| showType | 显示方式 | 'slide' | 可选属性,'slide','fade','show' |
| style | 显示位置 | 顶部中间位置 | right,top,left,bottom |

```
function(r){
	// 第一个按钮：ok,返回true,
	// 第二个按钮：no,返回false,
	// 第三个按钮：cancel,返回undefined
	// 示例见[confirm三选]
}
```
