use egui::{
    Color32, Stroke, Visuals,
    style::{Selection, WidgetVisuals, Widgets},
};

pub const TOP_BAR_HEIGHT: f32 = 21.0;
pub const NAV_MIN_WIDTH: f32 = 210.0;

pub const PARENT_BUTTON: f32 = 40.0;
pub const PARENT_BUTTON_V2: [f32; 2] = [PARENT_BUTTON, PARENT_BUTTON];
pub const CHILD_BUTTON: f32 = 30.0;
pub const CHILD_BUTTON_V2: [f32; 2] = [CHILD_BUTTON, CHILD_BUTTON];

pub const BORDER_WIDTH: f32 = 1.2;

pub const FG: Color32 = Color32::WHITE;
pub const BG: Color32 = Color32::from_rgba_premultiplied(0, 0, 0, 255);
// TODO: Make that alpha configurable
pub const ASIDE_BG: Color32 = Color32::from_rgba_premultiplied(20, 20, 20, 160);

pub const BORDER: Color32 = Color32::from_gray(100);

pub const ACTIVE: Color32 = Color32::from_gray(255);
pub const BUTTON_ACTIVE_FG: Color32 = Color32::BLACK;
pub const HOVER: Color32 = Color32::from_gray(80);

pub fn apply(cc: &eframe::CreationContext) {
    let mut v = Visuals::dark();
    v.panel_fill = BG;
    v.window_shadow = egui::Shadow::NONE;
    v.popup_shadow = egui::Shadow::NONE;
    v.window_stroke = Stroke::new(BORDER_WIDTH, BORDER);

    v.selection = Selection {
        bg_fill: ASIDE_BG,
        stroke: Stroke::new(1.0, FG),
    };

    // Separated bg_fill (scrollbars/sliders) from weak_bg_fill (buttons)
    let flat =
        |bg_fill: Color32, weak_bg_fill: Color32, text: Color32, border: Color32| WidgetVisuals {
            bg_fill,
            weak_bg_fill,
            bg_stroke: Stroke::new(BORDER_WIDTH, border), // Fixed: replaced BOOL_WIDTH with BORDER_WIDTH
            fg_stroke: Stroke::new(1.0, text),
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
        // Arguments format: flat(bg_fill, weak_bg_fill, text_color, border_color)
        inactive: flat(Color32::from_gray(140), BG, FG, BORDER), // Scrollbar is gray, button is black
        hovered: flat(Color32::from_gray(180), HOVER, FG, BORDER), // Scrollbar gets brighter, button is dark gray
        active: flat(ACTIVE, ACTIVE, BUTTON_ACTIVE_FG, ACTIVE), // Both turn white, but button text turns black
        open: flat(ACTIVE, ACTIVE, BUTTON_ACTIVE_FG, ACTIVE),
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
        // Force scrollbars to use bg_fill configurations instead of matching text strokes
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
