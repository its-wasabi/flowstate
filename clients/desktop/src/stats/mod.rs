mod chart;

pub struct Stats {}

impl Stats {
    pub fn new() -> Self {
        Self {}
    }
}

impl super::View for Stats {
    fn main(&mut self, ui: &mut egui::Ui, core: &mut application::Core) {
        ui.heading("HELLO MAIN FROM STATS");
    }

    fn aside(&mut self, ui: &mut egui::Ui, core: &mut application::Core) {
        ui.heading("HELLO FROM ASIDE OF STATISTICS");
        ui.label("yess size");
    }
}
