# JHIP · Atelier v4 技能包与 MCP（vendored）

本目录是 **智慧医工集成平台（JHIP）** 项目里两件产物的**逐字节副本**（sha256 与源文件一致）：

| 来源（JHIP 仓库） | 本目录               | 内容                                                           |
| ----------------- | -------------------- | -------------------------------------------------------------- |
| `.claude/skills/` | [`skills/`](skills/) | 10 个 ObjectScript / IRIS 技能，每个一份 `SKILL.md`            |
| `mcp/`            | [`mcp/`](mcp/)       | `iris-v4-mcp.mjs`（853 行，零 npm 依赖）+ 它自己的 `README.md` |

JHIP 的实例是 **InterSystems HealthConnect 2020.1.1**（WSL2 本地部署，命名空间 `JHIP`），
Atelier REST API 最高只到 **v4**；而本仓库 README 写明要求 IRIS 2023.1+。这份副本的价值就在
这两件事上：一套**已经在活实例上跑通**的 v4-only 工具面，和一套**被真实项目验证过**的
ObjectScript 编码规则集。它是 `feat/atelier-v4-jhip` 分支的参考材料，不是本仓库的发布内容。

## 目录结构

```text
contrib/jhip-atelier-v4/
├── README.md                      # 本文件
├── skills/                        # 10 个 SKILL.md，来自 JHIP 的 .claude/skills/
│   ├── ensemble-production/
│   ├── iris-objectscript-eval/
│   ├── iris-sql/
│   ├── iris-v4-mcp/               # JHIP 独有：教代理用下面这个 MCP
│   ├── objectscript-guardrails/
│   ├── objectscript-list-patterns/
│   ├── objectscript-navigation/
│   ├── objectscript-review/
│   ├── objectscript-sql-patterns/
│   └── objectscript-unit-test/
└── mcp/
    ├── iris-v4-mcp.mjs            # stdio MCP 服务器，Atelier v4 only
    └── README.md                  # 原作者写的使用说明（相对链接指向 JHIP 仓库）
```

JHIP 的 `.agents/skills/` 与 `.claude/skills/` 是同一份内容（`.claude/` 是给 Claude Code 自动发现
用的镜像，由它的 `scripts/sync-agent-skills.ps1` 生成），这里复制的是用户指定的 `.claude/skills/`，
并已核对两份镜像 10 个文件全部 hash 相同。

## MCP 服务器：8 个工具

`mcp/iris-v4-mcp.mjs` 用 stdio 说 JSON-RPC，业务端点全部落在 `/api/atelier/v4` 下（只有探测走
版本无关的 `GET /api/atelier/`）：

| 工具              | v4 机制                           | 要点                                                                                                       |
| ----------------- | --------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| `check_config`    | `GET /api/atelier/` + 助手检查    | 打印解析出的 host/port/namespace/user 及**每个值的来源**、IRIS 版本、`api` 值、命名空间列表、exec 助手状态 |
| `iris_query`      | `POST /{ns}/action/query`         | 结果按 JSON 行返回，超过 500 行截断；`?` 占位符传参                                                        |
| `iris_execute`    | `CALL JHIP_MCP.Run_Exec(?)`       | v4 没有代码执行路由，用 SqlProc 助手代跑（见下）                                                           |
| `iris_doc`        | `GET/PUT/DELETE /{ns}/doc/{name}` | `put` **不编译**，要接着调 `iris_compile`                                                                  |
| `iris_docnames`   | `GET /{ns}/docnames/{cat}`        | 分类 `CLS`/`RTN`/`CSP`/`OTH`/`*`，`filter` 是 SQL LIKE                                                     |
| `iris_compile`    | `POST /{ns}/action/compile`       | 默认 `flags=cuk`，返回逐文档状态                                                                           |
| `iris_search`     | `GET /{ns}/action/search`         | 服务端内容搜索，`sys=true` 才含 `%` 系统文档                                                               |
| `iris_production` | 助手 + SQL                        | `status` / `items` / `event_log` / `messages` / `start` / `stop` / `restart`，改状态必须 `confirm: true`   |

### v4 的缺口：没有代码执行路由

代码执行是后来的 API 版本才有的。这里用一个单方法的 SqlProc 助手绕过去：

```text
CALL JHIP_MCP.Run_Exec(?)     -- ? 就是要执行的 ObjectScript
```

Atelier 的 query action 会把代码 `Write` 出来的东西放进响应的 `console` 数组，工具就显示这个。

助手类 `JHIP.MCP.Run` 的源码**内嵌在 `.mjs` 的 `HELPER_SOURCE`** 里，首次使用时推送到目标命名空间
并编译；之后每次调用先 `GET` 回来**逐行比对内容**，不一致才重推，所以改 `HELPER_SOURCE` 确实能生效。
比对忽略行尾空白——IRIS 会把空的 `///` 存成带一个尾随空格的 `///`，字节级比较会因此每次重推重编译。
`--install-helper` 强制重推。JHIP 仓库另存了一份可读的副本 `src/JHIP/MCP/Run.cls`，本目录没有复制它
（它不在 `.claude/skills/` 和 `mcp/` 范围内，且 `.mjs` 自带全文）。

