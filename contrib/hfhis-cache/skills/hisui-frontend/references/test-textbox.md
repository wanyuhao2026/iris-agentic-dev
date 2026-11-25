# 文本框(textbox)

- 官方页面标题：查询框示例
- 分类：测试
- 来源：http://hisui.cn/api/test/textbox.html
- 官网更新时间：Tue, 12 Mar 2024 03:51:34 GMT
- 提取时间：2026-07-25T13:18:46+00:00

> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。

### 基本文本框

```html
<div class="hisui-panel" style="width:400px;padding:30px 30px" title="注册">
<div style="margin-bottom:20px">
<div>邮箱</div>
<input class="hisui-textbox" data-options="prompt:'提示',validType:'email'" id="email" style="width:300px;height:32px"/>
</div>
<div style="margin-bottom:20px">
<div>姓名</div>
<input class="hisui-textbox" id="name" style="width:300px;height:32px"/>
</div>
<div style="margin-bottom:20px">
<div>公司</div>
<input class="hisui-textbox" id="company" style="width:300px;height:40px"/>
</div>
<div style="margin-bottom:20px">
<div>备注</div>
<input class="hisui-textbox" data-options="multiline:true" id="note" style="width:300px;height:100px" value="张三是个好人，李四是个好人，王二也是一个好人。。。。哈哈哈哈"/>
</div>
<div style="margin-bottom:20px">
<input class="hisui-textbox" data-options="multiline:true" id="note" label="label" labelposition="top" style="width:300px;height:100px" value="张三是个好人，李四是个好人，王二也是一个好人。。。。哈哈哈哈"/>
</div>
</div>
```
