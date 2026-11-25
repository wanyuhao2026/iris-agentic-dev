#!/usr/bin/env python3
"""Print one complete offline documentation record by object ID."""

from __future__ import annotations

import argparse
import json
import sqlite3
from pathlib import Path


def main() -> None:
    default_database = Path(__file__).resolve().parents[1] / "references" / "docs.sqlite3"
    parser = argparse.ArgumentParser()
    parser.add_argument("object_id", help="ID emitted by search_docs.py")
    parser.add_argument("--database", type=Path, default=default_database)
    parser.add_argument("--json", action="store_true")
    args = parser.parse_args()

    connection = sqlite3.connect(args.database)
    connection.row_factory = sqlite3.Row
    row = connection.execute(
        """
        SELECT object_id, title, text, breadcrumbs, topic_id, product,
               content_type, url, version, source_timestamp
          FROM records
         WHERE object_id = ?
        """,
        (args.object_id,),
    ).fetchone()
    connection.close()
    if row is None:
        parser.error(f"unknown object ID: {args.object_id}")
    document = dict(row)
    if args.json:
        print(json.dumps(document, ensure_ascii=False, indent=2))
        return
    print(f"# {document['title']}")
    print(f"Breadcrumbs: {document['breadcrumbs']}")
    print(f"Product: {document['product']}")
    print(f"Type: {document['content_type']}")
    print(f"Version: {document['version']}")
    print(f"Source: {document['url']}")
    print()
    print(document["text"])


if __name__ == "__main__":
    main()
