# Adversarial Fixture 4: Aliases and variable reassignment
# Tests dataframe aliasing (df_alias <- df) and column renaming (rename).

df <- data.frame(raw_score = 100, noise = 50)
df_alias <- df
df_renamed <- rename(df_alias, score = raw_score)
df_final <- mutate(df_renamed, final_score = score + 10, unused = noise * 2)
