# 树(tree)

- 官方页面标题：树
- 分类：表格及树
- 来源：http://hisui.cn/api/tree/tree.html
- 官网更新时间：Tue, 12 Mar 2024 03:51:35 GMT
- 提取时间：2026-07-25T13:18:46+00:00

> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。

## 本页目录

- 树
  - 说明:
  - 一、无线条树( lines:false )
  - 二、有线条树( lines:true )
  - 三、formatter实现节点自定义格式
  - 四、转成手风琴样式( class="accordiontree" )
  - tree新增属性、方法

## 树

### 说明:

树形菜单（Tree）可从标记创建，也可定义在 <ul> 元素中。无序列表的 <ul> 元素提供一个基础的树（Tree）结构。每一个 <li> 元素将产生一个树节点，子 <ul> 元素将产生一个父树节点。

### 一、无线条树( lines:false )

### 二、有线条树( lines:true )

### 三、formatter实现节点自定义格式

```javascript
$(function () {
				$('#sptt').tree({
					data: [{
						"id": 1,
						"text": "病程记录",
						"state": "open",
						"children": [{
							"id": 11,
							"text": "Node 11",
							"children": [{
								"id": 111,
								"text": "Node 999"
							}, {
								"id": 112,
								"text": "Node 112"
							}]
						}, {
							"id": 12,
							"text": "Node 12"
						}]
					}, {
						"id": 2,
						"text": "手术记录",
						"state": "open",
						"children": [{
							"id": 21,
							"text": "Node 21",
							"children": [{
								"id": 211,
								"text": "Node 211"
							}, {
								"id": 212,
								"text": "Node 212",
								"children": [{
									"id": 2121,
									"text": "Node 2121"
								}, {
									"id": 2122,
									"text": "Node 2122"
								}]
							}]
						}, {
							"id": 22,
							"text": "Node 22"
						}]
					}, {
						"id": 3,
						"text": "其他记录",
						"state": "closed",
						"children": [{
							"id": 31,
							"text": "Node 31",
							"children": [{
								"id": 311,
								"text": "Node 311"
							}, {
								"id": 312,
								"text": "Node 312"
							}]
						}, {
							"id": 32,
							"text": "Node 32"
						}]
					}, {
						"id": 4,
						"text": "病案首页"
					}],
					formatter:function(node){
						//formatter  此时isLeaf方法还无法判断是不是叶子节点 可通过children
						if (node.children){
							return node.text;
						}else{
							return "<div >"
								+"<span data-id='"+node.id+"' class='icon icon-edit' style='display:block;width:16px;height:16px;position:absolute;right:5px;top:5px;'></span>"
								+"<div style='height:20px;line-height:20px;color:gray'>"+(new Date()).toLocaleString()+"</div>"
								+"<div style='height:20px;line-height:20px;'>"+node.text+"</div>"
								+"</div>";
							//同时给此树下的tree-node 加上position: relative;   以实现小图标靠右
						}
						
					},
					lines:true,autoNodeHeight:true
				})

				//可能还需要监听事件
				$('#sptt').on('click','.icon-edit',function(){
					var id=$(this).data('id');
					var node=$('#sptt').tree('find',id);
					console.log(node);
					alert(node.text);
				})
			})
```

### 四、转成手风琴样式( class="accordiontree" )

```javascript
$(function(){
				$('#goto-menutree').click(function(){
					if(window.parent && window.parent!=window &&window.parent.$) {
						var p$=window.parent.$;
						var flag=false;
						p$('.api-navi-tab').each(function(){
							if($(this).text().indexOf('menutree')>-1) {
								p$(this).click();
								flag=true;
								return false;
							}
						})
						if(flag) return false;
					}


				})
			})
```

```javascript
$(function () {
					$('#mytt').tree({
						data: [{
							"id": 1,
							"text": "物资信息",
							"state": "open",
							"children": [{
								"id": 11,
								"text": "Node 11",
								"children": [{
									"id": 111,
									"text": "Node 111",
									"iconCls":"icon-drug"
								}, {
									"id": 112,
									"text": "Node 112",
									"iconCls":"icon-drug"
								}]
							}, {
								"id": 12,
								"text": "Node 12"
							}]
						}, {
							"id": 2,
							"text": "物资信息2",
							"state": "open",
							"children": [{
								"id": 21,
								"text": "Node 21",
								"children": [{
									"id": 211,
									"text": "Node 211"
								}, {
									"id": 212,
									"text": "Node 212",
									"children": [{
										"id": 2121,
										"text": "Node 2121"
									}, {
										"id": 2122,
										"text": "Node 2122"
									}]
								}]
							}, {
								"id": 22,
								"text": "Node 22"
							}]
						}, {
							"id": 3,
							"text": "物资信息3",
							"state": "closed",
							"children": [{
								"id": 31,
								"text": "Node 31",
								"children": [{
									"id": 311,
									"text": "Node 311"
								}]
							}]
						}],
						onClick: function (node) {
							$('#mytt').tree('toggle', node.target); //简单单吉展开关闭
							/*  //类手风琴 只开一个根节点操作
							if($(node.target).parent().parent().hasClass('accordiontree')&&node.state=="closed"){
								var roots=$('#mytt').tree('getRoots');
								$.each(roots,function(i,o){
									$('#mytt').tree('collapse',o.target);
								})
							}
							$('#mytt').tree('toggle',node.target);
							*/
						}
					})
				})
```

### tree新增属性、方法

| 属性 | 说明 | 默认值 |  |
| --- | --- | --- | --- |
| autoNodeHeight | 在 lines 为true时，formatter返回的html高度不定时，要配置此属性为true,自动计算线条高度 | false |  |
