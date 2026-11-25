---
name: cache-ensemble-docs
description: Search and apply the complete offline official InterSystems Caché and Ensemble 2018.1.12 documentation corpus, including DocBook guides, language and SQL references, administration and security manuals, web/CSP/SOAP/REST material, Ensemble productions and adapters, HL7/EDI/BPL/DTL guidance, and Caché/Ensemble Class Reference entries. Use for Caché, Cache, Ensemble, ObjectScript, globals, persistent classes, CSP, Zen, SQL, Management Portal, system administration, security, interoperability, productions, business services/processes/operations, adapters, HL7, DTL, BPL, or system-class questions—especially when internet access is unavailable or version-specific official evidence is needed. Do not treat this corpus as IRIS documentation.
---

# Caché and Ensemble 2018 Offline Documentation

Use the bundled SQLite corpus as the primary source. It contains the complete public English search index for version 2018.1.12: DocBook documentation and Class Reference records.

## Search workflow

1. Determine the target product and exact deployed version from repository context or the user. Preserve the spelling `Caché`; use `Cache` only where the current repository glossary requires it.
2. Translate Chinese or local terminology into likely official English terms. Search two or more variants when terminology is ambiguous.
3. Run:

   ```bash
   python3 scripts/search_docs.py 'search terms' --limit 8
   ```

4. Narrow guide questions with `--type Documentation`; narrow API/class questions with `--type 'Class Reference'`. Use `--product Caché` or `--product Ensemble` only when cross-product results add noise.
5. Retrieve a selected record in full using the emitted ID:

   ```bash
   python3 scripts/get_doc.py 'Core2018.1.12-record-id'
   ```

6. Cross-check consequential claims with at least one additional record. Prefer a task guide for workflow and a Class Reference record for signatures, parameters, or inherited behavior.
7. Answer concisely in the user's language. Name the source title and identify it as Caché/Ensemble 2018.1.12 documentation. Include the stored official URL when useful, while keeping the answer usable offline.

## Query guidance

- Search English identifiers exactly: class names, method names, commands, special variables, error codes, configuration labels, and HL7 schema terms.
- The default search safely tokenizes punctuation, so class names such as `EnsLib.HL7.Message` work directly.
- Use raw FTS5 syntax only for deliberate boolean or prefix searches, for example `--raw 'TSTART OR TROLLBACK'` or `--raw 'EnsLib*'`.
- Run several focused searches instead of one long natural-language query.
- Inspect [references/catalog.md](references/catalog.md) for corpus scope, provenance, and version cautions.

## Version discipline

- Treat every result as 2018.1.12 documentation unless its stored metadata says otherwise.
- For this iMedical repository, the deployed database is Cache 2016.2.3, not IRIS. Regard 2018.1 documentation as a later compatibility reference; verify feature availability against repository usage, the installed class dictionary, or a 2016.2 environment before proposing version-sensitive code or configuration.
- Never substitute IRIS-only APIs or syntax merely because an IRIS page is newer.
- If the target version is unknown, state the version assumption.
- Distinguish official documented behavior from repository-specific conventions. Repository `AGENTS.md`, `CONTEXT.md`, ADRs, and local coding rules take precedence for implementation choices.

## Evidence and code safety

- Do not infer a method signature from prose when a Class Reference entry exists.
- Do not infer operational defaults across versions. Flag uncertainty explicitly.
- Before emitting ObjectScript, apply the repository's ObjectScript hard gate and conventions in full.
- Avoid reproducing long documentation passages. Paraphrase and cite the stored title/URL.

## Maintenance

The skill works offline without refreshing. When explicitly asked to update the corpus and network access is allowed, run `python3 scripts/refresh_corpus.py`, then validate counts and representative searches before relying on the new database.
