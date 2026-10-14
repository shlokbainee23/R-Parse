# Adversarial Fixture 6: Nested arithmetic and function-call expressions
# Tests modeled builtins (log, sqrt, round) vs unmodeled custom function (my_custom_transform).

df <- data.frame(a = 10, b = 20, c = 30)
df2 <- mutate(df,
  modeled_calc = round(log(a) + sqrt(b), 2),
  unmodeled_calc = my_custom_transform(c) * 2
)
