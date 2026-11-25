# 下拉树(combotree)

- 官方页面标题：下拉树
- 分类：表格及树
- 来源：http://hisui.cn/api/combotree/combotree.html
- 官网更新时间：Tue, 12 Mar 2024 03:51:25 GMT
- 提取时间：2026-07-25T13:18:46+00:00

> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。

## 组合树

### 说明:

组合树（combotree）把选择控件和下拉树结合起来。它与组合框（combobox）相似，不同的是把列表替换成树组件。组合树（combotree）支持带有用于多选的树状态复选框的树。

### 如：

```javascript
var cbtree = $HUI.combotree('#cbtree',{editable:true});
            cbtree.loadData([{
                id: 1,
                text: 'Languages',
                children: [{
                    id: 11,
                    text: 'Java'
                },{
                    id: 12,
                    text: 'C'
                },{
                    id: 13,
                    text: 'C++'
                },{
                    id: 14,
                    text: 'CSS'
                }]
            }]);
```

```html
<table>
    <tr>
        <td class="r-label">下拉树</td>
        <td><input id="cbtree" class="hisui-combotree" placeholder='请点击下拉'/></td>
    </tr>
</table>
```

```javascript
<script type="text/javascript">
var cbtree = $HUI.combotree('#cbtree',{});
    cbtree.loadData([{
        id: 1,
        text: 'Languages',
        children: [{
            id: 11,
            text: 'Java'
        },{
            id: 12,
            text: 'C++'
        }]
    }]);
</script>
```
