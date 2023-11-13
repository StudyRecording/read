use std::{sync::mpsc, fs};
use std::thread;
use time::{UtcOffset, macros};
use tracing::{info, Level};
use tracing_subscriber::fmt::time::OffsetTime;

use crate::input::{cli, event};

mod input;
mod config;
mod file_read;
mod display;

fn main() {

    // 日志初始化
    let mut home_dir = dirs::home_dir().expect("获取目录失败");

    home_dir.push(".read/logs/");

    // 如果没有read目录，则创建目录
    let metadata = fs::metadata(home_dir.clone());
    if metadata.is_err() || metadata.unwrap().is_file() {
        // 如果不存在或者是文件
        fs::create_dir(home_dir.clone()).expect("创建.read日志目录失败");
    }

    let local_time = OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0).unwrap(),
        macros::format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"),
    );
    
    let file_apperder = tracing_appender::rolling::hourly( home_dir.to_str().unwrap(), "read.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_apperder);
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_ansi(false)
        .with_target(true)
        .with_timer(local_time)
        .with_line_number(true)
        .with_file(true)
        .with_writer(non_blocking)
        .init();

    info!("日志初始化完成");

    // 获取命令参数
    let args = cli::read();
    info!("程序初始化命令参数:{:?}", args);

    // 创建多线程通信通道
    let (tx, rx) = mpsc::channel();

    // 单开子线程
    let read_show_file = thread::spawn(move || {
        // 阅读文件
        display::terminal_display::display(args, rx);
    });

    // 负责按键事件监听
    event::keys_listener(tx, read_show_file);


}
