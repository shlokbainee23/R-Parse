# Adversarial Fixture 10: Unmodeled relational join operation
# Tests behavior when encountering unmodeled tidyverse verb `left_join`.

users <- data.frame(user_id = 1, name = "Alice")
purchases <- data.frame(user_id = 1, spent = 100)

joined <- left_join(users, purchases, by = "user_id")
result <- mutate(joined, total_spent = spent * 1.1)
