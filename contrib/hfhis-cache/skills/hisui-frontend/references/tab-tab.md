# 页签(tab)

- 官方页面标题：多页签
- 分类：布局
- 来源：http://hisui.cn/api/tab/tab.html
- 官网更新时间：Fri, 21 Feb 2025 02:30:45 GMT
- 提取时间：2026-07-25T13:18:46+00:00

> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。

## 本页目录

- 页签
  - 说明:
  - 如：
  - 一.灰色页签( tabs-gray )及点击监听
  - 二.页签位置定义( data-options="tabPosition:'left'" )
  - 三.上下页签可定义高度( tabHeight , tabs-gray-btm )
  - 四.页签过多滚动效果
  - 五.第一个页签作为标题的图表组( isBrandTabs:true )
  - 六.关键字Tabs(class='hisui-tabs tabs-keywords ')
  - tab新增属性或方法

## 页签

### 说明:

页签（Tabs ）一次仅显示一个面板（panel），每个面板（panel）都有标题、图标和关闭按钮。 当 Tabs 被选中时，将显示对应的面板（panel）的内容。

### 如：

```html
<div id="tt" class="hisui-tabs" style="width:500px;height:250px;">   
	<div title="电子病历" style="padding:20px;">   
		tab1    
	</div>   
	<div title="诊断录入" data-options="closable:true" style="overflow:auto;padding:20px;display:none;">   
		tab2    
	</div>   
	<div title="医嘱录入" data-options="closable:true" style="padding:20px;display:none;">   
		tab3    
	</div>
</div>
```

### 一.灰色页签( tabs-gray )及点击监听

```html
<div id="tt2" class="hisui-tabs tabs-gray" style="width:500px;height:250px;">   
	<div title="电子病历" style="padding:20px;">   
		此tab的class="hisui-tabs  tabs-gray"，所以为灰色的。 如果页签太多则自动隐藏，可滚动显示 
	</div>   
	<div title="诊断录入" data-options="closable:true" style="overflow:auto;padding:20px;display:none;">   
		tab2    
	</div>   
	<div title="医嘱录入" data-options="closable:true" style="padding:20px;display:none;">   
		tab3    
	</div>  
	<div title="检查列表" data-options="closable:true" style="padding:20px;display:none;">   
		tab3    
	</div> 
	<div title="检验列表" data-options="closable:true" style="padding:20px;display:none;">   
		tab3    
	</div> 
	<div title="危急值列表" data-options="closable:true" style="padding:20px;display:none;">   
		tab3    
	</div> 
</div>
```

```javascript
<script type="text/javascript">
$(function(){
	$HUI.tabs("#tt2",{
		onSelect:function(title){
			$.messager.popover({type:'info',msg:'切换到【'+title+'】'}); 					
		}
	});
});	
</script>
```

```javascript
$(function(){
			$HUI.tabs("#tt2",{
				onSelect:function(title){
					$.messager.popover({type:'info',msg:'切换到【'+title+'】'}); //alert('你确定要切换到【'+title+'】');					
				}
			});
			//$('#tt2').tabs('disableTab', 0); 
		});
```

### 二.页签位置定义( data-options="tabPosition:'left'" )

```html
<div style="margin:20px 0;">
	<span>tabPosition:</span>
	<select onchange="$('#tt3').tabs({tabPosition:this.value});$('#tabPosition').html('tabPosition:\''+this.value+'\'');">
		<!--<option value="top">Top</option>
		<option value="bottom">Bottom</option>--> 
		<option value="left" selected>Left</option>
		<option value="right">Right</option>
	</select>
</div>
<div class="hisui-panel" style="width:500px;height:300px;" data-options="title:'诊疗',headerCls:'panel-header-gray',iconCls:'icon-paper'">
	<div id="tt3" class="hisui-tabs tabs-gray"  data-options="tabPosition:'left',fit:true,border:false,headerWidth:100">   
		<div title="电子病历" style="padding:20px;">   
			<code id="tabPosition">tabPosition:'left'</code><br>
			<code>headerWidth:100</code><br>
			<code>border:false</code><br>
		</div>   
		<div title="诊断录入" style="overflow:auto;padding:20px;">   
			tab2    
		</div>   
		<div title="医嘱录入" style="padding:20px;">   
			tab3    
		</div>
	</div> 
</div>
```

### 三.上下页签可定义高度( tabHeight , tabs-gray-btm )

