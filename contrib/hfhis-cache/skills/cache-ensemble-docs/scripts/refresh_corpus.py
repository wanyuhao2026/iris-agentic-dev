#!/usr/bin/env python3
"""Download the public InterSystems documentation search index into SQLite."""

from __future__ import annotations

import argparse
import gzip
import json
import sqlite3
import sys
import time
import urllib.error
import urllib.request
from pathlib import Path


APP_ID = "EP91R43SFK"
SEARCH_KEY = "709759d92d99a5cf927e90c965741389"
INDEX = "docs"
VERSION = "2018.1.12"
ENDPOINT = f"https://{APP_ID}-dsn.algolia.net/1/indexes/{INDEX}/browse"


def fetch_page(cursor: str | None) -> dict:
    payload: dict[str, object] = {}
    if cursor:
        payload["cursor"] = cursor
    else:
        payload["hitsPerPage"] = 1000
        payload["filters"] = f'version:"{VERSION}" AND locale:"en-us"'
    request = urllib.request.Request(
        ENDPOINT,
        data=json.dumps(payload).encode("utf-8"),
        headers={
            "Content-Type": "application/json",
            "Accept-Encoding": "gzip",
            "X-Algolia-Application-Id": APP_ID,
            "X-Algolia-API-Key": SEARCH_KEY,
        },
        method="POST",
    )
    last_error: Exception | None = None
    for attempt in range(1, 6):
        try:
            with urllib.request.urlopen(request, timeout=30) as response:
                raw = response.read()
                if response.headers.get("Content-Encoding") == "gzip":
                    raw = gzip.decompress(raw)
                return json.loads(raw)
        except (urllib.error.URLError, TimeoutError) as exc:
            last_error = exc
            print(f"Request attempt {attempt} failed: {exc}", file=sys.stderr)
            if attempt < 5:
                time.sleep(attempt)
    raise RuntimeError(f"Algolia request failed after retries: {last_error}")


def normalized(record: dict) -> tuple[str, ...]:
    return (
        str(record.get("objectID", "")),
        str(record.get("title", "")),
        str(record.get("text", "")),
        str(record.get("breadcrumbs", "")),
        str(record.get("topicid", "")),
        str(record.get("product", "")),
        str(record.get("contentType", "")),
        str(record.get("URL", "")),
        str(record.get("version", "")),
        str(record.get("timestamp", "")),
    )


def rebuild_database(output: Path) -> int:
    output.parent.mkdir(parents=True, exist_ok=True)
    temporary = output.with_suffix(output.suffix + ".new")
    if temporary.exists():
        temporary.unlink()

    connection = sqlite3.connect(temporary)
    connection.executescript(
        """
        PRAGMA journal_mode=OFF;
        PRAGMA synchronous=OFF;
        CREATE TABLE records (
            object_id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            text TEXT NOT NULL,
            breadcrumbs TEXT NOT NULL,
            topic_id TEXT NOT NULL,
            product TEXT NOT NULL,
            content_type TEXT NOT NULL,
            url TEXT NOT NULL,
            version TEXT NOT NULL,
            source_timestamp TEXT NOT NULL
        );
        CREATE VIRTUAL TABLE docs_fts USING fts5(
            title, text, breadcrumbs, topic_id,
            product UNINDEXED, content_type UNINDEXED, url UNINDEXED,
            content='records', content_rowid='rowid',
            tokenize='unicode61 remove_diacritics 2'
        );
        CREATE INDEX records_topic_id ON records(topic_id);
        CREATE INDEX records_product ON records(product);
        CREATE INDEX records_content_type ON records(content_type);
        """
    )

    total = 0
    cursor = None
    while True:
        page = fetch_page(cursor)
        rows = [normalized(hit) for hit in page.get("hits", [])]
        connection.executemany(
            "INSERT OR REPLACE INTO records VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rows,
        )
        total += len(rows)
        print(f"Downloaded {total} records", file=sys.stderr)
        cursor = page.get("cursor")
        if not cursor:
            break

    connection.execute(
        "INSERT INTO docs_fts(docs_fts) VALUES ('rebuild')"
    )
    connection.execute(
        "CREATE TABLE metadata (key TEXT PRIMARY KEY, value TEXT NOT NULL)"
    )
    connection.executemany(
        "INSERT INTO metadata VALUES (?, ?)",
        [("version", VERSION), ("record_count", str(total)), ("source", ENDPOINT)],
    )
    connection.commit()
    connection.execute("PRAGMA optimize")
    connection.close()
    temporary.replace(output)
    return total


def main() -> None:
    default_output = Path(__file__).resolve().parents[1] / "references" / "docs.sqlite3"
    parser = argparse.ArgumentParser()
    parser.add_argument("--output", type=Path, default=default_output)
    args = parser.parse_args()
    count = rebuild_database(args.output.resolve())
    print(f"Created {args.output.resolve()} with {count} records")


if __name__ == "__main__":
    main()
