use iced::{
    Application,
    Settings,
    Command,
    Container,
    Length,
    Element,
    Column,
    Row,
    Text,
    TextInput,
    text_input,
    Button,
    button,
    Checkbox,
    Background,
};
use rand::thread_rng;
use rand::seq::SliceRandom;
use base64::{encode};

fn main() {
    SecretSanta::run(Settings::default());
}

#[derive(Debug)]
enum SecretSanta {
    Input(State),
}

#[derive(Debug, Default)]
struct State {
    participant_input: text_input::State,
    participant_input_value: String,
    participants: Vec<String>,
    add_button: button::State,
    generate_button: button::State,
    reveal: bool,
    result: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    ParticipantAdded,
    ParticipantRemoved(usize),
    Generate,
    UseLinks(bool),
}

impl Application for SecretSanta {
    type Message = Message;

    fn new() -> (SecretSanta, Command<Message>) {
        (
            SecretSanta::Input(State::default()),
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Secret Santa".to_owned()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match self {
            SecretSanta::Input(state) => {
                match message {
                    Message::InputChanged(value) => {
                        state.participant_input_value = value;
                    }
                    Message::ParticipantAdded => {
                        state.participants.push(state.participant_input_value.clone());
                        state.participant_input_value = String::new();
                    }
                    Message::Generate => {
                        state.result = generate_secret_santa(&state.participants);
                    }
                    Message::UseLinks(use_links) => {
                        state.reveal = !use_links;
                    }
                    _ => {}
                }
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        match self {
            SecretSanta::Input(State {
                participant_input,
                participant_input_value,
                participants,
                add_button,
                generate_button,
                reveal,
                result,
            }) => {

                let participant_rows = participants
                    .iter()
                    .fold(Column::new(), |column, p| {
                        column.push(Text::new(p))
                    });

                let result_rows = result
                    .iter()
                    .fold(Column::new(), |column, pair| {
                        let (first, second) = pair;
                        column.push(
                            Row::new()
                                .push(Text::new(first))
                                .push(if *reveal {
                                    Text::new(second)
                                } else {
                                    Text::new(generate_reveal_link(&second))
                                })
                        )
                    });

                let main_col = Column::new()
                    .push(participant_rows)
                    .push(
                        TextInput::new(
                            participant_input,
                            "Next participant",
                            participant_input_value,
                            Message::InputChanged,
                        )
                    )
                    .push(
                        Button::new(
                            add_button,
                            Text::new("Add participant"),
                        )
                        .on_press(Message::ParticipantAdded)
                        .padding(5)
                        .background(Background::Color([0.5, 0.5, 1.0].into()))
                    )
                    .push(
                        Button::new(
                            generate_button,
                            Text::new("Generate"),
                        )
                        .on_press(Message::Generate)
                        .padding(5)
                        .background(Background::Color([0.8, 0.7, 0.0].into()))
                    )
                    .push(
                        Checkbox::new(
                            !*reveal,
                            "Use links",
                            Message::UseLinks,
                        )
                    )
                    .push(result_rows);

                Container::new(main_col)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into()
            }
        }
    }
}

fn generate_secret_santa(participants: &Vec<String>) -> Vec<(String, String)> {
    let mut ps_refs: Vec<&String> = participants.iter().map(|p| p).collect();
    ps_refs.shuffle(&mut thread_rng());

    let mut paired: Vec<(String, String)> = ps_refs
        .iter()
        .enumerate()
        .map(|(i, p)| ((*p).clone(), ps_refs[(i + 1) % ps_refs.len()].clone()))
        .collect();

    paired.sort_by(|a, b| a.0.cmp(&b.0));

    paired
}

fn generate_reveal_link(name: &String) -> String {
    let mangled_name = encode(name.as_bytes());
    let url_encoded_name = mangled_name.replace('=', "%3D");

    format!("https://duck.com/?q=base64+decode+{}", url_encoded_name)
}

