# 下拉表格(combogrid)

- 官方页面标题：下拉表格
- 分类：表格及树
- 来源：http://hisui.cn/api/combogrid/combogrid.html
- 官网更新时间：Thu, 13 Nov 2025 08:41:20 GMT
- 提取时间：2026-07-25T13:18:46+00:00

> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。

## 组合网格

### 说明:

组合网格（combogrid）把可编辑的文本框和下拉数据网格面板结合起来，用户可以从下拉数据网格面板中快速查找和选择。组合网格（combogrid）提供了选择某个项目的键盘导航支持。

### 如：

```html
<select class="hisui-combogrid" id="combogrid" style="width:250px" data-options="
            panelWidth: 500,
            blurValidValue:true,
            idField: 'itemid',
            textField: 'productname',
            method: 'get',
            columns: [[
                {field:'itemid',title:'Item ID',width:80},
                {field:'productname',title:'Product',width:120},
                {field:'listprice',title:'List Price',width:80,align:'right'},
                {field:'unitcost',title:'Unit Cost',width:80,align:'right'},
                {field:'attr1',title:'Attribute',width:200},
                {field:'status',title:'Status',width:60,align:'center'}
            ]],
            fitColumns: true
        ">    
</select>
```

```javascript
<script type="text/javascript">
    $(function(){
        var trObj = $HUI.combogrid("#combogrid");
        var grid = trObj.grid();
        grid.datagrid("loadData",{"total":5,"rows":[
            {"itemid":"EST-1","productname":"Koi","listprice":36.50,"unitcost":10.00,"attr1":"Large","status":"P"},
            {"itemid":"EST-10","productname":"Dalmation","listprice":18.50,"unitcost":12.00,"attr1":"Spotted Adult Female","status":"P"},
            {"itemid":"EST-11","productname":"Rattlesnake","listprice":38.50,"unitcost":12.00,"attr1":"Venomless","status":"P"},
            {"itemid":"EST-12","productname":"Rattlesnake","listprice":26.50,"unitcost":12.00,"attr1":"Rattleless","status":"P"},
            {"itemid":"EST-13","productname":"Iguana","listprice":35.50,"unitcost":12.00,"attr1":"Green Adult","status":"P"}
        ]});
    });
</script>
```

| 属性 | 说明 | 默认值 |  |
| --- | --- | --- | --- |
| enterNullValueClear | 默认值true。当为false时，在输入框内回车，没有匹配值不清空输入框。 | true |  |
| blurValidValue | 为true时，光标离开时，检查是否选中值,没选中则置空输入框的值。 2018-12-26 | false |  |
| paginationNoWrap | 默认值false。当为true时,保存翻页条不折行 2025-11-13 | false |  |
