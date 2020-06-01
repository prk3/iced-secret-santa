use iced::*;
use iced_native;
use rand::thread_rng;
use rand::seq::SliceRandom;

mod icon;
mod theme;
mod participant;
mod result_row;
mod clip;

use theme::Theme;
use participant::Participant;
use result_row::{ResultRow, ResultRowMessage};

fn main() {
    let mut settings = Settings::default();
    settings.window.size = (800, 600);
    SecretSanta::run(settings);
}

#[derive(Debug)]
enum SecretSanta {
    Input(InputState),
    Result(ResultState),
}

#[derive(Debug, Default)]
struct InputState {
    participant_input: text_input::State,
    participant_input_value: String,
    participants: Vec<Participant>,
    add_button: button::State,
    generate_button: button::State,
    scrollable_state: scrollable::State,
}

#[derive(Debug, Default)]
struct ResultState {
    back_button: button::State,
    results: Vec<ResultRow>,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    ParticipantAdded,
    ParticipantRemoved(usize),
    EventOccurred(iced_native::Event),
    CopyUrl(String),
    Generate,
    Back,
}

impl Application for SecretSanta {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            SecretSanta::Input(InputState::default()),
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Secret Santa")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match self {
            SecretSanta::Input(state) => {
                match message {
                    Message::InputChanged(value) => {
                        state.participant_input_value = value;
                    }
                    Message::ParticipantAdded => {
                        let trimmed_name = state.participant_input_value.trim();

                        state.participants.push(Participant::new(String::from(trimmed_name)));
                        state.participant_input_value.clear();
                    }
                    Message::ParticipantRemoved(index) => {
                        state.participants.remove(index);
                    }
                    Message::Generate => {
                        let mut result_state = ResultState::default();
                        result_state.results = generate_results(&state.participants);
                        *self = SecretSanta::Result(result_state);
                    }
                    Message::EventOccurred(e) => {
                        use iced_native::input::keyboard::Event::Input;
                        use iced_native::input::keyboard::KeyCode;
                        use iced_native::input::ButtonState;
                        use iced_native::Event::Keyboard;

                        let trimmed_name = state.participant_input_value.trim();

                        if let Keyboard(Input { state: key_state, modifiers: _m, key_code }) = e {
                            if state.participant_input.is_focused()
                                && key_state == ButtonState::Pressed
                                && key_code == KeyCode::Enter
                                && is_name_allowed(trimmed_name, &state.participants) {

                                state.participants.push(Participant::new(String::from(trimmed_name)));
                                state.participant_input_value.clear();
                            }
                        }
                    }
                    _ => {}
                }
            }
            SecretSanta::Result(state) => {
                match message {
                    Message::Back => {
                        let mut input_state = InputState::default();
                        input_state.participants = state.results.iter().map(|row| Participant::new(row.first.clone())).collect();
                        *self = SecretSanta::Input(input_state);
                    }
                    Message::CopyUrl(url) => {
                        clip::write(url).unwrap();
                    }
                    _ => {}
                }
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let theme = Theme::default();

        let contents = match self {
            SecretSanta::Input(state) => {
                let generate_row = if state.participants.len() > 1 {
                    Row::new()
                        .push(
                            Button::new(
                                &mut state.generate_button,
                                Text::new("Generate"),
                            )
                            .on_press(Message::Generate)
                            .padding(6)
                            .style(theme.button)
                        )
                } else {
                    Row::new()
                };

                let input_row = Row::new()
                    .push(
                        TextInput::new(
                            &mut state.participant_input,
                            "Next participant",
                            &state.participant_input_value,
                            Message::InputChanged,
                        )
                        .padding(6)
                    )
                    .push(Space::new(Length::Units(6), Length::Units(0)))
                    .push({
                        let trimmed_name = &state.participant_input_value.trim();

                        if is_name_allowed(trimmed_name, &state.participants) {
                            Button::new(
                                &mut state.add_button,
                                Text::new("Add participant")
                            )
                            .on_press(Message::ParticipantAdded)
                            .padding(6)
                            .style(theme.button)
                        } else {
                            Button::new(
                                &mut state.add_button,
                                Text::new("Add participant")
                            )
                            .padding(6)
                            .style(theme.disabled_button)
                        }
                    });

                let participant_rows = Scrollable::new(&mut state.scrollable_state)
                    .push(state.participants
                        .iter_mut()
                        .enumerate()
                        .fold(Column::new(), |column, (i, participant)| {
                            column.push(participant.view().map(move |_message| {
                                Message::ParticipantRemoved(i)
                            }))
                        })
                    );

                Column::new()
                    .push(input_row)
                    .push(participant_rows)
                    .push(generate_row)
                    .spacing(20)
            }
            SecretSanta::Result(state) => {
                let result_rows = state.results
                    .iter_mut()
                    .fold(Column::new(), |column, row| {
                        column.push(row.view().map(move |message| {
                            match message {
                                ResultRowMessage::Copy(url) => Message::CopyUrl(url)
                            }
                        }))
                    });

                let back_row = Row::new()
                    .push(
                        Button::new(
                            &mut state.back_button,
                            Text::new("Back"),
                        )
                        .on_press(Message::Back)
                        .style(theme.button)
                    );

                Column::new()
                    .push(result_rows)
                    .push(back_row)
                    .spacing(20)
            }
        };

        let title_row = Text::new("Secret Santa")
            .width(Length::Fill)
            .size(40)
            .horizontal_alignment(HorizontalAlignment::Center);

        let main_col = Column::new()
            .push(title_row)
            .push(contents)
            .spacing(20)
            .max_width(500);

        Container::new(main_col)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .padding(20)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced_native::subscription::events().map(Message::EventOccurred)
    }
}

fn is_name_allowed(name: &str, participants: &Vec<Participant>) -> bool {
    // not empty and not a duplicate
    name.len() > 0 && participants.iter().all(|p| p.name != *name)
}

fn generate_results(participants: &Vec<Participant>) -> Vec<ResultRow> {
    let mut shuffled_refs: Vec<&String> = participants.iter().map(|p| &p.name).collect();
    shuffled_refs.shuffle(&mut thread_rng());

    let mut rotated_refs = shuffled_refs.clone();
    rotated_refs.rotate_left(1);

    let mut results: Vec<ResultRow> = shuffled_refs.iter()
        .zip(rotated_refs.iter())
        .map(|(a, b)| ResultRow::new((**a).clone(), (**b).clone())).collect();

    results.sort_by(|a, b| a.first.cmp(&b.first));

    results
}
