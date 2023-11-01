use std::cell::RefCell;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;

/// 封装了文件阅读操作的结构体
#[warn(dead_code)]
pub struct FileRead {
    /// 当前行（下一行的索引）
    current_line_no: u64,

    /// 总行数
    total_line: usize,

    /// 当前页数据
    current_page: Rc<RefCell<Vec<String>>>,

    /// 文件路径
    file_path: String,

    /// 文件读取数据
    content: Box<Vec<String>>,

    /// 提示信息
    msg: String,

    /// 开始
    start: bool,

    /// 结束
    end: bool,
}

impl FileRead {

    /// 初始化FileRead
    /// # Arguments
    /// - start_line: 起始行数
    /// - file_path: 文件路径
    /// - line_num: 每页显示行数
    pub fn new(start_line: &u64, file_path: &String, line_num: &u16) -> FileRead {

        // 打开文件并读取
        let file = File::open(file_path)
            .expect("打开文件失败");
        let file = BufReader::new(file);

        // 获取文件主要内容
        let lines = file.lines().map(|line| line.expect("解析内容错误")).collect();
        let content : Box<Vec<String>> = Box::new(lines);
        // 获取文件总行数
        let total_line = content.len();
        if total_line <= 0 {
            panic!("文件为空, 无可读内容");
        }

        // 获取当前页, 并填充当前页数据
        let mut page_content = Vec::with_capacity(*line_num as usize);

        // 开始索引
        let start_index = start_line - 1;
        // 第一页结尾行数索引
        let end_index = start_index + (*line_num as u64);

        // 获取当前索引
        let mut current_line_no = *start_line;
        for index in start_index..end_index {

            // 如果文章只有一页，当读到最后一行后直接跳出
            if index >= total_line as u64 {
                break;
            }

            // 保存当前页数据
            let line = content.get(index as usize).map_or("", |l| l);
            page_content.push(line.clone().parse().unwrap());

            // 修改当前行码
            current_line_no = index + 1;

            // 容器已满直接跳出
            if page_content.len() == page_content.capacity() {
                break;
            }
        }

        FileRead {
            current_line_no, // 为防止仅有一页的内容，因此初始化为0
            total_line,
            current_page: Rc::new(RefCell::new(page_content)),
            file_path: file_path.clone(),
            content,
            msg: String::from(""),
            start: false,
            end: false,
        }
    }

    /// 下一页
    /// # Arguments
    /// - line_num: 每页显示行数
    pub fn next_page(&mut self, line_num: &u16) {
        if self.current_line_no >= self.total_line as u64 {
            self.msg = String::from("当前文档已到结尾, end......");
            self.end = true;
            return;
        }

        // 获取开始索引
        let start_index = match self.current_line_no {
            0 => 1,
            _ => self.current_line_no,
        };
        // 结束索引
        let end_index = start_index + *line_num as u64;

        // 清空当前页容器
        let mut page_content = self.current_page.borrow_mut();
        page_content.clear();

        // 读取新页面中的数据
        for index in start_index..end_index {
            // 保存数据到页面容器中
            let line = self.content.get(index as usize).map_or("", |l| l);
            page_content.push(line.clone().parse().unwrap());

            // 更新当前行
            self.current_line_no = index + 1;

            // 读取到最后一行，直接跳出
            if index >= (self.total_line - 1) as u64 {
                break;
            }
        }

        // 清空提示信息
        self.msg = String::from("");
    }

    /// 上一页
    /// # Arguments
    /// - line_num: 每页显示行数
    pub fn pre_page(&mut self, line_num: &u16) {
        if self.current_line_no <= *line_num as u64 {
            self.msg = String::from("当前文档已到第一页, start......");
            self.start = true;
            return;
        }

        // 获取上一页的结束索引
        let end_index = self.current_line_no - *line_num as u64;
        // 获取上一页的开始索引
        let start_index: u64 = match end_index - (*line_num as u64) > 0 {
            true => end_index - *line_num as u64,
            _flase => 0,
        };

        // 清空当前页容器
        let mut page_content = self.current_page.borrow_mut();
        page_content.clear();

        // 填充新页内容
        for index in start_index..end_index {
            // 填充内容
            let line = self.content.get(index as usize).map_or("", |l| l);
            page_content.push(line.clone().parse().unwrap());

            // 修改当前行
            self.current_line_no = index + 1;
        }
    }

    /// 判断是否结尾
    pub fn is_end(&self) -> bool {
        self.end
    }

    /// 获取当前页内容
    pub fn get_current_page(&self) -> Rc<RefCell<Vec<String>>> {
        self.current_page.clone()
    }

    /// 获取提示信息
    pub fn get_tip(&self) -> &str {
        &self.msg
    }
}