use iced::*;

#[derive(Default, Debug, Clone, Copy)]
pub struct Theme {
    pub button: ButtonStyle,
    pub clear_button: ClearButtonStyle,
    pub disabled_button: DisabledButton,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct DisabledButton;

impl button::StyleSheet for DisabledButton {
    fn active(&self) -> button::Style {
        button::Style {
            text_color: [1.0, 1.0, 1.0].into(),
            background: Some(Background::Color([0.5, 0.5, 0.5].into())),
            border_radius: 3,
            ..button::Style::default()
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct ButtonStyle;

impl button::StyleSheet for ButtonStyle {
    fn active(&self) -> button::Style {
        button::Style {
            text_color: [1.0, 1.0, 1.0].into(),
            background: Some(Background::Color([0.96, 0.26, 0.21].into())),
            border_radius: 3,
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            text_color: [1.0, 1.0, 1.0].into(),
            background: Some(Background::Color([0.94, 0.33, 0.31].into())),
            border_radius: 3,
            ..button::Style::default()
        }
    }

    fn pressed(&self) -> button::Style {
        button::Style {
            text_color: [1.0, 1.0, 1.0].into(),
            background: Some(Background::Color([0.9, 0.45, 0.45].into())),
            border_radius: 3,
            ..button::Style::default()
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct ClearButtonStyle;

impl button::StyleSheet for ClearButtonStyle {
    fn active(&self) -> button::Style {
        button::Style {
            background: None,
            ..button::Style::default()
        }
    }
}
