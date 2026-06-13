pub struct History {}

#[derive(Debug, Clone)]
pub enum HistoryMessage {}

impl History {
    pub const fn new() -> Self {
        Self {}
    }
}

impl crate::Display for History {
    type Message = HistoryMessage;

    fn update(&mut self, message: Self::Message) {
        match message {}
    }

    fn view_center(&self) -> iced::Element<'_, Self::Message> {
        iced::widget::text("HELLO CONFIG MAIN").into()
    }

    fn view_aside(&self) -> iced::Element<'_, Self::Message> {
        iced::widget::text("HELLO CONFIG ASID#E").into()
    }
}
