mod tree;

pub struct Tasks {
    current_task: automerge::ObjId,
    tree_state: tree::TreeState,
}

impl Tasks {
    pub const fn new() -> Self {
        Self {
            current_task: automerge::ROOT,
            tree_state: tree::TreeState::new(),
        }
    }
}

impl super::View for Tasks {
    fn main(
        &mut self,
        ui: &mut egui::Ui,
        core: &mut application::Core,
    ) -> Result<(), Box<dyn std::error::Error>> {
        egui::Panel::top("tasks_top_bar")
            .frame(egui::Frame::default())
            .exact_size(crate::theme::TOP_BAR_HEIGHT)
            .show_inside(ui, |ui| {
                ui.add(
                    egui::ProgressBar::new(
                        0.2,
                        // node_data.task_completed as f32 / node_data.task_total as f32,
                    )
                    .corner_radius(0)
                    .fill(crate::theme::FG)
                    .desired_height(ui.available_height())
                    .desired_width(ui.available_width())
                    .text(egui::RichText::new("[ 12 / 32 ]").color(crate::theme::BG)),
                );
            });

        if let Ok(node) = core.tree.get_node(&self.current_task) {
            ui.horizontal(|ui| {
                if crate::theme::styled_square_button(ui, "<", 50.0).clicked() {
                    if let Ok((id, _)) = core.tree.get_parent(&self.current_task) {
                        self.current_task = id;
                    } else {
                        self.current_task = automerge::ROOT;
                    }
                };

                egui::Frame::default()
                    .outer_margin(egui::Margin::symmetric(0, 6))
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.heading(node.name);
                            ui.label(node.desc);
                        });
                    });
            });

            ui.add(egui::Separator::default().spacing(0.0));
        }

        let children = core.tree.get_children(&self.current_task)?;
        egui::ScrollArea::vertical().show(ui, |ui| {
            for child_node_data in children {
                ui.label(child_node_data.1.name);
            }
        });

        Ok(())
    }

    fn aside(
        &mut self,
        ui: &mut egui::Ui,
        core: &mut application::Core,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.tree_state.show(ui, &core.tree, &mut self.current_task);
        Ok(())
    }
}
