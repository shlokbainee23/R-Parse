# Uncertainty: Explicit UNKNOWN handling for dynamic column resolution.
#
# Scenario:
# - Input dataframe `df` contains known columns `price` and `quantity`.
# - Transformation creates `dynamic_col` via dynamic indexing `.data[[get_col_name()]]`.
# - Column name cannot be determined statically.
#
# Analytical Expectations:
# - The analyzer does NOT guess an arbitrary input column.
# - Emits explicit `UNKNOWN_STATIC` provenance edge and PDG `Unknown` node.
# - Slicing on `df2$dynamic_col`:
#     - Retains `dynamic_col` and the `Unknown` dependency node.
#     - Flags `Contains Unknowns: YES`.
#     - Sets `Confidence: PARTIAL_CONTAINS_UNKNOWNS`.
#     - Retains 0 false columns from `df`.
#
# Exact CLI Commands:
#   rparse analyze fixtures/uncertainty.R --graph pdg --format dot
#   rparse slice fixtures/uncertainty.R --criterion 'df2$dynamic_col'
#   rparse slice fixtures/uncertainty.R --criterion 'df2$dynamic_col' --format json

df <- data.frame(price = 10, quantity = 5)
df2 <- mutate(df, dynamic_col = .data[[get_col_name()]])
