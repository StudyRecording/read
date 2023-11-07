use std::{fs::{File, self}, io::{BufReader, BufWriter}};

use serde::{Serialize, Deserialize};

use crate::input::cli::Cli;


/// 配置
#[derive(Serialize, Deserialize, Clone)]
pub struct Config {

    /// 命令参数
    #[serde(flatten)]
    cli: Cli,

    /// txt文件路径
    file_path: String,

    /// 当前在读行数
    current_line_no: u64,
}

impl Config {

    /// 获取新Config对象
    pub fn new(cli: &Cli, current_line_no: u64) -> Config {
        let cli = cli.clone();
        let file_path = cli.file.to_string();
        Config { cli, file_path: file_path, current_line_no, }
    }

    /// 更新配置文件, 如果存在配置信息则更新，否则则添加配置信息
    pub fn update_config(&self, current_line_no: u64) {
        let file = get_or_create_config_dir();
        let reader = BufReader::new(file);
        let mut content: Vec<Config> = serde_json::from_reader(reader).expect("读取文件内容，反序列化失败");
    
        // 默认不存在配置
        let mut exist_config = false;

        // 查找已存在的配置并更新
        let mut index = 0;
        while index < content.len() {
            let config = content.get(index).expect("获取配置文件内容失败");
            if self.file_path == config.file_path {
                let mut update_config = content.remove(index);
                update_config.current_line_no = current_line_no;
                content.push(update_config);
                exist_config = true;
                break;
            }
            index += 1;
        }
        
        // 如果不存在，则新增
        if !exist_config {
            content.push(self.clone());
        }        

        // 序列化到文件中
        let file = get_or_create_config_dir();
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &content).expect("写入配置文件内容失败");
    }
}

/// 获取用户配置目录
pub fn get_or_create_config_dir() -> File {
    let mut config_dir = dirs::config_dir().expect("获取配置目录失败");
    
    config_dir.push("/read/config.json");

    let metadata = fs::metadata(config_dir.clone());

    if metadata.is_ok() && metadata.unwrap().is_file() {
        // 存在，且是file，直接获取文件句柄
        return File::open(config_dir).expect("获取配置文件失败")
    } else {
        // 创建
        return File::create(config_dir).expect("创建配置文件失败")
    }
}