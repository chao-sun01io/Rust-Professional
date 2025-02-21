# 第一期 Rust 入门训练营专业阶段实验

## 使用教程

* 首先 fork 本仓库至自己的 GitHub 账号下。
* 将 fork 后的仓库 clone 至本地进行实验。
* 完成实验后 git push 到 fork 后的 GitHub 仓库。
* 仓库会运行CI/CD 系统评测实验。
* 实验成绩可在 [https://opencamp.ai/Rust/camp/S01/stage/2?tab=rank](https://opencamp.ai/Rust/camp/S01/stage/2?tab=rank) 查看。

在本地查看实验结果，于仓库根目录使用。

```bash
cargo run all
```

## 题目说明

**简单题（easy）**：

- 总共 20 道题目，每道题目分值为 `1` 分，所有简单题的总分为 `20` 分。

**普通题（normal）**：

- 总共 5 道题目，每道题目分值为 `6` 分，所有普通题的总分为 `30` 分。

**困难题（hard）**：

- 总共 5 道题目，每道题目分值为 `10` 分，所有困难题的总分为 `50` 分。

**如有不明之处，或在实验过程中遇到问题，可随时联系助教解决。**

**完成实验后请及时加入项目阶段群，群聊二维码在完成试验后会显示在排行榜页面。**


## Tips 

How to let cargo find all tests in exercise/easy?

创建 tests 目录，符号链接所有文件到 tests目录下， cargo 自动就会发现。 注意cargo test 不会自动找子目录下的测试，所以 tests/easy 没用

```
# at Rust-Professional root dir
mkdir tests
ln -s exercises/easy/* tests
```

之后只需要跑对应的测试即可

```
cargo test --test algorithm5
```