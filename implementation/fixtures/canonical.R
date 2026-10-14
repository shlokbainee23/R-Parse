df <- data.frame(price = 10, quantity = 5)
df$total <- df$price * df$quantity
df2 <- mutate(df, tax = total * 0.1)
