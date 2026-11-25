#!/usr/bin/env python3
"""Extract the public HISUI API site into offline Markdown references."""

from __future__ import annotations

import argparse
import difflib
import html
import re
import sys
import tempfile
import time
import urllib.error
import urllib.parse
import urllib.request
from collections import defaultdict
from datetime import datetime, timezone
from pathlib import Path

try:
    from bs4 import BeautifulSoup, Comment, Tag
except ImportError:
    sys.exit("Missing dependency: install beautifulsoup4 before running update_docs.py")


DEFAULT_BASE_URL = "http://hisui.cn/api/"
SCRIPT_DIR = Path(__file__).resolve().parent
DEFAULT_OUTPUT_DIR = SCRIPT_DIR.parent / "references"
USER_AGENT = "Mozilla/5.0 (compatible; HISUI-offline-doc-extractor/1.0)"
SHARED_SCRIPT_PATTERNS = (
    "jquery-",
    "jquery.hisui",
    "hisui-lang-",
    "jquery-tag-demo",
    "mock-min",
)


def fetch(url: str, attempts: int = 4) -> tuple[str, dict[str, str]]:
    last_error: urllib.error.URLError | None = None
    for attempt in range(1, attempts + 1):
        request = urllib.request.Request(url, headers={"User-Agent": USER_AGENT})
        try:
            with urllib.request.urlopen(request, timeout=30) as response:
                charset = response.headers.get_content_charset() or "utf-8"
                content = response.read().decode(charset, errors="replace")
                headers = {key.lower(): value for key, value in response.headers.items()}
                return content, headers
        except urllib.error.URLError as exc:
            last_error = exc
            if attempt < attempts:
                time.sleep(attempt)
    raise RuntimeError(f"Failed to fetch {url} after {attempts} attempts: {last_error}")


def clean_text(value: str) -> str:
    value = html.unescape(value).replace("\xa0", " ")
    value = re.sub(r"[ \t]+", " ", value)
    value = re.sub(r"\s*\n\s*", "\n", value)
    return value.strip()


def slug_for(source_path: str) -> str:
    path = source_path.removesuffix(".html")
    return re.sub(r"[^a-z0-9]+", "-", path.lower()).strip("-") + ".md"


def code_language(tag: Tag) -> str:
    classes = " ".join(tag.get("class", []))
    for marker, language in (
        ("lang-html", "html"),
        ("lang-js", "javascript"),
        ("lang-css", "css"),
        ("language-html", "html"),
        ("language-javascript", "javascript"),
    ):
        if marker in classes:
            return language
    child = tag.find("code")
    if child:
        return code_language(child)
    return ""


def pre_code(tag: Tag, body: Tag) -> str:
    code = tag.get_text("\n", strip=False).strip()
    if code:
        return code

    target_id = tag.get("id")
    if target_id:
        sources = body.select(f'[prettyprintfor="#{target_id}"]')
        rendered = []
        for source in sources:
            rendered.append(
                source.get_text("\n", strip=False).strip()
                if source.name == "script"
                else str(source)
            )
        code = "\n".join(part for part in rendered if part.strip()).strip()
        if code:
            return code

    parent = tag.parent
    if isinstance(parent, Tag) and "demo-exp-code" in parent.get("class", []):
        siblings = [
            str(sibling)
            for sibling in tag.previous_siblings
            if isinstance(sibling, Tag) and sibling.name not in {"script", "style"}
        ]
        return "\n".join(reversed(siblings)).strip()
    return ""


def normalized_code(value: str) -> str:
    return re.sub(r"\s+", "", value)


def substantially_duplicates(code: str, examples: list[str]) -> bool:
    normalized = normalized_code(code)
    if not normalized:
        return True
    for example in examples:
        candidate = normalized_code(example)
        if normalized == candidate:
            return True
        shorter, longer = sorted((normalized, candidate), key=len)
        if len(shorter) >= 80 and shorter in longer:
            return True
        if min(len(normalized), len(candidate)) >= 80:
            ratio = difflib.SequenceMatcher(None, normalized, candidate).ratio()
            if ratio >= 0.82:
                return True
    return False


