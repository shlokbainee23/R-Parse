# Partial Retention: Single-statement partial dataframe transformation.
#
# Scenario:
# - Input dataframe `df` contains columns `price`, `quantity`, and `age`.
# - A single `mutate` statement defines two independent columns:
#     - `total = price * quantity` (contributes to slice criterion)
#     - `unused = age * 2` (independent sibling derivation)
#
# Analytical Expectations:
# - Slicing criterion `df2$total` produces a PARTIALLY_RETAINED statement:
#     - [KEEP] total = price * quantity
#     - [DROP] unused = age * 2
# - Retains input columns `price` and `quantity`.
# - Excludes input column `age` and output column `unused`.
#
# Exact CLI Commands:
#   rparse slice fixtures/partial_retention.R --criterion 'df2$total'
#   rparse slice fixtures/partial_retention.R --criterion 'df2$total' --format json

df <- data.frame(price = 25, quantity = 4, age = 40)
df2 <- mutate(df, total = price * quantity, unused = age * 2)
