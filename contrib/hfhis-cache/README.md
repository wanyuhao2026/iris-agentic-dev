# hfhis-cache — Caché 2016.2.3 适配的 MCP 与技能

为 `F:\projects\work\hfhis-8.17`（东华 iMedical 8.2.0 HIS）适配的一套 agent 工具：一个零依赖
Node MCP 服务器，加上 8 个针对本项目实测过的技能。

参照 jhip-atelier-v4 那套的结构（在 `feat/atelier-v4-jhip` 分支上，合入 master 后路径为
`../jhip-atelier-v4/`），但目标环境完全不同——那套是 IRIS 2026.2 + Atelier v4，这套是
**Caché 2016.2.3 + Atelier v1**，能力边界小得多，因此工具从 18 个减到 12 个，砍掉的能力在下面
逐条说明原因。

## 为什么重做

原项目里已有 `.tools/iris-agentic-dev.exe`（Rust 实现，50 MB，18 个 `iris_*` 工具）和 22 个
技能目录。问题：

- **exe 体积 50 MB**，且每次 `cache_execute` 都在命名空间里建临时类、清理失败，留下了
  **841 个 `IrisDevTmp.*` 残留类**（已清除，见下文「清理记录」）。
- **22 个技能里 19 个是空目录**，剩下的 3 个中 `cache-xml-export` 还引用着不存在的工具名。
- 上游 36 个技能里 **21 个在本环境完全用不上**：vector/AI、Embedded Python、MDX（本实例 0 个
  cube）、pgwire、7 个 Docker 技能、IRIS 产品文档、`iris_coverage`、JDBC 配置错误的
  `iris-connectivity`、opencode、Ensemble（本实例 `ENSEMBLE=false`，零 `Ens.*` 类）。

所以按本环境的真实能力重新实现，工具名统一 `cache_*`，MCP 名 `cache-v1`。

## 部署形态

```text
F:\projects\work\hfhis-8.17\
├── .cache-v1.toml                        # 连接事实（host/端口/命名空间/账号/同步映射）
├── .tools\cache-v1\cache-v1.mjs          # MCP 服务器，零依赖，node 直接跑
├── .mcp.json                             # 注册为 cache-v1
├── .codex\config.toml                    # Codex 注册 + 写工具 approval_mode
├── .codex\hooks\post_tool_use_sync.ps1   # Codex 保存钩子
├── .claude\settings.json                 # Claude Code 保存钩子
├── .claude\skills\<8 个技能>              # Claude Code 读这里
└── .agents\skills\<同样 8 个>             # Codex 读这里
```

没有编译步骤，没有 npm 依赖。`node .tools\cache-v1\cache-v1.mjs --check` 一条命令自检。

## 12 个工具

| 工具                              | 用途                                             | 实测耗时                       |
| --------------------------------- | ------------------------------------------------ | ------------------------------ |
| `cache_check_config`              | 连接 + 版本 + 命名空间 + exec 助手自检           | —                              |
| `cache_info`                      | 服务器版本、API 版本、特性开关                   | 105 ms                         |
| `cache_query`                     | 动态 SQL，返回 JSON 行                           | 12 ms                          |
| `cache_execute`                   | 单行 ObjectScript（`XECUTE`），捕获 `Write` 输出 | 12 ms                          |
| `cache_execute_method`            | 调类方法                                         | 14 ms                          |
| `cache_doc`                       | 读/写/删服务器文档（类、例程、web 资产）         | 19 ms                          |
| `cache_docnames`                  | 按名称列文档                                     | 13.7 s（全量）/ 更快带 pattern |
| `cache_compile`                   | 批量编译，返回 `OK`/`ERROR` 行                   | 21 ms                          |
| `cache_global`                    | globals 读/遍历/写/删                            | 10–17 ms                       |
| `cache_info` → `cache_table_info` | 类的属性、索引、**SQL 表名**                     | 33 ms                          |
| `cache_resolve_storage`           | 类的全局名与 SQL 表名                            | 62 ms                          |
| `cache_sync`                      | 推送本地文件并编译                               | 392 ms（代码）/ 26 ms（web）   |

### 砍掉的能力与原因

| 上游有                            | 本环境状态                                | 结论                                                                                                                                 |
| --------------------------------- | ----------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| `iris_search`（代码搜索）         | `/action/search` 返回 **404**，路由不存在 | 无法实现。`$preprocess` 不存在，`%Dictionary` 的 `Implementation` 列全空，`^oddDEF` 只有签名没有方法体——**没有代码搜索，也造不出来** |
| `iris_doc mode=fragment/compiled` | v1 只返回整份文档                         | 砍掉。`cache_doc` 总是返回全文                                                                                                       |
| `docs_introspect`                 | 无等价路由                                | 砍掉。系统类用 `cache_doc` 读                                                                                                        |
| `iris_test`                       | —                                         | 砍掉。用 `cache_execute` 调 `%UnitTest.Manager`                                                                                      |
| `iris_debug` / `journal_search`   | `journal_search` 在本版本会挂起           | 砍掉                                                                                                                                 |
| `iris_admin` / production 工具    | `ENSEMBLE=false`，零 `Ens.*` 类           | 砍掉                                                                                                                                 |
| `use_session` 会话态              | 会持有许可                                | 砍掉。每次调用独立                                                                                                                   |

