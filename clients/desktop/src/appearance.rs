use egui::{
    Color32, Stroke, Visuals,
    style::{Selection, WidgetVisuals, Widgets},
};

pub const FG: Color32 = Color32::WHITE;
pub const BG: Color32 = Color32::from_rgba_premultiplied(0, 0, 0, 255);
// TODO: Make that alpha configurable
pub const ASIDE_BG: Color32 = Color32::from_rgba_premultiplied(20, 20, 20, 160);
pub const BORDER: Color32 = Color32::from_gray(100);

pub const BORDER_WIDTH: f32 = 1.2;

pub const TOP_BAR_HEIGHT: f32 = 21.0;

pub const BUTTON_SMALL: f32 = 16.0;
pub const BUTTON_SMALL_V2: [f32; 2] = [BUTTON_SMALL, BUTTON_SMALL];

pub const BUTTON_MID: f32 = 30.0;
pub const BUTTON_MID_V2: [f32; 2] = [BUTTON_MID, BUTTON_MID];

pub const BUTTON_BIG: f32 = 44.0;
pub const BUTTON_BIG_V2: [f32; 2] = [BUTTON_BIG, BUTTON_BIG];

pub fn apply(cc: &eframe::CreationContext) {
    let mut v = Visuals::dark();
    v.panel_fill = BG;
    v.window_shadow = egui::Shadow::NONE;
    v.popup_shadow = egui::Shadow::NONE;
    v.window_stroke = Stroke::new(BORDER_WIDTH, BORDER);

    v.selection = Selection {
        bg_fill: FG,
        stroke: Stroke::new(1.0, BG),
    };

    let flat =
        |bg_fill: Color32, weak_bg_fill: Color32, fg: Color32, border: Color32| WidgetVisuals {
            bg_fill,
            weak_bg_fill,
            fg_stroke: Stroke::new(BORDER_WIDTH, fg),
            bg_stroke: Stroke::new(BORDER_WIDTH, border),
            corner_radius: egui::CornerRadius::ZERO,
            expansion: 0.0,
        };

    v.widgets = Widgets {
        noninteractive: WidgetVisuals {
            bg_fill: BG,
            weak_bg_fill: BG,
            bg_stroke: Stroke::new(BORDER_WIDTH, BORDER),
            fg_stroke: Stroke::new(1.0, FG),
            corner_radius: egui::CornerRadius::ZERO,
            expansion: 0.0,
        },
        inactive: flat(Color32::from_gray(140), BG, FG, BORDER),
        hovered: flat(Color32::from_gray(180), BORDER, FG, BORDER),
        active: flat(FG, FG, FG, BORDER),
        open: flat(FG, FG, FG, FG),
    };

    cc.egui_ctx.set_visuals(v);

    let mut style = (*cc.egui_ctx.global_style()).clone();

    style.interaction.resize_grab_radius_side = 12.0;
    style.interaction.resize_grab_radius_corner = 12.0;

    style.spacing = egui::style::Spacing {
        item_spacing: egui::vec2(0.0, 0.0),
        window_margin: egui::Margin::ZERO,
        button_padding: egui::vec2(4.0, 4.0),
        menu_margin: egui::Margin::ZERO,
        interact_size: egui::Vec2::new(BORDER_WIDTH, BORDER_WIDTH),
        indent: 0.0,
        scroll: egui::style::ScrollStyle {
            foreground_color: false,
            ..style.spacing.scroll
        },
        ..style.spacing
    };

    cc.egui_ctx.set_global_style(style);

    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "IosevkaCode".to_owned(),
        std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
            "../../../clients/assets/fonts/IosevkaCode-Regular.ttf"
        ))),
    );

    if let Some(vec) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
        vec.insert(0, "IosevkaCode".to_owned());
    }

    cc.egui_ctx.set_fonts(fonts);
}
