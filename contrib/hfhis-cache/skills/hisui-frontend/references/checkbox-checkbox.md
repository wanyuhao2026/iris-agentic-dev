# 复选框(checkbox)

- 官方页面标题：勾选框
- 分类：表单
- 来源：http://hisui.cn/api/checkbox/checkbox.html
- 官网更新时间：Mon, 09 Sep 2024 09:08:41 GMT
- 提取时间：2026-07-25T13:18:46+00:00

> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。

## 选择框, 通过图片得到蓝色系勾选框

### 说明:

点击勾选，常用于多项选择。

### 一.各种状态例子

### 二. 框在文本右边( right )

### 三、必选功能

```javascript
var flag = true;
		function setSexRequired(){
			$("#requiredMsg").html('当前'+(flag?"非必选":"必选"));
			if (flag){ $("#reqLabel span").remove();}
			else{$('#reqLabel').prepend('<span style="color: red;" id="myredstar">*</span>');}
			$('[name="sex"]').checkbox('setRequired',flag=!flag);
			
		};
```

### 复选框组件的相关配置、事件、方法

| 属性 | 说明 | 默认值 |  |
| --- | --- | --- | --- |
| id | 复选框的Id | '' |  |
| label | 描述 | '' |  |
| disabled | 是否禁用。默认可用 | false |  |
| checked | 是否选中。默认不选中 | false |  |
| boxPosition | 勾选框位置 | 'left' | 可选'right' 20191104 |
| required | name相同判断为一组，且是否必选一项 | false | true时为必选一项,不选会变红提示 20200726 |
| 事件名 | 说明 | 入参 |  |
| onCheckChange | 选中改变事件 | e,value |  |
| onChecked | 选中事件 | e,value |  |
| onUnchecked | 取消选中时触发 | e,value |  |
| 方法名 | 说明 | 入参 | 返回值 |
| options | 拿到配置项对象 |  | checkbox的options对象 |
| setValue | 选中元素 | true\|false | jquery对象 |
| getValue | 拿到是否选中 |  | true\|false |
| setDisable | 禁用 | false\|true | jquery对象 |
| check | 选中 |  | jquery对象 |
| uncheck | 取消选中 |  | jquery对象 |
| toggle | 切换选中 |  | jquery对象 |
| disable | 禁用选框 |  | jquery对象 |
| enable | 启用选框 |  | jquery对象 |
| update | 重绘选框 |  | jquery对象 |
| destroy | 销毁选框 |  | jquery对象 |
| isValid | 必选一项验证 20200726 |  | Boolean |
| setRequired | 调置或取消必项状态 20210205 | true\|false | jquery对象 |

```javascript
$(function(){
			$("#selectba").click(function(){
				$HUI.radio("#ba").setValue(true);
			});
		});
```

```javascript
function onResizePretty(){
            $('.hisui-checkbox').checkbox('isValid');
        }
```
