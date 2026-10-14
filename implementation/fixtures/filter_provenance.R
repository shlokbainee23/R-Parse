# Filter Provenance: Sequential row filtering with RowSelectionDependency.
#
# Scenario:
# - Input dataframe `df` contains `price`, `quantity`, and `status`.
# - Two sequential filtering steps subset rows:
#     1. `price > 0`
#     2. `status == 1`
# - A final calculation computes `total = price * quantity`.
#
# Analytical Expectations:
# - In the PDG, filter predicates create `RowSelectionDependency` edges,
#   strictly distinct from CFG `ControlDependency` edges (e.g. if/else branching).
# - Surviving columns in the slice inherit row-selection dependencies on `price` and `status`.
# - Slicing on `df3$total` retains both filter predicates without introducing
#   false control flow blocks.
#
# Exact CLI Commands:
#   rparse analyze fixtures/filter_provenance.R --graph pdg --format dot
#   rparse slice fixtures/filter_provenance.R --criterion 'df3$total'

df <- data.frame(price = 10, quantity = 5, status = 1)
df1 <- filter(df, price > 0)
df2 <- filter(df1, status == 1)
df3 <- mutate(df2, total = price * quantity)