def render_table(table: Tag) -> str:
    rows: list[list[str]] = []
    for row in table.find_all("tr"):
        cells = [
            clean_text(cell.get_text(" ", strip=True)).replace("|", "\\|")
            for cell in row.find_all(["th", "td"], recursive=False)
        ]
        if cells:
            rows.append(cells)
    if not rows:
        return ""
    width = max(len(row) for row in rows)
    rows = [row + [""] * (width - len(row)) for row in rows]
    header = rows[0]
    body = rows[1:]
    lines = [
        "| " + " | ".join(header) + " |",
        "| " + " | ".join("---" for _ in range(width)) + " |",
    ]
    lines.extend("| " + " | ".join(row) + " |" for row in body)
    return "\n".join(lines)


def is_inside(tag: Tag, names: set[str]) -> bool:
    return any(parent.name in names for parent in tag.parents if isinstance(parent, Tag))


def is_demo_text(tag: Tag) -> bool:
    return any(
        isinstance(parent, Tag)
        and {"demo-exp-code", "hisui-panel"}.intersection(parent.get("class", []))
        for parent in tag.parents
    )


def heading_text(tag: Tag) -> str:
    parts: list[str] = []
    for node in tag.descendants:
        if isinstance(node, Tag):
            continue
        parent_names = {
            parent.name for parent in node.parents if isinstance(parent, Tag) and parent is not tag
        }
        if not parent_names.intersection({"pre", "script", "style", "table"}):
            parts.append(str(node))
    return clean_text(" ".join(parts))


def render_body(body: Tag) -> list[str]:
    blocks: list[str] = []
    seen: set[tuple[str, str]] = set()
    pre_examples = [pre_code(pre, body) for pre in body.find_all("pre")]

    def append(kind: str, content: str) -> None:
        content = content.strip()
        key = (kind, content)
        if not content or key in seen:
            return
        seen.add(key)
        blocks.append(content)

    for tag in body.find_all(
        ["h1", "h2", "h3", "h4", "p", "span", "pre", "table", "ul", "ol", "script"]
    ):
        if not isinstance(tag, Tag):
            continue
        if tag.name in {"p", "span", "ul", "ol"} and is_inside(
            tag, {"pre", "table", "script", "li"}
        ):
            continue

        if tag.name in {"h1", "h2", "h3", "h4"}:
            level = int(tag.name[1])
            append("heading", f"{'#' * level} {heading_text(tag)}")
        elif tag.name == "pre":
            language = code_language(tag)
            code = pre_code(tag, body)
            append("code", f"```{language}\n{code}\n```")
        elif tag.name == "table":
            if "table" in tag.get("class", []):
                append("table", render_table(tag))
        elif tag.name == "script":
            if tag.get("src"):
                continue
            code = tag.get_text("\n", strip=False).strip()
            if (
                len(code) >= 30
                and "HISUIStyleCode" not in code
                and not substantially_duplicates(code, pre_examples)
            ):
                append("script", f"```javascript\n{code}\n```")
        elif tag.name in {"ul", "ol"}:
            if is_demo_text(tag):
                continue
            items = [
                clean_text(item.get_text(" ", strip=True))
                for item in tag.find_all("li", recursive=False)
            ]
            marker = "1." if tag.name == "ol" else "-"
            append("list", "\n".join(f"{marker} {item}" for item in items if item))
        else:
            if is_demo_text(tag):
                continue
            text = clean_text(tag.get_text(" ", strip=True))
            if text:
                append("text", text)
    return blocks


def table_of_contents(blocks: list[str]) -> list[str]:
    headings: list[tuple[int, str]] = []
    for block in blocks:
        match = re.fullmatch(r"(#{2,4}) (.+)", block)
        if match:
            headings.append((len(match.group(1)), match.group(2)))
    if sum(len(block.splitlines()) + 1 for block in blocks) <= 100 or not headings:
        return []
    minimum_level = min(level for level, _ in headings)
    lines = ["## 本页目录", ""]
    for level, title in headings:
        indent = "  " * (level - minimum_level)
        lines.append(f"{indent}- {title}")
    lines.append("")
    return lines


