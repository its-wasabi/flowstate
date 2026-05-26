use crate::appearance::ButtonsExt;

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
    fn main(&mut self, ui: &mut egui::Ui, core: &mut application::Core);
    fn aside(&mut self, ui: &mut egui::Ui, core: &mut application::Core);
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
        ui.spacing_mut().item_spacing = egui::Vec2::ZERO;
        ui.spacing_mut().button_padding = egui::Vec2::ZERO;

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
                        if ui
                            .selectable_button_borderless(
                                ui.available_size(),
                                egui::Color32::WHITE,
                                self.current_tab == target,
                                label,
                            )
                            .clicked()
                        {
                            self.current_tab = target;
                        }
                    });
                }
            });
    }
}

impl eframe::App for App {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.viewport().close_requested()) {
            if let Some((egui_id, obj_id)) = self.tasks.active_name_edit.take()
                && let Some(name) = ctx.data_mut(|d| d.get_temp::<String>(egui_id))
                && let Err(err) = self.core.tree.change_node_name(&obj_id, name)
            {
                eprintln!("FAILED: To commit dangling name on exit: {err:?}");
            }

            if let Some((egui_id, obj_id)) = self.tasks.active_desc_edit.take()
                && let Some(desc) = ctx.data_mut(|d| d.get_temp::<String>(egui_id))
                && let Err(err) = self.core.tree.change_node_desc(&obj_id, desc)
            {
                eprintln!("FAILED: To commit dangling desc on exit: {err:?}");
            }
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
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());

                        match self.current_tab {
                            Tab::Tasks => self.tasks.aside(ui, &mut self.core),
                            Tab::Stats => self.stats.aside(ui, &mut self.core),
                            Tab::Config => self.config.aside(ui, &mut self.core),
                        }
                    })
            });

        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(crate::appearance::BG))
            .show_inside(ui, |ui| match self.current_tab {
                Tab::Tasks => self.tasks.main(ui, &mut self.core),
                Tab::Stats => self.stats.main(ui, &mut self.core),
                Tab::Config => self.config.main(ui, &mut self.core),
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
