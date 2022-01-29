use super::*;

#[derive(Debug)]
pub struct UserNameView {
    pub config:       Config,
    pub input_widget: InputWidget,
}

#[derive(Debug)]
pub enum UserNameHandled {
    Redraw,
    Enter,
}

impl UserNameView {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            input_widget: InputWidget::new("Name: ", "<anon>", "", 1, 1, config),
        }
    }

    pub fn render<W: Write>(&self, mut w: W) {
        self.input_widget.render(w);
    }

    pub fn handle(&mut self, event: x::Event) -> Option<UserNameHandled> {
        if let Some(handled) = self.input_widget.handle(event) {
            Some(match handled {
                InputHandled::Redraw => UserNameHandled::Redraw,
                InputHandled::Enter => UserNameHandled::Enter,
            })
        } else {
            None
        }
    }
}
