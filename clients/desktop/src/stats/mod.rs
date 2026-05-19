pub struct Stats {}

impl Stats {
    pub fn new() -> Self {
        Self {}
    }
}

impl super::View for Stats {
    fn main(&mut self, ui: &mut egui::Ui, core: &mut application::Core) {
        ui.label("HELLO MAIN FROM STATS");
    }

    fn aside(&mut self, ui: &mut egui::Ui, core: &mut application::Core) {
        ui.label("HELLO FROM ASIDE OF STATISTICS");
    }
}
