use iced::*;

const ICONS: Font = Font::External {
    name: "Icons",
    bytes: include_bytes!("../assets/MaterialIcons-Regular.ttf"),
};

pub fn icon(unicode: char) -> Text {
    Text::new(&unicode.to_string()).font(ICONS)
}
