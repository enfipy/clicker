// mod style;

// use enigo::{Enigo, MouseButton, MouseControllable};
// use iced::{
//     button, slider, text_input, window, Align, Button, Checkbox, Column, Container, Element,
//     Length, ProgressBar, Radio, Row, Sandbox, Settings, Slider, Text, TextInput,
// };

use iced::{
    executor, Align, Application, Checkbox, Column, Command, Container,
    Element, Length, Settings, Subscription, Text,
};

pub fn main() -> iced::Result {
    // println!("Hello, world!");
    // let mut enigo = Enigo::new();
    // enigo.mouse_move_to(500, 200);
    // enigo.mouse_click(MouseButton::Left);

    // Styling::run(Settings {
    //     window: window::Settings {
    //         size: (300, 400),
    //         ..Default::default()
    //     },
    //     ..Default::default()
    // })

    Events::run(Settings::default())
}

#[derive(Debug, Default)]
struct Events {
    last: Vec<iced_native::Event>,
    enabled: bool,
}

#[derive(Debug, Clone)]
enum Message {
    EventOccurred(iced_native::Event),
    Toggled(bool),
}

impl Application for Events {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Events, Command<Message>) {
        (Events::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Events - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::EventOccurred(event) => {
                println!("{:?}", event);
                self.last.push(event);

                if self.last.len() > 5 {
                    let _ = self.last.remove(0);
                }
            }
            Message::Toggled(enabled) => {
                self.enabled = enabled;
            }
        };

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.enabled {
            iced_native::subscription::events().map(Message::EventOccurred)
        } else {
            Subscription::none()
        }
    }

    fn view(&mut self) -> Element<Message> {
        let events = self.last.iter().fold(
            Column::new().spacing(10),
            |column, event| {
                column.push(Text::new(format!("{:?}", event)).size(40))
            },
        );

        let toggle = Checkbox::new(
            self.enabled,
            "Listen to runtime events",
            Message::Toggled,
        );

        let content = Column::new()
            .align_items(Align::Center)
            .spacing(20)
            .push(events)
            .push(toggle);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

// #[derive(Default)]
// struct Styling {
//     theme: style::Theme,
//     input: text_input::State,
//     input_value: String,
//     button: button::State,
//     slider: slider::State,
//     slider_value: f32,
//     toggle_value: bool,
// }

// #[derive(Debug, Clone)]
// enum Message {
//     ThemeChanged(style::Theme),
//     InputChanged(String),
//     ButtonPressed,
//     SliderChanged(f32),
//     CheckboxToggled(bool),
// }

// impl Sandbox for Styling {
//     type Message = Message;

//     fn new() -> Self {
//         Styling::default()
//     }

//     fn title(&self) -> String {
//         String::from("Mouse Clicker")
//     }

//     fn update(&mut self, message: Message) {
//         match message {
//             Message::ThemeChanged(theme) => self.theme = theme,
//             Message::InputChanged(value) => self.input_value = value,
//             Message::ButtonPressed => println!("Button pressed"),
//             Message::SliderChanged(value) => self.slider_value = value,
//             Message::CheckboxToggled(value) => self.toggle_value = value,
//         }
//     }

//     fn view(&mut self) -> Element<Message> {
//         let choose_theme = style::Theme::ALL.iter().fold(
//             Column::new().spacing(10).push(Text::new("Choose a theme:")),
//             |column, theme| {
//                 column.push(
//                     Radio::new(
//                         *theme,
//                         &format!("{:?}", theme),
//                         Some(self.theme),
//                         Message::ThemeChanged,
//                     )
//                     .style(self.theme),
//                 )
//             },
//         );

//         let text_input = TextInput::new(
//             &mut self.input,
//             "Type something...",
//             &self.input_value,
//             Message::InputChanged,
//         )
//         .padding(10)
//         .size(20)
//         .style(self.theme);

//         let button = Button::new(&mut self.button, Text::new("Submit"))
//             .padding(10)
//             .on_press(Message::ButtonPressed)
//             .style(self.theme);

//         let slider = Slider::new(
//             &mut self.slider,
//             0.0..=100.0,
//             self.slider_value,
//             Message::SliderChanged,
//         )
//         .style(self.theme);

//         let progress_bar = ProgressBar::new(0.0..=100.0, self.slider_value).style(self.theme);

//         let checkbox = Checkbox::new(self.toggle_value, "Toggle me!", Message::CheckboxToggled)
//             .width(Length::Fill)
//             .style(self.theme);

//         let content = Column::new()
//             .spacing(20)
//             .padding(20)
//             .max_width(600)
//             .push(choose_theme)
//             .push(Row::new().spacing(10).push(text_input).push(button))
//             .push(slider)
//             .push(progress_bar)
//             .push(
//                 Row::new()
//                     .spacing(10)
//                     .height(Length::Units(100))
//                     .align_items(Align::Center)
//                     .push(checkbox),
//             );

//         Container::new(content)
//             .width(Length::Fill)
//             .height(Length::Fill)
//             .center_x()
//             .center_y()
//             .style(self.theme)
//             .into()
//     }
// }
