mod appearance;
mod config;
mod icons;
mod stats;
mod tasks;

const UI_VERSION: (u32, u32, u32) = utils::crate_version!();

pub struct App {
    core: application::Core,
    current_tab: Tab,

    tasks: tasks::Tasks,
    stats: stats::Stats,
    config: config::Config,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum Tab {
    #[default]
    Tasks,
    Stats,
    Config,
}

pub trait View {
    /// # Errors
    /// Depends on the internal implementation
    fn main(
        &mut self,
        ui: &mut egui::Ui,
        core: &mut application::Core,
    ) -> Result<(), Box<dyn std::error::Error>>;

    /// # Errors
    /// Depends on the internal implementation
    fn aside(
        &mut self,
        ui: &mut egui::Ui,
        core: &mut application::Core,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

impl App {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            core: application::Core::new()?,
            current_tab: Tab::default(),

            tasks: tasks::Tasks::new(),
            stats: stats::Stats::new(),
            config: config::Config::new(),
        })
    }

    fn nav(&mut self, ui: &mut egui::Ui) {
        let selection = &mut ui.visuals_mut().selection;
        selection.bg_fill = egui::Color32::WHITE;
        selection.stroke = egui::Stroke::new(1.0, egui::Color32::BLACK);

        egui_extras::StripBuilder::new(ui)
            .sizes(egui_extras::Size::remainder().at_least(0.0), 3)
            .horizontal(|mut strip| {
                for (label, target) in [
                    ("TASKS", Tab::Tasks),
                    ("STATS", Tab::Stats),
                    ("CONFIG", Tab::Config),
                ] {
                    strip.cell(|ui| {
                        let button = egui::Button::selectable(self.current_tab == target, label)
                            .corner_radius(0)
                            .stroke(egui::Stroke::new(0.0, egui::Color32::TRANSPARENT));

                        if ui.add_sized(ui.available_size(), button).clicked() {
                            self.current_tab = target;
                        }
                    });
                }
            });
    }
}

impl eframe::App for App {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let current_zoom = ctx.zoom_factor();
        let min_zoom = 0.8;
        let max_zoom = 2.0;
        if current_zoom < min_zoom || current_zoom > max_zoom {
            // FIX: When zooming instead of clamping it creates flickering
            // ctx.set_zoom_factor(current_zoom.clamp(min_zoom, max_zoom));
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let full_width = ui.available_width();
        egui::Panel::left("aside")
            .frame(egui::Frame::default().fill(appearance::ASIDE_BG))
            .min_size(appearance::NAV_MIN_WIDTH)
            .max_size(full_width / 1.3)
            .show_inside(ui, |ui| {
                egui::Panel::top("tabs")
                    .frame(egui::Frame::default())
                    .exact_size(appearance::TOP_BAR_HEIGHT)
                    .show_inside(ui, |ui| {
                        self.nav(ui);
                    });

                egui::ScrollArea::vertical()
                    .content_margin(egui::Margin::ZERO)
                    .max_width(f32::INFINITY)
                    // TODO: Implement the visible rows
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());

                        match self.current_tab {
                            Tab::Tasks => self.tasks.aside(ui, &mut self.core).unwrap(),
                            Tab::Stats => self.stats.aside(ui, &mut self.core).unwrap(),
                            Tab::Config => self.config.aside(ui, &mut self.core).unwrap(),
                        }
                    })
            });

        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(crate::appearance::BG))
            .show_inside(ui, |ui| match self.current_tab {
                Tab::Tasks => self.tasks.main(ui, &mut self.core).unwrap(),
                Tab::Stats => self.stats.main(ui, &mut self.core).unwrap(),
                Tab::Config => self.config.main(ui, &mut self.core).unwrap(),
            });
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new()?;

    Ok(eframe::run_native(
        application::APP_NAME,
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_title(application::APP_NAME)
                .with_app_id(application::APP_NAME)
                // TODO: Make that transparent configurable
                .with_transparent(true),

            ..Default::default()
        },
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            appearance::apply(cc);
            Ok(Box::new(app) as _)
        }),
    )?)
}