## 本环境的实测陷阱

这些是写这套工具时踩出来的，都已固化进代码或技能：

### 1. 类名不是表名（最重要）

```text
业务类 web.DHCBL.CT.CTLoc    0 个属性，纯逻辑，不能 SELECT
持久类 User.CTLoc            103 个属性，有 Storage
SQL 表 SQLUser.CT_Loc        这才是 FROM 后面该写的
```

本命名空间有 **5,842 个持久类的 `SqlTableName` 与类名不同**。`FROM User.CTLoc` 报
SQLCODE -1（`USER` 是保留字），`FROM "User".CTLoc` 和 `FROM User_CTLoc` 也都失败。

`cache_table_info` 和 `cache_resolve_storage` 都会打印 `SQL table :` 一行。

### 2. 没有 `$SYSTEM.SQL.Explain`，但有 `ShowPlan`

`$ClassMethod("%SYSTEM.SQL","Explain",...)` → `<METHOD DOES NOT EXIST>`。
`%SQL.Statement` 也没有 `%GetPlan()`。真正的入口：

```objectscript
Set sql=1,sql(1)="SELECT TOP 3 Name FROM %Dictionary.ClassDefinition" Do ##class(%SYSTEM.SQL).ShowPlan(.sql,0)
```

`sql` 按引用传（`sql=行数`，`sql(1)=第一行`），`array=0` 直接打印，`array=1` 结果进
`%plan()`。输出形如：

```xml
<plan>
<sql>SELECT TOP 3 Name FROM %Dictionary.ClassDefinition</sql>
<cost value="20400200"/>
Read master map %Dictionary.ClassDefinition.Master, looping on ID.
</plan>
```

### 3. 文档名要从文件内容取，不能从文件名猜

- `.cls` → 读文件内 `Class Pkg.Name` 声明。本仓库有三个不同的 `CTLoc.cls`，按 basename 推会
  新建错文档。
- `.mac`/`.int`/`.inc` → 读首行 `ROUTINE <name>`，**可以带点号**且可能与文件名不符：17 个例程
  如此，如 `BalanceDetails.mac` 声明 `ROUTINE DHCWL.Account.BalanceDetails`。
- web 资产 → 路径形式，**不带前导斜杠**：`dthealth/web/csp/ANInfo.csp`。裸名报 `#16006`，
  前导斜杠 404，`%2F` 也 404。

### 4. web 资产在 v1 上可读可写

与旧文档说法相反。`PUT /v1/DHC-APP/doc/dthealth/web/probe.js` → 201，`db:"@FS"`，能读回。
全部 48,443 个 web 资产都在 `src\dthealth\web` 下，一个根映射就够。

### 5. 其他

| 现象                                                     | 原因 / 绕法                                                   |
| -------------------------------------------------------- | ------------------------------------------------------------- |
| `NOT %STARTSWITH` 语法错误                               | 写 `NOT (x %STARTSWITH y)`                                    |
| `If SQLCODE` 在 100 时也成立                             | 用 `SQLCODE = 0`                                              |
| 编译输出和错误是**德文**                                 | 读 `OK`/`ERROR` 前缀和 `#NNNN`，不匹配英文                    |
| `$ISLIST`、`$LISTVALID`、`$preprocess` 不存在            | `$LISTLENGTH` 可用                                            |
| `docnames/CSP` 分类返回 504 + HTML                       | 用路径形式直接读                                              |
| `%Dictionary.StorageDataDefinition` 无 `DataLocation` 列 | `cache_resolve_storage` 解析类源码                            |
| 重 SQL 触发网关返回 HTML                                 | 加 `TOP` 和窄 `WHERE`                                         |
| 许可约 300 单位                                          | 一次一个请求，间隔 1–2 秒，退出时 `?CacheLogout=end`          |
| Cookie 名含端口和路径                                    | 按前缀 `CSPSESSIONID-SP-57772-UP-api-atelier-` 匹配，不硬编码 |
| `Set tRef = ^Foo` 会求值                                 | `Set tRef = "^Foo"` 再用 `@tRef@(sub)`                        |
| `$System.OBJ.Compile` 要裸类名                           | `cache_compile` 自动剥 `.cls`                                 |

## 8 个技能

| 技能                        | 何时用                                      | 行数              |
| --------------------------- | ------------------------------------------- | ----------------- |
| `objectscript-guardrails`   | 写或改任何 ObjectScript                     | 185               |
| `cache-sql`                 | 写/改/调 SQL，或遇 SQLCODE 报错             | 220               |
| `objectscript-navigation`   | 找一个方法在哪、改代码前摸清结构            | 176               |
| `cache-query-plans`         | SQL 慢、怀疑没走索引、要看计划              | 169               |
| `objectscript-mac-routines` | 写 `.mac`/`.int`/`.inc`，读栈里的例程行     | 149               |
| `cache-ensemble-docs`       | Caché 产品语义、CSP 服务端、globals、系统类 | 55（原有）        |
| `cache-xml-export`          | 导出 Studio 兼容 XML                        | 201（原有，已修） |
| `hisui-frontend`            | HISUI 组件 API 与页面开发                   | 51（原有）        |

