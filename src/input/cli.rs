use clap::Parser;

/// 一个简单的clap命令行参数读取测试
#[derive(Parser, Debug)]
#[command(name = "Read", author = "hpc", version = "0.1", about = "txt阅读器", long_about = None)]
pub struct Cli {

    /// txt文件路径
    #[arg(short, long)]
    file: String,

    /// 开始显示所在行数
    #[arg(short, long, default_value = "1")]
    start: usize,

    /// 每页显示行数
    #[arg(short, long, default_value = "1")]
    num: usize,

    /// 是否自动阅读
    #[arg(short, long)]
    auto: bool,

    /// 开启自动阅读时, 每页刷新间隔时间(秒)
    #[arg(short, long, default_value = "2")]
    time: usize,


}

impl Cli {

    /// 获取文件路径
    pub fn get_file(&self) -> &str {
        &self.file
    }

    /// 获取每页显示行数
    pub fn get_num(&self) -> &usize {
        &self.num
    }

    /// 获取开始显示时所在行数
    pub fn get_start(&self) -> &usize {
        &self.start
    }
}

/// 读取命令参数
pub fn read() -> Cli {
    Cli::parse()
}
