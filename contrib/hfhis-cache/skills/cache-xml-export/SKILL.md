---
name: cache-xml-export
description: 从 Caché 2016.2.3 (DHC-APP) 导出代码为 Studio 兼容的 XML。当用户要求"导出 XML""导出代码""打包给 Studio（导入/迁移）""export"，或涉及 $system.OBJ.Export、Studio XML 导出格式时使用。覆盖 .cls、.csp、js 等 web 静态文件、.mac/.inc、global、Studio 项目；只导出，不做导入。
---

# Caché 代码 XML 导出（Studio 兼容格式）

复刻 Caché Studio「Tools > Export」：通过 `cache-v1` MCP 的 `cache_execute`
调用 `$system.OBJ.Export*` 家族（Studio 同款 API，内部走 `^%apiOBJ`），产出
Studio「Tools > Import Local」可直接再导入的 `<Export>` XML。

> 本技能全部模板与数值均实测于 DHC-APP @ Cache for Windows (x86-64)
> 2016.2.3 (Build 907_11_20753U)，2026-10-14；2026-11-25 在 `cache-v1` MCP 上复核。
> 模板中 `<ITEMS>`/`<FNAME>`/`<i>` 为占位符，执行前替换。

## 边界

- **只导出**。导入（`$system.OBJ.Load/LoadStream/LoadDir`）属于人工操作：
  Terminal `Do $system.OBJ.Load("file.xml","ck")` 或 Studio Import Local。
  `cache-v1` 不再像旧 exe 那样拦截这些 API，所以这条是约定而非门禁——不要用
  agent 自动导入覆盖服务器代码。
- csp/js 等 web 文档 **可以** 用 `cache_doc` 按路径形式读写（如
  `dthealth/web/csp/ANInfo.csp`，注意无前导斜杠）；`cache_sync` 也会推送
  `src/dthealth/web/**`。XML 导出仍是**批量**备份/迁移的首选通道，但不再是唯一通道。

## 主流程

1. 按「items 命名规则」规范化导出清单，逗号分隔拼成 items 字符串
2. 跑模板 A 探测：总输出 ≤ 15000 字节时单次调用直接回传完成
3. `TOO BIG` 则走模板 B 四步（服务器文件中转 + base64 分块）
4. 交付前校验：开头 `<?xml version="1.0" encoding="UTF-8"?>` +
   `<Export generator="Cache" version="25"`，结尾 `</Export>`；
   本地文件字节数 === 服务器报的 `BYTES`

## items 命名规则（实测，最易踩坑）

| 类型          | items 写法                           | 示例                                        |
| ------------- | ------------------------------------ | ------------------------------------------- |
| .cls 类       | 类名 + `.cls`，支持通配              | `web.DHCBL.CT.CTLoc.cls`、`web.BOE.*.cls`   |
| .mac/.int/.inc| 例程名 + 后缀                        | `addloc.mac`                                |
| .csp 页面     | **完整 URL 路径**（含应用前缀）      | `/dthealth/web/csp/dhc.logon.csp`           |
| js/css/html 等静态 | **完整 URL 路径**               | `/dthealth/web/BOE/HISUI/BOE.AdrShare.js`   |

- 多项逗号分隔（`web.DHCBL.CT.CTLoc.cls,/dthealth/web/csp/dhc.logon.csp,addloc.mac`），
  合成**一个** `<Export>` XML，条目按名称排序
- 类型由后缀决定：CLS / CSP / CSR / MAC / INT / BAS / INC / **GBL（global）** /
  **PRJ（项目）** / OBJ / PKG。GBL/PRJ 的 items 写法未实测，首次使用先单项试
- csp/js 的 URL 前缀 = CSP 应用路径（本环境 `/dthealth/web`，绑定 DHC-APP、
  数据库存储）；对应本地仓库 `src/dthealth/web/**` 目录
- 报错对照：`#6306 没有关联的应用程序` = 用了相对路径；`#6304 XML导出不支持此类型` = 裸名
- qspec（第 3 参）：`"/recursive=1"` 连带依赖（include 文件、超类），导出"可编译全集"时用；
  Export 落文件时文件名以 `.gz` 结尾自动 gzip；Charset（第 5 参）默认 UTF-8

