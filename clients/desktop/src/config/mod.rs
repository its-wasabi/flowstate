pub struct Config {}

impl Config {
    pub const fn new() -> Self {
        Self {}
    }
}

impl super::View for Config {
    fn main(&mut self, ui: &mut egui::Ui, _core: &mut application::Core) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("System info");
            ui.label(format!(
                "APP: {}.{}.{}",
                application::APP_VERSION.0,
                application::APP_VERSION.1,
                application::APP_VERSION.2
            ));

            ui.label(format!(
                "UI: {}.{}.{}",
                super::UI_VERSION.0,
                super::UI_VERSION.1,
                super::UI_VERSION.2
            ));
        });
    }

    fn aside(&mut self, ui: &mut egui::Ui, _core: &mut application::Core) {
        ui.centered_and_justified(|ui| {
            ui.heading("empty");
        });
    }
}