执行前会把多行代码压成一行再 `XECUTE`（`Flatten`）：`XECUTE` 的一个参数里带换行是 `<SYNTAX>`，
而 ObjectScript 里换行只是语句分隔符，用空格替代语义等价。压平前先剥掉 `//` 和 `;` 注释（字符串字面量
里的不剥），丢掉空行和 `#;` 注释，行尾 `_` 续行不需要特殊处理。

这个执行上下文的坑（`skills/iris-v4-mcp/SKILL.md` 里也写了）：

- 读命名空间用 `$ZNSPACE`；`$ZNAMESPACE` 和 `$ZLEVEL` 在这里抛 `<SYNTAX>`。
- 没有交互式 `Read`；不要 `HALT`（会杀掉 worker 进程）。
- **`$$$` 宏不可用**：`XECUTE` 不跑预处理器，`$$$ISOK` / `$$$ISERR` / `$$$OK` 全是 `<SYNTAX>`，
  `#include` 也不可用。改用 `$System.Status.IsOK(sc)` / `$System.Status.IsError(sc)` / `1`。
- 报错的 `+n^JHIP.MCP.Run.1` 行号指向**生成的例程**，不是你发过去的那段代码。

### 连接解析顺序

1. 环境变量：`IRIS_HOST`、`IRIS_WEB_PORT`、`IRIS_SCHEME`、`IRIS_USERNAME`、`IRIS_PASSWORD`、`IRIS_NAMESPACE`
2. 工作区 `.vscode/settings.json` → `objectscript.conn`（server 引用 + ns）
3. VS Code 用户设置 → `intersystems.servers[IRIS_SERVER_NAME || "jhip"]`（host/port/凭据）
4. 默认值：`http://localhost:52773`，命名空间 `USER`

WSL 的 NAT IP 每次重启会变：请求在网络层失败时，服务器执行 `wsl.exe hostname -I` 拿到当前 IP，
切换后重试一次；`check_config` 会同时显示「配置的 host」和「实际用的 host」。

### SQL 表名规则（v4 同样适用）

类 `Package.Sub.Class` → 表 `Package_Sub.Class`（包名的点变下划线，类短名保持原样）：
`Ens.MessageHeader`（一级包保持点号）、`Ens_Config.Item`、`Ens_Util.Log`。
写成 `Ens_MessageHeader` 是错的。`INFORMATION_SCHEMA.ROUTINES` 可用。

非 SELECT 语句能执行，但 v4 不返回 rowcount，要用后续 SELECT 自己确认。

### CLI

```powershell
node contrib/jhip-atelier-v4/mcp/iris-v4-mcp.mjs --check           # 连接报告，失败退出码 1
node contrib/jhip-atelier-v4/mcp/iris-v4-mcp.mjs --install-helper  # 强制（重）装 exec 助手
node contrib/jhip-atelier-v4/mcp/iris-v4-mcp.mjs --verbose         # MCP 模式 + HTTP 日志到 stderr
```

## 技能清单，以及哪些是派生副本

10 个技能里有 9 个和本仓库 `skills/skills/` 下的同名技能同源，只有 `iris-v4-mcp` 是 JHIP 独有。
下表是逐行比对的结果（差异列只列实质内容，表格对齐、空行、YAML 缩进这类不算）：

| 技能                         | 本仓库对应物 | 差异                                                                 |
| ---------------------------- | ------------ | -------------------------------------------------------------------- |
| `ensemble-production`        | 有           | 13 行含 `<...>` 的内容被剥离；`&quot;` 未转义；表格重排              |
| `iris-objectscript-eval`     | 有           | 4 行 `<NOROUTINE>` / `<SYNTAX>` 被剥离                               |
| `iris-sql`                   | 有           | 仅格式差异，另多出 `sql-001..008` 元数据                             |
| `objectscript-guardrails`    | 有           | 仅格式差异，另多出 `jira-001..008` 元数据                            |
| `objectscript-list-patterns` | 有           | 仅格式差异，另多出 `jira-001..008` 元数据                            |
| `objectscript-navigation`    | 有           | 仅 2 行空行差异                                                      |
| `objectscript-review`        | 有           | `` `Quit <value>` `` → `` `Quit ` ``                                 |
| `objectscript-sql-patterns`  | 有           | 1 行 `<DIVIDE>` 被剥离                                               |
| `objectscript-unit-test`     | 有           | `` `Test.<OriginalPackage>.<ClassName>` `` → `` `Test.. ` `` 等 3 行 |
| `iris-v4-mcp`                | **无**       | JHIP 独有，未受损                                                    |

### 已知损坏：`<...>` 被当成 HTML 标签剥掉了

导入 JHIP 时经过了一道 HTML 清洗，凡是被 `<` `>` 包住的文本都被删掉，于是占位符和 IRIS 错误码
变成了空的反引号或空字符串。原文与副本对照：

