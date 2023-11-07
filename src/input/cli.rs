use clap::Parser;
use serde::{Serialize, Deserialize};

/// 一个简单的clap命令行参数读取
#[derive(Parser, Debug, Clone, Serialize, Deserialize)]
#[command(name = "Read", author = "hpc", version = "0.1", about = "txt阅读器", long_about = None)]
pub struct Cli {

    /// txt文件路径
    #[arg(short, long)]
    pub file: String,

    /// 开始显示所在行数
    #[arg(short, long, default_value = "1")]
    pub start: u64,

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

/// 读取并验证命令参数
pub fn read() -> Cli {
    let cli = Cli::parse();
    if cli.num <= 0 {
        panic!("每页行数不能小于1");
    }
    if cli.start <= 0 {
        panic!("开始行数不能小于1");
    }
    if cli.time <= 0 {
        panic!("自动阅读刷新时间不能小于1s");
    }
    if String::is_empty(&cli.file) {
        panic!("文件路径错误");
    }
    // 验证文件格式
    let extend_name = cli.file.split(".")
        .last()
        .expect("不能正常获取文件扩展名");
    if extend_name != "txt" {
        panic!("非txt文件, 不可处理");
    }

    cli
}
