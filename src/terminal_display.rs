use std::io::{BufWriter, Stdout, stdout};
use std::sync::mpsc::{Receiver};
use std::thread::sleep;
use std::time::Duration;
use crossterm::cursor::{MoveTo, MoveToColumn, MoveUp};
use crossterm::{ExecutableCommand, execute};
use crossterm::style::{Color, Print, SetBackgroundColor};
use crossterm::terminal::{Clear, ClearType};
use crate::file_read::FileRead;
use crate::input::cli::Cli;
use crate::input::event::KeyEvent;

/// 光标移动到最做列
fn move_to_leftmost_column(out: &mut BufWriter<Stdout>) {
    out.execute(MoveToColumn(0)).expect("光标移到最左行失败");
}

/// 清除一页
fn clear_page(out: &mut BufWriter<Stdout>, line_num: &u16) {
    // 光标移动到最左列
    move_to_leftmost_column(out);
    // 上移line_num行
    out.execute(MoveUp(*line_num)).expect("光标上移失败");
    // 擦除光标到屏幕末尾位置
    out.execute(Clear(ClearType::FromCursorDown)).expect("擦除光标到屏幕末尾位置操作失败");
}

/// 输出一行文字
/// # Argument
/// - out
/// - line：文字内容
/// - coler：文字颜色
fn write_line(out: &mut BufWriter<Stdout>, line: String, color: Color) {
    // 移到最左列
    move_to_leftmost_column(out);
    // 输出
    execute!(out, SetBackgroundColor(color), Print(line), Print("\r\n")).expect("输出到终端失败");
}

/// 输出一行文字，但不换行
fn write(out: &mut BufWriter<Stdout>, line: String, color: Color) {
    // 移到最左列
    move_to_leftmost_column(out);
    // 输出
    execute!(out, SetBackgroundColor(color), Print(line)).expect("输出到终端失败");
}


/// 在位置(x, y)开始打印信息
fn write_tip(out: &mut BufWriter<Stdout>, x: u16, y: u16, line: String, color: Color) {
    execute!(out, MoveTo(x, y), Clear(ClearType::UntilNewLine) ,SetBackgroundColor(color), Print(line))
        .expect("输出到终端失败");
    execute!(out, SetBackgroundColor(Color::Reset), Print("\r\n")).expect("换行失败");
}

/// 切换光标位置
fn change_cursor_position(out: &mut BufWriter<Stdout>, x: u16, y: u16) {
    execute!(out, MoveTo(x, y)).expect("切换光标位置失败");
}


/// 显示，主要函数
pub fn display(args: Cli, rx: Receiver<KeyEvent>) {
    // 读取参数
    let line_num = &args.num;
    let start_line = &args.start;
    let file_path = &args.file;
    let mut auto = &args.auto;
    let time = &args.time;
    // println!("请求参数: {:?}", &args);

    // 获取输出流
    let out = stdout();
    let mut out = BufWriter::new(out);
    let msg = String::from("操作按键:【n | ↓】下一页  【p | ↑】上一页  【a】自动翻页  【Esc | e】退出程序, Tips: ");
    write(&mut out, msg, Color::Reset);
    let (x, y) = crossterm::cursor::position().expect("获取点位失败");
    write_tip(&mut out, x, y,"hpc制作!!!".to_string(), Color::Red);


    // 初始化FileRead
    let mut fr = FileRead::new(start_line, file_path, line_num);

    // 不是最后一页就一直循环
    while !fr.is_end() {
        
        // 获取当前页并打印
        let current_page = fr.get_current_page();
        for line in current_page.borrow().iter() {
            write_line(&mut out, line.to_string(), Color::Reset);
        }

        // 自动阅读
        if *auto {
            // 如果自动阅读，则停顿一下
            auto_read_sleep(&mut auto, time);
            // 自动阅读时，无论是否接收到按键指令，都自动进行到下一行
            fr.next_page(line_num);
        }

        // 获取按键事件
        let key = get_key(auto, &rx);
        // 匹配到有效键，则直接跳出监听循环
        match key {
            KeyEvent::NextPage => { fr.next_page(line_num) }
            KeyEvent::PreviousPage => { fr.pre_page(line_num) }
            KeyEvent::AutoRead => { auto = &true }
            KeyEvent::StopAuto => { auto = &false }
            KeyEvent::ESC => {
                break;
            }
            KeyEvent::Other => {}
        }

        // 清屏
        clear_page(&mut out, line_num);
    }

    write_line(&mut out, "end......".to_string(), Color::Reset);
}

/// 自动阅读睡眠时间
fn auto_read_sleep(auto: &bool, time: &u64) {
    if *auto {
        sleep(Duration::from_secs(*time));
    }
}

fn get_key(auto: &bool, rx: &Receiver<KeyEvent>) -> KeyEvent {
    // 按键监听获取
    if *auto {
        match rx.try_recv() {
            Ok(k) => k,
            Err(_) => KeyEvent::Other,
        }
    } else {
        match rx.recv() {
            Ok(k) => k,
            Err(_) => KeyEvent::Other,
        }
    }
}
