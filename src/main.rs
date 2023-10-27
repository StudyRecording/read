use crate::input::cli;

mod input;
mod context;

fn main() {
    // 获取命令参数
    let args = cli::read();
    println!("请求参数:{:?}", args);

    // 阅读文件
    context::detail_file(args);

}
