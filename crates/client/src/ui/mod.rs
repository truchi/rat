mod config;
mod views;
mod widgets;

pub use config::*;
pub use views::*;
pub use widgets::*;

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
    pub use crossterm::event::KeyModifiers;
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

pub fn exit() {
    leave();
    std::process::exit(0);
}

pub async fn main(mut client: Client) {
    let out = stdout();
    let mut out = out.lock();

    let config = Config::new();
    let mut user_name_view = UserNameView::new(config);

    user_name_view.render(&mut out);
    out.flush();

    let mut stream = x::EventStream::new();

    while let Some(Ok(event)) = stream.next().await {
        let mut redraw = false;

        let quit = x::KeyEvent {
            code:      x::KeyCode::Char('c'),
            modifiers: x::KeyModifiers::CONTROL,
        };

        if event == x::Event::Key(quit) {
            return exit();
        }

        match user_name_view.handle(event) {
            Some(UserNameHandled::Redraw) => {
                x::queue!(out, x::Clear(x::ClearType::All));
                user_name_view.render(&mut out);
                out.flush();
            }
            Some(UserNameHandled::Enter) => {
                let input = user_name_view.input_widget;
                let user_name = if input.value.is_empty() {
                    input.placeholder
                } else {
                    input.value
                };
                leave();

                let mut connection = client.connect_user(user_name.clone()).await;
                println!("GG {}", user_name);
                connection.enter_world().await;
                dbg!(connection.db());

                return std::process::exit(0);
            }
            _ => {}
        }
    }
}
