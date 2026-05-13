mod chart;

struct App {
    core: application::Core,
}

impl App {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            core: application::Core::new()?,
        })
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("HELLO").strong().size(28.0));
            if ui.button("SAVE").clicked() {
                _ = self.core.save();
            }
        });

        let chart = chart::line2diff::ChartLine2diff::new(
            "idk",
            &[
                application::analytics::Point([0.0, 0.0]),
                application::analytics::Point([1.0, 3.0]),
                application::analytics::Point([2.0, 12.0]),
                application::analytics::Point([3.0, 13.0]),
                application::analytics::Point([4.0, 14.0]),
                application::analytics::Point([5.0, 14.0]),
                application::analytics::Point([6.0, 14.0]),
                application::analytics::Point([7.0, 20.0]),
            ],
            &[
                application::analytics::Point([0.0, 0.0]),
                application::analytics::Point([1.0, 2.0]),
                application::analytics::Point([2.0, 8.0]),
                application::analytics::Point([3.0, 8.0]),
                application::analytics::Point([4.0, 8.0]),
                application::analytics::Point([5.0, 9.0]),
                application::analytics::Point([6.0, 9.0]),
                application::analytics::Point([7.0, 10.0]),
            ],
        );
        let chart2 = chart::line2diff::ChartLine2diff::new(
            "idk2",
            &[
                application::analytics::Point([0.0, 0.0]),
                application::analytics::Point([1.0, 3.0]),
                application::analytics::Point([2.0, 12.0]),
                application::analytics::Point([3.0, 13.0]),
                application::analytics::Point([4.0, 14.0]),
                application::analytics::Point([5.0, 14.0]),
                application::analytics::Point([6.0, 14.0]),
                application::analytics::Point([7.0, 20.0]),
            ],
            &[
                application::analytics::Point([0.0, 0.0]),
                application::analytics::Point([1.0, 2.0]),
                application::analytics::Point([2.0, 8.0]),
                application::analytics::Point([3.0, 8.0]),
                application::analytics::Point([4.0, 8.0]),
                application::analytics::Point([5.0, 9.0]),
                application::analytics::Point([6.0, 9.0]),
                application::analytics::Point([7.0, 10.0]),
            ],
        );

        let height = 300.0;
        let available_size = ui.available_size_before_wrap();
        let size = egui::vec2(available_size.x, height);

        ui.allocate_ui_with_layout(size, egui::Layout::top_down(egui::Align::Min), |ui| {
            ui.columns(2, |cols| {
                chart.show_plot(&mut cols[0]);
                chart2.show_plot(&mut cols[1]);
            });
        });
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new()?;

    Ok(eframe::run_native(
        application::APP_NAME,
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_title(application::APP_NAME),
            ..Default::default()
        },
        Box::new(|_cc| Ok(Box::new(app) as _)),
    )?)
}
