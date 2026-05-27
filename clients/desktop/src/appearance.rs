use egui::{
    Color32, Stroke, Visuals,
    style::{Selection, WidgetVisuals, Widgets},
};

pub const TOP_BAR_HEIGHT: f32 = 21.0;
pub const NAV_MIN_WIDTH: f32 = 210.0;

pub const PARENT_BUTTON: f32 = 44.0;
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
pub const HOVER: Color32 = Color32::from_gray(80);

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
        hovered: flat(Color32::from_gray(180), HOVER, FG, BORDER),
        active: flat(ACTIVE, ACTIVE, ACTIVE, BORDER),
        open: flat(ACTIVE, ACTIVE, ACTIVE, ACTIVE),
    };

    cc.egui_ctx.set_visuals(v);

    let mut style = (*cc.egui_ctx.global_style()).clone();

    style.spacing = egui::style::Spacing {
        item_spacing: egui::vec2(0.0, 0.0),
        window_margin: egui::Margin::ZERO,
        button_padding: egui::vec2(4.0, 4.0),
        menu_margin: egui::Margin::ZERO,
        interact_size: egui::Vec2::new(BORDER_WIDTH, BORDER_WIDTH),
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

use egui::{Image, Response, Ui, Vec2};

pub trait ButtonsExt {
    fn selectable_button_borderless(
        &mut self,
        size: Vec2,
        color: Color32,
        selected: bool,
        label: &str,
    ) -> Response;

    fn icon_button(&mut self, size: Vec2, image: Image<'static>, color: Color32) -> Response;

    fn icon_button_borderless(
        &mut self,
        size: Vec2,
        image: Image<'static>,
        color: Color32,
    ) -> Response;
}

impl ButtonsExt for Ui {
    fn selectable_button_borderless(
        &mut self,
        size: Vec2,
        color: Color32,
        selected: bool,
        label: &str,
    ) -> Response {
        self.scope(|ui| {
            let hover_bg = color.gamma_multiply(0.32);
            let active_bg = color.gamma_multiply(0.40);
            let visuals = ui.visuals_mut();

            visuals.widgets.inactive.bg_fill = Color32::TRANSPARENT;
            visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
            visuals.widgets.inactive.fg_stroke.color = color;

            visuals.widgets.hovered.bg_fill = hover_bg;
            visuals.widgets.hovered.weak_bg_fill = hover_bg;
            visuals.widgets.hovered.fg_stroke.color = color;

            visuals.widgets.active.bg_fill = active_bg;
            visuals.widgets.active.weak_bg_fill = color;
            visuals.widgets.active.fg_stroke.color = Color32::BLACK;
            visuals.widgets.active.bg_stroke.color = color;

            ui.add_sized(
                size,
                egui::Button::selectable(selected, label)
                    .stroke(egui::Stroke::new(0.0, egui::Color32::TRANSPARENT)),
            )
        })
        .inner
    }

    fn icon_button(&mut self, size: Vec2, image: Image<'static>, color: Color32) -> Response {
        self.scope(|ui| {
            let hover_bg = color.gamma_multiply(0.32);
            let active_bg = color.gamma_multiply(0.40);
            let visuals = ui.visuals_mut();

            visuals.widgets.inactive.bg_fill = Color32::TRANSPARENT;
            visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
            visuals.widgets.inactive.fg_stroke.color = color;

            visuals.widgets.hovered.bg_fill = hover_bg;
            visuals.widgets.hovered.weak_bg_fill = hover_bg;
            visuals.widgets.hovered.fg_stroke.color = color;

            visuals.widgets.active.bg_fill = active_bg;
            visuals.widgets.active.weak_bg_fill = color;
            visuals.widgets.active.fg_stroke.color = Color32::BLACK;
            visuals.widgets.active.bg_stroke.color = color;

            ui.add_sized(
                size,
                egui::Button::image(image).image_tint_follows_text_color(true),
            )
        })
        .inner
    }

    fn icon_button_borderless(
        &mut self,
        size: Vec2,
        image: Image<'static>,
        color: Color32,
    ) -> Response {
        self.scope(|ui| {
            let hover_bg = color.gamma_multiply(0.32);
            let active_bg = color.gamma_multiply(0.40);
            let visuals = ui.visuals_mut();

            visuals.widgets.inactive.bg_fill = Color32::TRANSPARENT;
            visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
            visuals.widgets.inactive.fg_stroke.color = color;

            visuals.widgets.hovered.bg_fill = hover_bg;
            visuals.widgets.hovered.weak_bg_fill = hover_bg;
            visuals.widgets.hovered.fg_stroke.color = color;

            visuals.widgets.active.bg_fill = active_bg;
            visuals.widgets.active.weak_bg_fill = color;
            visuals.widgets.active.fg_stroke.color = Color32::BLACK;
            visuals.widgets.active.bg_stroke.color = color;

            ui.add_sized(
                size,
                egui::Button::image(image)
                    .image_tint_follows_text_color(true)
                    .stroke(egui::Stroke::new(0.0, egui::Color32::TRANSPARENT)),
            )
        })
        .inner
    }
}

pub trait ProgressBarExt {
    fn top_progress_bar(&mut self, id: &'static str, progress: f32) -> Response;
    fn frame_progress_bar(&mut self, progress: f32) -> Response;
}

impl ProgressBarExt for Ui {
    fn top_progress_bar(&mut self, id: &'static str, progress: f32) -> Response {
        egui::Panel::top(id)
            .frame(egui::Frame::default())
            .min_size(crate::appearance::TOP_BAR_HEIGHT)
            .show_inside(self, |ui| {
                ui.add(
                    egui::ProgressBar::new(progress / 100.0)
                        .corner_radius(0)
                        .fill(crate::appearance::FG)
                        .desired_height(ui.available_height())
                        .text(
                            egui::RichText::new(format!(" {progress}%"))
                                .color(crate::appearance::BORDER),
                        ),
                );
            })
            .response
    }

    fn frame_progress_bar(&mut self, progress: f32) -> Response {
        let response = self.add(
            egui::ProgressBar::new(progress / 100.0)
                .corner_radius(0)
                .fill(crate::appearance::FG)
                .desired_height(18.0)
                .text(
                    egui::RichText::new(format!(" {progress}%"))
                        .size(13.0)
                        .color(crate::appearance::BORDER)
                        .strong(),
                ),
        );

        self.add(egui::Separator::default().spacing(0.0));

        response
    }
}
