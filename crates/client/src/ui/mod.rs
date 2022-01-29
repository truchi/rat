mod config;
mod views;

pub use config::*;
pub use views::*;

use super::*;
use futures::FutureExt;
use futures::StreamExt;
use std::io::stdout;
use std::io::StdoutLock;
use std::io::Write;

mod x {
    pub use crossterm::cursor::position;
    pub use crossterm::cursor::Hide;
    pub use crossterm::cursor::MoveTo;
    pub use crossterm::cursor::Show;
    pub use crossterm::event::DisableMouseCapture;
    pub use crossterm::event::EnableMouseCapture;
    pub use crossterm::event::Event;
    pub use crossterm::event::EventStream;
    pub use crossterm::event::KeyCode;
    pub use crossterm::event::KeyEvent;
    pub use crossterm::event::MouseEvent;
    pub use crossterm::event::MouseEventKind;
    pub use crossterm::execute;
    pub use crossterm::queue;
    pub use crossterm::style::Color;
    pub use crossterm::terminal::disable_raw_mode;
    pub use crossterm::terminal::enable_raw_mode;
    pub use crossterm::terminal::size;
    pub use crossterm::terminal::Clear;
    pub use crossterm::terminal::ClearType;
    pub use crossterm::terminal::EnterAlternateScreen;
    pub use crossterm::terminal::LeaveAlternateScreen;
}

pub fn enter() {
    x::enable_raw_mode().unwrap();
    x::execute!(
        stdout(),
        x::EnableMouseCapture,
        x::EnterAlternateScreen,
        x::Hide
    )
    .unwrap();
}

pub fn leave() {
    x::execute!(
        stdout(),
        x::DisableMouseCapture,
        x::LeaveAlternateScreen,
        x::Show
    )
    .unwrap();
    x::disable_raw_mode().unwrap();
}

pub async fn main(mut client: Client) {
    let out = stdout();
    let mut out = out.lock();

    let config = Config::new();
    let mut user_name_view = UserNameView {
        config,
        label: "Name".into(),
        placeholder: "<anon>".into(),
        value: "".into(),
    };

    user_name_view.render(&mut out);
    out.flush();

    let mut stream = x::EventStream::new();

    while let Some(Ok(event)) = stream.next().await {
        let mut redraw = false;

        match event {
            x::Event::Key(x::KeyEvent {
                code: x::KeyCode::Esc,
                modifiers,
            }) => {
                leave();
                return std::process::exit(0);
            }
            x::Event::Key(x::KeyEvent { code, modifiers }) => match code {
                x::KeyCode::Char(c) => {
                    user_name_view.value.push(c);
                    redraw = true;
                }
                x::KeyCode::Backspace => {
                    user_name_view.value.pop();
                    redraw = true;
                }
                x::KeyCode::Enter => {
                    let user_name = if user_name_view.value.is_empty() {
                        user_name_view.placeholder
                    } else {
                        user_name_view.value
                    };
                    leave();
                    client.connect_user(user_name.clone()).await;
                    println!("GG {}", user_name);

                    return std::process::exit(0);
                }
                _ => {}
            },
            x::Event::Mouse(x::MouseEvent {
                kind,
                column,
                row,
                modifiers,
            }) => {}
            x::Event::Resize(columns, rows) => {}
        }

        if redraw {
            x::queue!(out, x::Clear(x::ClearType::All));
            user_name_view.render(&mut out);
            out.flush();
        }
    }
}
