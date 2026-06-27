const RIGHT_BYTES: &[u8] = include_bytes!("../../assets/icons/right.svg");
const LEFT_BYTES: &[u8] = include_bytes!("../../assets/icons/left.svg");
const PLUS_BYTES: &[u8] = include_bytes!("../../assets/icons/plus.svg");
const MINUS_BYTES: &[u8] = include_bytes!("../../assets/icons/minus.svg");

fn make_icon<Message>(
    bytes: &'static [u8],
    style: impl Fn(&iced::Theme, iced::widget::svg::Status) -> iced::widget::svg::Style + 'static,
) -> iced::Element<'static, Message> {
    let handle = iced::widget::svg::Handle::from_memory(bytes);
    iced::widget::svg(handle)
        .content_fit(iced::ContentFit::Contain)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .style(style)
        .into()
}

pub fn right<Message>(
    style: impl Fn(&iced::Theme, iced::widget::svg::Status) -> iced::widget::svg::Style + 'static,
) -> iced::Element<'static, Message> {
    make_icon(RIGHT_BYTES, style)
}

pub fn left<Message>(
    style: impl Fn(&iced::Theme, iced::widget::svg::Status) -> iced::widget::svg::Style + 'static,
) -> iced::Element<'static, Message> {
    make_icon(LEFT_BYTES, style)
}

pub fn plus<Message>(
    style: impl Fn(&iced::Theme, iced::widget::svg::Status) -> iced::widget::svg::Style + 'static,
) -> iced::Element<'static, Message> {
    make_icon(PLUS_BYTES, style)
}

pub fn minus<Message>(
    style: impl Fn(&iced::Theme, iced::widget::svg::Status) -> iced::widget::svg::Style + 'static,
) -> iced::Element<'static, Message> {
    make_icon(MINUS_BYTES, style)
}
