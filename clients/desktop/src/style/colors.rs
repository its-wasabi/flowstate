pub(super) fn lerp_color(start: iced::Color, end: iced::Color, t: f32) -> iced::Color {
    assert!(
        (0.0..=1.0).contains(&t),
        "t should be in range from 0.0 to 1.0"
    );

    iced::Color::from_rgb(
        (end.r - start.r).mul_add(t, start.r),
        (end.g - start.g).mul_add(t, start.g),
        (end.b - start.b).mul_add(t, start.b),
    )
}

pub(super) fn border_color(palette: iced::theme::Palette, is_active: bool) -> iced::Color {
    if is_active {
        palette.primary
    } else {
        lerp_color(palette.background, palette.text, 0.3)
    }
}

pub(super) fn button_colors(
    palette: iced::theme::Palette,
    status: iced::widget::button::Status,
) -> (Option<iced::Color>, iced::Color) {
    match status {
        iced::widget::button::Status::Active => (None, palette.primary),
        iced::widget::button::Status::Hovered => (
            Some(lerp_color(palette.background, palette.text, 0.3)),
            palette.primary,
        ),
        iced::widget::button::Status::Pressed => (Some(palette.primary), palette.background),
        iced::widget::button::Status::Disabled => (
            Some(lerp_color(palette.background, palette.text, 0.3)),
            palette.background,
        ),
    }
}

pub(super) const fn icon_colors(palette: iced::theme::Palette, is_active: bool) -> iced::Color {
    if is_active {
        palette.background
    } else {
        palette.primary
    }
}
