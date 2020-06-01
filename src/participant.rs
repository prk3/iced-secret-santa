use iced::*;
use super::icon::icon;
use super::theme::Theme;

#[derive(Debug, Clone)]
pub struct Participant {
    pub name: String,
    state: ParticipantState,
}

#[derive(Debug, Clone)]
struct ParticipantState {
    remove_button: button::State,
}

#[derive(Debug, Clone)]
pub enum ParticipantMessage {
    Remove,
}

impl Participant {

    pub fn new(name: String) -> Participant {
        Participant {
            name,
            state: ParticipantState {
                remove_button: button::State::new(),
            }
        }
    }

    pub fn view(&mut self) -> Element<ParticipantMessage> {
        let theme = Theme::default();

        Row::new()
            .push(
                Text::new(self.name.clone())
                    .width(Length::Fill)
            )
            .push(
                Button::new(
                    &mut self.state.remove_button,
                    icon('\u{e5cd}'),
                )
                .on_press(ParticipantMessage::Remove)
                .style(theme.clear_button)
            )
            .into()
    }
}
