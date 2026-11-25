# 下拉框(combobox)

- 官方页面标题：下拉框
- 分类：表单
- 来源：http://hisui.cn/api/combobox/combobox.html
- 官网更新时间：Tue, 09 Apr 2024 13:42:32 GMT
- 提取时间：2026-07-25T13:18:46+00:00

> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。

## 本页目录

- 组合框
  - 说明:
  - 如： 查看EasyUI 1.3.2的combobox说明
  - 一、html生成的单选下拉框
  - 二、通过配置项 rowStyle 生成带勾选框的多选下拉框
  - 三、通过 formatter 生成带勾选状态的多选下拉框（icon-ok）
  - 四、通过 defaultFilter 配置默认过滤规则
  - 五、远程数据
  - 六、单选且不可编辑
  - 七、数据延迟

## 组合框

### 说明:

组合框（combobox）显示一个可编辑的文本框和下拉列表，用户可以从下拉列表中选择一个或多个值。用户可以直接输入文本到列表的顶部，或者从列表中选择一个或多个现成的值。

### 如： 查看EasyUI 1.3.2的combobox说明

### 一、html生成的单选下拉框

```html
<select id="stateBox" class="hisui-combobox" name="state" style="width:200px;" data-options="enterNullValueClear:false,onSelect:selectHandler,blurValidValue:true">
	<option value="AK">Alaska</option>
	<option value="CT">Connecticut</option>
	<option value="DE">Delaware</option>
	<option value="LA">Louisiana</option>
	<option value="MD">Maryland</option>
	<option value="ND">North Dakota</option>
	<option value="OH" selected>Ohio</option>
	<option value="WY">Wyoming</option>
</select>
默认选择selected为Ohio
```

### 二、通过配置项 rowStyle 生成带勾选框的多选下拉框

```html
<select id="cbox" style="width:200px;"></select>
```

```javascript
$(function(){
	var cbox = $HUI.combobox("#cbox",{
		valueField:'id',
		textField:'text',
		multiple:true,
		rowStyle:'checkbox', //显示成勾选行形式
		selectOnNavigation:false,
		panelHeight:"auto",
		editable:false,
		data:[
			{id:'css',text:'CSS语言'},{id:'html',text:'HTML语言'},{id:'c',text:'C语言'}
			,{id:'cplus',text:'C++语言'},{id:'java',text:'JAVA语言'},{id:'cache',text:'M语言'},{id:'sql',text:'结构化查询语言'}
		]
	});
	cbox.setValues([]);
	$("#cboxbtn").click(function(){
		console.log(cbox.getValues());
		//cbox.setValues([]);
		//cbox.loadData([{id:'m',text:'m语言'}]);
	});
	$("#cboxselbtn").click(function(){
		cbox.setValues(["css","html","c","cplus","java","cache","sql"]);
		//$("#cbox").combobox('panel').parent().find('._hisui_combobox-selectall').trigger('click');
	});
});
```

