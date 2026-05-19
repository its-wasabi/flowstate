pub struct Config {}

impl Config {
    pub fn new() -> Self {
        Self {}
    }
}

impl super::View for Config {
    fn main(&mut self, ui: &mut egui::Ui, core: &mut application::Core) {
        ui.heading("HELLO MAIN (CONFIG)");
    }
    fn aside(&mut self, ui: &mut egui::Ui, core: &mut application::Core) {
        ui.heading("HELLO ASIDE (CONFIG)");
    }
}
