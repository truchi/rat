use super::*;
use textwrap::wrap;
use textwrap::Options;

#[derive(Debug)]
pub struct Message {
    pub message: Vec<String>,
    pub width:   u16,
    pub height:  u16,
}

impl Message {
    pub fn new(message: String, width: u16) -> Self {
        let message = Self::wrap(message, width);
        let height = message.len() as u16;

        Self {
            message,
            width,
            height,
        }
    }

    pub fn render<W: Write>(&self, x: u16, y: u16, config: Config, mut w: W) {}
}

impl Message {
    fn wrap(message: String, width: u16) -> Vec<String> {
        wrap(&message, width as usize)
            .into_iter()
            .map(|str| str.into_owned())
            .collect()
    }
}
