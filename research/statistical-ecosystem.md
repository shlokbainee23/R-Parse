# Statistical Ecosystem & Modeling Semantics Study

This document analyzes how existing R static analysis tools interact with statistical modeling workflows, statistical formulas, and model extraction patterns.

---

## 1. The Structure of R Statistical Workflows

Unlike general-purpose languages, R's core identity centers on statistical computing. A standard R statistical pipeline consists of four distinct phases:
1. **Model Specification:** Wilkinson-Rogers formula notation (`response ~ predictor1 + predictor2 + (1|group)`).
2. **Model Estimation:** Calling estimation routines (`stats::lm`, `stats::glm`, `survival::coxph`, `lme4::lmer`, `mgcv::gam`, `brms::brm`).
3. **Inference & Diagnostics:** Inspecting estimates, p-values, residuals, and variance components (`summary()`, `anova()`, `confint()`, `car::vif()`).
4. **Prediction & Tidying:** Generating predictions (`predict()`, `fitted()`) or converting models to tidy data frames (`broom::tidy()`, `broom::glance()`, `parameters::model_parameters()`).

---

## 2. Evidence of Existing Support in Surveyed Projects

### 2.1 CodeDepends: Handlers for Modeling & Formulas
- **Implementation:** `R/functionHandlers.R` lines 150-184 (`formulahandler`).
- **Mechanism:** Inspects formula calls (`~`). Allows the user to toggle `formulaInputs = TRUE/FALSE`.
- **Author's Code Comments (`functionHandlers.R` lines 158-164):**
  > *"eventually we want to be able to handle the situation where we are calling lm(y~x + z, data=dat) where y and x are in dat but z is not, but that is HARD to detect so for now we allow users to specify whether CodeDepends counts formula variables as inputs or not... I think the second one is the most common use-case in practice..."*
- **Verdict:** CodeDepends is the **only surveyed tool that attempted to model formula semantics**, but its authors explicitly conceded that statically distinguishing dataframe columns from global environment variables in formulas was an unsolved, hard problem.

### 2.2 targets: The Formula Substitution Workaround
- **Implementation:** `R/utils_language.R` lines 37-41 (`deps_function`).
- **Mechanism:**
  ```r
  deps_function <- function(fun) {
    env <- list(`~` = `identity`)
    body(fun) <- eval(call("substitute", body(fun), env), envir = baseenv())
    sort_chr(codetools::findGlobals(fun))
  }
  ```
- **Consequence:** Because `~` prevents normal evaluation of its operands, `codetools::findGlobals()` would otherwise ignore or mischaracterize symbols inside formulas. Targets works around this by syntactically substituting `~` with `identity` in the AST before passing it to `findGlobals()`.
- **Limitation:** This causes **false dependency edges**: if `y` and `x` are columns in `df`, running `tar_target(fit, lm(y ~ x, data = df))` causes `targets` to search for nonexistent global objects `y` and `x`, unless wrapped in custom helper functions.

### 2.3 flowR: Generic Operator Treatment
- **Implementation:** `src/r-bridge/lang-4.x/ast/model/nodes/` and `src/dataflow/`.
- **Mechanism:** Treats `~` strictly as a binary operator node (`BinaryOperator.Tilde`).
- **Consequence:** flowR constructs standard dataflow edges across `~`. Variables inside formulas are treated as standard identifier references, creating spurious dataflow edges if those symbols are dataframe columns.

### 2.4 lintr, languageserver, tree-sitter-r
- **Mechanism:** Purely syntactic. `tree-sitter-r` assigns `~` precedence level `PREC.TILDE`. `lintr` has zero rules validating formula syntax, model degrees of freedom, or collinearity. `languageserver` cannot provide autocomplete for formula variables based on the `data` parameter.

---

## 3. Evidence-Backed Gaps in Statistical Understanding

| Statistical Capability | Current Status Across Ecosystem | Technical Barrier |
| :--- | :--- | :--- |
| **Formula Variable vs. Environment Disambiguation** | **Unsolved / Gapped.** Tools either guess all are globals (targets) or ignore them (flowR). | Requires static tracking of data frame schema bound to the `data=` argument. |
| **Model Specification Validation** | **Completely Missing.** No static tool validates that variables in `y ~ x1 + x2` actually exist in the supplied dataset. | Parsers have no connection to data schemas or CSV/arrow headers. |
| **Formula Operator Semantics (`*`, `:`, `^`, `+`, `-`, `|`)** | **Completely Missing.** In formulas, `*` means interaction + main effects (`x1 + x2 + x1:x2`), not multiplication. Tools treat it as arithmetic. | Requires domain-specific formula AST lowering distinct from arithmetic lowering. |
| **Random Effects Syntax Parsing** | **Completely Missing.** `(1 | group)` or `(slope | group)` is parsed as bitwise OR or grouped expressions. | No grammar recognizes mixed-model groupings natively. |
| **Downstream Inference Flow (`broom` pipeline)** | **Completely Missing.** No tool tracks columns emitted by `broom::tidy(fit)` (`term`, `estimate`, `std.error`, `p.value`). | Requires inter-procedural return-shape knowledge for model classes. |

---

## 4. Architectural Opportunity for R-PARSE

1. **Formula-Specific AST Lowering:** R-PARSE should introduce a dedicated `FormulaNode` IR distinct from standard arithmetic `BinaryOp`, correctly distinguishing fixed effects, interactions (`:`), crossings (`*`), and group intercepts (`|`).
2. **Schema-Aware Data Parameter Binding:** Statically linking the `data = ...` argument of modeling functions (`lm`, `glm`, `lmer`) to known dataframe schemas, eliminating false global dependencies in pipeline DAGs and linters.
