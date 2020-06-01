use iced::*;
use base64::{encode};

use super::icon::icon;
use super::theme::Theme;

#[derive(Debug, Clone, Default)]
pub struct ResultRow {
    pub first: String,
    pub second: String,
    state: ResultRowState,
}

#[derive(Debug, Clone, Default)]
struct ResultRowState {
    copy_button: button::State,
}

#[derive(Debug, Clone)]
pub enum ResultRowMessage {
    Copy(String),
}

impl ResultRow {

    pub fn new(first: String, second: String) -> ResultRow {
        ResultRow {
            first,
            second,
            state: ResultRowState {
                copy_button: button::State::new(),
            }
        }
    }

    pub fn view(&mut self) -> Element<ResultRowMessage> {
        let theme = Theme::default();

        Row::new()
            .push(Text::new(self.first.clone()))
            .push(Space::new(Length::Fill, Length::Units(0)))
            .push(Button::new(
                    &mut self.state.copy_button,
                    icon('\u{e14d}'),
                )
                .on_press(ResultRowMessage::Copy(generate_link(&self.second)))
                .style(theme.clear_button)
            )
            .into()
    }
}

fn generate_link(name: &String) -> String {
    let mangled_name = encode(name.as_bytes());
    let url_encoded_name = mangled_name.replace('=', "%3D");

    format!("https://duck.com/?q=base64+decode+{}", url_encoded_name)
}
