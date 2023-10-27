use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, stdout, Write};
use std::ops::Add;
use std::thread::sleep;
use std::time::Duration;
use std::borrow::Borrow;
use crate::input::cli::Cli;

pub fn detail_file(args: Cli) {
    let line_num = args.get_num();

    let start_line = args.get_start();

    // 每页行数
    let mut context: Vec<String> = Vec::with_capacity(*line_num);

    // 读取文件
    let file_path = args.get_file();

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
    let sec = Duration::from_secs(2);

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

            // 睡眠1s
            sleep(sec);

            // 清屏
            // 参考文档:https://zh.wikipedia.org/wiki/ANSI%E8%BD%AC%E4%B9%89%E5%BA%8F%E5%88%97#%E4%BD%BF%E7%94%A8Shell%E8%84%9A%E6%9C%AC%E7%9A%84%E7%A4%BA%E4%BE%8B
            print!("\x1b[1G"); // 将光标移动到第一列
            let cursor_up_line = String::new();
            let cursor_up_line = cursor_up_line.add("\x1b[")
                .add(&*line_num.to_string())
                .add("A");
            print!("{}", cursor_up_line); // 将光标上移动3行
            print!("\x1b[J"); // 擦除光标到屏幕末尾位置
        }
    }
}