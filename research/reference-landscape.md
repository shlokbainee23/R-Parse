# R-PARSE — External Reference Landscape

**Status:** Canonical description of the external projects inspected for R-PARSE research.\
**This document is not** the scope freeze, the technical specification, an architecture decision, the gap analysis, or the adversarial validation. Those remain separate files in `research/`.

The per-project notes that used to live in `research/repositories/` were consolidated here. Raw third-party checkouts are not part of the public tree. The commits below are the local pins recorded in this working copy before those checkouts were moved out of the repository. They are observation pins, not a decision to vendor the projects.

Claims below are labeled:

- **Verified** — read from the named project file in the pinned checkout, or carried forward from a repository note that cites a file and line.
- **Recorded note** — stated in an earlier R-PARSE research note. Not re-audited line by line during this consolidation.
- **Hypothesis** — an inference. Not a measured absence.
- **Unresolved** — sources disagree, or a license/attribution fact was observed and deliberately not decided.

A capability is not treated as missing only because a README is silent.

## Inspection pins

| Project | Official source | Local commit | License observation (not a decision) |
| :--- | :--- | :--- | :--- |
| tree-sitter-r | https://github.com/r-lib/tree-sitter-r | `58a22794466c0fc15b0d3b40531db751593721e8` | Root `LICENSE` text is MIT. Copyright line observed: tree-sitter-r authors, 2025. |
| grammars-v4 (R grammar under `r/`) | https://github.com/antlr/grammars-v4 | `7df52be94698550d219d299d04105c6bafadd9c3` | Repository note calls the R grammar BSD. This checkout had no root `LICENSE` file. The grant was not re-verified. |
| xmlparsedata | https://github.com/r-lib/xmlparsedata | `216b40bbf74ba29299750e19aba205f037ef064d` | Repository note says MIT. Checkout `LICENSE` is an R-package stub (`YEAR` / `COPYRIGHT HOLDER`), not a full license text. |
| lintr | https://github.com/r-lib/lintr | `42f73ec680503b6aaa3ba8e6d5b4aa0e2f90973d` | `COPYING` begins with a MIT-style permission grant. Copyright line observed: James Hester, 2014–2024. |
| flowR | https://github.com/flowr-analysis/flowr | `3ea266c712b34a585402f1a3c66811850340313b` | Root `LICENSE` header is GNU GPL version 3. The repository note also says LGPL and other components are present. That mix was not re-audited. |
| languageserver | https://github.com/REditorSupport/languageserver | `0b195a3c6facf7c58f9735751b82aee539ea5090` | Repository note says MIT. Checkout `LICENSE` is an R-package stub. Copyright holder line observed: Randy Lai, year 2018. |
| CodeDepends | https://github.com/duncantl/CodeDepends | `5bf358ae2be0426852dfe12ec42a5f7896a49136` | `DESCRIPTION` says `License: GPL`. The repository note narrows that to GPL (>= 2). The `DESCRIPTION` line itself does not state the version. |
| rpy2 | https://github.com/rpy2/rpy2 | `a7a0ff1c612922d87ed1fb17203efa0576e45589` | Root `LICENSE` header is GNU GPL version 2, June 1991. The repository note says GPL-2.0-or-later. The “or later” clause was not re-read beyond that header. |
| targets | https://github.com/ropensci/targets | `06cae18bf0306ee98ce2f6f0a647e26410ddca22` | Repository note says MIT. Checkout `LICENSE` is an R-package stub. Copyright holder line observed: Eli Lilly and Company, year 2026. |
| rdtLite | https://github.com/End-to-end-provenance/rdtLite | `7366631621c824b6040ad144e8d8bac4f6e3804b` | Root `LICENSE` header is GNU GPL version 3. |

Local modifications at the time of the move: nine checkouts were clean relative to those commits. `grammars-v4` had one untracked directory, `less/build/`. That directory moved with the checkout. It is local build output, not an upstream commit.

## What stays outside this document

These R-PARSE documents were compared and left in place. They are not replaced by this landscape:

- `research/scope-freeze.md`
- `research/technical-specification.md`
- `research/ADR-001-architecture.md`
- `research/gap-analysis.md`
- `research/candidate-features.md`
- `research/opportunity-matrix.md` and `opportunity-matrix.csv`
- `research/adversarial-validation.md`
- `research/adversarial-validation-v2.md` and `adversarial-validation-v2.csv`
- `research/flowr-deep-gap-audit.md` and `flowr-deep-gap-audit.csv`
- `research/grammar-comparison.md`, `research/ast-comparison.md`, `research/feature-matrix.md`

`flowr-deep-gap-audit.md` is the later source-level account of flowR dataframe shape analysis. Where it is more specific than the earlier flowR repository note, this landscape follows the audit and points at it instead of copying it.

