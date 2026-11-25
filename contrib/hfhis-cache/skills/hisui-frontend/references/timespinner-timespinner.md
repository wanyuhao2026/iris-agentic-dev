# 时间框(timespinner)

- 官方页面标题：时间组件
- 分类：表单
- 来源：http://hisui.cn/api/timespinner/timespinner.html
- 官网更新时间：Tue, 12 Mar 2024 03:51:34 GMT
- 提取时间：2026-07-25T13:18:46+00:00

> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。

## 时间框

### 说明:

时间验证框（timespinner） 。

### 如：

```javascript
$.parser.onComplete= function(ctn){
						if (ctn){return ;}
						$("#tsp").timespinner("setValue","101010");
						$("#tsp").timespinner("setSpinStart",4);
					};
```

| 属性 | 说明 | 默认值 |  |
| --- | --- | --- | --- |
| setSpinStart | 设置当前微调位置。如:4。则默认光标在第4个字符位置，即分钟区域 20220613 | 当入参值小于0或大于内容总长度时无效 | 正整数 |
| hourStep | 调整时钟的增量值 20230217 | 1小时 |  |
| minutesStep | 调整分钟的增量值 20230217 | 1分钟 |  |
| secondStep | 调整秒钟的增量值 20230217 | 1秒钟 |  |
