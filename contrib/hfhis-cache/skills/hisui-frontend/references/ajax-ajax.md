# 数据访问（ajax）

- 官方页面标题：公共请求
- 分类：基础
- 来源：http://hisui.cn/api/ajax/ajax.html
- 官网更新时间：Tue, 12 Mar 2024 03:51:23 GMT
- 提取时间：2026-07-25T13:18:46+00:00

> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。

## 本页目录

- 数据访问
  - 说明:
  - 使用:
- $cm(data[,success][,error])调用后台方法
- 1.调用后台类方法示例：
- 2.调用后台Query示例支持 %String,%Integer,%Float,%Boolean ：
- 3.数组入参示例：
- 4.对象或数组返回示例：

## 数据访问

### 说明:

通过ajax调用后台方法或Query,且会对特殊敏感关键词进行过过滤insert,select,delete,update,conut,chr,declare,truncate,from,having,base,object,param,script,noscript,alert,
confirm,prompt,onmouseleave,onmousewheel,location,eval,function,onabort,onblur,onchange,onclick,ondblclick,onerror,onfocus,onkeydown,onkeypress,onkeyup,onload,
onmousemove,onmouseover,onmouseout,onmousedown,onmouseup,onreset,onselect,onsubmit,onunload,当请求的值中包含这些关键词，且关键词前后有特殊字符时，当前请求值会被置换成空

### 使用:

先引入jQuery，再引入websys.jquery.js

```html
<script type="text/javascript" src="../scripts_lib/jquery-easyui-1.3.2/jquery-1.8.0.min.js" charset="utf-8"></script>
<script type="text/javascript" src="../scripts/websys.jquery.js"></script>
```

## $cm(data[,success][,error])调用后台方法

| 参数 | 类型 |  | 说明 |
| --- | --- | --- | --- |
| data | { Object } | 必填项 | 属性 类型 说明 ClassName { String } 必需项 类名，即包名.类名 MethodName { String } 可选 后台类方法名或成员方法名 QueryName { String } 可选 后台Query名。如果配置了MethodName则QueryName无效 wantreturnval { Number } 可选 后台方法是否有返回值，1表示有，0表示无。 决定后台是以do还是set方式运行。默认为1 ResultSetType { String } 可选 配置ResultSetType后，运行Query返回的数据格式 值为 array 时返回值格式为: [{},{},{}] 不配置时返回格式为: {"rows":[{},{},{}],"total":3} 值为 Excel 时返回下载cvs链接，具体参考导出Excel菜单说明 值为 ExcelPlugin 时会导出Excel到本地，具体参考导出Excel菜单说明 totalFields { String } 可选 配置totalFields后，会对totalFields中的列进入数值合计 如：totalFields: 'amt,cost' ,返回格式为: {"rows":[...], "footer":[{"amt":110,"cost":120}], "total":3} totalFooter { String } 可选 配置totalFooter后，会在数值合计后，拼接到数据json前。 如：totalFooter: '"colName":"押金合计"' ,返回格式为: {"rows":[...],"footer":[{ "colName":"押金合计", "amt":1100}],"total":3} 20200224 dataType {String} 返回的数据类型，默认 "json" 见$.ajax的 dataType type {String} 请求类型，默认 "POST" 见$.ajax的 type 20190413 当ajax请求发生在界面卸载事件内，应使用type:"BEACON"来处理当前请求，在chrome49后此时情况下没有返回值 20230404 其它 可选 对应后台方法或Query的入参名，可以为数组 |
| 属性 | 类型 |  | 说明 |
| ClassName | { String } | 必需项 | 类名，即包名.类名 |
| MethodName | { String } | 可选 | 后台类方法名或成员方法名 |
| QueryName | { String } | 可选 | 后台Query名。如果配置了MethodName则QueryName无效 |
| wantreturnval | { Number } | 可选 | 后台方法是否有返回值，1表示有，0表示无。 决定后台是以do还是set方式运行。默认为1 |
| ResultSetType | { String } | 可选 | 配置ResultSetType后，运行Query返回的数据格式 值为 array 时返回值格式为: [{},{},{}] 不配置时返回格式为: {"rows":[{},{},{}],"total":3} 值为 Excel 时返回下载cvs链接，具体参考导出Excel菜单说明 值为 ExcelPlugin 时会导出Excel到本地，具体参考导出Excel菜单说明 |
| totalFields | { String } | 可选 | 配置totalFields后，会对totalFields中的列进入数值合计 如：totalFields: 'amt,cost' ,返回格式为: {"rows":[...], "footer":[{"amt":110,"cost":120}], "total":3} |
| totalFooter | { String } | 可选 | 配置totalFooter后，会在数值合计后，拼接到数据json前。 如：totalFooter: '"colName":"押金合计"' ,返回格式为: {"rows":[...],"footer":[{ "colName":"押金合计", "amt":1100}],"total":3} 20200224 |
| dataType | {String} | 返回的数据类型，默认 "json" | 见$.ajax的 dataType |
| type | {String} | 请求类型，默认 "POST" | 见$.ajax的 type 20190413 当ajax请求发生在界面卸载事件内，应使用type:"BEACON"来处理当前请求，在chrome49后此时情况下没有返回值 20230404 |
| 其它 |  | 可选 | 对应后台方法或Query的入参名，可以为数组 |
| success | { Function \| Boolean } | 可选项 | Function 类型 ajax成功时回调方法 function(json){} Boolean 类型且值为false时，则表示发送同步请求 |
| error | { Function } | 可选项 | Function 类型 ajax错误时回调方法 function(rtn){} |

