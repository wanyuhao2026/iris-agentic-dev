# 单选框(radio)

- 官方页面标题：单选框
- 分类：表单
- 来源：http://hisui.cn/api/radio/radio.html
- 官网更新时间：Mon, 09 Sep 2024 09:08:32 GMT
- 提取时间：2026-07-25T13:18:46+00:00

> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。

## 单选框, 通过图片得到蓝色系勾选框

### 说明:

用于多选一的业务场情。

### 一、单选框示例

```javascript
$(function(){
    logger.level=1;
    $HUI.radio("[name='wantEat']",{
        onChecked:function(e,value){
            logger.info($(e.target).attr("label"));  //输出当前选中的label值
        }
    });
    $("#selectba").click(function(){
        $HUI.radio("#ba").setValue(true);
    });
    $("#insertRadio").click(function(){
        $("#eatlist").append("<input class='hisui-radio' type='radio' label='黄金梨' value='pear1' name='wantEat'>");
        $("#eatlist").append("<input class='hisui-radio' type='radio' label='秋月梨' value='pear2' name='wantEat'>");
        $("#eatlist").append("<input class='hisui-radio' type='radio' label='黄梨' value='pear3' name='wantEat'>");
        $("#eatlist").append("<input class='hisui-radio' type='radio' label='鸭梨' value='pear4' name='wantEat'>");
        $HUI.radio("#eatlist input.hisui-radio",{});
    });
    $("#getValue").click(function(){
        var checkedRadioJObj = $("input[name='wantEat']:checked");
        $.messager.alert("提示","value="+checkedRadioJObj.val()+" , label="+checkedRadioJObj.attr("label"));
    })
});
```

### 二、样式为复选框的单选框

### 三、必选功能

```javascript
var flag = true;
		function setSexRequired(){
			$("#requiredMsg").html('当前'+(flag?"非必选":"必选"));
			$('[name="sex"]').radio('setRequired',flag=!flag);
		};
```

### 四、不能取消选择功能(requiredSel)

### 单选组件的相关配置、事件、方法

| 属性 | 说明 | 默认值 |  |
| --- | --- | --- | --- |
| name | radio的名字 | '' |  |
| label | 描述 | '' |  |
| disabled | 是否禁用。默认可用 | false |  |
| checked | 是否选中。默认不选中 | false |  |
| boxPosition | 勾选框位置 | 'left' | 可选'right' 20191104 |
| requiredSel | 必选 | false | true不可取消选中 20191126 |
| required | name相同判断为一组，且是否必选一项 | false | true时为必选一项,不选会变红提示 20200726 |
| 事件名 | 说明 | 入参 |  |
| onCheckChange | 选中改变事件 | e,value |  |
| onChecked | 选中事件 | e,value |  |
| 方法名 | 说明 | 入参 | 返回值 |
| options | 拿到配置项对象 |  | jquery对象 |
| setValue | 选中元素,此方法入参为false无效。选中当前元素会自动清空同名radio | true | jquery对象 |
| getValue | 拿到值 |  | false\|true |
| setDisable | 禁用 | false\|true | jquery对象 |
| check | 选中 |  | jquery对象 |
| uncheck | 取消选中 |  | jquery对象 |
| toggle | 切换选中 |  | jquery对象 |
| disable | 禁用选框 |  | jquery对象 |
| enable | 启用选框 |  | jquery对象 |
| update | 重绘选框 |  | jquery对象 |
| destroy | 销毁选框 |  | jquery对象 |
| isValid | 验证必状态 20200726 |  | Boolean |
| setRequired | 调置或取消必项状态 20210205 | true\|false | jquery对象 |

```javascript
function onResizePretty(){
            $('.hisui-radio').radio('isValid');
        }
```
