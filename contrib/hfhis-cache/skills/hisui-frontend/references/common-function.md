# 公共方法(commfun)

- 官方页面标题：公共方法
- 分类：基础
- 来源：http://hisui.cn/api/common/function.html
- 官网更新时间：Tue, 12 Mar 2024 03:51:25 GMT
- 提取时间：2026-07-25T13:18:46+00:00

> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。

## 本页目录

- hisui中的一些公共方法
  - $.hisui.indexOfArray 获取元素索引
  - $.hisui.indexOfArray ( a , o , id )
  - $.hisui.removeArrayItem 移除元素
  - $.hisui.removeArrayItem ( a , o , id )
  - $.hisui.addArrayItem 增加或更新元素
  - $.hisui.addArrayItem ( a , o , r )
  - $.hisui.getArrayItem 获取元素
  - $.hisui.getArrayItem ( a , o , id )
  - $.hisui.getChineseSpellArray 汉字转拼音 ( 2018-08-27 )
  - $.hisui.getChineseSpellArray ( str )
  - $.hisui.toChineseSpell 汉字转拼音( 2018-08-27 )
  - $.hisui.toChineseSpell ( str )
  - $.fn.datebox.defaults.formatter 日期格式化
  - $.fn.datebox.defaults.formatter ( date )
  - $.fn.datebox.defaults.parser 日期解析
  - $.fn.datebox.defaults.parser ( s )

## hisui中的一些公共方法

### $.hisui.indexOfArray 获取元素索引

### $.hisui.indexOfArray ( a , o , id )

| 参数 | 说明 | 默认值 | 说明 |
| --- | --- | --- | --- |
| a | 数组 |  |  |
| o | 目标元素或字段名 |  | 目标元素或字段名 |
| id | 目标字段值 | undefined | 不传id时，获取是数组a中元素为o的索引值, 传了id 获取的是数组a中元素的o字段值为id的元素索引值 |

```javascript
console.log('test $.hisui.indexOfArray');
var item={id:'2',name:'李四'};
var arr=[{id:'1',name:'张三'},item,{id:'3',name:'王二'}];
console.log($.hisui.indexOfArray(arr,{id:'2',name:'李四'})); //-1 对象类型 不是同一引用就不是

console.log($.hisui.indexOfArray(arr,item)); //1 对象类型 同一引用

console.log($.hisui.indexOfArray(arr,'id',2)); //1 根据id字段获取
console.log($.hisui.indexOfArray(arr,'id',3)); //2
```

### $.hisui.removeArrayItem 移除元素

### $.hisui.removeArrayItem ( a , o , id )

| 参数 | 说明 | 默认值 | 说明 |
| --- | --- | --- | --- |
| a | 数组 |  |  |
| o | 目标元素或字段名 |  | 如果o是元素对象，移除数组中的此对象 如果o是字段名,移除数组中元素此字段值为id参数的元素 |
| id | 目标字段值 | undefined | o为string类型，此参数才生效 |

```javascript
console.log('test $.hisui.removeArrayItem');
var item={id:'2',name:'李四'};

var arr=[{id:'1',name:'张三'},item,{id:'3',name:'王二'}];
$.hisui.removeArrayItem(arr,{id:'2',name:'李四'});
console.log(arr); //删不掉 对象类型 不是同一引用就不是

var arr=[{id:'1',name:'张三'},item,{id:'3',name:'王二'}];
$.hisui.removeArrayItem(arr,item);
console.log(arr); //可以删掉 同一引用

var arr=[{id:'1',name:'张三'},item,{id:'3',name:'王二'}];
$.hisui.removeArrayItem(arr,'id',2);
console.log(arr); //可以删掉 根据id字段名
```

### $.hisui.addArrayItem 增加或更新元素

### $.hisui.addArrayItem ( a , o , r )

