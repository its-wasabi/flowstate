// TODO: Implement proper egui::Widget instead of wrapping egui::Button
// TODO: Rename itself to components after implementing proper Widgets

use egui::{Color32, Response, Stroke, Widget};

fn resolve_button_colors(
    color: egui::Color32,
    response: &egui::Response,
) -> (egui::Color32, egui::Color32, egui::Color32) {
    match &response {
        response if response.is_pointer_button_down_on() => (crate::appearance::BG, color, color),
        response if response.hovered() => (
            color,
            color.gamma_multiply(0.32),
            color.gamma_multiply(0.70),
        ),
        _ => (color, crate::appearance::BG, crate::appearance::BORDER),
    }
}

pub struct IconButton {
    color: egui::Color32,
    icon: crate::icons::Icon,
}

impl IconButton {
    pub const fn new(color: egui::Color32, icon: crate::icons::Icon) -> Self {
        Self { color, icon }
    }
}

impl Widget for IconButton {
    fn ui(self, ui: &mut egui::Ui) -> Response {
        let size = egui::Vec2::splat(ui.available_height());
        let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());

        if ui.is_rect_visible(rect) {
            let (fg, bg, border) = resolve_button_colors(self.color, &response);

            ui.painter().rect(
                rect,
                0,
                bg,
                Stroke::new(crate::appearance::BORDER_WIDTH, border),
                egui::StrokeKind::Inside,
            );

            let icon_margin = crate::appearance::BUTTON_ICON_MARGIN;
            let icon_size =
                egui::Vec2::new(rect.width() - icon_margin, rect.height() - icon_margin);
            let icon_rect = egui::Rect::from_center_size(rect.center(), icon_size);
            self.icon.image(icon_size).tint(fg).paint_at(ui, icon_rect);
        }

        response
    }
}

pub struct IconButtonBorderless {
    color: egui::Color32,
    icon: crate::icons::Icon,
}

impl IconButtonBorderless {
    pub const fn new(color: egui::Color32, icon: crate::icons::Icon) -> Self {
        Self { color, icon }
    }
}

impl Widget for IconButtonBorderless {
    fn ui(self, ui: &mut egui::Ui) -> Response {
        let size = ui.available_size();
        let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());

        if ui.is_rect_visible(rect) {
            let (fg, bg, _) = resolve_button_colors(self.color, &response);

            ui.painter().rect_filled(rect, 0, bg);

            let icon_margin = crate::appearance::BUTTON_ICON_MARGIN;
            let icon_size = egui::Vec2::new(
                rect.width().min(rect.height()) - icon_margin,
                rect.height() - icon_margin,
            );
            let icon_rect = egui::Rect::from_center_size(rect.center(), icon_size);
            self.icon.image(icon_size).tint(fg).paint_at(ui, icon_rect);
        }

        response
    }
}

pub trait IconButtonExt {
    fn selectable_icon_button_borderless(
        &mut self,
        size: impl Into<egui::Vec2>,
        color: Color32,
        label: &str,
        selected: bool,
    ) -> Response;
}

impl IconButtonExt for egui::Ui {
    fn selectable_icon_button_borderless(
        &mut self,
        size: impl Into<egui::Vec2>,
        color: Color32,
        label: &str,
        selected: bool,
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
}

pub trait ProgressBarExt {
    fn top_progress_bar(&mut self, id: &'static str, progress: f32) -> Response;
    fn progress_bar(&mut self, progress: f32) -> Response;
}

impl ProgressBarExt for egui::Ui {
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

    fn progress_bar(&mut self, progress: f32) -> Response {
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

pub trait EditExt {
    fn edit_text_singleline(&mut self, edit: &mut String, hint: &str) -> Response;

    fn edit_text_multiline(&mut self, edit: &mut String, hint: &str) -> Response;

    fn edit_drag_value(&mut self, edit: &mut u32) -> Response;
}

impl EditExt for egui::Ui {
    fn edit_text_singleline(&mut self, edit: &mut String, hint: &str) -> Response {
        self.add(
            egui::TextEdit::singleline(edit)
                .desired_rows(1)
                .desired_width(self.available_width())
                .font(egui::TextStyle::Heading)
                .frame(egui::Frame::default())
                .hint_text(hint),
        )
    }

    fn edit_text_multiline(&mut self, edit: &mut String, hint: &str) -> Response {
        self.add(
            egui::TextEdit::multiline(edit)
                .desired_rows(1)
                .desired_width(self.available_width())
                .font(egui::TextStyle::Body)
                .frame(egui::Frame::default())
                .hint_text(hint),
        )
    }

    fn edit_drag_value(&mut self, edit: &mut u32) -> Response {
        let visuals = self.visuals_mut();
        visuals.widgets.active.fg_stroke.color = crate::appearance::BG;

        self.add_sized(
            egui::Vec2::new(crate::appearance::BUTTON_MID * 2.0, self.available_height()),
            egui::DragValue::new(edit).range(1..=u32::MAX),
        )
    }
}
