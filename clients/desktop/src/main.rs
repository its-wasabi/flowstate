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
                [0.0, 12.0],
                [1.0, 11.0],
                [2.0, 13.0],
                [3.0, 14.0],
                [4.0, 18.0],
                [5.0, 21.0],
                [6.0, 23.0],
                [7.0, 24.0],
            ],
            &[
                [0.0, 1.0],
                [1.0, 1.0],
                [2.0, 12.0],
                [3.0, 12.0],
                [4.0, 12.0],
                [5.0, 11.0],
                [6.0, 13.0],
                [7.0, 14.0],
            ],
        );
        let chart2 = chart::line2diff::ChartLine2diff::new(
            "idk2",
            &[
                [0.0, 12.0],
                [1.0, 11.0],
                [2.0, 13.0],
                [3.0, 14.0],
                [4.0, 18.0],
                [5.0, 21.0],
                [6.0, 23.0],
                [7.0, 24.0],
            ],
            &[
                [0.0, 1.0],
                [1.0, 1.0],
                [2.0, 12.0],
                [3.0, 12.0],
                [4.0, 12.0],
                [5.0, 11.0],
                [6.0, 13.0],
                [7.0, 14.0],
            ],
        );

        ui.columns(2, |cols| {
            chart.show_plot(&mut cols[0]);
            chart2.show_plot(&mut cols[1]);
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
