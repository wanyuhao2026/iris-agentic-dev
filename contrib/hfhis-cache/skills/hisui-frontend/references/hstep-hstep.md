# 水平步骤(hstep)

- 官方页面标题：横向节点图
- 分类：布局
- 来源：http://hisui.cn/api/hstep/hstep.html
- 官网更新时间：Tue, 12 Mar 2024 03:51:27 GMT
- 提取时间：2026-07-25T13:18:46+00:00

> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。

## 横向节点图组件( hstep )

### 说明：

可用于步骤图，流程图，闭环图等。

### 一.步骤图

```javascript
$(function(){
    $('#prevbtn').click(function(){
        $('#hstp').hstep('prevStep');
    });
    $('#nextbtn').click(function(){
        $('#hstp').hstep('nextStep');
    });
    $("#hstp").hstep({
        //showNumber:false,
        stepWidth:200,
        currentInd:3,
        onSelect:function(ind,item){console.log(item);},
        //titlePostion:'top',
        items:[{
                title:'挂号',
                context:"<div class='cntt'><div>王二</div><div>2020-07-03 09:10</div></div>"
            },{
                title:'就诊',
                context:"<div class='cntt'><div>张三</div> 2020-07-03 10:10</div>"
            },{
                title:"收费", 
                context:"<div class='cntt'><div>李四</div> 2020-07-03 11:10</div>"
            },{
                title:"取药"
            },{
                title:"完成"
            }]
    });
});
```

| 配置项 | 说明 | 默认值 | 其它 |
| --- | --- | --- | --- |
| currentInd | 当前所在节点 | 1 |  |
| stepWidth | 单步宽度 | 100 |  |
| items | 具体节点配置 | Object | {
title:'title',
context:"html/text"
} |
| 方法名 | 说明 | 入参 | 返回值 |
| setStep | 设置当前流程处于哪个节点 | ind |  |
| getStep | 获得当前流程处于哪个节点 |  |  |
| getStepInd | 获得当前流程处于的索引位置 |  |  |
| prevStep | 上一节点 |  | 无 |
| nextStep | 下一节点 |  |  |
| 事件名 | 说明 | 入参 | 类型 |
| onSelect | 当前选中对象 | ind,item |  |
