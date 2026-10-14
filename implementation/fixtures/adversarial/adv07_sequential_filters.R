# Adversarial Fixture 7: Sequential filtering and row-selection dependencies
# Tests multi-step filtering predicates propagating to downstream columns.

df <- data.frame(price = 50, status = 1, region = "US", noise = 99)
df1 <- filter(df, price > 10)
df2 <- filter(df1, status == 1)
df3 <- filter(df2, region == "US")
df4 <- mutate(df3, revenue = price * 1.2)
