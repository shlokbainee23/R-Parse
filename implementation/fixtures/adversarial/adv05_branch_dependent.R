# Adversarial Fixture 5: Branch-dependent assignments
# Tests if-else branching with different column definitions.

df <- data.frame(base = 10, alt1 = 20, alt2 = 30)
flag <- TRUE

if (flag) {
  df2 <- mutate(df, score = base + alt1)
} else {
  df2 <- mutate(df, score = base + alt2)
}

df3 <- mutate(df2, total = score * 2)
