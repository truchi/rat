use super::*;

#[derive(Debug)]
pub enum Flow {
    Redraw,
    Enter,
}

#[derive(Debug)]
pub struct Input {
    pub label:       String,
    pub placeholder: String,
    pub value:       String,
}

impl Input {
    pub fn new(
        label: impl Into<String>,
        placeholder: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        Self {
            label:       label.into(),
            placeholder: placeholder.into(),
            value:       value.into(),
        }
    }

    pub fn value(&self) -> &str {
        if self.value.is_empty() {
            &self.placeholder
        } else {
            &self.value
        }
    }

    pub fn render<W: Write>(&self, x: u16, y: u16, config: Config, mut w: W) {
        write!(w, "{}{}{}", x::MoveTo(x, y), self.label, self.value());
    }

    pub fn handle(&mut self, event: x::Event) -> Option<Flow> {
        match event {
            x::Event::Key(x::KeyEvent { code, modifiers }) => match code {
                x::KeyCode::Char(c) => {
                    self.value.push(c);
                    return Some(Flow::Redraw);
                }
                x::KeyCode::Backspace => {
                    self.value.pop();
                    return Some(Flow::Redraw);
                }
                x::KeyCode::Enter => return Some(Flow::Enter),
                _ => {}
            },
            _ => {}
        }

        None
    }
}
