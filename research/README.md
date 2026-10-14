# R-PARSE Research Corpus

This directory contains the results of a deep, repository-level investigation of the existing R programming-language ecosystem, conducted to determine what R-PARSE should eventually become.

## Research Methodology

Every major project was:
1. Cloned locally (see `../research/repos/`)
2. Source code inspected at the file, function, and grammar-rule level
3. Tests, documentation, and architecture verified against actual implementation
4. Cross-compared with other projects

## Directory Structure

```
research/
├── README.md                          ← This file
├── repositories/                      ← Per-project deep analysis
│   ├── tree-sitter-r.md
│   ├── antlr-r.md
│   ├── xmlparsedata.md
│   ├── lintr.md
│   ├── flowr.md
│   ├── languageserver.md
│   ├── CodeDepends.md
│   ├── rpy2.md
│   ├── targets.md
│   ├── rdtLite.md
│   └── additional-projects.md
├── capability-vocabulary.md           ← Normalized capability taxonomy
├── feature-matrix.md                  ← Comprehensive capability comparison
├── feature-matrix.csv                 ← Machine-readable version
├── ast-comparison.md                  ← AST representation comparison
├── grammar-comparison.md              ← Grammar coverage comparison
├── statistical-ecosystem.md           ← Statistical workflow study
├── visualization-ecosystem.md         ← Visualization understanding study
├── workflow-analysis.md               ← Workflow/data-flow study
├── runtime-correlation.md             ← Runtime correlation study
├── gap-analysis.md                    ← Novelty/gap analysis
├── candidate-features.md              ← Candidate R-PARSE differentiators
├── architecture-options.md            ← Architecture possibilities
├── opportunity-matrix.md              ← Evaluated opportunity matrix
├── opportunity-matrix.csv             ← Machine-readable opportunity matrix
├── adversarial-validation-v2.md       ← Deep adversarial technical stress test
├── adversarial-validation-v2.csv      ← Machine-readable adversarial test results
├── benchmark-plan.md                  ← Benchmark strategy
└── FINAL-RESEARCH-REPORT.md           ← Comprehensive final report
```

## Key Findings Summary

See [FINAL-RESEARCH-REPORT.md](FINAL-RESEARCH-REPORT.md) for the complete synthesis.

## Evidence Standard

Every technical claim is traced to source code, tests, grammar definitions, or official documentation. Claims that could not be verified are marked as "Unknown" or "Likely limitation."
