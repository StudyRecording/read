use clap::Parser;

/// 一个简单的clap命令行参数读取测试
#[derive(Parser, Debug)]
#[command(name = "Read", author = "hpc", version = "0.1", about = "txt阅读器", long_about = None)]
pub struct Cli {

    /// txt文件路径
    #[arg(short, long)]
    file: String,

    /// 是否自动阅读，default = false
    #[arg(short, long)]
    auto: Option<bool>,
}

/// 读取命令参数
pub fn read() -> Cli {
     Cli::parse()
}