```javascript
var $URL = "rtcb";
	function selectHandler(rec){

		console.log({onSelect:rec});
		console.log($("#stateBox").combobox('getValue'));
		//$.messager.popover({msg: rec.text,type:'info',timeout: 2000,showType: 'show'});
	}

function onComplete2(){
	var cbox = $HUI.combobox("#cbox",{
		valueField:'id',
		textField:'text',
		multiple:true,
		rowStyle:'checkbox',
		selectOnNavigation:false,
		allSelectButtonPosition:'top',
		onAllSelectClick:function(e){
			var arr = $(this).combobox("getValues");
			console.log(arr);
		},
		data:[
			{id:'css',text:'CSS语言'}
			,{id:'html',text:'HTML语言'}
			,{id:'c',text:'C语言'}
			,{id:'cplus',text:'C++语言'}
			,{id:'java',text:'JAVA语言'}
			,{id:'cache',text:'M语言'}
			,{id:'sql',text:'结构化查询语言'}
		],
		editable:false,
		panelHeight:"200" //"auto"
		//onChange:function(newval,oldval){
			/*$(this).combobox("panel").find('input').prop('checked',false);
			for (var i=0;i<newval.length;i++){
				$(this).combobox("panel").find('#r'+newval[i]).prop('checked',true);
			}*/
		//}
	});
	cbox.setValues([]);
	$("#cboxbtn").click(function(){
		console.log(cbox.getValues());
		//cbox.setValues([]);
		//cbox.loadData([{id:'m',text:'m语言'}]);
	});
	/*回车事件测试*/
	$('#stateBox').combobox('textbox').bind('keydown',function(event){ if (event.keyCode == 13) {
		$('#iconokBox').combobox('textbox').focus();
    }})
	$("#cboxbtn1").click(function(){
		console.log($("#stateBox").combobox("getValue"));
	});
	$("#cboxselbtn").click(function(){
		cbox.setValues(["css","html","c","cplus","java","cache","sql"])
		//$("#cbox").combobox('panel').parent().find('._hisui_combobox-selectall').trigger('click');
	});
	return ;
	// easyloader.theme = "bootstrap";
	// easyloader.locale="zh_CN";
	// easyloader.load(['combobox'],function(){
	// 	$('#cbox').combobox({
	// 		valueField:'id',
	// 		textField:'text',
	// 		data:[{id:'java',text:'java语言'},{id:'css',text:'css语言'}],
	// 		onselect:function(rec){
	// 			alert(1);
	// 		}
	// 	});
	// });
};
```

### 三、通过 formatter 生成带勾选状态的多选下拉框（icon-ok）

```html
<style>.icon{width: 20px;height: 20px;margin-right: 10px;float: right;}</style>
<select id="iconokBox" style="width:200px;"></select>
```

```javascript
$(function(){
	var cbox = $HUI.combobox("#iconokBox",{
		valueField:'id', textField:'text', multiple:true,selectOnNavigation:false,panelHeight:"auto",editable:true,
		data:[
			{id:'css',text:'CSS语言'},{id:'html',text:'HTML语言'}
			,{id:'c',text:'C语言'},{id:'cplus',text:'C++语言'}
			,{id:'java',text:'JAVA语言'},{id:'cache',text:'M语言'}
			,{id:'sql',text:'结构化查询语言'}
		],
		formatter:function(row){  
			var rhtml;
			if(row.selected==true){
				rhtml = row.text+"<span id='i"+row.id+"' class='icon icon-ok'></span>";
			}else{
				rhtml = row.text+"<span id='i"+row.id+"' class='icon'></span>";
			}
			return rhtml;
		},
		onChange:function(newval,oldval){
			$(this).combobox("panel").find('.icon').removeClass('icon-ok');
			for (var i=0;i<newval.length;i++){
				$(this).combobox("panel").find('#i'+newval[i]).addClass('icon-ok');
			}
		}
	});
  });
```

### 四、通过 defaultFilter 配置默认过滤规则

```javascript
$(function(){
	var cbox = $HUI.combobox("#dfCB",{
		valueField:'id', textField:'text',panelHeight:"auto",
		data:[
			{id:'1',text:'高血压病',spell:'GXYB'}
			,{id:'2',text:'伤寒性肝炎',spell:'SHXGY'}
			,{id:'3',text:'猪霍乱沙门菌败血症',spell:'ZHLSMJBXZ'}
			,{id:'4',text:'细菌性食物中毒',spell:'XJXSWZD'}
			,{id:'5',text:'骨髓增生异常综合征',spell:'GSZSYCZHZ'}
			,{id:'6',text:'难治性贫血',spell:'NZXPX'}
			,{id:'7',text:'多尿',spell:'DN'}
		],
		defaultFilter:6
	});
});
```

