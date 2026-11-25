# 泡芙提示（popover）

- 官方页面标题：提示框
- 分类：基础
- 来源：http://hisui.cn/api/popover/popover.html
- 官网更新时间：Tue, 12 Mar 2024 03:51:32 GMT
- 提取时间：2026-07-25T13:18:46+00:00

> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。

## 提示框。源自jquery的 webui-popover 插件。

### 说明:

提示框（popover）是在某元素上显示一个提示说明界面。

### 如：

```javascript
$(function(){
				//var ccObj=$HUI.popover('#pp',{title:'许可证关于',content:'统一常用前端组件库，统一开发界面风格'});
				$("#pp").popover({title:'HUI关于',content:'统一常用前端组件库，统一开发界面风格'});
			});
```

### 手动触发提示

```javascript
$(function(){
				$("#manualpp").popover({
					trigger:'manual',placement:'top',title:'HUI关于',
					content:'HISUI是一款前端组件类库',//backdrop:true,
					delay:{show: null,hide:500}
				});
				$("#showPP").click(function(){
					$("#manualpp").popover('show');
				});
				$("#hidePP").click(function(){
					$("#manualpp").popover('hide');
				});
			});
```

### 提示组件的相关配置、事件、方法

| 属性 | 说明 | 默认值 |  |
| --- | --- | --- | --- |
| placement | 提示层弹出方位.
auto,top,right,bottom,left, top-right,top-left,bottom-right,bottom-left, auto-top,auto-right,auto-bottom,auto-left,horizontal,vertical | 'auto' |  |
| width | 提示层宽度,可以为数字 | 'auto' |  |
| height | 提示层高度,可以为数字 | 'auto' |  |
| trigger | 触发动作(click,hover,manual(手动控制),sticky(创建完显示);) | 'click' |  |
| animated | 动画效果('pop','fade') | null |  |
| style | 样式('','inverse') | '' |  |
| cache | 如果为false则提示层将销毁或重建 | true |  |
| multi | 如果为true则允许界面同时显示多个提示层 | false |  |
| arrow | 是否显示箭头 | true |  |
| title | 提示层标题 | '' |  |
| content | 提示层内容 | '' |  |
| closeable | 是否显示关闭按钮 | false |  |
| direction | 文字显示方向(ltr,rtl).默认ltr(左到右) | '' |  |
| padding | 是否增加内容padding | true |  |
| type | 内容类型('html','iframe','async') | 'html' |  |
| url | 如果type为'html'时,url可以配置jQuery选择器,如果type为'async',则通过url加载内容) | '' |  |
| backdrop | 如果backdrop为true时,弹出层将模态化打开 | false |  |
| dismissible | 为true时按esc键或点击非提示层位置，关闭提示层 | true |  |
| autoHide | 值为false或数字(1000=1s) | false |  |
| offsetTop | 弹出层与顶边偏移量 | 0 |  |
| offsetLeft | 弹出层与左边偏移量 | 0 |  |
| 事件名 | 说明 | 入参 |  |
| onShow | 显示时触发 | e,value |  |
| onHide | 隐藏时触发 | e,value |  |
| 方法名 | 说明 | 入参 | 返回值 |
| options | 拿到配置项对象 |  | jquery对象 |
| show | 显示弹出层 |  | jquery对象 |
| hide | 隐藏弹出层 |  |  |
| destroy | 销毁弹出层 |  |  |
| setContent | 修改弹出层内容 | value | jquery对象 |