| 属性 | 类型 |  | 说明 |
| --- | --- | --- | --- |
| ClassName | { String } | 必需项 | 类名，即包名.类名 |
| MethodName | { String } | 可选 | 后台类方法名或成员方法名 |
| QueryName | { String } | 可选 | 后台Query名。如果配置了MethodName则QueryName无效 |
| wantreturnval | { Number } | 可选 | 后台方法是否有返回值，1表示有，0表示无。 决定后台是以do还是set方式运行。默认为1 |
| ResultSetType | { String } | 可选 | 配置ResultSetType后，运行Query返回的数据格式 值为 array 时返回值格式为: [{},{},{}] 不配置时返回格式为: {"rows":[{},{},{}],"total":3} 值为 Excel 时返回下载cvs链接，具体参考导出Excel菜单说明 值为 ExcelPlugin 时会导出Excel到本地，具体参考导出Excel菜单说明 |
| totalFields | { String } | 可选 | 配置totalFields后，会对totalFields中的列进入数值合计 如：totalFields: 'amt,cost' ,返回格式为: {"rows":[...], "footer":[{"amt":110,"cost":120}], "total":3} |
| totalFooter | { String } | 可选 | 配置totalFooter后，会在数值合计后，拼接到数据json前。 如：totalFooter: '"colName":"押金合计"' ,返回格式为: {"rows":[...],"footer":[{ "colName":"押金合计", "amt":1100}],"total":3} 20200224 |
| dataType | {String} | 返回的数据类型，默认 "json" | 见$.ajax的 dataType |
| type | {String} | 请求类型，默认 "POST" | 见$.ajax的 type 20190413 当ajax请求发生在界面卸载事件内，应使用type:"BEACON"来处理当前请求，在chrome49后此时情况下没有返回值 20230404 |
| 其它 |  | 可选 | 对应后台方法或Query的入参名，可以为数组 |

## 1.调用后台类方法示例：

```
//后台类代码

Class dhc.Test Extends %RegisteredObject 
{
	Classmethod getPatInfo(UserName)
	{
		q "{""name"":"""_UserName"""}"
	}
}
```

```javascript
//异步调用类方法，推荐
$cm({
	ClassName:"dhc.Test",
	MethodName:"getPatInfo",
	UserName:"张三"
},function(jsonData){
	console.dir(jsonData);  //{"name":"张三"}
});
```

```javascript
//异步调用类方法,且返回值非json数据时请用 $m(data[,success][,error]);
$m({
	ClassName:"dhc.Test",
	MethodName:"getPatInfo",
	UserName:"张三"
},function(txtData){
	console.dir(txtData);  //'{"name":"张三"}'
});
```

```javascript
//同步调用类方法 ,不建议使用
var jsonData = $cm({
	ClassName:"dhc.Test",
	MethodName:"getPatInfo",
	UserName:"张三"
},false);
console.dir(jsonData);  //{"name":"张三"}
```

## 2.调用后台Query示例支持 %String,%Integer,%Float,%Boolean ：

```
Query LookUpActive(desc As %String) As %Library.SQLQuery(CONTAINID = 2, ROWSPEC = "Description:%String,HIDDEN:%Integer,Active:%Boolean")
{
	SELECT SSUSR_Name, SSUSR_RowID, SSUSR_Active
	FROM	SQLUser.SS_User
	WHERE	((%ALPHAUP SSUSR_Name [ %ALPHAUP :desc) OR (:desc IS NULL)) AND (SSUSR_Active = 'Y')
	ORDER	BY SSUSR_Name
}
```

```javascript
$cm({
	ClassName:"web.SSUser",
	QueryName:"LookUpActive",
	//增加配置项 ResultSetType："array" //则rs为[{"Description":"王二","HIDDEN":"1","Active":true},,...]
	page:1,    //可选项，页码，默认1
	rows:20    //可选项，获取多少条数据，默认50
},function(rs){
	console.dir(rs);    //{"rows":[{"Description":"王二","HIDDEN":"1","Active":true},...],"total":110}
	
});
```

## 3.数组入参示例：

```
//后台M类

Class dhc.Test Extends %RegisteredObject {
	//保存喜爱的水果列表
	Classmethod saveFavFruitList(plist)	{
		//下面前台$cm方法调用后plist值如下
		//plist(1)="orange"
		//plist(2)="apple"
		//plist(3)="pear"
		//plist(4)="banana"
		quit "{}"
	}
}
```

```javascript
//数组入参调用js代码示例
$cm({
	ClassName:"dhc.Test",
	MethodName:"saveFavFruitList",
	plist:["orange","apple","pear","banana"]
});
```

## 4.对象或数组返回示例：

```
//后台M类

Class dhc.Test Extends %RegisteredObject {
	Property Name As %String;
	Property Sex As %String;
	//返回对象或数组
	Classmethod getInfo() As dhc.Test{
		set obj = ##class(dhc.Test).%New()
		set obj.Name="neer"
		set obj.Sex="男"
		quit obj
	}
}
```

```javascript
//数组入参调用js代码示例
$cm({
	ClassName:"dhc.Test",
	MethodName:"getInfo"
},function(rtn){
	console.log(rtn); //{Name:"neer",Sex:"男"}
});
```
