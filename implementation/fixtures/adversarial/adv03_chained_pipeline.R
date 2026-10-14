# Adversarial Fixture 3: Chained dataframe transformations
# Tests pipe evaluation across select, filter, mutate in a single pipeline.

df <- data.frame(id = 1, x = 10, y = 20, z = 30)
df_out <- df %>%
  filter(x > 0) %>%
  mutate(temp = y * 2, dropped_calc = z + 1) %>%
  select(id, temp)