```javascript
var onComplete4 = function(){
	var div=$('<div style="padding:10px;display:inline"></div>').insertAfter('#dfCB');
	var cbox = $HUI.combobox("#dfCB",{
		valueField:'id', textField:'text',panelHeight:"auto",
		data:[
			{id:'1',text:'高血压病',spell:'GXYB'}
			,{id:'2',text:'伤寒性肝炎',spell:'SHXGY'}
			,{id:'3',text:'猪霍乱沙门菌败血症',spell:'ZHLSMJBXZ'}
			,{id:'4',text:'细菌性食物中毒',spell:'XJXSWZD'}
			,{id:'5',text:'骨髓增生异常综合征',spell:'GSZSYCZHZ'}
			,{id:'6',text:'难治性贫血',spell:'NZXPX'}
			,{id:'7',text:'多尿',spell:'DN'}
		],
		defaultFilter:6
	});


	$('<span style="padding-left:20px;padding-right:10px;color:red;font-weight:bold;">defaultFilter</span>').appendTo(div);
	for(var i=1;i<=6;i++){
		$('<input type="radio" name="dfCB-df" id="dfCB-df-'+i+'" label="值'+i+'" value="'+i+'"/>').appendTo(div);
	}
	$('input[type=radio][name=dfCB-df]').radio({
		onChecked:function(){
			$('#dfCB').combobox('options').defaultFilter=parseInt( $(this).attr('value'));
			$('#dfCB').combobox('clear').combobox('setValue','');
		}
	})
	$('#dfCB-df-6').radio('check');
	$('<span style="padding-left:20px;padding-right:10px;color:red;font-weight:bold;">spellField</span>').appendTo(div);
	$('<input type="radio" id="dfCB-spellField-empty" name="dfCB-spellField" label="值空" value=""/>').appendTo(div);
	$('<input type="radio" id="dfCB-spellField-spell" name="dfCB-spellField" label="值spell" value="spell"/>').appendTo(div);
	$('input[type=radio][name=dfCB-spellField]').radio({
		onChecked:function(){
			$('#dfCB').combobox('options').spellField=$(this).attr('value');
			$('#dfCB').combobox('clear').combobox('setValue','');
		}
	})
	$('#dfCB-spellField-empty').radio('check');


};
```

### 五、远程数据

```javascript
var onComplete5 = function(){
		$("#rtCB").combobox({
			valueField:'id', textField:'text',panelHeight:"auto",
			mode:'remote',
			url:$URL+"?ClassName=ClassName&QueryName=QueryName&ResultSetType=array"
		});
	};
```

```javascript
Mock.mock(/rtcb.+/,function(param){
			if (param.body){
				var b = param.body;
				if (b=="q=w"){
					return [{id:"w1",text:"wang1"},{id:"w2",text:"wang2"}];
				}
				else if (b=="q=h"){
					return [{id:"h1",text:"hang1"},{id:"h2",text:"hang2"}];
				}
				else if (b=="q=z"){
					return [{id:"1",text:"z1"},{id:"2",text:"z2"}];
				}
				
			}
			return [{id:1,text:"hang1"},{id:2,text:"hang2"},{id:"w1",text:"wang1"},{id:"w2",text:"wang2"},{id:"1",text:"z1"},{id:"2",text:"z2"}];
		});
```

### 六、单选且不可编辑

```javascript
$(function(){
	$("#editCB").combobox({editable:false,allowNull:true,...});
});
```

```javascript
var onComplete6 = function(){
		var cbox = $HUI.combobox("#editCB",{
			valueField:'id', textField:'text',panelHeight:"auto",editable:false,allowNull:true,
			mode:'local',data:[{id:1,text:'张三'},{id:2,text:'李四'},{id:3,text:'王二'},{id:4,text:'麻子'}]
		});
	};
```

### 七、数据延迟

```javascript
$(function(){
	$("#loaddataCB").combobox({  
		valueField: 'id',
		textField: 'text',
		value:20,data:[{id:20,text:"中国"}], //当前框中默认选中【中国】，如果默认空时可以去掉这行
		onShowPanel: function () { // 只有在下拉层显示时,才去关联url拉取数据,提高首屏速度
			var url = $(this).combobox('options').url;
			if (!url){
				//$(this).combobox('options').mode = 'remote';
				var url = $URL+"?ClassName=web.Nation&QueryName=Find&ResultSetType=array";
				$(this).combobox('reload',url);
			}
		},
		onBeforeLoad: function(param){
			param.ndesc = param.q;
		}
	}); 
	// $("#loaddataCB").combobox("setValue","20").combobox("setText","中国");
});
```