## tree-sitter-r

**Role.** Incremental parser and concrete syntax tree for R. Repository note: used by Positron and by flowR’s tree-sitter backend. The parser is not an analyzer.

**Verified structure.** `grammar.js` is the grammar. An external scanner handles context-sensitive tokens. The repository note cites `scanner.c` token kinds including newline, semicolon, raw-string pieces, `else`, and bracket pairs including `[[` / `]]`. Named CST nodes include `program`, `function_definition`, `call`, `subset`, `subset2`, `binary_operator`, and `identifier`. Comments are grammar extras. The note cites a known deviation: `]]` is one token, while R allows that sequence to be split across lines (project README in that checkout).

**Pipe placeholder, reconciled at this commit.** Two earlier notes disagreed.

- `research/repositories/tree-sitter-r.md` listed “pipe placeholder `_`” as supported modern syntax.
- `research/grammar-comparison.md` section 3.2 says the scanner and `grammar.js` do not treat `_` as a placeholder and parse it as an unexpected token or an error.
- **Verified** in `grammar.js` at `58a2279`, in the identifier-rule comment: `_` is not a valid start of an R identifier, and the grammar is intentionally lax. A lone `_` is accepted as an identifier so it can serve as the pipe placeholder. A separate `"_"` token was considered and not used, because `_foo` would then lex as two identifiers. Downstream consumers are expected to check identifier validity. There is no dedicated placeholder node in that comment.

So, at this pin, lone `_` is an identifier by design. It is not a separate placeholder token, and the grammar comment does not describe it as an error. `grammar-comparison.md` was not rewritten; its section 3.2 is the note that does not match this comment. See Unreconciled sources.

**Recorded note, not re-counted here.** Operator precedence comments in `grammar.js` mirror R’s `gram.y` table, including native pipe, lambda `\(`, raw strings, and `:=`. Test corpus files named in the repository note: `test/corpus/expressions.txt`, `literals.txt`, and the two error corpora. No static analysis, formula node, or ggplot model lives in this repository. Formulas are a `binary_operator` whose operator is `~`. S3/S4/R6/R5 forms are ordinary calls to the parser. **Hypothesis** from the repository note: the grammar cannot by itself distinguish non-standard evaluation from ordinary calls.

## ANTLR R grammar (`grammars-v4/r`)

**Role.** Community ANTLR 4 grammar, not an analyzer.

**Verified from the repository note, which cites `r/R.g4`.** Combined lexer and parser. Header says a companion `RFilter.g4` is required to drop newlines that are not statement ends, and that filter is not in the repository. The note cites a 2013 copyright and derivation from an old `gram.y`. Comments are mapped onto the newline token, so they are not kept as comment nodes. The note records these absences inside `R.g4`: raw strings, native pipe `|>`, and lambda `\()`. It also records that `:=` is present in the assignment alternatives, and that a lowercase `inf` literal does not match R’s `Inf`. Precedence is ANTLR rule order. **Hypothesis** in the note: that order may not match R on every edge.

The note’s line count for `R.g4` is 268. That count was not re-measured during consolidation.

## xmlparsedata

**Role.** Turns `utils::getParseData()` into an XML document so other packages can run XPath 1.0. It does not parse R itself.

