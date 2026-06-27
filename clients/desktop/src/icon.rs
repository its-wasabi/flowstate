const RIGHT_BYTES: &[u8] = include_bytes!("../../assets/icons/right.svg");
pub fn right<Message>() -> iced::Element<'static, Message> {
    let handle = iced::widget::svg::Handle::from_memory(RIGHT_BYTES);
    iced::widget::svg(handle)
        .content_fit(iced::ContentFit::Contain)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .style(crate::style::icon)
        .into()
}

const LEFT_BYTES: &[u8] = include_bytes!("../../assets/icons/left.svg");
pub fn left<Message>() -> iced::Element<'static, Message> {
    let handle = iced::widget::svg::Handle::from_memory(LEFT_BYTES);
    iced::widget::svg(handle)
        .content_fit(iced::ContentFit::Contain)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .style(crate::style::icon)
        .into()
}

const PLUS_BYTES: &[u8] = include_bytes!("../../assets/icons/plus.svg");
pub fn plus<Message>() -> iced::Element<'static, Message> {
    let handle = iced::widget::svg::Handle::from_memory(PLUS_BYTES);
    iced::widget::svg(handle)
        .content_fit(iced::ContentFit::Contain)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .style(crate::style::icon)
        .into()
}

const MINUS_BYTES: &[u8] = include_bytes!("../../assets/icons/minus.svg");
pub fn minus<Message>() -> iced::Element<'static, Message> {
    let handle = iced::widget::svg::Handle::from_memory(MINUS_BYTES);
    iced::widget::svg(handle)
        .content_fit(iced::ContentFit::Contain)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .style(crate::style::icon)
        .into()
}
