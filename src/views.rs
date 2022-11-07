use iced::{button, Button, Column, Row, Text};
use crate::{Length, Message};
use crate::styles::*;

pub struct SimpleView {
    button_0: button::State,
    button_1: button::State,
    button_2: button::State,
    button_3: button::State,
    button_4: button::State,
    button_5: button::State,
    button_6: button::State,
    button_7: button::State,
    button_8: button::State,
    button_9: button::State,
    button_point: button::State,
    button_clear: button::State,
    button_all_clear: button::State,
    button_plus: button::State,
    button_min: button::State,
    button_mult: button::State,
    button_div: button::State,
    button_eql: button::State,
    button_prct: button::State,
}

// views
impl SimpleView {
    pub fn view(&mut self) -> Column<Message> {
        Column::new()
            .push(
                Row::new()
                    .push(
                        Button::new(&mut self.button_clear, Text::new("AC"))
                            .on_press(Message::ButtonACPressed)
                            .width(Length::Fill)
                            .style(ButtonStyleDark)
                    )
                    .push(
                        Button::new(&mut self.button_all_clear, Text::new("C"))
                            .on_press(Message::ButtonClearPressed)
                            .width(Length::Fill)
                            .style(ButtonStyleDark)
                    )
                    .push(
                        Button::new(&mut self.button_prct, Text::new("%"))
                            .on_press(Message::ButtonPrctPressed)
                            .width(Length::Fill)
                            .style(ButtonStyleDark)
                    )
                    .push(
                        Button::new(&mut self.button_plus, Text::new("+"))
                            .on_press(Message::ButtonPlusPressed)
                            .width(Length::Fill)
                            .style(ButtonStyleDark)
                    )
                    .height(Length::Fill)
            )
            .push(
                Row::new()
                    .push(
                        Button::new(&mut self.button_7, Text::new("7"))
                            .on_press(Message::Button7Pressed)
                            .width(Length::Fill)
                            .style(ButtonStyleDark)
                    )
                    .push(
                        Button::new(&mut self.button_8, Text::new("8"))
                            .on_press(Message::Button8Pressed)
                            .width(Length::Fill)
                            .style(ButtonStyleDark)
                    )
                    .push(
                        Button::new(&mut self.button_9, Text::new("9"))
                            .on_press(Message::Button9Pressed)
                            .width(Length::Fill)
                            .style(ButtonStyleDark)
                    )
                    .push(
                        Button::new(&mut self.button_min, Text::new("-"))
                            .on_press(Message::ButtonMinusPressed)
                            .width(Length::Fill)
                            .style(ButtonStyleDark)
                    )
                    .height(Length::Fill)
            )
            .push(
                Row::new()
                    .push(
                        Button::new(&mut self.button_4, Text::new("4"))
                            .on_press(Message::Button4Pressed)
                            .width(Length::Fill)
                            .style(ButtonStyleDark)
                    )
                    .push(
                        Button::new(&mut self.button_5, Text::new("5"))
                            .on_press(Message::Button5Pressed)
                            .width(Length::Fill)
                            .style(ButtonStyleDark)
                    )
                    .push(
                        Button::new(&mut self.button_6, Text::new("6"))
                            .on_press(Message::Button6Pressed)
                            .width(Length::Fill)
                            .style(ButtonStyleDark)
                    )
                    .push(
                        Button::new(&mut self.button_div, Text::new("\u{F7}"))
                            .on_press(Message::ButtonDivPressed)
                            .width(Length::Fill)
                            .style(ButtonStyleDark)
                    )
                    .height(Length::Fill)
            )
            .push(
                Row::new()
                    .push(
                        Button::new(&mut self.button_1, Text::new("1"))
                            .on_press(Message::Button1Pressed)
                            .width(Length::Fill)
                            .style(ButtonStyleDark)
                    )
                    .push(
                        Button::new(&mut self.button_2, Text::new("2"))
                            .on_press(Message::Button2Pressed)
                            .width(Length::Fill)
                            .style(ButtonStyleDark)
                    )
                    .push(
                        Button::new(&mut self.button_3, Text::new("3"))
                            .on_press(Message::Button3Pressed)
                            .width(Length::Fill)
                            .style(ButtonStyleDark)
                    )
                    .push(
                        Button::new(&mut self.button_mult, Text::new("\u{D7}"))
                            .on_press(Message::ButtonMultPressed)
                            .width(Length::Fill)
                            .style(ButtonStyleDark)
                    )
                    .height(Length::Fill)
            )
            .push(
                Row::new()
                    .push(
                        Button::new(&mut self.button_0, Text::new("0"))
                            .on_press(Message::Button0Pressed)
                            .width(Length::FillPortion(2))
                            .style(ButtonStyleDark)
                    )
                    .push(
                        Button::new(&mut self.button_point, Text::new("."))
                            .on_press(Message::ButtonPointPressed)
                            .width(Length::FillPortion(1))
                            .style(ButtonStyleDark)
                    )
                    .push(
                        Button::new(&mut self.button_eql, Text::new("="))
                            .on_press(Message::ButtonEqlPressed)
                            .width(Length::FillPortion(1))
                            .style(ButtonStyleDark)
                    )
                    .height(Length::Fill)
            )
            .width(Length::Fill)
            .height(Length::FillPortion(3))
    }
}

// impliment default
impl Default for SimpleView {
    fn default() -> Self {
        SimpleView {
            button_0: button::State::new(),
            button_1: button::State::new(),
            button_2: button::State::new(),
            button_3: button::State::new(),
            button_4: button::State::new(),
            button_5: button::State::new(),
            button_6: button::State::new(),
            button_7: button::State::new(),
            button_8: button::State::new(),
            button_9: button::State::new(),
            button_point: button::State::new(),
            button_clear: button::State::new(),
            button_all_clear: button::State::new(),
            button_plus: button::State::new(),
            button_min: button::State::new(),
            button_mult: button::State::new(),
            button_div: button::State::new(),
            button_eql: button::State::new(),
            button_prct: button::State::new()
        }

    }
}
