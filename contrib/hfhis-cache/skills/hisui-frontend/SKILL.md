---
name: hisui-frontend
description: HISUI 前端组件的离线官方参考与开发流程。编写、修改、解释或审查东华 HIS 前端页面，或任务涉及 hisui、$HUI、datagrid、dialog、combobox、datebox、tree、treegrid、lookup、messager 等 HISUI 控件时使用。
---

# HISUI 前端开发

以仓库内提取的 HISUI 官方文档为事实来源，完成组件选型、页面实现、旧代码修改和 API 审查。

## 工作流程

1. 先阅读 [`references/index.md`](references/index.md)，按中文名、插件名或场景定位组件。
2. 只读取任务涉及的组件参考文件；涉及组合控件时，同时读取它依赖的基础组件。
3. 修改现有页面前，检查页面当前引入的 HISUI、jQuery、语言包和初始化方式，保持版本与风格一致。
4. 从参考文件核对组件的标记类、初始化参数、属性、事件和方法，不凭 jQuery EasyUI 经验补全 HISUI 接口。
5. 优先沿用项目现有封装；只有项目没有封装时，才直接使用官方示例中的 HISUI API。
6. 交付前检查选择器、事件回调参数、方法名、返回值、数据格式和销毁/关闭行为是否与参考文件一致。

## 查询文档

先用索引定位；名称不确定时，在整个参考目录检索：

```bash
rg -n -i '组件名|属性名|方法名|事件名' .claude/skills/hisui-frontend/references
```

常用入口：

- 表格与编辑：[`references/datagrid-datagrid.md`](references/datagrid-datagrid.md)、[`references/datagrid-datagrid-edit.md`](references/datagrid-datagrid-edit.md)、[`references/datagrid-datagrid-celledit.md`](references/datagrid-datagrid-celledit.md)
- 弹窗与消息：[`references/dialog-dialog.md`](references/dialog-dialog.md)、[`references/window-window.md`](references/window-window.md)、[`references/messager-messager.md`](references/messager-messager.md)
- 表单选择：[`references/combobox-combobox.md`](references/combobox-combobox.md)、[`references/datebox-datebox.md`](references/datebox-datebox.md)、[`references/lookup-lookup.md`](references/lookup-lookup.md)
- 树与树表：[`references/tree-tree.md`](references/tree-tree.md)、[`references/treegrid-treegrid.md`](references/treegrid-treegrid.md)
- 布局与页签：[`references/layout-layout.md`](references/layout-layout.md)、[`references/tab-tab.md`](references/tab-tab.md)
- 公共方法与图标：[`references/common-function.md`](references/common-function.md)、[`references/icon-icon.md`](references/icon-icon.md)

## 使用约束

- 将离线参考视为抓取时官网内容的快照；若代码库中的 HISUI 版本更旧，以该版本实际行为和既有代码为准。
- 区分普通组件与“极速系列”组件，例如 `datebox` 与 `dateboxq`，不要混用接口。
- 示例可能包含演示数据或历史写法；提取 API 事实，但让最终实现服从当前项目的安全、可访问性和编码规范。
- 参考文件中的“页面脚本示例”来自官网演示页面，不代表所有代码都必须复制。

## 更新离线文档

仅在可访问外网且用户要求刷新文档时运行：

```bash
python3 .claude/skills/hisui-frontend/scripts/update_docs.py
```

脚本会重新读取官网索引、覆盖生成 `references/*.md`，并删除官网索引中已经下线的旧组件参考。完成后检查差异并运行 skill 校验。
