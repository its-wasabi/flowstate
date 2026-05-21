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

    fn top_bar(
        &self,
        core: &application::Core,
        ui: &mut egui::Ui,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let progress = core.tree.get_progress(&self.current_task)?;
        egui::Panel::top("tasks_top_bar")
            .frame(egui::Frame::default())
            .exact_size(crate::theme::TOP_BAR_HEIGHT)
            .show_inside(ui, |ui| {
                ui.add(
                    egui::ProgressBar::new(progress.progress())
                        .corner_radius(0)
                        .fill(crate::theme::FG)
                        .desired_height(ui.available_height())
                        .desired_width(ui.available_width())
                        .text(
                            egui::RichText::new(format!(
                                " [ {} / {} ]",
                                progress.completed, progress.total
                            ))
                            .color(crate::theme::BORDER),
                        ),
                );
            });

        Ok(())
    }

    fn parent_task(&mut self, core: &application::Core, ui: &mut egui::Ui) {
        if let Ok(node) = core.tree.get_node(&self.current_task) {
            ui.horizontal(|ui| {
                egui::Frame::default()
                    .outer_margin(egui::Margin::same(4))
                    .show(ui, |ui| {
                        if ui
                            .add_sized(
                                crate::theme::PARENT_BUTTON_V2,
                                egui::Button::image(crate::icons::left()),
                            )
                            .clicked()
                        {
                            if let Ok((id, _)) = core.tree.get_parent(&self.current_task) {
                                self.current_task = id;
                            } else {
                                self.current_task = automerge::ROOT;
                            }
                        }
                    });

                egui::Frame::default()
                    .outer_margin(egui::Margin::symmetric(2, 6))
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.heading(node.name);
                            ui.label(node.desc);
                        });
                    });
            });
            ui.add(egui::Separator::default().spacing(0.0));
        }
    }

    fn children(
        &mut self,
        core: &mut application::Core,
        ui: &mut egui::Ui,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let children = core.tree.get_children(&self.current_task)?;
        egui::ScrollArea::vertical().show(ui, |ui| {
            for (child_id, child_data) in children {
                Self::child(self, ui, core, child_id, child_data);
            }
        });
        Ok(())
    }

    fn child(
        &mut self,
        ui: &mut egui::Ui,
        core: &mut application::Core,
        child_id: automerge::ObjId,
        child_data: application::tree::Node,
    ) {
        egui::Frame::default()
            .outer_margin(egui::Margin::symmetric(6, 4)) // Small margin around cards
            .inner_margin(egui::Margin {
                ..Default::default()
            }) // <-- Clean zero margin so progress bar touches edges
            .corner_radius(0)
            .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
            .show(ui, |ui| {
                ui.set_width(ui.available_width());

                ui.vertical(|ui| {
                    let progress_response = ui.add(
                        egui::ProgressBar::new(child_data.task.progress())
                            .corner_radius(0)
                            .fill(crate::theme::FG)
                            .desired_height(17.0)
                            .desired_width(ui.available_width())
                            .text(
                                egui::RichText::new(format!(
                                    " [ {} / {} ]",
                                    child_data.task.completed, child_data.task.total
                                ))
                                .size(10.0)
                                .color(crate::theme::BORDER)
                                .strong(),
                            ),
                    );

                    // exactly at its bottom edge.
                    let stroke_color = ui.visuals().widgets.noninteractive.bg_stroke.color;
                    let line_y = progress_response.rect.bottom();
                    let line_left = progress_response.rect.left();
                    let line_right = progress_response.rect.right();

                    ui.painter().line_segment(
                        [
                            egui::pos2(line_left, line_y),
                            egui::pos2(line_right, line_y),
                        ],
                        egui::Stroke::new(1.0, stroke_color),
                    );

                    // --- ROW 2: Content Area (Manually padded underneath the progress bar) ---
                    egui::Frame::default()
                        // Top margin creates spacing directly below the progress bar.
                        // Bottom, left, and right use your desired inner margin.
                        .inner_margin(egui::Margin {
                            top: 4,
                            left: 4,
                            right: 4,
                            bottom: 4,
                        })
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        let response = ui.add_sized(
                                            crate::theme::CHILD_BUTTON_V2,
                                            egui::Button::image(crate::icons::right())
                                                .corner_radius(0),
                                        );
                                        if response.clicked() {
                                            self.current_task = child_id.clone();
                                        }

                                        ui.add_space(6.0);

                                        let response = ui.add_sized(
                                            crate::theme::CHILD_BUTTON_V2,
                                            egui::Button::image(crate::icons::plus())
                                                .corner_radius(0),
                                        );
                                        if response.clicked() {
                                            self.current_task = child_id.clone();
                                        }
                                        let response = ui.add_sized(
                                            crate::theme::CHILD_BUTTON_V2,
                                            egui::Button::image(crate::icons::minus())
                                                .corner_radius(0),
                                        );
                                        if response.clicked() {
                                            self.current_task = child_id.clone();
                                        }

                                        ui.add_space(6.0);

                                        let response = ui.add_sized(
                                            crate::theme::CHILD_BUTTON_V2,
                                            egui::Button::image(crate::icons::down())
                                                .corner_radius(0),
                                        );

                                        if response.clicked() {
                                            core.tree.remove(child_id.clone());
                                        }

                                        ui.add_space(6.0);

                                        let response = ui.add_sized(
                                            crate::theme::CHILD_BUTTON_V2,
                                            egui::Button::image(crate::icons::trash())
                                                .corner_radius(0),
                                        );

                                        if response.clicked() {
                                            core.tree.remove(child_id.clone());
                                        }

                                        ui.with_layout(
                                            egui::Layout::left_to_right(egui::Align::Center),
                                            |ui| {
                                                ui.add(
                                                    egui::Label::new(&child_data.name).truncate(),
                                                );
                                            },
                                        );
                                    },
                                );
                            });
                        });
                });
            });
    }
}

impl super::View for Tasks {
    fn main(
        &mut self,
        ui: &mut egui::Ui,
        core: &mut application::Core,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Self::top_bar(self, core, ui)?;
        Self::parent_task(self, core, ui);
        Self::children(self, core, ui)?;

        let min_button_height = 40.0;
        let dynamic_height = (ui.available_height() - 8.0).max(min_button_height);

        egui::Frame::default()
            .outer_margin(egui::Margin::symmetric(6, 4))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());

                let add_btn = egui::Button::new(
                    egui::RichText::new("+")
                        .color(ui.visuals().widgets.noninteractive.text_color())
                        .size(28.0),
                )
                .frame(true)
                .corner_radius(0);

                if ui
                    .add_sized([ui.available_width(), dynamic_height], add_btn)
                    .clicked()
                {
                    let new_node = core
                        .tree
                        .append_child(
                            &self.current_task,
                            application::tree::node::Node {
                                name: String::new(),
                                desc: String::new(),
                                task: application::tree::node::Progress {
                                    total: 10,
                                    completed: 0,
                                },
                            },
                        )
                        .unwrap();

                    self.current_task = new_node;
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
