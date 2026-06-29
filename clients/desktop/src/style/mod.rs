mod border;
mod button;
mod colors;

pub use border::{border, split_border};
pub use button::{button_with_icon, tab_button_style};

pub const PADDING: f32 = 4.0;
pub const BORDER_WIDTH: f32 = 1.0;

pub const TOP_BAR_HEIGHT: f32 = 20.0;
pub const DEFAULT_ASIDE_WIDTH: f32 = 280.0;

pub const BIG_BUTTON_SIZE: f32 = 48.0;
pub const SMALL_BUTTON_SIZE: f32 = 28.0;

#[derive(Clone, Copy)]
pub enum Variant {
    Default,
    Danger,
    Warn,
    Ok,
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
        background: Some(colors::lerp_color(palette.background, palette.text, 0.08).into()),
        ..Default::default()
    }
}

pub fn progress(theme: &iced::Theme) -> iced::widget::progress_bar::Style {
    let palette = theme.palette();
    iced::widget::progress_bar::Style {
        background: iced::Color::TRANSPARENT.into(),
        bar: palette.text.into(),
        border: iced::Border {
            color: iced::Color::TRANSPARENT,
            width: 0.0,
            radius: iced::border::Radius::new(0),
        },
    }
}

pub fn container(border: bool) -> impl Fn(&iced::Theme) -> iced::widget::container::Style {
    move |theme| {
        let palette = theme.palette();
        iced::widget::container::Style {
            border: iced::Border {
                color: colors::lerp_color(palette.background, palette.text, 0.3),
                width: crate::style::BORDER_WIDTH,
                radius: iced::border::radius(0),
            },
            snap: true,
            ..Default::default()
        }
    }
}

pub fn text_input(
    theme: &iced::Theme,
    status: iced::widget::text_input::Status,
) -> iced::widget::text_input::Style {
    let palette = theme.palette();
    iced::widget::text_input::Style {
        background: palette.background.into(),
        border: iced::Border {
            color: iced::Color::TRANSPARENT,
            width: 0.0,
            radius: iced::border::Radius::new(0),
        },
        icon: palette.primary,
        placeholder: colors::lerp_color(palette.background, palette.text, 0.3),
        value: palette.text,
        selection: palette.primary,
    }
}

pub fn scroll(
    _theme: &iced::Theme,
    status: iced::widget::scrollable::Status,
) -> iced::widget::scrollable::Style {
    let (width, handle) = match status {
        iced::widget::scrollable::Status::Hovered { .. } => (
            32.0,
            iced::Background::Color(iced::Color::from_rgb8(180, 180, 180)),
        ),
        iced::widget::scrollable::Status::Dragged { .. } => {
            (8.0, iced::Background::Color(iced::Color::WHITE))
        }
        iced::widget::scrollable::Status::Active { .. } => (
            8.0,
            iced::Background::Color(iced::Color::from_rgba8(255, 255, 255, 0.4)),
        ),
    };

    iced::widget::scrollable::Style {
        container: iced::widget::container::Style::default(),

        vertical_rail: iced::widget::scrollable::Rail {
            background: Some(iced::Background::Color(iced::Color::TRANSPARENT)),
            border: iced::Border::default(),
            scroller: iced::widget::scrollable::Scroller {
                background: handle,
                border: iced::Border {
                    radius: 0.0.into(),
                    width,
                    ..Default::default()
                },
            },
        },

        horizontal_rail: iced::widget::scrollable::Rail {
            background: Some(iced::Background::Color(iced::Color::TRANSPARENT)),
            border: iced::Border::default(),
            scroller: iced::widget::scrollable::Scroller {
                background: handle,
                border: iced::Border {
                    radius: 0.0.into(),
                    width,
                    ..Default::default()
                },
            },
        },

        gap: None,

        auto_scroll: iced::widget::scrollable::AutoScroll {
            background: iced::Background::Color(iced::Color::TRANSPARENT),
            border: iced::Border::default(),
            shadow: iced::Shadow::default(),
            icon: iced::Color::TRANSPARENT,
        },
    }
}
