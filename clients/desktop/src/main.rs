// mod chart;
//
// struct TreeState {
//     selected: Option<(automerge::ObjId, application::tree::NodeData)>,
// }
//
// impl TreeState {
//     const fn new() -> Self {
//         Self { selected: None }
//     }
//
//     fn render(
//         &mut self,
//         ui: &mut egui::Ui,
//         tree: &application::tree::Tree,
//         current_node: &mut automerge::ObjId,
//     ) {
//         ui.scope(|ui| {
//             let children = tree.get_children(&automerge::ROOT).unwrap();
//             for (i, (id, node)) in children.into_iter().enumerate() {
//                 self.render_node(ui, tree, &id, &node, 0, i, current_node);
//             }
//         });
//     }
//
//     fn render_node(
//         &mut self,
//         ui: &mut egui::Ui,
//         tree: &application::tree::Tree,
//         id: &automerge::ObjId,
//         node: &application::tree::NodeData,
//         depth: usize,
//         index: usize,
//         current_node: &mut automerge::ObjId,
//     ) {
//         let children = tree.get_children(id).unwrap();
//         let is_selected = self
//             .selected
//             .as_ref()
//             .map(|(sid, _)| sid == id)
//             .unwrap_or(false);
//         let label = node.name.as_str();
//
//         ui.push_id((depth, index), |ui| {
//             if children.is_empty() {
//                 if ui.selectable_label(is_selected, label).clicked() {
//                     self.selected = Some((id.clone(), node.clone()));
//                     *current_node = id.clone();
//                 }
//             } else {
//                 let collapsing_id = ui.make_persistent_id((depth, index, "open"));
//                 let state = egui::collapsing_header::CollapsingState::load_with_default_open(
//                     ui.ctx(),
//                     collapsing_id,
//                     false,
//                 );
//
//                 let node_clone = node.clone();
//                 let id_clone = id.clone();
//
//                 state
//                     .show_header(ui, |ui| {
//                         if ui.selectable_label(is_selected, label).clicked() {
//                             self.selected = Some((id_clone.clone(), node_clone.clone()));
//                             *current_node = id_clone.clone(); // <-- set it
//                         }
//                     })
//                     .body(|ui| {
//                         for (i, (child_id, child_node)) in children.into_iter().enumerate() {
//                             self.render_node(
//                                 ui,
//                                 tree,
//                                 &child_id,
//                                 &child_node,
//                                 depth + 1,
//                                 i,
//                                 current_node,
//                             );
//                         }
//                     });
//             }
//         });
//     }
// }
//
// struct App {
//     core: application::Core,
//     current_node: automerge::ObjId,
//     tree_state: TreeState,
// }
//
// impl App {
//     fn new() -> Result<Self, Box<dyn std::error::Error>> {
//         Ok(Self {
//             core: application::Core::new()?,
//             current_node: automerge::ROOT,
//             tree_state: TreeState::new(),
//         })
//     }
// }
//
// impl eframe::App for App {
//     fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
//         egui::Panel::left("tree_panel")
//             // TODO: Make that resizable
//             .resizable(false)
//             .default_size(220.0)
//             .show_inside(ui, |ui| {
//                 egui::ScrollArea::vertical().show(ui, |ui| {
//                     self.tree_state
//                         .render(ui, &self.core.tree, &mut self.current_node);
//                 });
//             });
//
//         egui::CentralPanel::default()
//             .frame(egui::Frame::default())
//             .show_inside(ui, |ui| {
//                 if let Ok(node) = self.core.tree.get_node(&self.current_node) {
//                     let header_response = ui.scope(|ui| {
//                         ui.spacing_mut().item_spacing = egui::Vec2::ZERO;
//
//                         ui.horizontal(|ui| {
//                             let header_height =
//                                 20.0 + ui.text_style_height(&egui::TextStyle::Heading) + 12.0;
//
//                             if self.current_node != automerge::ROOT {
//                                 ui.scope(|ui| {
//                                     let w = &mut ui.visuals_mut().widgets;
//                                     w.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;
//                                     w.inactive.bg_stroke = egui::Stroke::NONE;
//                                     w.hovered.weak_bg_fill = egui::Color32::from_gray(50);
//                                     w.hovered.bg_stroke = egui::Stroke::NONE;
//                                     w.active.weak_bg_fill = egui::Color32::from_gray(35);
//                                     w.active.bg_stroke = egui::Stroke::NONE;
//
//                                     let btn = ui.add_sized(
//                                         egui::vec2(32.0, header_height),
//                                         egui::Button::new("<").corner_radius(0),
//                                     );
//
//                                     ui.painter().line_segment(
//                                         [btn.rect.right_top(), btn.rect.right_bottom()],
//                                         ui.visuals().widgets.noninteractive.bg_stroke,
//                                     );
//
//                                     if btn.clicked() {
//                                         self.current_node = self
//                                             .core
//                                             .tree
//                                             .get_parent(&self.current_node)
//                                             .map(|(id, _)| id)
//                                             .unwrap_or(automerge::ROOT);
//                                     }
//                                 });
//                             }
//
//                             ui.vertical(|ui| {
//                                 ui.add(
//                                     egui::ProgressBar::new(
//                                         node.task_completed as f32 / node.task_total as f32,
//                                     )
//                                     .corner_radius(0)
//                                     .fill(egui::Color32::WHITE),
//                                 );
//                                 egui::Frame::default().show(ui, |ui| ui.heading(node.name));
//                             });
//                         });
//                     });
//
//                     let rect = header_response.response.rect;
//                     ui.painter().line_segment(
//                         [rect.left_bottom(), rect.right_bottom()],
//                         ui.visuals().widgets.noninteractive.bg_stroke,
//                     );
//                 }
//
//                 egui::ScrollArea::vertical().show(ui, |ui| {
//                     if let Ok(children) = self.core.tree.get_children(&self.current_node) {
//                         for (id, node) in children {
//                             let inner_response = egui::Frame::group(ui.style())
//                                 .outer_margin(egui::Margin {
//                                     top: 6,
//                                     bottom: 0,
//                                     right: 6,
//                                     left: 6,
//                                 })
//                                 .corner_radius(0)
//                                 .show(ui, |ui| {
//                                     ui.vertical(|ui| {
//                                         ui.label(&node.name);
//                                         ui.add(
//                                             egui::ProgressBar::new(
//                                                 node.task_completed as f32 / node.task_total as f32,
//                                             )
//                                             .desired_width(ui.available_width())
//                                             .corner_radius(0)
//                                             .fill(egui::Color32::WHITE),
//                                         );
//                                     })
//                                 });
//
//                             let rect = inner_response.response.rect;
//
//                             let click_id = ui.id().with(id.clone());
//                             let click_response = ui.interact(rect, click_id, egui::Sense::click());
//
//                             if click_response.clicked() {
//                                 self.current_node = id;
//                                 println!("CLICKED");
//                             }
//                         }
//                     }
//
//                     let add_response = egui::Frame::group(ui.style())
//                         .outer_margin(egui::Margin {
//                             top: 6,
//                             bottom: 6,
//                             left: 6,
//                             right: 6,
//                         })
//                         .corner_radius(0)
//                         .show(ui, |ui| {
//                             ui.set_min_width(ui.available_width());
//                             ui.set_min_height(32.0);
//                             ui.centered_and_justified(|ui| ui.label(egui::RichText::new("ADD")))
//                         });
//
//                     if ui
//                         .interact(
//                             add_response.response.rect,
//                             ui.id().with("add_node"),
//                             egui::Sense::click(),
//                         )
//                         .clicked()
//                     {
//                         if let Ok(id) = self.core.tree.append_child(
//                             &self.current_node,
//                             application::tree::NodeData {
//                                 name: "APP ADDED".into(),
//                                 desc: "KJDFLKSDJFLKSDJFKL".into(),
//                                 task_completed: 12,
//                                 task_total: 30,
//                             },
//                         ) {
//                             self.current_node = id;
//                         };
//                     }
//                 })
//             });
//     }
// }
//
// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let app = App::new()?;
//     Ok(eframe::run_native(
//         application::APP_NAME,
//         eframe::NativeOptions {
//             viewport: egui::ViewportBuilder::default().with_title(application::APP_NAME),
//             ..Default::default()
//         },
//         Box::new(|_cc| Ok(Box::new(app) as _)),
//     )?)
// }

mod config;
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
                            .add_sized(
                                ui.available_size(),
                                egui::Button::selectable(self.current_tab == target, label)
                                    .corner_radius(0),
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
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let full_width = ui.available_width();
        egui::Panel::left("aside")
            .frame(egui::Frame::default())
            .max_size(full_width / 1.2)
            .show_inside(ui, |ui| {
                egui::Panel::top("nav")
                    .frame(egui::Frame::default())
                    .show_inside(ui, |ui| {
                        self.nav(ui);
                    });

                egui::ScrollArea::vertical()
                    .content_margin(egui::Margin::symmetric(4, 2))
                    .max_width(f32::INFINITY)
                    // TODO: Implement the visible rows
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
            .frame(egui::Frame::default())
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
            viewport: egui::ViewportBuilder::default().with_title(application::APP_NAME),
            ..Default::default()
        },
        Box::new(|_cc| Ok(Box::new(app) as _)),
    )?)
}
