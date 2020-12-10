mod style;

// use enigo::{Enigo, MouseButton, MouseControllable};
use iced::{
    button, scrollable, slider, text_input, window, Align, Button, Checkbox, Column, Container,
    Element, Length, ProgressBar, Radio, Row, Sandbox, Scrollable, Settings, Slider, Space, Text,
    TextInput,
};

pub fn main() -> iced::Result {
    println!("Hello, world!");
    // let mut enigo = Enigo::new();
    // enigo.mouse_move_to(500, 200);
    // enigo.mouse_click(MouseButton::Left);

    Styling::run(Settings {
        window: window::Settings {
            size: (300, 400),
            ..Default::default()
        },
        ..Default::default()
    })
}

#[derive(Default)]
struct Styling {
    theme: style::Theme,
    scroll: scrollable::State,
    input: text_input::State,
    input_value: String,
    button: button::State,
    slider: slider::State,
    slider_value: f32,
    toggle_value: bool,
}

#[derive(Debug, Clone)]
enum Message {
    ThemeChanged(style::Theme),
    InputChanged(String),
    ButtonPressed,
    SliderChanged(f32),
    CheckboxToggled(bool),
}

impl Sandbox for Styling {
    type Message = Message;

    fn new() -> Self {
        Styling::default()
    }

    fn title(&self) -> String {
        String::from("Mouse Clicker")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ThemeChanged(theme) => self.theme = theme,
            Message::InputChanged(value) => self.input_value = value,
            Message::ButtonPressed => (),
            Message::SliderChanged(value) => self.slider_value = value,
            Message::CheckboxToggled(value) => self.toggle_value = value,
        }
    }

    fn view(&mut self) -> Element<Message> {
        let choose_theme = style::Theme::ALL.iter().fold(
            Column::new().spacing(10).push(Text::new("Choose a theme:")),
            |column, theme| {
                column.push(
                    Radio::new(
                        *theme,
                        &format!("{:?}", theme),
                        Some(self.theme),
                        Message::ThemeChanged,
                    )
                    .style(self.theme),
                )
            },
        );

        let text_input = TextInput::new(
            &mut self.input,
            "Type something...",
            &self.input_value,
            Message::InputChanged,
        )
        .padding(10)
        .size(20)
        .style(self.theme);

        let button = Button::new(&mut self.button, Text::new("Submit"))
            .padding(10)
            .on_press(Message::ButtonPressed)
            .style(self.theme);

        let slider = Slider::new(
            &mut self.slider,
            0.0..=100.0,
            self.slider_value,
            Message::SliderChanged,
        )
        .style(self.theme);

        let progress_bar = ProgressBar::new(0.0..=100.0, self.slider_value).style(self.theme);

        let scrollable = Scrollable::new(&mut self.scroll)
            .width(Length::Fill)
            .height(Length::Units(100))
            .style(self.theme)
            .push(Text::new("Scroll me!"))
            .push(Space::with_height(Length::Units(800)))
            .push(Text::new("You did it!"));

        let checkbox = Checkbox::new(self.toggle_value, "Toggle me!", Message::CheckboxToggled)
            .width(Length::Fill)
            .style(self.theme);

        let content = Column::new()
            .spacing(20)
            .padding(20)
            .max_width(600)
            .push(choose_theme)
            .push(Row::new().spacing(10).push(text_input).push(button))
            .push(slider)
            .push(progress_bar)
            .push(
                Row::new()
                    .spacing(10)
                    .height(Length::Units(100))
                    .align_items(Align::Center)
                    .push(scrollable)
                    .push(checkbox),
            );

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(self.theme)
            .into()
    }
}
