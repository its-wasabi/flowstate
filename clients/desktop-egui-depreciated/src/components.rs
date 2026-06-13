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

impl egui::Widget for IconButton {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let size = egui::Vec2::splat(ui.available_height());
        let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());

        if ui.is_rect_visible(rect) {
            let (fg, bg, border) = resolve_button_colors(self.color, &response);

            ui.painter().rect(
                rect,
                0,
                bg,
                egui::Stroke::new(crate::appearance::BORDER_WIDTH, border),
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

impl egui::Widget for IconButtonBorderless {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
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

pub struct SelectableLabelButtonBorderless<'a> {
    color: egui::Color32,
    label: &'a str,
    selected: bool,
}

impl<'a> SelectableLabelButtonBorderless<'a> {
    pub const fn new(color: egui::Color32, label: &'a str, selected: bool) -> Self {
        Self {
            color,
            label,
            selected,
        }
    }
}

impl egui::Widget for SelectableLabelButtonBorderless<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let size = ui.available_size();
        let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());

        if ui.is_rect_visible(rect) {
            let (mut fg, mut bg, _) = resolve_button_colors(self.color, &response);

            if self.selected {
                fg = crate::appearance::BG;
                bg = self.color;
            }

            ui.painter().rect_filled(rect, 0, bg);

            let font_id = egui::TextStyle::Button.resolve(ui.style());
            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                self.label,
                font_id,
                fg,
            );
        }

        response
    }
}

pub struct TopProgressBar {
    progress: f32,
}

impl TopProgressBar {
    pub const fn new(progress: f32) -> Self {
        Self { progress }
    }
}

impl egui::Widget for TopProgressBar {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.add(
            egui::ProgressBar::new(self.progress / 100.0)
                .corner_radius(0)
                .fill(crate::appearance::FG)
                .desired_height(ui.available_height())
                .text(
                    egui::RichText::new(format!(" {}%", self.progress))
                        .size(13.0)
                        .color(crate::appearance::BORDER)
                        .strong(),
                ),
        )
    }
}

pub struct TextEditSingleLine<'a> {
    edit: &'a mut String,
    hint: &'static str,
    style: egui::TextStyle,
}

impl<'a> TextEditSingleLine<'a> {
    pub const fn new(hint: &'static str, style: egui::TextStyle, edit: &'a mut String) -> Self {
        Self { edit, hint, style }
    }
}

impl egui::Widget for TextEditSingleLine<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.add(
            egui::TextEdit::singleline(self.edit)
                .desired_rows(1)
                .desired_width(ui.available_width())
                .font(self.style)
                .frame(egui::Frame::default())
                .hint_text(self.hint),
        )
    }
}

pub struct TextEditMultiLine<'a> {
    edit: &'a mut String,
    hint: &'static str,
    style: egui::TextStyle,
}

impl<'a> TextEditMultiLine<'a> {
    pub const fn new(hint: &'static str, style: egui::TextStyle, edit: &'a mut String) -> Self {
        Self { edit, hint, style }
    }
}

impl egui::Widget for TextEditMultiLine<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.add(
            egui::TextEdit::multiline(self.edit)
                .desired_rows(1)
                .desired_width(ui.available_width())
                .font(self.style)
                .frame(egui::Frame::default())
                .hint_text(self.hint),
        )
    }
}

pub struct DragValueEdit<'a> {
    value: &'a mut u32,
    color: egui::Color32,
}

impl<'a> DragValueEdit<'a> {
    pub const fn new(color: egui::Color32, value: &'a mut u32) -> Self {
        Self { value, color }
    }
}

impl egui::Widget for DragValueEdit<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let id = ui.next_auto_id();
        let desired_size = ui.available_size();
        let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::drag());

        if response.hovered() || response.dragged() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
        }

        if ui.input(|i| i.pointer.any_pressed() || i.pointer.any_released()) {
            ui.data_mut(|d| d.remove::<f64>(id));
        }

        if response.dragged() {
            let delta = response.drag_delta().x - response.drag_delta().y;

            if delta != 0.0 {
                let mut precise_val = ui
                    .data_mut(|d| d.get_temp::<f64>(id))
                    .unwrap_or_else(|| f64::from(*self.value));

                precise_val += f64::from(delta) * 0.5;
                let bounded_val = precise_val.max(1.0);

                *self.value = bounded_val.round() as u32;

                ui.data_mut(|d| d.insert_temp(id, bounded_val));
                response.mark_changed();
            }
        }

        if ui.is_rect_visible(rect) {
            let (fg, bg, border) = resolve_button_colors(self.color, &response);

            ui.painter().rect(
                rect,
                0,
                bg,
                egui::Stroke::new(crate::appearance::BORDER_WIDTH, border),
                egui::StrokeKind::Inside,
            );

            let text = self.value.to_string();
            let font_id = egui::TextStyle::Button.resolve(ui.style());
            let galley = ui.painter().layout_no_wrap(text, font_id, fg);

            let text_pos = rect.center() - galley.size() / 2.0;
            ui.painter().galley(text_pos, galley, fg);
        }

        response
    }
}
