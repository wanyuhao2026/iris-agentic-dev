# 按钮(linkbutton)

- 官方页面标题：按钮
- 分类：表单
- 来源：http://hisui.cn/api/linkbutton/linkbutton.html
- 官网更新时间：Wed, 13 Mar 2024 08:55:18 GMT
- 提取时间：2026-07-25T13:18:46+00:00

> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。

## 本页目录

- 链接按钮
  - 如：
  - 深色可用类
  - 不同色系按钮
  - 禁用 事件处理
  - 大图按钮

## 链接按钮

链接按钮（Link Button）是使用 <a>元素来创建的，所以实际上一个链接按钮（Link Button）就是一个显示为按钮样式的 <a>元素。

### 如：

```html
<a href="#" class="hisui-linkbutton" data-options="iconCls:'icon-w-find'" disabled>禁用</a>            
<a href="#" class="hisui-linkbutton" data-options="iconCls:'icon-w-find'" style="width:200px">自定义宽度按钮</a>
<a href="#" class="hisui-linkbutton" data-options="iconImg:'update.png'">自定义图片按钮</a>
<a href="#" class="hisui-linkbutton" id="btn1">点我提示</a>
<script type="text/javascript">
    $(function () {
        $("#btn1").click(function (argument) {
            alert("你确定？");
        });
    });
</script>
```

### 深色可用类

如果需要鼠标悬浮时按钮变色更深，在class中加入 hover-dark .如病人信息条.

```html
<a href="#" class="hisui-linkbutton hover-dark" data-options="iconCls:'icon-w-find'">时间线查看</a>
<a href="#" class="hisui-linkbutton hover-dark">确诊</a>
```

### 不同色系按钮

按钮默认是蓝色，如果需要使用其它色系的按钮，在class中加入 green 、 yellow 、或者 red ，按钮就会改变色系

其中 green 表示通过， yellow 表示警示， red 表示拒绝

### 禁用 事件处理

原来按钮禁用后，通过jq绑定的事件，还是会被触发，现增加一属性 stopAllEventOnDisabled ,当为true时，阻止事件

通过监听生成的子元素span的点击事件，当stopAllEventOnDisabled为true，且按钮被禁用时，阻止事件冒泡

```javascript
$.parser.onComplete = function(context){
                $("#event1").click(function () {
                   alert("jquery为按钮【上一步】点击事件");
                });
                $("#event2").click(function () {
                   alert("jquery为按钮【下一步】点击事件");
                });
                $("#event1").linkbutton('disable');
                $("#event2").linkbutton('disable');

            };
```

```html
<a href="#" id="event1" class="hisui-linkbutton " >上一步</a>  原本的事件会触发
<br><br>
<a href="#" id="event2" class="hisui-linkbutton " data-options="stopAllEventOnDisabled:true">下一步</a>  属性<code>stopAllEventOnDisabled</code>为true时，事件不会触发
<script type="text/javascript"> 
    $(function () {
        $("#event1").click(function () {
            alert("jquery为按钮【上一步】点击事件");
        });
        $("#event2").click(function () {
            alert("jquery为按钮【下一步】点击事件");
        });
        $("#event1").linkbutton('disable');
        $("#event2").linkbutton('disable');

    });
</script>
```

```javascript
$(function () {
        $("#btn1").click(function (argument) {
           alert("你确定？");
        });
    });
    function goIconTab(){
        parent.addTab("图标(icon)", "icon/icon.html")
    }
```

### 大图按钮

在 class 中加入 big ，
同时 data-options 中加入 plain:true 可实现大图按钮功能。
大图标定义请查看 图标(icon) 界面

```html
<a class="hisui-linkbutton big" data-options="iconCls:'icon-big-save',plain:true">保存</a>
<a class="hisui-linkbutton big" data-options="iconCls:'icon-big-print',plain:true">打印</a>
<a class="hisui-linkbutton big" data-options="iconCls:'icon-big-del',plain:true">删除</a>
<div class="datagrid-btn-separator big"></div>
<a class="hisui-linkbutton big" data-options="iconCls:'icon-big-unlock',plain:true">手工解锁</a>
```

| 属性 | 说明 | 默认值 |  |
| --- | --- | --- | --- |
| plain | 是否素色 | false | 为true时,按钮素色 |
| iconImg | 图片地址 | null | 'unlock.png' |
| stopAllEventOnDisabled | 在disabled时是否阻止按钮事件 | false | 通过监听生成的子元素span的点击事件，当stopAllEventOnDisabled为true，且按钮被禁用时，阻止事件冒泡。 |
| notTrans | 不自动翻译与否 | false | 默认自动翻译。true表示不翻译 20230801 |
| clickWaitingTime | 点击后禁用时长(毫秒) | 200 | 默认禁用200毫秒后激活; 点击后把按钮设置成waiting状态，禁用再次点击 20231211 |
| waitingAlert | waiting状态的按钮再次点击时提示内容 | 默认提示：'按钮已点击过,系统响应中,请等待...' | 为空时不提示 20231211 |
