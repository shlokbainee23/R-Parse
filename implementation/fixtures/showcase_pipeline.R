# Showcase Pipeline: Multi-step dataframe transformation pipeline.
#
# Scenario:
# - Input dataframe `df` contains `price`, `quantity`, and `age`.
# - Rows are filtered by `price > 0`.
# - Multiple derived columns are computed:
#     - `total = price * quantity` (target lineage)
#     - `discount = age * 0.05` (unrelated column calculation)
# - Downstream consumer computes `tax = total * 0.18`.
#
# Analytical Expectations:
# - Slicing criterion `df2$total` retains:
#     - Initial input schema `price` and `quantity`
#     - Filter predicate `price > 0` (RowSelectionDependency)
#     - Derivation `total = price * quantity`
# - Strictly excludes:
#     - Unrelated column `discount` and input column `age`
#     - Downstream consumer `df3 <- mutate(df2, tax = total * 0.18)`
#
# Exact CLI Commands:
#   rparse analyze fixtures/showcase_pipeline.R --graph pdg --format dot
#   rparse provenance fixtures/showcase_pipeline.R --column 'df2$total'
#   rparse slice fixtures/showcase_pipeline.R --criterion 'df2$total'

df <- data.frame(price = 100, quantity = 2, age = 30)
filtered <- filter(df, price > 0)
df2 <- mutate(filtered, total = price * quantity, discount = age * 0.05)
df3 <- mutate(df2, tax = total * 0.18)
