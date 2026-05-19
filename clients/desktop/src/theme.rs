use egui::{
    Color32, Stroke, Visuals,
    style::{Selection, WidgetVisuals, Widgets},
};

pub const TEXT: Color32 = Color32::WHITE;
pub const TEXT_ON_ACTIVE: Color32 = Color32::BLACK;
pub const BG: Color32 = Color32::BLACK;
pub const BORDER: Color32 = Color32::from_gray(100);
pub const ACTIVE: Color32 = Color32::WHITE;
pub const HOVER: Color32 = Color32::from_gray(80);
pub const ASIDE_BG: Color32 = Color32::from_gray(40);

pub fn apply(cc: &eframe::CreationContext) {
    let mut v = Visuals::dark();
    v.panel_fill = BG;

    let flat = |bg: Color32, text: Color32| WidgetVisuals {
        bg_fill: bg,
        weak_bg_fill: bg,
        bg_stroke: Stroke::NONE,
        fg_stroke: Stroke::new(1.0, text),
        corner_radius: egui::CornerRadius::ZERO,
        expansion: 0.0,
    };

    v.widgets = Widgets {
        noninteractive: WidgetVisuals {
            bg_fill: BG,
            weak_bg_fill: BG,
            bg_stroke: Stroke::new(1.0, BORDER),
            fg_stroke: Stroke::new(1.0, TEXT),
            corner_radius: egui::CornerRadius::ZERO,
            expansion: 0.0,
        },
        inactive: flat(BG, TEXT),
        hovered: flat(HOVER, TEXT),
        active: flat(ACTIVE, TEXT_ON_ACTIVE),
        open: flat(ACTIVE, TEXT_ON_ACTIVE),
    };

    v.selection = Selection {
        bg_fill: ASIDE_BG,
        stroke: Stroke::new(1.0, TEXT), // white text when selected at rest
    };

    v.window_shadow = egui::Shadow::NONE;
    v.popup_shadow = egui::Shadow::NONE;
    v.window_stroke = Stroke::new(1.0, BORDER);
    cc.egui_ctx.set_visuals(v);
}