`cache-ensemble-docs` 的离线文档语料 `references/docs.sqlite3`（86 MB）**不随仓库分发**。
需要时在该技能目录下跑：

```powershell
python scripts/refresh_corpus.py
```

`hisui-frontend` 的 53 个 `references/*.md` 正常分发（约 280 KB）。

## 部署到新环境

1. 复制 `mcp/cache-v1.mjs` 到 `<项目>\.tools\cache-v1\cache-v1.mjs`。
2. 复制 `skills/*` 到 `<项目>\.claude\skills\` 和 `<项目>\.agents\skills\`。
3. 写 `.cache-v1.toml`（参考本目录部署形态里的字段）。
4. 注册 MCP：

   ```json
   {
     "mcpServers": {
       "cache-v1": {
         "type": "stdio",
         "command": "node",
         "args": [".tools/cache-v1/cache-v1.mjs"],
         "env": {}
       }
     }
   }
   ```

5. 跑 `node .tools\cache-v1\cache-v1.mjs --check`。首次会自动安装 exec 助手类
   `CacheV1.MCP.Run.cls`（用 `--install-helper` 可单独执行）。
6. 装保存钩子（可选）：

   ```json
   {
     "hooks": {
       "PostToolUse": [
         {
           "matcher": "Edit|Write",
           "hooks": [
             {
               "type": "command",
               "command": "node .tools/cache-v1/cache-v1.mjs --sync --hook"
             }
           ]
         }
       ]
     }
   }
   ```

钩子在成功时**静默**，只在编译失败时输出并以非零码退出。

## CLI

```powershell
node .tools\cache-v1\cache-v1.mjs --check              # 自检
node .tools\cache-v1\cache-v1.mjs --install-helper     # 只装 exec 助手类
node .tools\cache-v1\cache-v1.mjs --sync <文件...>      # 推送并编译
node .tools\cache-v1\cache-v1.mjs --sync --hook        # 钩子模式（读 stdin）
node .tools\cache-v1\cache-v1.mjs --logout             # 释放 CSP 会话
node .tools\cache-v1\cache-v1.mjs --verbose            # 调试输出
```

## 自检输出示例

```text
Configuration
  config file : F:\projects\work\hfhis-8.17\.cache-v1.toml
  server      : http://10.146.8.17:57772  [config]
  namespace   : DHC-APP  [config]
  write tools : enabled

Server
  version : Cache for Windows (x86-64) 2016.2.3 (Build 907_11_20753U) Mon Apr 5 2021 19:51:59 EDT
  api     : v1
  features: DEEPSEE=true ENSEMBLE=false HEALTHSHARE=false
  status  : OK

Exec helper
  class   : CacheV1.MCP.Run.cls (present)
  call    : CALL CacheV1_MCP.Run_Exec(?)
  status  : OK

All checks passed.
```

## 验证记录

20 项调用矩阵全部通过（`--check` + 18 个工具 + `%Activate.Enum.cls` 读取 + web 同步），
保存钩子对 Claude Code 和 Codex 两种载荷都验过：`.md` 静默退出 0，坏的 `.cls` 输出德文编译
错误并退出 1。

`skills/` 与 `mcp/` 和部署副本逐字节一致（133 个文件对逐一比对哈希），三处例外都是有意为之：

- `.claude\skills\cache-ensemble-docs\references\docs.sqlite3`（86 MB）只在部署侧，不进仓库，
  见上文重建方法。
- `.agents\skills\cache-ensemble-docs` 和 `.agents\skills\hisui-frontend` 只有 `SKILL.md`，
  内容是各自 `.claude` 版本的精简指针，把 `references/` 与 `scripts/` 的路径指向 `.claude`
  一侧，这样 86 MB 语料和 53 个参考文件各只有一份。这两个技能不是本次新增的，结构沿用仓库
  原有安排。

其余 6 个新技能在两个技能树里都是完整副本。

## 清理记录

旧 exe 留下的 **841 个 `IrisDevTmp.IrisDevRun<hex>` 临时类已全部删除**。删前确认过：命名统一
（无例外）、零持久类、零外部子类、无 `^IrisDevTmp` 数据 global。执行方式是一条 `cache_execute`
里用 `$System.OBJ.Delete` 逐个删，**0 错误，26.8 秒，只占 1 次 HTTP 请求**——841 次单独调用会
把 300 个 license 单位耗光。

结果核对：类文档数 34,197 → 33,356（正好少 841），`%Dictionary.CompiledClass` 剩余 0，
包定义 `IrisDevTmp` 已消失，`^oddDEF("IrisDevTmp")` 为 0。

## 已知遗留

- 服务器 `DHC-APP` 的 `CacheV1.MCP.Run.cls` 是本工具安装的助手类，属于有意保留。
