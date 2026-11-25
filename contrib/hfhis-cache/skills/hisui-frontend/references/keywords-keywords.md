# 关键字列表(keywords)

- 官方页面标题：关键字集组件
- 分类：表单
- 来源：http://hisui.cn/api/keywords/keywords.html
- 官网更新时间：Tue, 12 Mar 2024 03:51:29 GMT
- 提取时间：2026-07-25T13:18:46+00:00

> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。

## 本页目录

- 关键字集组件( keywords )
  - 说明：
  - 一.多选关键字
  - 二.红色的标签页( labelCls:'red' )

## 关键字集组件( keywords )

### 说明：

可以用于关键字多选，或切换关键字功能。

### 一.多选关键字

```html
<div id="kw"></div>
```

```javascript
$(function(){
            $("#kw").keywords({
                onClick:function(v){console.log("点击->"+v.text)},
                onUnselect:function(v){console.log("取消选择->"+v.text);},
                onSelect:function(v){console.log("选择->"+v.text);},
                items:[{
                        text:"诊断",
                        type:"chapter",
                        items:[
                            {
                                text:'西医诊断',type:"section",items:[
                                    {text:'上呼吸道感染',id:"x1"},{text:'接触性皮炎',id:"x2"},{text:'过敏性皮炎',id:'x3'},{text:'神经性皮炎',id:'x4'},
                                    {text:'腰间盘突出',id:'x5',disabled:true},{text:'腰肌老损',id:'x6'},{text:'急性病毒性肝炎',id:'x7'},{text:'中风后遗症',id:'x8'},{text:'高血压脑病',id:'x9'},{text:'过敏性休克',id:'x10'},
                                    {text:'急性酒精中毒',id:'x11',selected:true},{text:'神经性脑鸣',id:'x12'},{text:'偏头痛',id:'x13'},{text:'血管性神经头痛',id:'x14'},{text:'面神经麻痹',id:'x15'}
                                ]
                            },
                            {
                                text:"中医诊断",type:'section',items:[
                                    {text:'肝热病',id:"z1"},{text:'白喉',id:"z2"},{text:'麻疹',id:'z3'},{text:'中恶',id:'z4'},
                                    {text:'百日咳',id:'z5'},{text:'吸虫病',id:'z6'}
                                ]
                            }
                        ]
                    },{
                        text:"病历",    /*无段落*/ 
                        type:"chapter",
                        items:[
                            {text:"住院病历"},
                            {text:"门诊病历"}
                        ]
                    },{
                        text:"过敏药品", /*无章节情况*/
                        type:"section",
                        items:[
                            {text:"青霉素"},{text:"链霉素"},
                            {text:"磺胺药"},{text:"阿司匹林"},
                            {text:"痢特灵"},{text:"保泰松"},{text:"巴比妥类药普鲁卡因"},
                        ]
                    },{
                        text:"高质", /*text:""时章节名称为空时不显示蓝条,但有横线*/
                        type:"chapter",
                        items:[{
                            text:"高质耗材", 
                            type:"section",
                            items:[
                                {text:"透析管路"},{text:"心脏起搏器"},{text:"人工关节"},
                                {text:"人工瓣膜"},{text:"人工补片"},{text:"人工血管"}
                            ]
                        }]
                    }]
            });
        });
```

### 二.红色的标签页( labelCls:'red' )

```html
<div id="redkw"></div>
```

```javascript
$(function(){
            $("#redkw").keywords({
                singleSelect:true,
                onClick:function(v){console.log("点击->"+v.text)},
                onUnselect:function(v){console.log("取消选择->"+v.text);},
                onSelect:function(v){console.log("选择->"+v.text);},
                labelCls:'red',
                items:[
                    {text:'知识库',id:'red1'},
                    {text:'医嘱',id:'red2',disabled:true},
                    {text:'中草药',id:'red3'},
                    {text:'检查',id:'red4',selected:true},
                    {text:'检验',id:'red5'},
                    {text:'生命体征',id:'red6'},
                    {text:'诊断',id:'red7'},
                    {text:'病理',id:'red8'}
                ]
            });
            function btnHandler(){
                var rs = $("#kw").keywords("getSelected");
                console.log(rs);
                $("#kw").keywords("clearAllSelected");
            }
            $("#btn").click(btnHandler);
        })
```

| 配置项 | 说明 | 默认值 | 其它 |
| --- | --- | --- | --- |
| items | 具体元素数组。元素属性包括：text,id,selected,disabled |  |  |
|  | text | null | 标签文本。必需项 |
|  | id | null | 标签id。可选项 |
|  | selected | false | 选中当前标签。可选项 |
|  | disabled | false | 禁用当前标签。可选项 20230822 |
| singleSelect | 当前是否为单选模式 | false | true时为单选 |
| labelCls | 选择块样式名 | 'blue' | 可选项 'red' , 'blue' |
| 方法名 | 说明 | 入参 | 返回值 |
| select | 选中对象,入参为元素的id | id | 无 |
| getSelected | 获得所有已选对象 | 无 | Array |
| switchById | 切换元素的选中状态,入参为元素的id | id | 无 |
| clearAllSelected | 清除所有选中 | 无 | 无返回值.不触发事件. 20180822 |
| 事件名 | 说明 | 入参 | 类型 |
| onClick | 当前点击对象 | item |  |
| onUnselect | 当前取消选中对象 | item | 单选时不会触发onUnselect事件 |
| onSelect | 当前选中对象 | item | 单选，多选都会触发 20220124 |
