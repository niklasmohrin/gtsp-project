library(ggplot2)
library(dplyr)
library(scales)

# Sources:
# https://www.r-bloggers.com/2021/03/making-bar-plots-using-ggplot-in-r/
# https://www.andrewheiss.com/blog/2022/06/23/long-labels-ggplot/

results = read.csv("results.csv", header=TRUE, sep=",")
results$problem = sub("^instances/", "", results$problem)
results = results %>%
  group_by(problem, name) %>%
  summarise(
    mean_weight = mean(weight),
    std_weight = sd(weight),
    n_samples = n(),
  )

ggplot(data = results, aes(x = name, y = mean_weight, fill = as.factor(problem))) +
  geom_bar(stat = "identity", color = "black", position = position_dodge()) +
  geom_errorbar(aes(ymin = mean_weight - std_weight, ymax = mean_weight + std_weight), width = 0.2,
                position = position_dodge(0.9)) +
  labs(x = "Algorithm", y = "Weight", fill = "Instance") +
  theme_minimal() +
  theme(text = element_text(size = 10)) +
  scale_x_discrete(labels = label_wrap(10))
ggsave("out.pdf", width = 20, height = 10, units = "cm")
