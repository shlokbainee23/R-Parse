# Adversarial Fixture 2: Reassignment of an existing column
# Tests column versioning when an existing column is overwritten in mutate.

df <- data.frame(val = 10, bonus = 5, noise = 99)
df2 <- mutate(df,
  val = val * 2,
  other = bonus + 1
)
df3 <- mutate(df2,
  result = val + 10,
  noise = noise * 3
)
