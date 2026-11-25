# 开关(switchbox)

- 官方页面标题：切换开关
- 分类：表单
- 来源：http://hisui.cn/api/switchbox/switchbox.html
- 官网更新时间：Tue, 12 Mar 2024 03:51:33 GMT
- 提取时间：2026-07-25T13:18:46+00:00

> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。

## 开/关。源自Bootstrap的 switch插件 。

### 说明:

切换开关

### 如：

```html
<div id="switch1" class="hisui-switchbox" style="margin-left:5px" data-options="onText:'启用',offText:'禁用',
size:'small',animated:true,onClass:'primary',offClass:'gray',onSwitchChange:function(event,obj){console.log(event);console.log(obj);}">
</div>
```

### 用js生成

```javascript
$(function(){
    $HUI.switchbox('#switch2',{
        onText:'开',
        offText:'关',
        onClass:'primary',
        offClass:'gray',
        onSwitchChange:function(e,obj){
            console.log(e);
            console.log(obj);
        }
    })
})
```

### 单选组件的相关配置、事件、方法

| 属性 | 说明 | 默认值 |  |
| --- | --- | --- | --- |
| id | switchbox的名字 | '' |  |
| onText | 开状态时显示的文本。 | '开' |  |
| offText | 关状态时显示的文本。 | '关' |  |
| disabled | 是否禁用。默认可用 | false |  |
| checked | 是否选中。默认开启 | true |  |
| animated | 是否启用动画效果 | false |  |
| size | 开关大小配置，可选范围 'mini' ｜ 'small' \| 'large'。默认'mini' | 'mini' |  |
| onClass | 开状态时类样式，可选范围 'primary' \| 'info' \| 'success' \| 'warning' \| 'danger'\|'gray'。默认'success' | 'success' |  |
| offClass | 开状态时类样式，可选范围 'primary' \| 'info' \| 'success' \| 'warning' \| 'danger'\|'gray'。默认'warning' | 'warning' |  |
| 事件名 | 说明 | 入参 |  |
| onSwitchChange | 选中改变事件 | e,obj |  |
| 方法名 | 说明 | 入参 | 返回值 |
| options | 拿到配置项对象 |  | jquery对象 |
| toggleActivation | 切换禁用与可用状态 |  | jquery对象 |
| isActive | 判断当前开关是否启用 |  |  |
| setActive | 禁用或启用开关 | false\|true | jquery对象 |
| toggle | 切换开与关状态 |  | jquery对象 |
| setValue | 设置值,同时是否要触发切换事件,skipOnChange为可选参数 | false\|true[,skipOnChange] | jquery对象 |
| getValue | 获得值 | false\|true | jquery对象 |
| setOnText | 设置开状态对应的文本 |  | jquery对象 |
| setOffText | 设置关状态对应的文本 |  | jquery对象 |
| setOnClass | 设置开状态的样式类 |  | jquery对象 |
| setOffClass | 设置关状态的样式类 |  | jquery对象 |
| destroy | 销毁开关 |  | jquery对象 |
