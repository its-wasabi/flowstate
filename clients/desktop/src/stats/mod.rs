mod chart;

pub struct Stats {
    chart_tasks: chart::line2diff::ChartLine2diff,
    chart_effort: chart::line2diff::ChartLine2diff,
}

impl Stats {
    pub fn new() -> Self {
        let x = [
            application::analytics::Point::new(1.0, 0.0),
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

        let chart_tasks = chart::line2diff::ChartLine2diff::new("chart_tasks", &x, &y);
        let chart_effort = chart::line2diff::ChartLine2diff::new("chart_effort", &x, &y);

        Self {
            chart_tasks,
            chart_effort,
        }
    }
}

impl super::View for Stats {
    fn main(
        &mut self,
        ui: &mut egui::Ui,
        _core: &mut application::Core,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let center_split = ui.available_height() / 2.0;
        let min_quarter = ui.available_height() * 0.15;
        let max_quarter = ui.available_height() * 0.85;

        egui::Panel::top("top_panel_chart_tasks")
            .frame(egui::Frame::default().fill(crate::theme::BG))
            .resizable(true)
            .default_size(center_split)
            .size_range(min_quarter..=max_quarter)
            .show_inside(ui, |ui| {
                ui.centered_and_justified(|ui| {
                    self.chart_tasks.show_plot(ui);
                });
            });

        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(crate::theme::BG))
            .show_inside(ui, |ui| {
                ui.centered_and_justified(|ui| {
                    self.chart_effort.show_plot(ui);
                });
            });

        Ok(())
    }

    fn aside(
        &mut self,
        ui: &mut egui::Ui,
        _core: &mut application::Core,
    ) -> Result<(), Box<dyn std::error::Error>> {
        ui.heading("FROM: [set here]");
        ui.heading("TO: [set here]");
        ui.label("TODO: Make it basically control panel for the graph content e.g.: some constraint toggles etc...");

        Ok(())
    }
}
