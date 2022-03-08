# rust-samples

Rust 实战小项目

# 小游戏

游戏四大金刚: 贪吃蛇、五子棋、俄罗斯方块、扫雷

## 工具集

- 统计代码数据（行数，个数）counter-code
- 写一个chrome的书签打开器，思路是从配置文件夹里读bookmark（serde_json递归读json），然后通过输入框（tui比较简单）fuzzy查找（考虑使用simsearch）书签