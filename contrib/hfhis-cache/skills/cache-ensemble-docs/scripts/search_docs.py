#!/usr/bin/env python3
"""Search the bundled Caché and Ensemble documentation corpus."""

from __future__ import annotations

import argparse
import json
import re
import sqlite3
from pathlib import Path


def main() -> None:
    default_database = Path(__file__).resolve().parents[1] / "references" / "docs.sqlite3"
    parser = argparse.ArgumentParser(
        description="Search official Caché and Ensemble 2018.1 documentation offline."
    )
    parser.add_argument("query", help="SQLite FTS5 query; English terms work best")
    parser.add_argument("--limit", type=int, default=8)
    parser.add_argument("--product", choices=["Caché", "Ensemble"])
    parser.add_argument(
        "--type", dest="content_type", choices=["Documentation", "Class Reference"]
    )
    parser.add_argument("--database", type=Path, default=default_database)
    parser.add_argument("--json", action="store_true")
    parser.add_argument(
        "--raw",
        action="store_true",
        help="Treat query as raw SQLite FTS5 syntax instead of quoting its terms",
    )
    args = parser.parse_args()

    clauses = ["docs_fts MATCH ?"]
    terms = re.findall(r"[^\W_]+", args.query, flags=re.UNICODE)
    if not terms:
        parser.error("query must contain at least one searchable term")
    fts_query = args.query if args.raw else " AND ".join(
        '"' + term.replace('"', '""') + '"' for term in terms
    )
    parameters: list[object] = [fts_query]
    if args.product:
        clauses.append("records.product = ?")
        parameters.append(args.product)
    if args.content_type:
        clauses.append("records.content_type = ?")
        parameters.append(args.content_type)
    parameters.extend([args.query, max(1, min(args.limit, 50))])

    sql = f"""
        SELECT records.object_id, records.title, records.breadcrumbs, records.product,
               records.content_type, records.topic_id, records.url,
               snippet(docs_fts, 1, '[', ']', ' … ', 42) AS snippet,
               bm25(docs_fts, 7.0, 1.0, 2.0, 4.0) AS score
          FROM docs_fts
          JOIN records ON records.rowid = docs_fts.rowid
         WHERE {' AND '.join(clauses)}
         ORDER BY CASE WHEN lower(records.title) = lower(?) THEN 0 ELSE 1 END,
                  score
         LIMIT ?
    """
    connection = sqlite3.connect(args.database)
    connection.row_factory = sqlite3.Row
    rows = [dict(row) for row in connection.execute(sql, parameters)]
    connection.close()

    if args.json:
        print(json.dumps(rows, ensure_ascii=False, indent=2))
        return
    for index, row in enumerate(rows, 1):
        print(f"{index}. {row['title']} [{row['product']} / {row['content_type']}]")
        print(f"   ID: {row['object_id']}")
        if row["breadcrumbs"]:
            print(f"   {row['breadcrumbs']}")
        print(f"   {row['snippet']}")
        print(f"   {row['url']}")


if __name__ == "__main__":
    main()