```html
<div class="hisui-tabs tabs-gray-btm" style="width:500px;height:120px" tabHeight="26px">
	<div title="常用" style="padding:20px">增加属性tabHeight可以让页签变小 tabHeight="26px"</div>
	<div title="编辑" data-options="closable:false" style="padding:20px;overflow:auto;">
		<div>人人尽说江南好，游人只合江南老。春水碧于天，画船听雨眠。</div>
		<div>垆边人似月，皓腕凝霜雪。未老莫还乡，还乡须断肠。</div>
		<div>如今却忆江南乐，当时年少春衫薄。骑马倚斜桥，满楼红袖招。</div>
	</div>
	<div title="功能" data-options="closable:false" style="padding:20px;display:none;"></div>
	<div title="收藏" data-options="closable:false" style="padding:20px;display:none;"></div>
	<div title="表格" data-options="closable:false" style="padding:20px;display:none;">我是否被隐藏了？</div>
<div>
```

### 四.页签过多滚动效果

```html
<div id="mytabs1" class="hisui-tabs tabs-gray" style="width:500px;height:100px;" data-options="">   
			<div title="电子病历" data-options="" style="padding:20px;"></div>   
			<div title="诊断录入" data-options="" style="padding:20px;"></div>   
			<div title="医嘱录入" data-options="" style="padding:20px;"></div>
			<div title="检查列表" data-options="" style="padding:20px;"></div>
			<div title="检验列表" data-options="" style="padding:20px;"></div>
			<div title="过敏记录" data-options="" style="padding:20px;"></div>
			<div title="生命体征" data-options="" style="padding:20px;"></div>
			<div title="就诊列表" data-options="" style="padding:20px;"></div>
		</div>
```

```html
<div id="mytabs2" class="hisui-tabs" style="width:500px;height:100px;margin-top: 10px" data-options="">   
			<div title="电子病历" data-options="" style="padding:20px;"></div>   
			<div title="诊断录入" data-options="" style="padding:20px;"></div>   
			<div title="医嘱录入" data-options="" style="padding:20px;"></div>
			<div title="检查列表" data-options="" style="padding:20px;"></div>
			<div title="检验列表" data-options="" style="padding:20px;"></div>
			<div title="过敏记录" data-options="" style="padding:20px;"></div>
			<div title="生命体征" data-options="" style="padding:20px;"></div>
			<div title="就诊列表" data-options="" style="padding:20px;"></div>
		</div>
```

### 五.第一个页签作为标题的图表组( isBrandTabs:true )

```html
<div id="tt-brand-gray" class="hisui-tabs tabs-gray" style="width:500px;height:150px;" data-options="isBrandTabs:true>
	<div title="医嘱浏览" data-options="iconCls:'icon-add-note'" style="padding:20px;"></div>   
	<div title="全部" data-options="closable:true" style="padding:20px;"></div>   
	<div title="药品" data-options="closable:true" style="padding:20px;"></div>  
	<div title="检查" data-options="closable:true" style="padding:20px;"></div>
	<div title="检查" data-options="closable:true" style="padding:20px;"></div>
</div>
```

```html
<div id="tt-brand" class="hisui-tabs " style="width:500px;height:150px;" data-options="isBrandTabs:true">   
				<div title="病历质控" data-options="iconCls:'icon-w-list'" style="padding:20px;"></div>   
				<div title="环节质控缺陷" data-options="closable:false" style="padding:20px;"></div>   
				<div title="时效性缺陷" data-options="closable:false" style="padding:20px;"></div>
			</div>
```

### 六.关键字Tabs(class='hisui-tabs tabs-keywords ')

```html
<div id="tt-keywords" class="hisui-tabs tabs-keywords" style="width:500px;height:150px;">   
				<div title="病历质控" data-options="" style="padding:20px;">我是 '病历质控' ,去看看类似组件<a class="plain-anchor" data-url="keywords/keywords.html">keywords</a>的功能。</div>
				<div title="环节质控缺陷" data-options="closable:false" style="padding:20px;">我是 '环节质控' ,去看看类似组件<a class="plain-anchor" data-url="keywords/keywords.html">keywords</a>的功能。</div>
				<div title="时效性缺陷" data-options="closable:false" style="padding:20px;">我是 '时效性缺陷' ,去看看类似组件<a class="plain-anchor" data-url="keywords/keywords.html">keywords</a>的功能。</div>
			</div>
```

### tab新增属性或方法

| 属性 | 说明 | 默认值 |  |
| --- | --- | --- | --- |
| simpleContextMenu | 显示简单右键菜单 | false | 当为true时，显示刷新、关闭、关闭其他等右键菜单 |
