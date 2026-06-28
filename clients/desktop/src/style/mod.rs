mod border;
mod button;
mod colors;

pub use border::{border, split_border};
pub use button::{button_with_icon, tab_button_style};

pub const TOP_BAR_HEIGHT: f32 = 20.0;
pub const DEFAULT_ASIDE_WIDTH: f32 = 280.0;
pub const BORDER_WIDTH: f32 = 1.0;

// pub enum Variant {
//     Default,
//     Danger,
//     Warn,
//     Ok,
// }

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

pub fn icon(theme: &iced::Theme, status: iced::widget::svg::Status) -> iced::widget::svg::Style {
    let palette = theme.palette();
    iced::widget::svg::Style {
        color: Some(palette.text),
    }
}
