use clap::Parser;

/// 一个简单的clap命令行参数读取
#[derive(Parser, Debug)]
#[command(name = "Read", author = "hpc", version = "0.1", about = "txt阅读器", long_about = None)]
pub struct Cli {

    /// txt文件路径
    #[arg(short, long)]
    pub file: String,

    /// 开始显示所在行数
    #[arg(short, long, default_value = "1")]
    pub start: usize,

    /// 每页显示行数
    #[arg(short, long, default_value = "1")]
    pub num: u16,

    /// 是否自动阅读
    #[arg(short, long)]
    pub auto: bool,

    /// 开启自动阅读时, 每页刷新间隔时间(秒)
    #[arg(short, long, default_value = "2")]
    pub time: u64,


}

/// 读取命令参数
pub fn read() -> Cli {
    Cli::parse()
}
