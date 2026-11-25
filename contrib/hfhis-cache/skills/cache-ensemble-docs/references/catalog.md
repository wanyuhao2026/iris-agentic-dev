# Offline corpus catalog

## Contents

- [Scope](#scope)
- [Record counts](#record-counts)
- [Provenance](#provenance)
- [Version cautions](#version-cautions)
- [Retrieval notes](#retrieval-notes)

## Scope

The corpus stores the complete public English Algolia search index exposed by the official Caché & Ensemble documentation site for `version = 2018.1.12` and `locale = en-us`. It includes both `Documentation` and `Class Reference` content types. Each record preserves its title, full indexed text, breadcrumbs, topic ID, product, content type, official URL, version, and source timestamp.

The source documentation home page identifies the collection as Caché & Ensemble 2018.1.4–2018.1.12. The linked Documatic Class Reference UI may display 2018.1.11 even though the search records are tagged 2018.1.12; retain this discrepancy when exact patch-level behavior matters.

## Record counts

| Product | Content type | Records |
|---|---:|---:|
| Caché | Documentation | 11,469 |
| Caché | Class Reference | 4,560 |
| Ensemble | Documentation | 1,880 |
| Ensemble | Class Reference | 1,412 |
| **Total** |  | **19,321** |

There are 19,318 distinct topic IDs and 19,318 distinct official URLs. Three topics are indexed once for each applicable product, which explains the record/topic difference.

Database file: `references/docs.sqlite3`  
SHA-256 at creation: `48537a141e6eb51311154aae65afac8cef1936dd7fce2ced60fcf3a470f3e46f`

## Provenance

- Official home: `https://docs.intersystems.com/latest/csp/docbook/DocBook.UI.Page.cls`
- Official public search index: Algolia application and search-only key published in the home page HTML
- Indexed version: `2018.1.12`
- Locale: `en-us`
- Corpus created: 2026-08-14 (Asia/Shanghai)

The corpus is an offline retrieval copy of official indexed records, not a translation and not project-authored documentation.

## Version cautions

The current iMedical repository uses Cache 2016.2.3. Do not assume a feature documented for 2018.1 exists in 2016.2.3. Verify version-sensitive APIs, class members, SQL behavior, security defaults, adapter settings, and Management Portal steps against the deployed system.

Do not use the “See the InterSystems IRIS version” links as evidence for Caché behavior unless the user explicitly asks for IRIS comparison or migration guidance.

## Retrieval notes

The SQLite database uses FTS5 with weighted title, breadcrumb, topic-ID, and body matching. Exact title matches are promoted. Search results provide short snippets; retrieve the full indexed record by object ID only after choosing a relevant result. This keeps context use proportional to the question.
