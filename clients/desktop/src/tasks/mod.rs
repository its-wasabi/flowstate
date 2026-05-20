pub struct Tasks {
    current_task: automerge::ObjId,
}

impl Tasks {
    pub const fn new() -> Self {
        Self {
            current_task: automerge::ROOT,
        }
    }
}

impl super::View for Tasks {
    fn main(&mut self, ui: &mut egui::Ui, core: &mut application::Core) {
        ui.add(
            egui::ProgressBar::new(0.4)
                .corner_radius(0)
                .fill(crate::theme::FG)
                .desired_height(crate::theme::TOP_BAR_HEIGHT)
                .desired_width(ui.available_width())
                .text(egui::RichText::new("[ 12 / 32 ]").color(crate::theme::BG)),
        );

        ui.add(egui::Separator::default().spacing(0.0));

        ui.horizontal(|ui| {
            crate::theme::styled_square_button(ui, "<", 50.0);

            egui::Frame::default()
                .outer_margin(egui::Margin::symmetric(0, 6))
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.heading("My Header");
                        ui.label("s");
                    });
                });
        });

        ui.add(egui::Separator::default().spacing(0.0));
    }

    fn aside(&mut self, ui: &mut egui::Ui, core: &mut application::Core) {
        for _ in 0..200 {
            ui.label("ASIDE");
        }
    }
}
