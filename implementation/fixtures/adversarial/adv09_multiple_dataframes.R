# Adversarial Fixture 9: Multiple dataframe objects and column-name reuse
# Tests that operations on two dataframes sharing column names remain isolated.

orders <- data.frame(id = 101, amount = 250, status = 1)
customers <- data.frame(id = 101, score = 90, status = 1)

orders_processed <- mutate(orders, final_amount = amount * 1.05)
cust_processed <- mutate(customers, final_score = score + 5)
