library(ggplot2)
library(dplyr)
library(scales)

# Sources:
# https://www.r-bloggers.com/2021/03/making-bar-plots-using-ggplot-in-r/
# https://www.andrewheiss.com/blog/2022/06/23/long-labels-ggplot/

results = read.csv("results.csv", header=TRUE, sep=",")
results = results %>%
  mutate(problem = sub("^instances/", "", problem)) %>%
  group_by(problem, name) %>%
  group_modify(function(data, keys) { mutate(data, opt = as.integer(readLines(paste("solutions", keys$problem, sep="/"))[2])) }) %>%
  mutate(problem = sub(".txt$", "", problem)) %>%
  summarise(
    mean_weight = mean(weight / opt),
    std_weight = sd(weight / opt),
  )

ggplot(data = results, aes(x = name, y = mean_weight, fill = as.factor(problem))) +
  geom_bar(stat = "identity", color = "black", position = position_dodge()) +
  geom_errorbar(aes(ymin = mean_weight - std_weight, ymax = mean_weight + std_weight), width = 0.2,
                position = position_dodge(0.9)) +
  geom_hline(yintercept = 1, color = "firebrick1", alpha = 0.9, linewidth = 0.4, linetype = "longdash") +
  labs(x = "Algorithm", y = "Weight / Opt (Mean and Std-Dev)", fill = "Instance") +
  theme_minimal() +
  theme(text = element_text(size = 10)) +
  scale_x_discrete(labels = label_wrap(10)) +
  coord_cartesian(ylim=c(1, NA))
ggsave("out.pdf", width = 20, height = 10, units = "cm")
