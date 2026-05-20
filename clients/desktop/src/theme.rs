use egui::{
    Color32, Stroke, Visuals,
    style::{Selection, WidgetVisuals, Widgets},
};

pub const TOP_BAR_HEIGHT: f32 = 18.0;
pub const NAV_MIN_WIDTH: f32 = 210.0;

pub const FG: Color32 = Color32::WHITE;
pub const BG: Color32 = Color32::BLACK;
pub const ASIDE_BG: Color32 = Color32::from_gray(40);

pub const BORDER: Color32 = Color32::from_gray(100);

pub const ACTIVE: Color32 = Color32::from_gray(100);
pub const BUTTON_ACTIVE_FG: Color32 = Color32::BLACK;
pub const HOVER: Color32 = Color32::from_gray(80);

pub fn apply(cc: &eframe::CreationContext) {
    let mut v = Visuals::dark();
    v.panel_fill = BG;
    v.window_shadow = egui::Shadow::NONE;
    v.popup_shadow = egui::Shadow::NONE;
    v.window_stroke = Stroke::new(1.0, BORDER);

    v.selection = Selection {
        bg_fill: ASIDE_BG,
        stroke: Stroke::new(1.0, FG),
    };

    let flat = |bg: Color32, text: Color32, border: Color32| WidgetVisuals {
        bg_fill: bg,
        weak_bg_fill: bg,
        bg_stroke: Stroke::new(1.0, border),
        fg_stroke: Stroke::new(1.0, text),
        corner_radius: egui::CornerRadius::ZERO,
        expansion: 0.0,
    };

    v.widgets = Widgets {
        noninteractive: WidgetVisuals {
            bg_fill: BG,
            weak_bg_fill: BG,
            bg_stroke: Stroke::new(1.0, BORDER),
            fg_stroke: Stroke::new(1.0, FG),
            corner_radius: egui::CornerRadius::ZERO,
            expansion: 0.0,
        },
        inactive: flat(BG, FG, BORDER),
        hovered: flat(HOVER, FG, BORDER),
        active: flat(ACTIVE, FG, ACTIVE),
        open: flat(ACTIVE, BUTTON_ACTIVE_FG, ACTIVE),
    };

    cc.egui_ctx.set_visuals(v);

    let mut style = (*cc.egui_ctx.global_style()).clone();

    style.spacing = egui::style::Spacing {
        item_spacing: egui::vec2(0.0, 0.0),
        window_margin: egui::Margin::ZERO,
        button_padding: egui::vec2(4.0, 4.0),
        menu_margin: egui::Margin::ZERO,
        interact_size: egui::vec2(0.0, 0.0),
        indent: 0.0,
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

    if let Some(prop) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
        prop.insert(0, "IosevkaCode".to_owned());
    } else {
        eprintln!("FAILED TO LOAD FONT");
    }

    cc.egui_ctx.set_fonts(fonts);
}

pub fn styled_square_button(ui: &mut egui::Ui, label: &str, row_height: f32) -> egui::Response {
    let margin = 4.0;
    let square_size = (row_height - (margin * 2.0)).max(0.0);

    #[allow(clippy::cast_possible_truncation)]
    egui::Frame::default()
        .outer_margin(egui::Margin::same(margin as i8))
        .show(ui, |ui| {
            ui.add_sized([square_size, square_size], egui::Button::new(label))
        })
        .inner
}
