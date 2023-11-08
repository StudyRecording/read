# Read
终端txt文件阅读器(学习Rust项目使用)

# 使用
- 安装rust
- clone 代码
- 执行命令
  ```shell
  cargo run -- --help
  ```
- 生成release包
  ```shell
  cargo build --release
  ```
- 找到可执行文件并执行相关命令
  ```shell
  read --help
  ```
  ![img.png](img/img.png)

# 计划（未完成功能）
- [ ] 代码重构（关于终端显示的部分可以抽出来）(好难，懒得搞)
- [ ] 终端每页显示行数自定义（好难，懒得搞）  
- [ x ]本地配置文件，可持久化保存，可重读
- [ ] 添加日志
- [ ] 清屏退出功能