use super::*;

#[derive(Debug)]
pub struct UserNameView {
    pub config:      Config,
    pub label:       String,
    pub placeholder: String,
    pub value:       String,
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
}
