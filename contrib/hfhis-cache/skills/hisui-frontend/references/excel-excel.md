# 导出Excel（excel）

- 官方页面标题：Excel导出
- 分类：基础
- 来源：http://hisui.cn/api/excel/excel.html
- 官网更新时间：Mon, 22 Sep 2025 09:05:50 GMT
- 提取时间：2026-07-25T13:18:46+00:00

> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。

## 后台Query查询结果导出Excel

通过公共方法调用Query生成文件

## 一.导成CSV文件功能说明

#### 1.1 发送请求给后台,后台运行Query生成 CVS文件 ,然后再通过location.href下载到本地

```javascript
// js文件中代码 ，导出rowid为1的安全组所有菜单
// excelname不用包含后缀
var rtn = tkMakeServerCall("websys.Query","ToExcel","excelname","web.Util.Menu","SelectGroupMenu",1);
location.href = rtn;
```

```javascript
//引用websys.jquery.js后，导出文件中代码，
var rtn = $cm({
	dataType:'text',
	ResultSetType:"Excel",
	ExcelName:"excelname", //默认DHCCExcel
	ClassName:"web.Util.Menu",
	QueryName:"SelectGroupMenu",
	GroupId:1,
	// HiddenColumnHeader:1 //1表示隐藏列头，默认显示列头
},false);
location.href = rtn;
```

## websys.Query类中ToExcel类方法

| 参数 | 类型 |  | 说明 |
| --- | --- | --- | --- |
| ResultSetType | {String} | 必填项 | Excel表示导出csv，ExcelPlugin表示导出标准Excel |
| ExcelName | { String } | 必填项 | 最终要导出的Excel名称,不用带后缀 |
| PageName | {String} | 选填项 | ResultSetType为ExcelPlugin时，且要读配置打印内容时必填 |
| ClassName | { String } | 必填项 | 后台类名 |
| QueryName | { String } | 必填项 | 后台Query名 |
| p1 | { String\|Int } | 可选项 | Query第1个入参 |
| p2 | { String\|Int } | 可选项 | Query第2个入参 |
| ... | { String\|Int } | 可选项 | Query第n个入参 |
| p16 | { String\|Int } | 可选项 | Query第16个入参 |
