use std::{fs::{File, self, OpenOptions}, io::{BufReader, BufWriter, Read}, path::PathBuf};

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
        let path = PathBuf::from(file_path);
        let absolute_path = fs::canonicalize(&path).expect("txt文件转化绝对路径失败")
                                        .into_os_string().into_string().expect("txt文件转化绝对路径失败");
        Config { cli, file_path: absolute_path, current_line_no, }
    }

    /// 更新配置文件, 如果存在配置信息则更新，否则则添加配置信息
    pub fn update_config(&mut self, current_line_no: u64) {
        let file = get_or_create_config_dir();
        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        reader.read_to_string(&mut contents).expect("文件读取失败");
        let mut content : Vec<Config> = Vec::new();
        if !contents.is_empty() {
            content = serde_json::from_str(&contents).expect("读取文件内容，反序列化失败");
        }
        
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
            self.current_line_no = current_line_no;
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
    let mut home_dir = dirs::home_dir().expect("获取配置目录失败");

    home_dir.push(".read/");

    // 如果没有read目录，则创建目录
    let metadata = fs::metadata(home_dir.clone());
    if metadata.is_err() || metadata.unwrap().is_file() {
        // 如果不存在或者是文件
        fs::create_dir(home_dir.clone()).expect("创建.read配置目录失败");
    }

    // 获取文件
    home_dir.push("config.json");
    OpenOptions::new()
            .write(true)
            .create(true)
            // .truncate(true)
            .read(true)
            .open(home_dir)
            .expect("创建配置文件失败")
}