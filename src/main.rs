
mod cli;
fn main() {
    // 获取命令参数
    let args = cli::read();
    println!("请求参数:{:?}", args);

}
