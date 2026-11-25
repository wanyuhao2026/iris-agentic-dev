# JSON(json)

- 官方页面标题：Json转换
- 分类：基础
- 来源：http://hisui.cn/api/json/json.html
- 官网更新时间：Tue, 12 Mar 2024 03:51:29 GMT
- 提取时间：2026-07-25T13:18:46+00:00

> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。

## JSON转换。

### 说明: 为兼容所有IE版本下JSON转换,引入json2.js,源自于 json2.js

### 使用:

序列化方法（json对象转字符串）

```javascript
var jsonObj = {id:'01',name:'wang'};
JSON.stringify(jsonObj);
```

反序列化方法（字符串转json对象）

```javascript
//推荐后台输出修改成标准json格式，json的"key"与"字符值"应该使用双引号包裹
var jsonStr = '{"id":"01","name":"wang"}';  //标准json字符串
JSON.parse(jsonStr); 

var str = "{id:'01',name:'wang'}";   //不符合规范的json字符串，尽量避免
JSON.parse(str);
```
