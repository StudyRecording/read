use std::sync::mpsc;
use std::thread;
use crate::input::cli;



mod input;
mod context;

fn main() {
    // 获取命令参数
    let args = cli::read();

    // 创建多线程通信通道
    let (tx, rx) = mpsc::channel();

    // 单开子线程
    let read_show_file = thread::spawn(move || {
        // 阅读文件
        context::detail_file(args, rx);
    });

    // 负责按键事件监听
    context::keys_listener(tx, read_show_file);

    // read_show_file.join().expect("按键监听线程join失败");

}
