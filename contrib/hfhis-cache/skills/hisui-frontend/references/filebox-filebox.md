# 文件框(filebox)

- 官方页面标题：文件选择框
- 分类：表单
- 来源：http://hisui.cn/api/filebox/filebox.html
- 官网更新时间：Tue, 12 Mar 2024 03:51:27 GMT
- 提取时间：2026-07-25T13:18:46+00:00

> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。

## 本页目录

- 文件选择框
  - 说明:
  - 如：

## 文件选择框

### 说明:

用于选择客户端文件

### 如：

```html
<div class="hisui-panel" title="登记" style="width:700px;padding:10px;"
	data-options="iconCls:'icon-paper',headerCls:'panel-header-gray',closable:false,collapsible:false,minimizable:false,maximizable:false">
	<table cellpadding="0">
		<tr>
			<td class="label">文件1</td>
			<td>
				<input class="hisui-filebox" name="file1" data-options="width:400,plain:true" />
			</td>
		</tr>
		<tr>
			<td class="label">文件2</td>
			<td>
				<input class="hisui-filebox" name="file2"
					data-options="width:400,buttonText:'选择',prompt:'excel文件：*.xls,*.xlsx',plain:true"
					accept="application/vnd.ms-excel,application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" />
			</td>
		</tr>
		<tr>
			<td class="label">文件3</td>
			<td>
				<input class="hisui-filebox" name="file3" data-options="width:400,plain:true,buttonText:'选择',onClickButton:function(){alert('按钮点击');},
				onChange:function(nv,ov){alert('改变:新:'+nv+',旧:'+ov);} " />
			</td>
		</tr>
		<tr>
			<td class="label">文件4</td>
			<td>
				<input class="hisui-filebox" name="file4" style="width:400px;"
					data-options="multiple:true,plain:true,buttonText:'多选',prompt:'请选择文件(多选的哦..)'" />
			</td>
		</tr>
		<tr>
			<td class="label">文件5</td>
			<td>
				<input id="f5" class="hisui-filebox" name="file5" style="width:400px;"
					data-options="prompt:'请选择文件',plain:true" />
			</td>
			<td style="padding-left: 10px;"><a href="#" id='btn' onclick="$('#f5').filebox('clear')"
					class="hisui-linkbutton">清空</a></td>
		</tr>
		<tr>
			<td class="label">文件6</td>
			<td>
				<input id="f6" name="f6" />
			</td>
		</tr>
		<tr>
			<td class="label">文件7</td>
			<td>
				<input id="f7" name="f7" />
			</td>
		</tr>
		<tr>
			<td class="label">文件8</td>
			<td>
				<input class="hisui-filebox" name="file8"
					data-options="width:400,buttonText:'选择',buttonIcon:'icon-folder',plain:true" />
			</td>
		</tr>
		<tr>
			<td class="label">文件9</td>
			<td>
				<input class="hisui-filebox" name="file9"
					data-options="width:400,buttonText:'',buttonIcon:'icon-folder',plain:true" />
			</td>
		</tr>
		<tr>
			<td class="label">文件10</td>
			<td>
				<input id='f10' class="hisui-filebox" name="file10" disabled='true'
					data-options="width:400,buttonText:'选择',plain:true" />
			</td>
			<td style="padding-left: 10px;">
				<a href="#" onclick="$('#f10').filebox('enable')" class="hisui-linkbutton">启用</a>
			</td>
			<td style="padding-left: 10px;">
				<a href="#" onclick="$('#f10').filebox('disable')" class="hisui-linkbutton">禁用</a>
			</td>
		</tr>
	</table>
</div>
```

```javascript
$(function () {
	$('#f6').filebox({
		width: 400,
		prompt: '请选择文件',
		buttonText: '选择',
		buttonAlign: 'left',
		plain: true
	})

	$HUI.filebox('#f7', {
		width: 400,
		plain: true
	})

})
```

| 属性 | 说明 | 默认值 |  |
| --- | --- | --- | --- |
| buttonText | 按钮文字 | 'Choose File' |  |
| buttonAlign | 按钮位置 | 'right' | 'right'\|'left' |
| accept | 规定能够通过文件上传进行提交的文件类型 | '' | w3cSchool:请避免使用该属性。应该在服务器端验证文件上传 |
| capture | 可以捕获到系统默认的设备 | '' | camera--照相机；camcorder--摄像机；microphone--录音 |
| multiple | 多选 | false | true\|false |
| separator | 分隔符 | ',' | 多选时文件名分隔符 |
| prompt | 提示 | '' | 未选择文件时提示文字 |
| width | 宽度 | 177 | 整体宽度 |
| disabled | 禁用 | false | true\|false |
| buttonIcon | 图标类 | null | 一般使用 buttonIcon:'icon-folder',plain:true |
| plain 2019-12-23 | 图标背景色是否透明 | false | true\|false |
| 事件名 | 说明 | 入参 |  |
| onClickButton | 按钮点击事件 |  |  |
| onChange | 改变事件 | newVal,oldVal |  |
| 方法名 | 说明 | 入参 | 返回值 |
| options | 拿到配置项对象 |  | filebox的options对象 |
| files | 获取选定的文件列表对象 |  | 选定的文件列表对象 |
| clear | 清空 |  | jq对象 |
| reset | 清空 |  | jq对象 |
| button | 获取按钮jq对象 |  | 返回按钮jq对象 |
| enable | 启用 |  | jq对象 |
| disable | 禁用 |  | jq对象 |