| 参数 | 说明 | 默认值 | 说明 |
| --- | --- | --- | --- |
| a | 数组 |  |  |
| o | 元素或字段名 |  |  |
| r | 元素 |  | 1.不传r,o为要增加的元素，判断o在数组是否存在，不存在增加，存在则替换 2.传r,o为字段名，r为要增加的元素，判断数组中是否存在字段和r[o]相等的元素，不存在增加，存在则替换 |

```javascript
console.log('test $.hisui.addArrayItem');
var item={id:'2',name:'李四'};

var arr=[{id:'1',name:'张三'},item,{id:'3',name:'王二'}];
$.hisui.addArrayItem(arr,{id:'2',name:'李四'});
console.log(arr); //引用不一致判断为不存在，会增加一个新的 

var arr=[{id:'1',name:'张三'},item,{id:'3',name:'王二'}];
$.hisui.addArrayItem(arr,item);
console.log(arr); //引用一致判断存在 会替换 但是没啥意义

var arr=[{id:'1',name:'张三'},item,{id:'3',name:'王二'}];
$.hisui.addArrayItem(arr,'id',{id:'2',name:'李四（新）'});
console.log(arr); //根据id字段判断存在进行替换 这个才有实际使用意义

var arr=[{id:'1',name:'张三'},item,{id:'3',name:'王二'}];
$.hisui.addArrayItem(arr,'id',{id:'4',name:'麻子'});
console.log(arr); //根据id字段判断不存在 增加
```

### $.hisui.getArrayItem 获取元素

### $.hisui.getArrayItem ( a , o , id )

| 参数 | 说明 | 默认值 | 说明 |
| --- | --- | --- | --- |
| a | 数组 |  |  |
| o | 字段名 |  |  |
| id | 目标字段值 |  |  |

```javascript
console.log('test $.hisui.getArrayItem');
var item={id:'2',name:'李四'};
var arr=[{id:'1',name:'张三'},item,{id:'3',name:'王二'}];
console.log($.hisui.getArrayItem(arr,'id',2)); // 根据id字段获取
console.log($.hisui.getArrayItem(arr,'id',3));
```

### $.hisui.getChineseSpellArray 汉字转拼音 ( 2018-08-27 )

### $.hisui.getChineseSpellArray ( str )

```javascript
console.log('test $.hisui.getChineseSpellArray');
console.log($.hisui.getChineseSpellArray('你好')) 
console.log($.hisui.getChineseSpellArray('你好,hello')) //英文字符不处理
console.log($.hisui.getChineseSpellArray('俱往矣数风流人物还看今朝朝')) //多音字  多音字实际可以考虑语义、词库，但是这儿显然不可能
```

### $.hisui.toChineseSpell 汉字转拼音( 2018-08-27 )

### $.hisui.toChineseSpell ( str )

```javascript
console.log('test $.hisui.toChineseSpell');
console.log($.hisui.toChineseSpell('你好')) 
console.log($.hisui.toChineseSpell('你好,hello')) //英文字符不处理
console.log($.hisui.toChineseSpell('俱往矣数风流人物还看今朝朝')) //多音字只取第一个结果  多音字实际可以考虑语义、词库，但是这儿显然不可能
```

### $.fn.datebox.defaults.formatter 日期格式化

### $.fn.datebox.defaults.formatter ( date )

| 参数 | 说明 |
| --- | --- |
| date | 日期(Date对象) |
| 返回值 | 说明 |
| ret | 格式化后的日期字符串 |

```javascript
console.log('test $.fn.datebox.defaults.formatter');
console.log($.fn.datebox.defaults.formatter(new Date()))
```

### $.fn.datebox.defaults.parser 日期解析

### $.fn.datebox.defaults.parser ( s )

| 参数 | 说明 |
| --- | --- |
| s | 格式化的日期字符串 |
| 返回值 | 说明 |
| ret | 日期(Date)对象 |

```javascript
console.log('test $.fn.datebox.defaults.parser');
var date=new Date();
console.log(date);
var datestr=$.fn.datebox.defaults.formatter(date);
console.log(datestr);
console.log($.fn.datebox.defaults.parser(datestr)) ;
```
