use super::*;

#[derive(Debug)]
pub struct UserNameView {
    pub config:      Config,
    pub label:       String,
    pub placeholder: String,
    pub value:       String,
}

#[derive(Debug)]
pub enum UserNameHandled {
    Redraw,
    Enter,
}

impl UserNameView {
    pub fn render<W: Write>(&self, mut w: W) {
        write!(w, "{}{}: ", x::MoveTo(1, 1), self.label);

        if self.value.is_empty() {
            write!(w, "{}", self.placeholder);
        } else {
            write!(w, "{}", self.value);
        }
    }

    pub fn handle(&mut self, event: x::Event) -> Option<UserNameHandled> {
        match event {
            x::Event::Key(x::KeyEvent { code, modifiers }) => match code {
                x::KeyCode::Char(c) => {
                    self.value.push(c);
                    return Some(UserNameHandled::Redraw);
                }
                x::KeyCode::Backspace => {
                    self.value.pop();
                    return Some(UserNameHandled::Redraw);
                }
                x::KeyCode::Enter => return Some(UserNameHandled::Enter),
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

        None
    }
}
