pub fn border(theme: &iced::Theme) -> iced::widget::rule::Style {
    let palette = theme.palette();
    iced::widget::rule::Style {
        color: super::colors::border_color(palette, false),
        radius: iced::border::Radius::new(0),
        fill_mode: iced::widget::rule::FillMode::Full,
        snap: true,
    }
}

pub fn split_border(
    theme: &iced::Theme,
    status: iced_resizable_split::Status,
) -> iced_resizable_split::Style {
    let palette = theme.palette();
    let is_active = status == iced_resizable_split::Status::Dragging;
    iced_resizable_split::Style {
        divider_color: super::colors::border_color(palette, is_active),
        divider_width: super::BORDER_WIDTH,
        snap: true,
    }
}
