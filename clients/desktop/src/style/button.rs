pub fn tab_button_style(
    is_active: bool,
) -> impl Fn(&iced::Theme, iced::widget::button::Status) -> iced::widget::button::Style {
    move |theme, status| {
        let palette = theme.palette();
        let (background, text_color) = super::colors::button_colors(
            palette,
            if is_active {
                iced::widget::button::Status::Pressed
            } else {
                status
            },
        );
        let background = background.map(std::convert::Into::into);

        iced::widget::button::Style {
            background,
            text_color,
            border: iced::Border {
                color: iced::Color::TRANSPARENT,
                width: 0.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        }
    }
}

pub fn button_with_icon(
    border: bool,
) -> (
    impl Fn(&iced::Theme, iced::widget::button::Status) -> iced::widget::button::Style,
    impl Fn(&iced::Theme, iced::widget::svg::Status) -> iced::widget::svg::Style,
) {
    let button_status =
        std::rc::Rc::new(std::cell::Cell::new(iced::widget::button::Status::Active));
    let svg_status = button_status.clone();

    let btn = move |theme: &iced::Theme, status: iced::widget::button::Status| {
        let is_active = status == iced::widget::button::Status::Pressed;
        button_status.set(status);
        let palette = theme.palette();
        let (background, text_color) = super::colors::button_colors(palette, status);
        let background = background.map(std::convert::Into::into);
        let border_color = super::colors::border_color(palette, is_active);

        iced::widget::button::Style {
            background,
            text_color,
            border: iced::Border {
                color: if border {
                    border_color
                } else {
                    iced::Color::TRANSPARENT
                },
                width: if border { super::BORDER_WIDTH } else { 0.0 },
                radius: iced::border::Radius::new(0),
            },
            snap: true,
            ..Default::default()
        }
    };

    let icon = move |theme: &iced::Theme, _: iced::widget::svg::Status| {
        let is_active = (svg_status.get() == iced::widget::button::Status::Pressed);
        let palette = theme.palette();
        let icon_color = super::colors::icon_colors(palette, is_active);

        iced::widget::svg::Style {
            color: Some(icon_color),
        }
    };

    (btn, icon)
}
