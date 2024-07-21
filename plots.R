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
  group_modify(function(data, keys) {
    mutate(data, opt = as.integer(readLines(paste("solutions", sub(" \\(asymm\\.\\)$", "", keys$problem), sep="/"))[2]))
  }) %>%
  mutate(problem = sub(".txt", "", problem)) %>%
  summarise(
    mean_weight_err = mean((weight - opt)/ opt),
    std_weight_err = sd((weight - opt) / opt),
  )

ggplot(data = results, aes(x = name, y = mean_weight_err, fill = as.factor(problem))) +
  geom_bar(stat = "identity", color = "black", position = position_dodge()) +
  geom_errorbar(aes(ymin = mean_weight_err - std_weight_err, ymax = mean_weight_err + std_weight_err), width = 0.2,
                position = position_dodge(0.9)) +
  labs(x = "Algorithm", y = "(Weight - Opt) / Opt (Mean and Std-Dev)", fill = "Instance") +
  theme_minimal() +
  theme(text = element_text(size = 10)) +
  scale_x_discrete(labels = label_wrap(10))
ggsave("out.pdf", width = 30, height = 10, units = "cm")
