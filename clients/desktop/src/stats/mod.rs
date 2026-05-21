mod chart;

pub struct Stats {
    chart: chart::line2diff::ChartLine2diff,
}

impl Stats {
    pub fn new() -> Self {
        let x = [
            application::analytics::Point::new(1.0, 12.0),
            application::analytics::Point::new(2.0, 16.0),
            application::analytics::Point::new(3.0, 16.0),
            application::analytics::Point::new(5.0, 18.0),
            application::analytics::Point::new(7.0, 19.0),
            application::analytics::Point::new(9.0, 22.0),
        ];

        let y = [
            application::analytics::Point::new(1.0, 12.0),
            application::analytics::Point::new(2.0, 20.0),
            application::analytics::Point::new(3.0, 22.0),
            application::analytics::Point::new(5.0, 22.0),
            application::analytics::Point::new(7.0, 22.0),
            application::analytics::Point::new(9.0, 23.0),
        ];

        let chart = chart::line2diff::ChartLine2diff::new("", &x, &y);

        Self { chart }
    }
}

impl super::View for Stats {
    fn main(
        &mut self,
        ui: &mut egui::Ui,
        core: &mut application::Core,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.chart.show_plot(ui);

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
