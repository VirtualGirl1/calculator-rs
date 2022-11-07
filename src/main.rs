mod views;
mod styles;

use views::*;
use styles::*;

use iced::{Color, Column, Container, Element, Length, Sandbox, Scrollable, scrollable, Settings, Text, text_input, TextInput};


fn main() {
    let mut settings = Settings::default();
    settings.window.size = (200, 320);

    <Calc as Sandbox>::run(settings).unwrap()
}


struct Calc {
    current_view: View,
    prev_expressions: Vec<String>,
    expression: String,
    expression_input: text_input::State,
    prev_expr_scroll: scrollable::State,

    // views
    simple: SimpleView
}

#[derive(Debug, Clone)]
pub enum Message {
    TextChanged(String),
    Button0Pressed,
    Button1Pressed,
    Button2Pressed,
    Button3Pressed,
    Button4Pressed,
    Button5Pressed,
    Button6Pressed,
    Button7Pressed,
    Button8Pressed,
    Button9Pressed,

    ButtonEqlPressed,
    ButtonClearPressed,
    ButtonACPressed,
    ButtonPlusPressed,
    ButtonMinusPressed,
    ButtonMultPressed,
    ButtonDivPressed,
    ButtonPrctPressed,
    ButtonPointPressed,
}
#[derive(Debug, Clone)]
enum View {
    Simple,
}

impl Sandbox for Calc {
    type Message = Message;

    fn new() -> Self {
        Calc {
            current_view: View::Simple,
            prev_expressions: Vec::new(),
            expression: String::new(),
            expression_input: text_input::State::new(),
            prev_expr_scroll: scrollable::State::new(),
            simple: SimpleView::default()
        }
    }

    fn title(&self) -> String {
        String::from("Calculator")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Button0Pressed => self.expression.push('0'),
            Message::Button1Pressed => self.expression.push('1'),
            Message::Button2Pressed => self.expression.push('2'),
            Message::Button3Pressed => self.expression.push('3'),
            Message::Button4Pressed => self.expression.push('4'),
            Message::Button5Pressed => self.expression.push('5'),
            Message::Button6Pressed => self.expression.push('6'),
            Message::Button7Pressed => self.expression.push('7'),
            Message::Button8Pressed => self.expression.push('8'),
            Message::Button9Pressed => self.expression.push('9'),

            Message::TextChanged(val) => {
                self.expression = val;
            }
            _ => {}
        }
    }

    fn view(&mut self) -> Element<'_, Message> {

        // get view
        let view: Column<Message>;
        match self.current_view {
            View::Simple => {
                view = self.simple.view();
            }
        }

        Column::new()
            .push(get_previous(&mut self.prev_expr_scroll, &mut self.prev_expressions))
            .push(
                TextInput::new(
                    &mut self.expression_input,
                    "Enter expression",
                    &self.expression,
                    Message::TextChanged
                    )
                    .style(InputStyleDark)
                    .size(25)
            )
            .push(view)
            .spacing(10)
            .height(Length::Fill)
            .into()

    }

    fn background_color(&self) -> Color {
        Color::from_rgb8(0x2b, 0x2b, 0x2b)
    }
}

fn get_previous<'a>(state: &'a mut scrollable::State, prev_vec: &mut Vec<String>) -> Scrollable<'a, Message> {

    let mut return_element: Scrollable<Message> = Scrollable::new(state);
    if prev_vec.len() == 0 {
        return_element = return_element.push(
            Container::new(
                Text::new(" ")
                    .size(25)
            )
                .height(Length::Units(30))
        );
    }
    else {
        for str in prev_vec {
            return_element = return_element.push(
                Container::new(
                    Text::new(&*str)
                        .size(25)
                )
                    .height(Length::Units(30))
            );
        }
    }
    return_element
}