```javascript
var onComplete7 = function(){
	Mock.mock("FindNation",function(param){
		return [{id:1,text:"美国"},{id:19,text:"英国"},{id:20,text:"中国"},{id:21,text:"德国"},{id:22,text:"法国"}];
		
	});
	$("#loaddataCB").combobox({  
		valueField: 'id',
		textField: 'text',
		panelHeight:'auto',
		value:20,data:[{id:20,text:"中国"}], //当前框中默认选中【中国】，如果默认空时可以去掉这行
		onShowPanel: function () { // 只有在下拉层显示时,才去关联url拉取数据,提高首屏速度
			var url = $(this).combobox('options').url;
			if (!url){
				//var url = $URL+"?ClassName=web.Nation&QueryName=Find&ResultSetType=array";
				//$(this).combobox('reload',url);
				//var url = "FindNation" ;
				//$(this).combobox('reload',url);
				//$(this).combobox('options').mode = 'remote';
				$(this).combobox('options').url = "FindNation";
				$(this).combobox('reload');
			}
		},
		onBeforeLoad: function(param){
			param.ndesc = param.q;
		}
	}); 
	//$("#loaddataCB").combobox("setValue","20").combobox("setText","中国");
};
$.parser.onComplete = function(){
	onComplete2();
	onComplete3();
	onComplete4();
	onComplete5();
	onComplete6();
	onComplete7();
}
```

| 属性 | 说明 | 默认值 | 查看EasyUI 1.3.2的combobox说明 |
| --- | --- | --- | --- |
| enterNullValueClear | 默认值 true 。当为 false 时，在输入框内回车，没有匹配值不清空输入框。 | true |  |
| selectOnNavigation | 默认值 true 。当为 false 时，DOWN键不选中行记录 | true |  |
| defaultFilter | 默认的filter过滤规则 1 ：text字段左匹配 不区分大小写 2 ：text字段包含匹配 不区分大小写 3 ： text字段左匹配或拼音首字母左匹配(多音字只取获取到的第一个拼音简拼) 不区分大小写 改为同 5 2020-06-12 4 ： text字段包含匹配或拼音首字母包含匹配(多音字只取获取到的第一个拼音简拼) 不区分大小写 改为同 6 2020-06-12 5 ：text字段左匹配或拼音首字母左匹配(考虑多音字) 不区分大小写 2020-05-26 6 ：text字段包含匹配或拼音首字母包含匹配(考虑多音字) 不区分大小写 2020-05-26 | 1 |  |
| spellField | 简拼字段名,用于指定行数据中的简拼字段 2020-05-26 defaultFilter为 3-6 时才有效 当defaultFilter为 3-6 时，如果配置了此配置项,则获取行数据的此字段值作为简拼，而不是程序自己转化简拼 程序自动转的简拼由于没有词库以及语义分析在多音字处理上是不准的 |  |  |
| rowStyle | 行显示样式。可选值'checkbox',显示成可勾选行 2018-10-17 | '' |  |
| blurValidValue | 为true时，光标离开时，检查是否选中值,没选中则置空输入框的值。 2018-12-26 | false |  |
| allSelectButtonPosition | 全选按钮位置。 multiple:true,rowStyle:'checkbox' 时此配置项有效。 2019-01-21 | 'top' | 默认为'top',可配置为'bottom' |
| allowNull | 单选时选中记录后不能取消选择时，可以配置此项为true 2019-02-18 | false | 默认为false,用于单选且不可编辑时 |
| panelHeight | 下拉面板高度，可配置成auto表示自动高度 | 200 | 默认高度200px |
| defaultHoverFirstRow | 查询数据时默认hover第一行,回车选中第一行，减少向下按键 | false | 默认false |
| 事件 | 描述 | 入参 | 说明 |
| onAllSelectClick | 点击【全选/取消全选】按钮时触发 2019-01-22 | e | 只有按钮显示时才有效。 onAllSelectClick:function(e){var arr = $(this).combobox("getValues");} |
