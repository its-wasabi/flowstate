use egui::{Color32, Image, Response, Ui, Vec2};

pub trait IconButtonExt {
    fn icon_button(&mut self, size: Vec2, image: Image<'static>, color: Color32) -> Response;

    fn icon_button_borderless(
        &mut self,
        size: Vec2,
        image: Image<'static>,
        color: Color32,
    ) -> Response;

    fn selectable_icon_button_borderless(
        &mut self,
        size: Vec2,
        color: Color32,
        selected: bool,
        label: &str,
    ) -> Response;
}

impl IconButtonExt for Ui {
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

    fn selectable_icon_button_borderless(
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
}

pub trait ProgressBarExt {
    fn top_progress_bar(&mut self, id: &'static str, progress: f32) -> Response;
    fn progress_bar(&mut self, progress: f32) -> Response;
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

impl EditExt for Ui {
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
            egui::Vec2::new(
                crate::appearance::CHILD_BUTTON * 2.0,
                self.available_height(),
            ),
            egui::DragValue::new(edit).range(1..=u32::MAX),
        )
    }
}
