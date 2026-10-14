# Adversarial Fixture 8: Dynamic column access and unsupported operations
# Tests .data[[col_name]] dynamic indexing and unmodeled across() helper.

df <- data.frame(price = 100, volume = 5, noise = 42)
col_name <- "price"

df2 <- mutate(df,
  dynamic_val = .data[[col_name]] * 2,
  static_val = volume + 10
)
