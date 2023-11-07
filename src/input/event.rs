use std::sync::mpsc::Sender;
use std::thread::JoinHandle;
use std::time::Duration;
use crossterm::event::{Event, KeyCode, poll, read};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crate::input::event;

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
        Ok(status) => status,
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
    match_event_key(event_key)
}

#[cfg(not(target_os = "windows"))]
fn match_event_key(event_key: Event) -> KeyEvent {
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
            } else if key.code == KeyCode::Esc || key.code == KeyCode::Char('e') {
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

#[cfg(target_os = "windows")]
fn match_event_key(event_key: Event) -> KeyEvent {
    match event_key {
        Event::FocusGained => KeyEvent::Other,
        Event::FocusLost => KeyEvent::Other,
        Event::Key(key) => {
            if key.kind == KeyEventKind::Release {
                if key.code == KeyCode::Char('n') || key.code == KeyCode::Down {
                    return KeyEvent::NextPage
                } else if key.code == KeyCode::Char('p') || key.code == KeyCode::Up {
                    return KeyEvent::PreviousPage
                } else if key.code == KeyCode::Char('a') {
                    return KeyEvent::AutoRead
                } else if key.code == KeyCode::Char('s') {
                    return KeyEvent::StopAuto
                } else if key.code == KeyCode::Esc || key.code == KeyCode::Char('e') {
                    return KeyEvent::ESC
                }
            }
            KeyEvent::Other

        }
        Event::Mouse(_) => KeyEvent::Other,
        Event::Paste(_) => KeyEvent::Other,
        Event::Resize(_, _) => KeyEvent::Other,
    }
}

/// 按键监听
pub fn keys_listener(tx: Sender<KeyEvent>, show_thread: JoinHandle<()>) {
    enable_raw_mode().expect("开启终端raw mode模式失败");
    loop {
        // show线程结束，按键监听线程也直接跳出循环结束线程
        if show_thread.is_finished() {
            break;
        }
        let ke = event::read_event();
        if ke == KeyEvent::Other {
            // 如果是该事件，则是无效按键，继续循环监听其他按键
            continue;
        }
        // println!("监听按键:{:?}", ke);
        if !show_thread.is_finished() {
            tx.send(ke.clone()).expect("发送键盘监听事件失败");
        }
        if ke == KeyEvent::ESC {
            break;
        }
    }
    disable_raw_mode().expect("关闭终端raw mode模式失败");
}



