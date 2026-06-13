pub const TOP_BAR_HEIGHT: f32 = 20.0;
pub const DEFAULT_ASIDE_WIDTH: f32 = 280.0;
pub const BORDER_WIDTH: f32 = 1.0;

pub const DRAG_AREA: f32 = 22.0;

pub fn lerp_color(start: iced::Color, end: iced::Color, t: f32) -> iced::Color {
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

pub fn tab_button_style(
    is_active: bool,
) -> impl Fn(&iced::Theme, iced::widget::button::Status) -> iced::widget::button::Style {
    move |theme, status| {
        let palette = theme.palette();
        let base_style = iced::widget::button::Style {
            border: iced::Border {
                color: iced::Color::TRANSPARENT,
                width: 0.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        };

        match (status, is_active) {
            (_, true) | (iced::widget::button::Status::Pressed, false) => {
                iced::widget::button::Style {
                    background: Some(palette.text.into()),
                    text_color: palette.background,
                    ..base_style
                }
            }

            (iced::widget::button::Status::Hovered, false) => iced::widget::button::Style {
                background: Some(lerp_color(palette.background, palette.text, 0.3).into()),
                text_color: palette.text,
                ..base_style
            },

            (iced::widget::button::Status::Active, false) => iced::widget::button::Style {
                background: Some(iced::Color::TRANSPARENT.into()),
                text_color: palette.text,
                ..base_style
            },

            (iced::widget::button::Status::Disabled, false) => iced::widget::button::Style {
                background: Some(iced::Color::TRANSPARENT.into()),
                text_color: palette.primary,
                ..base_style
            },
        }
    }
}

pub fn default_panel(theme: &iced::Theme) -> iced::widget::container::Style {
    let palette = theme.palette();
    iced::widget::container::Style {
        background: Some(palette.background.into()),
        ..Default::default()
    }
}

pub fn accent_panel(theme: &iced::Theme) -> iced::widget::container::Style {
    let palette = theme.palette();
    iced::widget::container::Style {
        background: Some(lerp_color(palette.background, palette.text, 0.08).into()),
        ..Default::default()
    }
}

pub fn border(theme: &iced::Theme) -> iced::Border {
    let palette = theme.palette();
    iced::Border {
        color: lerp_color(palette.background, palette.text, 0.3),
        width: BORDER_WIDTH,
        radius: 0.0.into(),
    }
}

pub fn border_interactive(is_active: bool) -> impl Fn(&iced::Theme) -> iced::Border {
    move |theme| {
        let palette = theme.palette();
        iced::Border {
            color: if is_active {
                palette.text
            } else {
                lerp_color(palette.background, palette.text, 0.3)
            },
            width: BORDER_WIDTH,
            radius: 0.0.into(),
        }
    }
}
