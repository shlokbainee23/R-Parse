# Adversarial Fixture 1: Basic column derivation and multiple inputs
# Validates grouped multi-input derivation (total <- price * quantity)
# and sibling exclusion (drop unrelated discount and tax).

df <- data.frame(price = 100, quantity = 5, tax_rate = 0.08, discount = 10)
df2 <- mutate(df,
  subtotal = price * quantity,
  total = subtotal * (1 + tax_rate),
  savings = discount * 2
)
