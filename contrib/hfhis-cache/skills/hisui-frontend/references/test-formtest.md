# 表单们(formtest)

- 官方页面标题：查询框示例
- 分类：测试
- 来源：http://hisui.cn/api/test/formtest.html
- 官网更新时间：Tue, 12 Mar 2024 03:51:34 GMT
- 提取时间：2026-07-25T13:18:46+00:00

> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。

## 本页目录

- 基础查询框
  - 说明:
  - 如：
  - 禁用的表单们

## 基础查询框

### 说明:

基础查询框（searchbox）用于查询。

### 如：

```html
<div class="hisui-panel" title="查询" style="width:400px;padding:10px 30px 20px 30px" data-options="iconCls:'icon-save'">
	<table cellpadding="5">
		<tr>
			<td>查询框</td>
			<td>
				<input id="ss" class="hisui-searchbox " data-options="width:177" />
			</td>
		</tr>
		<tr>
			<td>时间框</td>
			<td>
				<input class="hisui-timespinner" data-options="width:177">
			</td>
		</tr>
		<tr>
			<td>数字框</td>
			<td>
				<input id="ns" class="hisui-numberspinner " data-options="width:177"></input>
			</td>
		</tr>	
		<tr>
			<td>数字框</td>
			<td>
				<input class="hisui-numberbox textbox" >
			</td>
		</tr>
		<tr>
			<td>普通框</td>
			<td>
				<input id="patno" class="textbox">
			</td>
		</tr>
		<tr>
			<td>选择框</td>
			<td>
				<select id="stateBox" class="hisui-combobox" name="state" style="width:177px;">
					<option value="AK">Alaska</option>
					<option value="CT">Connecticut</option>
					<option value="DE">Delaware</option>
					<option value="LA">Louisiana</option>
					<option value="MD">Maryland</option>
					<option value="ND">North Dakota</option>
					<option value="OH" selected>Ohio</option>
					<option value="WY">Wyoming</option>
				</select>
			</td>
		</tr>
		<tr>
			<td>日期框</td>
			<td>
				<input id="db" class="hisui-datebox " data-options="width:177"></input>
			</td>
		</tr>
		<tr>
			<td>日时框</td>
			<td>
				<input id="dtb" class="hisui-datetimebox " data-options="width:177"></input>
			</td>
	
	</table>
</div>
```

### 禁用的表单们

```html
<div class="hisui-panel" title="查询" style="width:400px;padding:10px 30px 20px 30px" data-options="iconCls:'icon-save'">
	<table cellpadding="5">
		<tr>
			<td>查询框</td>
			<td>
				<input id="ss2" class="hisui-searchbox " data-options="width:177,disabled:true" />
			</td>
		</tr>
		<tr>
			<td>时间框</td>
			<td>
				<input class="hisui-timespinner" data-options="width:177,disabled:true" id="mytsp"/>
			</td>
		</tr>
		<tr>
			<td>数字框</td>
			<td>
				<input id="ns" class="hisui-numberspinner " data-options="width:177,disabled:true"></input>
			</td>
		</tr>	
		<tr>
			<td>数字框</td>
			<td>
				<input class="hisui-numberbox textbox" disabled>
			</td>
		</tr>
		<tr>
			<td>普通框</td>
			<td>
				<input id="patno2" class="textbox" disabled>
			</td>
		</tr>
		<tr>
			<td>选择框</td>
			<td>
				<select id="stateBox2" class="hisui-combobox" name="state" style="width:177px;" data-options="disabled:true">
					<option value="AK">Alaska</option>
					<option value="CT">Connecticut</option>
					<option value="DE">Delaware</option>
					<option value="LA">Louisiana</option>
					<option value="MD">Maryland</option>
					<option value="ND">North Dakota</option>
					<option value="OH" selected>Ohio</option>
					<option value="WY">Wyoming</option>
				</select>
			</td>
		</tr>
		<tr>
			<td>日期框</td>
			<td>
				<input id="db2" class="hisui-datebox " data-options="width:177,disabled:true"></input>
			</td>
		</tr>
		<tr>
			<td>日时框</td>
			<td>
				<input id="dtb2" class="hisui-datetimebox " data-options="width:177,disabled:true"></input>
			</td>
		</tr>
	</table>
</div>
```

```javascript
$(function(){
				$('#form_clear').click(function(){
					$('#form').form('clear');
					$.messager.popover({msg:'reset',type:'clear'})
				})
				$('#form_reset').click(function(){
					
					$('#form').form('reset');
					$.messager.popover({msg:'reset',type:'info'})
				})
			})
```
