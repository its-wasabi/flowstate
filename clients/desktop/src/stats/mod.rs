mod chart;

pub struct Stats {}

impl Stats {
    pub fn new() -> Self {
        Self {}
    }
}

impl super::View for Stats {
    fn main(
        &mut self,
        ui: &mut egui::Ui,
        core: &mut application::Core,
    ) -> Result<(), Box<dyn std::error::Error>> {
        ui.heading(egui::RichText::new("HELLO MAIN FROM STATS").monospace());
        ui.heading(egui::RichText::new("HELLO MAIN FROM STATS"));
        Ok(())
    }

    fn aside(
        &mut self,
        ui: &mut egui::Ui,
        core: &mut application::Core,
    ) -> Result<(), Box<dyn std::error::Error>> {
        ui.centered_and_justified(|ui| {
            ui.heading("empty");
        });

        Ok(())
    }
}