| 本仓库原文                                                                      | JHIP 副本                          |
| ------------------------------------------------------------------------------- | ---------------------------------- |
| `class lookup succeeds but method dispatch can fail with <NOROUTINE> or return` | ``...can fail with ` ` or return`` |
| `<call target="Agent" async="0" timeout="5">` with `<request>` and `<response>` | `` ` ` with ` ` and ` ` ``         |
| `Test.<OriginalPackage>.<ClassName>`                                            | `` `Test.. ` ``（点后只剩空格）    |

受影响的是 `ensemble-production`（13 行）、`iris-objectscript-eval`（4 行）、`objectscript-unit-test`
（3 行）、`objectscript-review`（1 行）、`objectscript-sql-patterns`（1 行）；其余 4 个同名技能只有格式
差异。`ensemble-production` 丢的恰好是 BPL 的 XML 片段，对这个项目最要紧的部分。

**结论：这 9 个技能在本仓库的 `skills/skills/` 下有更好的版本，不要用这里的副本反向覆盖。**
保留它们的原因是它们是 JHIP 代理**实际加载**的那份文件，差异本身就是要研究的东西。

### `iris-v4-mcp` 技能

唯一没有对应物的一个，也是唯一完好无损的一个。它写给代理看：8 个工具里哪些存在、哪些不存在
（`iris_symbols`、`iris_global`、`ws_*`、`iris_admin`、`iris_test` 在 v4 服务器上**都没有**）、
执行上下文的坑、SQL 表名规则、以及三条日常动线（编译循环、追一条消息、Production 操作）。

## 在本仓库里运行它的注意事项

`.mjs` 用 `PROJECT_ROOT = path.resolve(SCRIPT_DIR, '..')` 定位工作区配置。原位置下那是 JHIP 仓库根，
会读到 JHIP 的 `.vscode/settings.json`；搬到 `contrib/jhip-atelier-v4/mcp/` 之后，它去找的是
`contrib/jhip-atelier-v4/.vscode/settings.json`（不存在）。所以在这里跑要么给环境变量，要么放一份
settings：

```powershell
$env:IRIS_HOST='<wsl-ip>'; $env:IRIS_WEB_PORT='52773'
$env:IRIS_USERNAME='_SYSTEM'; $env:IRIS_PASSWORD='<password>'
$env:IRIS_NAMESPACE='JHIP'
node contrib/jhip-atelier-v4/mcp/iris-v4-mcp.mjs --check
```

`mcp/README.md` 里那些相对链接（`../AGENTS.md`、`../src/JHIP/MCP/Run.cls`、`../.vscode/settings.json`、
`../.agents/skills/`）是按 JHIP 仓库的目录写的，在这里点不开，属于预期。

要把这个服务器挂到 Claude Code 或 DeepSeek Harness，照 `mcp/README.md` 的 Registration 一节做，
只把脚本路径换成本目录的绝对路径即可；那一节里的 `IRIS_SERVER_NAME=jhip` 只有在你确实有同名
VS Code 服务器条目时才需要，否则用上面的环境变量更直接。

## 和本仓库 Rust 服务器的差距

要在本仓库里支持这类实例，缺口大致是这几处（行号以 `feat/atelier-v4-jhip` 分支为准）：

| 位置                                                                          | 现状                                                                                  |
| ----------------------------------------------------------------------------- | ------------------------------------------------------------------------------------- |
| `crates/iris-agentic-dev-core/src/iris/connection.rs:250`                     | `AtelierVersion` 只有 `V1`/`V2`/`V7`/`V8`，没有 v4                                    |
| `crates/iris-agentic-dev-core/src/iris/discovery.rs:233`、`connection.rs:786` | 版本探测把 `api: 4` 归到 **`V2`**，于是 URL 走 `/api/atelier/v2/...`，永远不是 `/v4/` |
| `crates/iris-agentic-dev-core/src/iris/ws_session.rs:237`                     | WebSocket 终端写死 `/api/atelier/v7/...`                                              |
| `README.md`                                                                   | 产品要求 IRIS 2023.1+                                                                 |

这份 `.mjs` 因此是很好的对照物：它用**同一批 v4 端点**实现了 SQL、文档读写、编译、搜索、Production
观测，并用 SqlProc 助手补上了 v4 缺的执行路由——这三件事正好是本仓库在 2020.1 上要过的三关。

## 维护约定

- 本目录的 `skills/` 和 `mcp/` 是**逐字节副本**，不要就地格式化或「顺手修好」。改内容请回 JHIP
  仓库改，再整份复制过来；本目录只负责保存和说明。
- 因此这两个子目录被排除在 prettier 和 markdownlint 之外（见根目录 `.prettierignore` 与
  `.markdownlint-cli2.jsonc`），与本仓库对 `packaging/winget/schemas/` 这类 vendored 目录的处理一致。
  本文件本身仍然受两个 gate 约束。
- 不进入 `skills/skills/`：那里的每个技能都会进 `EMBEDDED_SKILLS`、`skills.sh.json` 分组、
  `docs/skills.md` 清单和文档契约测试，而这里的技能是某个具体项目的产物，不是本仓库的发布内容。