## 模板 A：探测 + 小文件单次直传（一次 cache_execute）

```objectscript
Set xml = ##class(%Stream.GlobalCharacter).%New()
Set sc = $system.OBJ.ExportToStream("<ITEMS>", .xml)
If $System.Status.IsError(sc) {
    Write "EXPORT FAIL",!
    Write $System.Status.GetErrorText(sc),!
} Else {
    Set bytes = $ZCONVERT(xml.Read(xml.Size),"O","UTF8")
    If $Length(bytes) > 15000 {
        Write "TOO BIG bytes=", $Length(bytes),!
    } Else {
        Write "XMLB64=", $SYSTEM.Encryption.Base64Encode(bytes),!
    }
}
```

- 输出预算：15000 字节 → base64 ≈ 20000 字符（+76 折行 +`XMLB64=` 前缀 + Export
  日志行约 150 字符），单次上限 ~30000 内安全
- Export 的日志行跟随**服务器语言**，本实例是**德文**（见坑清单 2），不是中文
- 注意 `$ZCONVERT(...,"O","UTF8")` 后才按**字节**计长（字符流的 Size 是字符数，
  中文差 3 倍，不能直接用）
- 本地处理：取 `XMLB64=` 之后内容，去除所有空白，base64 解码，二进制落盘

## 模板 B：服务器文件中转（任意大小）

### 步骤 1 —— 导出落盘 + 报块数

```objectscript
Set fname = $System.Util.ManagerDirectory() _ "agentexp_" _ $ZDate($H,8) _ "_" _ $Random(99999) _ ".xml"
Set sc = $system.OBJ.Export("<ITEMS>", .fname)
If $System.Status.IsError(sc) {
    Write "EXPORT FAIL",!
    Write $System.Status.GetErrorText(sc),!
} Else {
    Set fs = ##class(%Stream.FileBinary).%New()
    Set fs.Filename = fname
    Set chunks = (fs.Size \ 15000) + $Select((fs.Size # 15000) > 0:1, 1:0)
    Write "FNAME=", fname,!
    Write "BYTES=", fs.Size,!
    Write "CHUNKS=", chunks,!
    Do fs.%Close()
}
```

临时文件落在服务器 Caché 管理目录（实测 `C:\InterSystems\Cache\mgr\`），
随机名防冲突。记下 FNAME / BYTES / CHUNKS。

### 步骤 2 —— 逐块 base64 回传（每块一次调用，i = 0..CHUNKS-1）

```objectscript
Set fs = ##class(%Stream.FileBinary).%New()
Set fs.Filename = "<FNAME>"
Set skip = fs.Read(<i> * 15000)
Write "CHUNKB64=", $SYSTEM.Encryption.Base64Encode(fs.Read(15000)),!
Do fs.%Close()
```

- **二进制读**：块大小精确按字节，中文（UTF-8 每字 3 字节）不会撑爆输出；
  磁盘上的 XML 本来就是 UTF-8 字节，直接编码，无需 TranslateTable / $ZCONVERT
- 每次调用从头跳读 `<i> * 15000` 字节再取块（流无 seek，O(offset) 跳读，
  几十 MB 内可接受；再大就分批 Export）
- 最后一块不足 15000 字节时 Read 自动返回剩余部分

### 步骤 3 —— 本地解码拼接落盘（agent 侧）

先用 Write 工具把每块的 base64（`CHUNKB64=` 之后的内容，保留原样）存为
`chunk1.b64`、`chunk2.b64`…（文件名序号 = i+1），然后：

```bash
python - <<'PYEOF'
import base64, re, glob
out = open('export.xml', 'wb')
total = 0
for f in sorted(glob.glob('chunk*.b64'),
                key=lambda x: int(re.search(r'\d+', x).group())):
    d = base64.b64decode(re.sub(r'\s', '', open(f, encoding='ascii').read()))
    out.write(d)
    total += len(d)
