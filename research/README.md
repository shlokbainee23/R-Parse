# R-PARSE Research Corpus

This directory contains the written research record for R-PARSE. The canonical description of the external projects is [`reference-landscape.md`](reference-landscape.md). Scope, architecture, the technical specification, gap analysis, and adversarial validation stay in their own files.

The third-party checkouts used during that research are not in this repository. Their official URLs and the local commit pins are in the reference landscape. A later working copy may keep those checkouts in a sibling directory; `research/repos/` is ignored so they are not added back by accident.

## Research Methodology

Every major project in the landscape was:

1. Cloned locally and inspected at a recorded commit.
2. Read at the file, function, and grammar-rule level cited in the landscape.
3. Compared with the other projects.
4. Distinguished as a verified observation, a recorded note, or a hypothesis.

## Directory Structure

```
research/
├── README.md
├── reference-landscape.md             ← Canonical external-project record
├── scope-freeze.md                    ← Frozen R-PARSE scope
├── technical-specification.md
├── ADR-001-architecture.md
├── gap-analysis.md
├── candidate-features.md
├── opportunity-matrix.md
├── adversarial-validation.md
├── adversarial-validation-v2.md
├── flowr-deep-gap-audit.md            ← Later flowR dataframe audit
├── grammar-comparison.md
├── ast-comparison.md
├── feature-matrix.md
├── capability-vocabulary.md
├── statistical-ecosystem.md
├── visualization-ecosystem.md
├── workflow-analysis.md
├── runtime-correlation.md
├── architecture-options.md
├── benchmark-plan.md
└── FINAL-RESEARCH-REPORT.md           ← Historical pre-scope-freeze synthesis
```

## Key Findings Summary

Start with [`reference-landscape.md`](reference-landscape.md) for what the external tools were observed to do. [`FINAL-RESEARCH-REPORT.md`](FINAL-RESEARCH-REPORT.md) is the earlier synthesis and still contains a few rows the landscape marks as superseded.

## Evidence Standard

Every technical claim is traced to source code, tests, grammar definitions, or official documentation. Claims that could not be verified are marked as "Unknown" or "Likely limitation."
