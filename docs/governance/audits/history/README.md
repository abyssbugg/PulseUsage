# Audit Run History

One file per completed audit run, named `<CHARTER>-<YYYY-MM-DD>.md` (e.g. `SEC-2026-07-14.md`).
Charter prefixes: SEC, ARC, PERF, REL, UX — see [../AuditProcess.md](../AuditProcess.md).

Each record contains: trigger, scope, invariant-check results, findings (refuted ones included and marked),
register links, and for PERF the measured baseline numbers. Filenames sort chronologically per charter;
this directory is the only home for run records.