def collect_entries(index_html: str) -> list[dict[str, str]]:
    soup = BeautifulSoup(index_html, "html.parser")
    for comment in soup.find_all(string=lambda node: isinstance(node, Comment)):
        comment.extract()

    entries: list[dict[str, str]] = []
    seen_paths: set[str] = set()
    for anchor in soup.select("a.api-navi-tab[src]"):
        source_path = anchor.get("src", "").strip()
        if not source_path.endswith(".html") or source_path in seen_paths:
            continue
        category_node = anchor.find_parent("div", attrs={"title": True})
        category = category_node.get("title", "其他").strip() if category_node else "其他"
        entries.append(
            {
                "category": category,
                "label": clean_text(anchor.get_text(" ", strip=True)),
                "path": source_path,
                "file": slug_for(source_path),
            }
        )
        seen_paths.add(source_path)
    return entries


def page_markdown(
    entry: dict[str, str],
    page_html: str,
    page_url: str,
    last_modified: str,
    extracted_at: str,
) -> tuple[str, str]:
    soup = BeautifulSoup(page_html, "html.parser")
    title = clean_text(soup.title.get_text(" ", strip=True)) if soup.title else entry["label"]
    body = soup.body
    if body is None:
        raise RuntimeError(f"No <body> found in {page_url}")

    blocks = render_body(body)
    contents = table_of_contents(blocks)
    lines = [
        f"# {entry['label']}",
        "",
        f"- 官方页面标题：{title}",
        f"- 分类：{entry['category']}",
        f"- 来源：{page_url}",
        f"- 官网更新时间：{last_modified or '未提供'}",
        f"- 提取时间：{extracted_at}",
        "",
        "> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。",
        "",
        *contents,
        *sum(([block, ""] for block in blocks), []),
    ]
    return title, "\n".join(lines).rstrip() + "\n"


def index_markdown(
    entries: list[dict[str, str]], extracted_at: str, base_url: str
) -> str:
    grouped: dict[str, list[dict[str, str]]] = defaultdict(list)
    for entry in entries:
        grouped[entry["category"]].append(entry)

    lines = [
        "# HISUI 离线文档索引",
        "",
        f"- 来源：{base_url}",
        f"- 提取时间：{extracted_at}",
        f"- 页面数量：{len(entries)}",
        "",
        "按组件中文名、英文插件名、属性、方法或事件检索本目录。组件接口不确定时，必须打开对应参考文件核对。",
        "",
    ]
    for category, items in grouped.items():
        lines.extend([f"## {category}", ""])
        for item in items:
            title = item.get("title") or item["label"]
            lines.append(
                f"- [{item['label']}]({item['file']}) — 官网标题：{title}；源路径：`{item['path']}`"
            )
        lines.append("")
    return "\n".join(lines).rstrip() + "\n"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--base-url", default=DEFAULT_BASE_URL)
    parser.add_argument("--output-dir", type=Path, default=DEFAULT_OUTPUT_DIR)
    args = parser.parse_args()

    base_url = args.base_url.rstrip("/") + "/"
    output_dir = args.output_dir.resolve()
    output_dir.mkdir(parents=True, exist_ok=True)
    extracted_at = datetime.now(timezone.utc).replace(microsecond=0).isoformat()

    index_html, _ = fetch(base_url)
    entries = collect_entries(index_html)
    if not entries:
        raise RuntimeError(f"No component pages found at {base_url}")

    generated_files: set[str] = {"index.md"}
    with tempfile.TemporaryDirectory(
        prefix=".hisui-reference-update-", dir=output_dir.parent
    ) as temporary_dir:
        staging_dir = Path(temporary_dir)
        for position, entry in enumerate(entries, start=1):
            page_url = urllib.parse.urljoin(base_url, entry["path"])
            page_html, headers = fetch(page_url)
            title, markdown = page_markdown(
                entry,
                page_html,
                page_url,
                headers.get("last-modified", ""),
                extracted_at,
            )
            entry["title"] = title
            (staging_dir / entry["file"]).write_text(markdown, encoding="utf-8")
            generated_files.add(entry["file"])
            print(
                f"[{position:02d}/{len(entries):02d}] "
                f"{entry['path']} -> {entry['file']}"
            )

        (staging_dir / "index.md").write_text(
            index_markdown(entries, extracted_at, base_url), encoding="utf-8"
        )
        for generated_file in generated_files:
            (staging_dir / generated_file).replace(output_dir / generated_file)
        for old_file in output_dir.glob("*.md"):
            if old_file.name not in generated_files:
                old_file.unlink()
                print(f"Removed stale reference: {old_file.name}")

    print(f"Generated {len(entries)} references in {output_dir}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
