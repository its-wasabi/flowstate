pub struct Tasks {}

#[derive(Debug, Clone)]
pub enum TasksMessage {}

impl Tasks {
    pub const fn new() -> Self {
        Self {}
    }
}

impl crate::Display for Tasks {
    type Message = TasksMessage;

    fn update(&mut self, message: Self::Message) {
        match message {}
    }

    fn view_center(&self) -> iced::Element<'_, Self::Message> {
        iced::widget::column![
            iced::widget::text("HELLO")
                .height(crate::style::TOP_BAR_HEIGHT)
                .style(iced::widget::text::success),
            iced::widget::text("HELLO")
                .width(iced::Length::Fill)
                .center(),
            iced::widget::text("HELLO"),
        ]
        .padding(4)
        .spacing(12)
        .into()
    }

    fn view_aside(&self) -> iced::Element<'_, Self::Message> {
        iced::widget::text("ASIDE").into()
    }
}
