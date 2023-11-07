use std::cell::RefCell;
use std::io::{BufWriter, Stdout, stdout};
use std::rc::Rc;
use std::sync::mpsc::{Receiver};
use std::thread::sleep;
use std::time::Duration;
use crossterm::cursor::{MoveTo, MoveToColumn, MoveUp};
use crossterm::{ExecutableCommand, execute};
use crossterm::style::{Color, Print, SetBackgroundColor};
use crossterm::terminal::{Clear, ClearType, size};
use crate::file_read::FileRead;
use crate::input::cli::Cli;
use crate::input::event::KeyEvent;

/// 光标移动到最做列
fn move_to_leftmost_column(out: &mut BufWriter<Stdout>) {
    out.execute(MoveToColumn(0)).expect("光标移到最左行失败");
}

/// 清除一页
fn clear_page(out: &mut BufWriter<Stdout>, write_rows: u16) {
    // 光标移动到最左列
    move_to_leftmost_column(out);
    // // 上移line_num行
    if write_rows > 0 {
        out.execute(MoveUp(write_rows)).expect("光标上移失败");
    }
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

/// 中文字符串获取完整字符索引
fn fix_index(s: &String, index: usize) -> usize {
    if index >= s.len() - 1 {
        s.len() - 1
    } else {
        let mut  result = index;
        loop {
            if s.is_char_boundary(result) {
                result = result - 1;
                break;
            } else {
                result = result + 1;
            }
        }
        result
    }
}

/// 输出一页
fn write_page(out: &mut BufWriter<Stdout>, page: Rc<RefCell<Vec<String>>>, width: &u16) -> u16 {

    // 留点余地
    let show_with = width;

    // 移到最左列
    move_to_leftmost_column(out);
    // let (x, y) = crossterm::cursor::position().expect("获取位置失效");

    let mut row_total = 0;
    for line in page.borrow().iter() {
       // 长度为0，直接跳过
        if line.len() == 0 {
           continue;
       }
        let rows = line.len() / (*show_with as usize) + 1;

        let mut start_index = 0;
        let mut end_index = *show_with as usize;
        for i in 0..rows  {

            end_index = fix_index(line, end_index);


            let show_str = &line[start_index..=end_index];
            write_line(out, show_str.to_string(), Color::Reset);

            start_index = end_index + 1;
            end_index = start_index + *show_with as usize;
        }
        row_total += rows;
    }

    row_total as u16

    // change_cursor_position(out, x, y);
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
    let msg = String::from("操作按键:【n | ↓】下一页  【p | ↑】上一页  【a】自动翻页 【s】停止自动翻页 【Esc | e】退出程序, Tips: ");
    write(&mut out, msg, Color::Reset);
    let (x, y) = crossterm::cursor::position().expect("获取点位失败");
    write_tip(&mut out, x, y,"hpc制作!!!".to_string(), Color::Red);


    // 初始化FileRead
    let mut fr = FileRead::new(start_line, file_path, line_num);

    // 获取终端宽度
    let (width, _) = size().expect("获取终端尺寸失败");

    // 不是最后一页就一直循环
    while !fr.is_end() {
        

        // let current_page = fr.get_current_page();
        // for line in current_page.borrow().iter() {
        //     write_line(&mut out, line.to_string(), Color::Reset);
        // }
        // 获取当前页并打印
        let write_rows = write_page(&mut out, fr.get_current_page(), &width);

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
        clear_page(&mut out, write_rows);
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
