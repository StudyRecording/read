use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, stdout, Write};
use std::ops::Add;
use std::thread::{JoinHandle, sleep};
use std::time::Duration;
use std::sync::mpsc::{Receiver, Sender};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crate::input::cli::Cli;
use crate::input::event;
use crate::input::event::KeyEvent;

pub fn detail_file(args: Cli, rx: Receiver<KeyEvent>) {

    // 读取参数
    // let line_num = args.get_num();
    // let start_line = args.get_start();
    // let file_path = args.get_file();
    // let mut auto = args.get_auto();
    // let time = args.get_time();
    let line_num = &args.num;
    let start_line = &args.start;
    let file_path = &args.file;
    let mut auto = &args.auto;
    let time = &args.time;
    println!("请求参数: {:?}", &args);

    // 每页行数
    let mut context: Vec<String> = Vec::with_capacity(*line_num);

    // 验证文件格式
    let extend_name = file_path.split(".")
        .last()
        .expect("不能正常获取文件扩展名");
    if extend_name != "txt" {
        panic!("非txt文件, 不可处理");
    }

    // 获取输出流
    let out = stdout();
    let mut out = BufWriter::new(out);

    let sec = Duration::from_secs(*time);

    // 打开文件并读取
    let file = File::open(file_path)
        .expect("打开文件失败");
    let file = BufReader::new(file);

    // println!("文件行数: {}", file.lines().count());
    for (num, line) in file.lines().enumerate() {

        if (num + 1) < *start_line {
            continue;
        }

        let line_str = line.expect("读取失败");

        context.insert(num % line_num, line_str);

        // 填充完毕，输出
        if context.capacity() == context.len() {
            // 输出
            for (i, str) in context.iter().enumerate() {
                let index: isize = (2 + i + num - line_num) as isize;
                writeln!(out, "{}: {str}", index).expect("写入失败");
            }
            // 强制刷屏
            out.flush().expect("刷新失败");

            // 清空容器
            context.clear();

            // 睡眠
            let mut key;
            if *auto {
                sleep(sec);

                // 获取key
                key = match rx.try_recv() {
                    Ok(k) => k,
                    Err(_) => KeyEvent::Other,
                };
            } else {
                // 获取key
                key = match rx.recv() {
                    Ok(k) => k,
                    Err(_) => KeyEvent::Other,
                };
            }

            match key {
                KeyEvent::NextPage => {}
                KeyEvent::PreviousPage => {
                    panic!("暂未实现上一页功能")
                }
                KeyEvent::AutoRead => {auto = &true }
                KeyEvent::StopAuto => {auto = &false }
                KeyEvent::ESC => { break; }
                KeyEvent::Other => {}
            }

            // 清屏
            clear_screen(line_num);
        }
    }
    writeln!(out, "文件阅读结束, end......").expect("写入失败");
}

/// 清屏
fn clear_screen(line_num: &usize) {
    // 擦除光标到屏幕末尾位置
    // 参考文档:https://zh.wikipedia.org/wiki/ANSI%E8%BD%AC%E4%B9%89%E5%BA%8F%E5%88%97#%E4%BD%BF%E7%94%A8Shell%E8%84%9A%E6%9C%AC%E7%9A%84%E7%A4%BA%E4%BE%8B
    print!("\x1b[1G"); // 将光标移动到第一列
    let cursor_up_line = String::new();
    let cursor_up_line = cursor_up_line.add("\x1b[")
        .add(&*line_num.to_string())
        .add("A");
    print!("{}", cursor_up_line); // 将光标上移动line_num行
    print!("\x1b[J"); // 擦除光标到屏幕末尾位置
}

/// 按键监听
pub fn keys_listener(tx: Sender<KeyEvent>, show_thread: JoinHandle<()>) {
    enable_raw_mode().expect("开启终端raw mode模式失败");
    loop {
        let ke = event::read_event();
        println!("监听按键:{:?}", ke);
        if !show_thread.is_finished() {
            tx.send(ke.clone()).expect("发送键盘监听事件失败");
        } else {
            // 如果显示线程结束，则按键监听同样结束，整个程序资源回收，结束
            break;
        }
        if ke == KeyEvent::ESC {
            break;
        }


        // if ke != KeyEvent::Other {
        //     // println!("按键:{:?}", ke);
        //     // print!("\x1b[G"); // 光标移到第一列
        //
        // }
    }
    disable_raw_mode().expect("关闭终端raw mode模式失败");
}