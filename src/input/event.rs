use std::time::Duration;
use crossterm::event::{Event, KeyCode, poll, read};

/// 键盘事件读取
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum KeyEvent {
    // n: 下一行
    NextPage,

    // p: 上一行
    PreviousPage,

    // a: 自动阅读
    AutoRead,

    // s: 停止自动阅读,
    StopAuto,

    // esc: 退出应用
    ESC,

    // 其他按键
    Other,
}

/// 读取按键事件
pub fn read_event() -> KeyEvent {
    // 在500ms内获取输入，成功则为true，未获取则为false
    let poll = match poll(Duration::from_millis(500)) {
        Ok(_) => true,
        Err(_) => false,
    };
    if poll {
        return get_key_event()
    }
    KeyEvent::Other
}

/// 实际获取键盘事件并转换为本地功能按键事件枚举
fn get_key_event() -> KeyEvent {
    // 获取事件
    let event_key = match read() {
        Ok(key) => key,
        Err(_) => Event::FocusGained,
    };
    // 匹配事件
    match event_key {
        Event::FocusGained => KeyEvent::Other,
        Event::FocusLost => KeyEvent::Other,
        Event::Key(key) => {
            if key.code == KeyCode::Char('n') || key.code == KeyCode::Down {
                KeyEvent::NextPage
            } else if key.code == KeyCode::Char('p') || key.code == KeyCode::Up {
                KeyEvent::PreviousPage
            } else if key.code == KeyCode::Char('a') {
                KeyEvent::AutoRead
            } else if key.code == KeyCode::Char('s') {
                KeyEvent::StopAuto
            } else if key.code == KeyCode::Esc {
                KeyEvent::ESC
            } else {
                KeyEvent::Other
            }
        }
        Event::Mouse(_) => KeyEvent::Other,
        Event::Paste(_) => KeyEvent::Other,
        Event::Resize(_, _) => KeyEvent::Other,
    }
}