**Verified from the repository note, citing `R/package.R`.** `xml_parse_data()` calls `getParseData()`. Comments with `parent = -1` are rewritten. An octal-escape workaround is cited against R bug 18323. Operator spellings that are illegal XML names are renamed (`~` to `OP-TILDE`, `\` to `OP-LAMBDA`, and the other punctuation mappings listed in that note). Root element is `exprlist`. Attributes kept: `line1`, `col1`, `line2`, `col2`, `start`, `end`. If the input was parsed without source references, the note says the package `dput`s it to a temp file and parses again.

**Recorded limitations.** Requires the GNU R that produced the parse table, so coverage matches that R, not a vendored grammar. No incremental parse. The XML round trip is described as the cost center for `lintr` and `languageserver`. No symbol resolution.

## lintr

**Role.** Style and bug-pattern linting for R, R Markdown, Quarto, and Sweave-style documents.

**Verified from the repository note.** Analysis is XPath over the `xmlparsedata` tree, after base R `parse()`. The note counts 118 `*_linter.R` files and 152 R sources, and names dependencies `xmlparsedata`, `xml2`, `rex`, `knitr`, `digest`, `cli`. Examples it ties to behavior: `assignment_linter` (`=` versus `<-`), `equals_na_linter`, `class_equals_linter`, `seq_linter` (`1:length(x)`), `vector_logic_linter` (`&` / `|` inside `if`), `cyclocomp_linter` counting `IF`, `FOR`, `WHILE`, `AND`, `OR`. Literate documents are in scope.

**Recorded gaps in that note.** No reaching-definition dataflow, no cross-file call graph, no type inference, no schema tracking through dplyr verbs. Tidyverse checks are described as XPath shape matches. **These are claims about what the inspected linter architecture does, not a proof that no individual rule mentions a dataframe.**

## languageserver

**Role.** Language Server Protocol implementation for R editors. The repository note names VS Code, Sublime, Vim/Neovim, and Emacs as clients.

**Verified from the repository note, citing files and sizes.** Parse path is base R plus `xmlparsedata`, with a per-document XML cache. Definition lookup matches `LEFT_ASSIGN`, `RIGHT_ASSIGN`, `EQ_ASSIGN`, `SYMBOL_FORMALS`, and `for` variables. `type_hierarchy.R` (1281 lines in the note) recognizes `R6Class`, `setClass`, `setRefClass`, and S3 constructor / `class(x) <-` patterns, including cross-file supertype links. `refactor.R` (897 lines) extracts variables and functions. `completion.R` (1081 lines) mixes token matches with runtime inspection of packages, formals, and `df$` columns. Diagnostics are delegated to `lintr`. A syntax error that makes `parse()` fail drops the tree until the file parses again.

**Recorded gaps.** Definition lookup is lexical, not SSA. No program slicer. Column completion does not statically follow a chain of dplyr verbs. The note calls the server single-threaded in R and describes XML rebuild cost on large files. Those performance sentences are recorded notes, not new measurements.

## flowR

**Role.** Static dataflow analysis, control-flow graphs, queries, lint rules, and program slicing for R. TypeScript. Academic project (the repository note names the University of Ulm). Related projects named in the note: `vscode-flowr`, `rstudio-addin-flowr`, `flowr-r-adapter`.

**Verified from the repository note.** Two parser backends: an R subprocess using `utils::getParseData()`, and tree-sitter via `web-tree-sitter`. The note cites `src/r-bridge/parser.ts` and the names `r-shell` and `tree-sitter`. Normalization produces flowR `RType` nodes (`RPipe`, `RFunctionCall`, `RSymbol`, `RComment`, and the rest of that enum). Source metadata includes range, lexeme, and extra tokens for inline comments. The note lists 31 query directories under `src/queries/catalog/`, including `static-slice-query`, `dependencies-query`, and `provenance-query`. It also lists 18 linter rule names from the README, including `dataframe-access-validation`. Tests use Vitest. The note’s “~200 stars” and “v2.15.9+” figures were not re-checked.

**Dataframes, later audit.** The early repository note said flowR has no statistical domain model and treats formulas as binary operators, and it did not describe column provenance. `research/flowr-deep-gap-audit.md` is the later reading of the same kind of tree. It reports a dataframe abstract domain that tracks column-name sets and row/column intervals, a linter and a query that consult that domain, and a static slicer that still walks the dataflow graph with dataframes as whole variables. That audit is the source for those distinctions. This landscape does not repeat its line citations. The early sentence “flowR models dataframes only as monolithic scalars” is too coarse once that audit is included: shape information exists, and the slicer does not consume it.

**Recorded limitations that the audit does not withdraw.** No ggplot layer model. The note says `+` and `aes()` are not a visualization grammar. No formula-structure node. GPLv3 is the observed root license. **Hypothesis** in the note: TypeScript plus an R subprocess may be slow on very large projects, and abstract interpretation is partial because R is dynamic. Those are not benchmarks.

## CodeDepends

**Role.** Expression-level input/output and script dependency graphs. The repository note names Duncan Temple Lang and Gabriel Becker.

**Verified from the repository note, citing `R/functionHandlers.R`.** Handlers for magrittr `%>%`, and NSE-aware handlers for `filter`, `mutate`, `transmute`, `rename`, `slice`, `summarise`, `arrange`, `select`, `group_by`, and `distinct`. In `filter(df, a > 5)`, `df` is an ordinary input and `a` is recorded as an NSE variable. Separate handlers for `gather`, `spread`, `separate`, `unnest`, purrr `map` family, `apply` family, and `~` formulas (`formulaInputs` chooses global versus data-frame lookup). `isPlot.R` detects plotting calls. `inputCollector` tracks libraries, files, variables read, variables set, removals, updates, side effects, and NSE variables. Some handlers call `eval()` or `codetools::findGlobals()` during analysis (note cites `funchandler`).

**Recorded gaps.** Block-level dependencies, not SSA. No slicer. No column types. The note says modern `across()`, `pick()`, and tidyselect helpers such as `where()` are outside the handler set. That is a statement about the handlers the note inspected, not a claim that no later commit added them. This pin is `5bf358a`.

## targets

**Role.** Pipeline runner. Static analysis is only there to decide which targets are stale.

**Verified from the repository note, citing `R/utils_language.R` and `R/tar_deps.R`.** A command that is not already a function is wrapped in an empty function body (`embody_expr`). `~` is substituted to `identity` so both sides of a formula become visible to `codetools::findGlobals()`. `tar_tidy_eval()` uses `rlang::quo_squash()`. The graph is an igraph of target names. Code changes are detected by hashing deparsed function text. Dependencies named in the note: `codetools`, `callr`, `cli`, `digest`, `igraph`, `rlang`, `tidyselect`, `vctrs`.

**Recorded gaps.** A target command is one node. The note says `findGlobals` reports a dplyr column such as `y` in `mutate(df, x = y + 1)` as a global when `y` is not a local binding. Dynamic `pattern = map(...)` branches are not known until the upstream target runs. Comment-only edits can change the deparsed hash. **Hypothesis:** those false globals are why a column-aware analyzer would change target invalidation. That is a product inference, not a `targets` test result.

## rdtLite

**Role.** Dynamic provenance recorder. Output is W3C PROV JSON (`prov.json`). It is a runtime oracle, not a static slicer.

**Verified from the repository note.** `prov.run()` parses with `parse(file, keep.source = TRUE)` and walks statements. Variable uses and assignments are read from R language objects (`<-`, `=`, `<<-`, `$`, and indexed assignment). Executed steps become procedure nodes. Values, files, and URLs become data nodes. Edges are data-in, data-out, and control order. `IOTrace.R` hooks graphics devices (`pdf`, `png`, `dev.off`). Unexecuted branches are absent. The note describes NSE false edges for a name such as `age` inside `filter()`. Snapshots are bounded by `max.snapshot.size`. The “2x–10x slowdown” sentence is a recorded note, not a measurement from this consolidation.

**Recorded non-goal.** No formula or model semantics. Statistical calls are ordinary expressions.

## rpy2

**Role.** In-process Python binding to `libR`. Not a parser and not an analyzer.

**Verified from the repository note.** Two layers: `rpy2.rinterface` (CFFI against R headers, including `Rinternals.h`) and `rpy2.robjects` (Python API). Unevaluated calls are wrapped as language objects. Parsing, when used, is R’s parser through the embedded runtime. Converters named in the note: NumPy and pandas. A fault in R C code takes down the host Python process. R’s single-threaded runtime is called out as a constraint on Python threads. Installation needs matching R headers.

**Recorded contrast.** Embedding has full R semantics and a shared-library dependency. A subprocess bridge has a process boundary. Neither fact chooses an R-PARSE architecture. That choice is `ADR-001-architecture.md`.

## Cross-checks against the other research files

- **Gap analysis, candidate features, opportunity matrix.** They rank what R-PARSE might do. They stay separate so a public reader can see the decision record without treating every opportunity as an implemented feature. Nothing in those files was copied in as a new verified absence.
- **Adversarial validation.** It scores R-PARSE behavior on fixtures. It is not a description of the external tools, so it was not merged here.
- **Feature matrix and AST comparison.** They compare representations across tools. They remain the matrix. This file is the per-project evidence those matrices were drawn from.
- **Grammar comparison.** Kept. Only the `_` placeholder row conflicts with the pinned `grammar.js` comment, as recorded above.

## Unreconciled sources

1. `research/grammar-comparison.md` section 3.2 says tree-sitter-r rejects `_`. The identifier comment at commit `58a2279` says lone `_` is accepted as an identifier and is not its own token. Both texts are still in the repository. The commit comment is the more specific observation. The comparison section was not edited, because this consolidation does not rewrite the comparison documents.
2. `research/FINAL-RESEARCH-REPORT.md` still says, in its inventory table, that tree-sitter-r is missing the pipe placeholder and that flowR models dataframes only as monolithic scalars. That report is a pre-scope-freeze snapshot. The placeholder row is superseded by the grammar comment above. The dataframe row is superseded in specificity by `flowr-deep-gap-audit.md`. The historical report was not rewritten line by line.
3. Several license labels in the old repository notes (MIT for `languageserver`, `targets`, and `xmlparsedata`; BSD for the ANTLR R grammar; GPL-2.0-or-later for rpy2; GPL (>= 2) for CodeDepends) do not match a full license text that this consolidation opened. The observations are in the table above. No license was selected for R-PARSE, and no third-party notice was copied into a new source file.

## Unresolved attribution fact for later discussion

R-PARSE’s Rust parser crate vendors generated Tree-sitter C sources derived from tree-sitter-r. tree-sitter-r’s root license, at the pin above, is MIT and requires the copyright and permission notice to travel with substantial portions of the software. This consolidation did not add, move, or choose a project license, and it did not copy grammar sources into a new file. Whether the vendored `parser.c` carries a sufficient notice is a separate decision.
