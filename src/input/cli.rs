use std::{path::PathBuf, fs};

use clap::Parser;
use serde::{Serialize, Deserialize};

/// 一个简单的clap命令行参数读取
#[derive(Parser, Debug, Clone, Serialize, Deserialize)]
#[command(name = "Read", author = "hpc", version = "0.1", about = "txt阅读器", long_about = None)]
pub struct Cli {

    /// txt文件路径
    #[arg(short, long)]
    #[serde(default)]
    pub file: Option<String>,

    /// 开始显示所在行数, 存在-f参数时，默认为1，不存在-f时，从配置文件中读取
    #[arg(short, long)]
    #[serde(default)]
    pub start: Option<u64>,

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
    let mut cli = Cli::parse();
    if cli.num <= 0 {
        panic!("每页行数不能小于1");
    }
    if cli.time <= 0 {
        panic!("自动阅读刷新时间不能小于1s");
    }
    if cli.file.is_some() {

        let file_path = cli.file.unwrap();

        let extend_name = file_path.split(".")
            .last()
            .expect("不能正常获取文件扩展名");
        if extend_name != "txt" {
            panic!("非txt文件, 不可处理");
        }

        // 转换为绝对路径
        let file_path = file_path.to_string();
        let path = PathBuf::from(file_path);
        let absolute_path = fs::canonicalize(&path).expect("txt文件转化绝对路径失败")
                                        .into_os_string().into_string().expect("txt文件转化绝对路径失败");
        cli.file = Option::Some(absolute_path);
        
    }

    cli
}
