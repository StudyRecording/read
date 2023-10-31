use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, stdout, Stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use std::sync::mpsc::Receiver;
use crossterm::cursor::{MoveToColumn, MoveUp};
use crossterm::ExecutableCommand;
use crossterm::terminal::{Clear, ClearType};
use crate::input::cli::Cli;
use crate::input::event::KeyEvent;

pub fn detail_file(args: Cli, rx: Receiver<KeyEvent>) {

    // 读取参数
    let line_num = &args.num;
    let start_line = &args.start;
    let file_path = &args.file;
    let mut auto = &args.auto;
    let time = &args.time;
    println!("请求参数: {:?}", &args);

    // 每页行数
    let mut context: Vec<String> = Vec::with_capacity(*line_num as usize);

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
    let mut key_exist = false;
    for (num, line) in file.lines().enumerate() {

        if (num + 1) < *start_line {
            continue;
        }

        let line_str = line.expect("读取失败");

        context.insert(num % (*line_num as usize), line_str);

        // 填充完毕，输出
        if context.capacity() == context.len() {
            // 输出
            for (i, str) in context.iter().enumerate() {
                let index: isize = (2 + i + num - (*line_num as usize)) as isize;
                // 移动光标到最左边
                out.execute(MoveToColumn(0)).expect("光标移动到最左列失败");
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
                KeyEvent::ESC => {
                    key_exist = true;
                    break;
                }
                KeyEvent::Other => {}
            }

            // 清屏
            clear_screen(&mut out, line_num);
        }
    }
    if !key_exist {
        // 移动光标到最左边
        out.execute(MoveToColumn(0)).expect("光标移动到最左列失败");
        writeln!(out, "文件阅读结束, end......").expect("写入失败");
    }
}

/// 清屏
fn clear_screen(out: &mut BufWriter<Stdout>, line_num: &u16) {

    // 移到最左行
    out.execute(MoveToColumn(0)).expect("光标移到最左行失败");
    // 上移line_num行
    out.execute(MoveUp(*line_num)).expect("光标上移失败");
    // 擦除光标到屏幕末尾位置
    out.execute(Clear(ClearType::FromCursorDown)).expect("擦除光标到屏幕末尾位置操作失败");
}

