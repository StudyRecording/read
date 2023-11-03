use std::sync::mpsc;
use std::thread;
use crate::input::{cli, event};

mod input;

mod file_read;
mod display;

fn main() {
    // 获取命令参数
    let args = cli::read();

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
