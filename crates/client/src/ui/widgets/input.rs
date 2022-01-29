use super::*;

#[derive(Debug)]
pub enum InputHandled {
    Redraw,
    Enter,
}

#[derive(Debug)]
pub struct InputWidget {
    pub config:      Config,
    pub x:           u16,
    pub y:           u16,
    pub label:       String,
    pub placeholder: String,
    pub value:       String,
}

impl InputWidget {
    pub fn new(
        label: impl Into<String>,
        placeholder: impl Into<String>,
        value: impl Into<String>,
        x: u16,
        y: u16,
        config: Config,
    ) -> Self {
        Self {
            config,
            x,
            y,
            label: label.into(),
            placeholder: placeholder.into(),
            value: value.into(),
        }
    }

    pub fn render<W: Write>(&self, mut w: W) {
        write!(w, "{}{}", x::MoveTo(self.x, self.y), self.label);

        if self.value.is_empty() {
            write!(w, "{}", self.placeholder);
        } else {
            write!(w, "{}", self.value);
        }
    }

    pub fn handle(&mut self, event: x::Event) -> Option<InputHandled> {
        match event {
            x::Event::Key(x::KeyEvent { code, modifiers }) => match code {
                x::KeyCode::Char(c) => {
                    self.value.push(c);
                    return Some(InputHandled::Redraw);
                }
                x::KeyCode::Backspace => {
                    self.value.pop();
                    return Some(InputHandled::Redraw);
                }
                x::KeyCode::Enter => return Some(InputHandled::Enter),
                _ => {}
            },
            _ => {}
        }

        None
    }
}