out.close()
print('total bytes:', total)
PYEOF
```

- **必须二进制写**（`'wb'`），UTF-8 字节与 CRLF 换行原样保留
- 校验三件事：`total bytes` === 步骤 1 的 `BYTES`；开头 `<?xml`；结尾 `</Export>`
- 完成后删除 `chunk*.b64`

### 步骤 4 —— 清理（必须在**新的** cache_execute 调用中执行）

```objectscript
Do ##class(%File).Delete("<FNAME>")
Write "CLEANED=", '##class(%File).Exists("<FNAME>"),!
```

`CLEANED=1` 成功。同进程即使 `%Close()` 后删除也可能失败（Windows 句柄未释放，
实测两次）；新调用时上一进程已退出，稳定成功。若仍为 0，下一次调用再试。

## XML 格式速览（校验与解读产物用）

```xml
<?xml version="1.0" encoding="UTF-8"?>
<Export generator="Cache" version="25"
         zv="Cache for Windows (x86-64) 2016.2.3 (Build 907_11_20753U)" ts="…">
<Class name="web.DHCBL.CT.CTLoc">…类字典序列化…</Class>
<CSP name="csp/dhc.logon.csp" application="/dthealth/web/" default="1"><![CDATA[…]]></CSP>
<CSP name="BOE/HISUI/BOE.AdrShare.js" application="/dthealth/web/" default="1"><![CDATA[…]]></CSP>
<Routine name="addloc" type="MAC" languagemode="0" timestamp="61114,59584"><![CDATA[…]]></Routine>
</Export>
```

- `version="25"` = 2016.2 类字典版本（导入端据此做版本兼容协商）
- **js/css/html/png 等静态文件全部以 `<CSP>` 元素表示**：`name` 相对应用根，
  `application` = CSP 应用路径
- `<Class>` 是结构化类字典序列化（Description/Parameter/Method/Query/Storage/XData…，
  方法体在 CDATA），**不是 UDL 文本**；TimeChanged/TimeCreated 为 $HOROLOG 格式

## 坑清单（全部实测）

1. csp/js items 必须 URL 全路径，纯相对名/裸名报 #6306 / #6304（见命名规则表）
2. Export 每次都向输出写日志行——本实例是**德文**（实测：`Exportieren in XML
   gestartet am …`、`Exportiere Klasse: xxx`、`Export erfolgreich abgeschlossen.`），
   合计约 150 字符，占预算；两个模板均已预留。日志语言随服务器 locale 变化，
   不要按中文或英文关键字匹配
3. `%Stream.GlobalCharacter` 不能进 `use_session` 会话态：`%ToJSON` 序列化失败，
   且 `^CacheTemp` 是进程临时的 —— 分块**必须**走服务器文件中转，别走会话
4. 同进程删临时文件不可靠（`%Close()` 后仍可能失败）—— 清理放新调用（模板 B 步骤 4）
5. base64 输出按 76 字符折行 —— 本地解码前先去除**所有**空白
6. 字符流 `.Size` 是字符数、二进制流 `.Size` 是字节数，中文差 3 倍 ——
   输出预算一律按字节口径（模板 A 用 `$ZCONVERT` 后 `$Length`，模板 B 直接二进制流）
7. web 文档名用**路径形式且无前导斜杠**：`dthealth/web/csp/ANInfo.csp` 可以
   `cache_doc` get/put，加前导斜杠会 404，裸名报 #16006。批量导出仍按 URL 全路径
   （`/dthealth/web/...`）——两者前缀不同，别混
8. `cache-v1` 不拦 `$system.OBJ.Load*`；本技能只做导出是**约定**，不是门禁

## 批量 / 变体 API 速查

| API | 用途 |
| --- | --- |
| `$system.OBJ.ExportToStream(items, .stream)` | 模板 A：探测 + 小文件直传 |
| `$system.OBJ.Export(items, .fname)` | 模板 B：落服务器文件 |
| `$system.OBJ.ExportPattern("web.BOE.*.cls", .fname)` | 通配符批量 |
| `$system.OBJ.ExportPackage("web.BOE", .fname)` | 整包 |
| `$system.OBJ.ExportAllClasses(.fname)` | 命名空间全部类 |
| `%Studio.Project.Export(...)` | Studio 项目（PRJ）连同成员 |

均有对应 `*ToStream` 变体；`ExportToStream` 的 items 同样支持逗号分隔与通配。
