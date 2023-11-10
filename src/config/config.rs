use std::{fs::{File, self, OpenOptions}, io::{BufReader, BufWriter, Read}};

use serde::{Serialize, Deserialize};

use crate::input::cli::Cli;


/// 配置
#[derive(Serialize, Deserialize, Clone)]
pub struct Config {

    /// 命令参数
    cli: Cli,

    /// txt文件路径
    file_path: String,

    /// 当前在读行数
    current_line_no: u64,

    /// 其它配置
    #[serde(skip)]
    other_config: Vec<Config>,
}

impl Config {

    /// 获取新Config对象
    pub fn new(cli: &Cli, file_path: Option<String>) -> Config {
        let mut cli = cli.clone();


        // file_path为空，则从配置文件中获取, 并进行参数校验
        let mut content = get_config_by_file();
        if content.is_empty() && file_path.is_none() {
            panic!("未找到txt文件, 请传参[-f <FILE>]");
        }

        // file_path不为空，获取绝对路径
        if file_path.is_some() {
            let absolute_path = file_path.unwrap();
            // 手动保证命令行的start为1
            let mut cli_start = false;
            let current_line_no = match cli.start {
                Some(no) => {
                    cli_start = true;
                    no
                },
                None => 1,
            };

            // 查找绝对路径相同的配置
            let conf_index = content.iter().position(|item| item.file_path == absolute_path);
            if conf_index.is_none() {
                // 设置默认值
                default_cli_set(&mut cli);
                // 如果未找到，直接创建新conf对象并返回
                return Config { cli, file_path: absolute_path, current_line_no, other_config: content};
            } else {
                // 找到conf配置类
                let mut c = content.remove(conf_index.unwrap());
                if cli_start {
                    // 仅当用户传入开始行号，才修改当前行号
                    c.current_line_no = current_line_no;
                }
                update_cli(&mut c, &cli);
                c.other_config = content;
                return c;
            }
        }
        
        let mut config = content.pop().unwrap();
        // 如果命令参数中传入了开始行参数，则按照命令参数中为主
        if cli.start.is_some() {
            config.current_line_no = cli.start.unwrap();
        }
        update_cli(&mut config, &cli);
        config.other_config = content;
        config
    }

    /// 更新配置文件, 如果存在配置信息则更新，否则则添加配置信息
    pub fn update_config(&mut self, current_line_no: u64) {
        let mut all_config: Vec<&Config> = Vec::with_capacity(self.other_config.len() + 1);
        for conf in &self.other_config {
            all_config.push(&conf);
        }

        self.current_line_no = current_line_no;
        all_config.push(&self);

        // 序列化到文件中
        let file = get_or_create_config_dir(false);
        let writer = BufWriter::new(file);
        
        serde_json::to_writer(writer, &all_config).expect("写入配置文件内容失败");
    }

    /// 获取文件路径
    pub fn get_file_path(&self) -> &str {
        &self.file_path
    }

    /// 获取上传读取到的行数
    pub fn get_current_no(&self) -> &u64 {
        &self.current_line_no
    }

    /// 获取命令行参数
    pub fn get_cli(&self) -> &Cli {
        &self.cli
    }
}

/// 更新命令
fn update_cli(c: &mut Config, cli: &Cli) {
    // 更新命令
    c.cli.auto = cli.auto;

    // 赋值cli
    if cli.num.is_some() {
        c.cli.num = cli.num;
    }
    if cli.time.is_some() {
        c.cli.time = cli.time;
    }
}

/// 设置cli的默认值
fn default_cli_set(cli: &mut Cli) {
    // 设置每页行数和auto的默认值
    if cli.num.is_none() {
        cli.num = Some(1);
    }
    if cli.time.is_none() {
        cli.time = Some(2);
    }
}

/// 获取用户配置目录
/// # Args
/// - read 读取文件:true, 写入文件：false
pub fn get_or_create_config_dir(read: bool) -> File {
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
            .truncate(!read)
            .read(read)
            .open(home_dir)
            .expect("创建配置文件失败")
}

/// 获取配置通过文件
fn get_config_by_file() -> Vec<Config> {
    let file = get_or_create_config_dir(true);
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).expect("文件读取失败");
    let mut content : Vec<Config> = Vec::new();
    if !contents.is_empty() {
        content = serde_json::from_str(&contents).expect("读取文件内容，反序列化失败");
    }
    content
}