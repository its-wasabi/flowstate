// TODO: If element has no child instead of displaying empty scroll area make some coll control
// central element for that node without needing to expanding panel for more advanced things
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
        Self::top_bar(self, core, ui)?;
        Self::parent_task(self, core, ui);

        egui::Panel::bottom("panel_add_button")
            .frame(egui::Frame::default())
            .show_inside(ui, |ui| {
                Self::add_button(self, ui, core);
            });

        egui::CentralPanel::default()
            .frame(egui::Frame::default())
            .show_inside(ui, |ui| Self::children(self, ui, core))
            .inner?;

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

impl Tasks {
    #[inline]
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

    #[inline]
    fn parent_task(&mut self, core: &application::Core, ui: &mut egui::Ui) {
        if let Ok(node) = core.tree.get_node(&self.current_task) {
            egui::Panel::top("panel_parent_task")
                .frame(egui::Frame::default())
                .show_inside(ui, |ui| {
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
                });
        }
    }

    #[inline]
    fn add_button(&mut self, ui: &mut egui::Ui, core: &mut application::Core) {
        egui::Frame::default()
            .inner_margin(egui::Margin::same(4))
            .show(ui, |ui| {
                let button_size = egui::vec2(ui.available_width(), crate::theme::CHILD_BUTTON);
                let add_btn = egui::Button::image(crate::icons::add());
                if ui.add_sized(button_size, add_btn).clicked()
                    && let Ok(new_node) = core.tree.append_child(
                        &self.current_task,
                        application::tree::node::Node {
                            name: String::new(),
                            desc: String::new(),
                            progress: application::tree::node::Progress {
                                total: 10,
                                completed: 0,
                            },
                        },
                    )
                {
                    self.current_task = new_node;
                }
            });
    }

    #[inline]
    fn children(
        &mut self,
        ui: &mut egui::Ui,
        core: &mut application::Core,
    ) -> Result<(), Box<dyn std::error::Error>> {
        egui::ScrollArea::vertical()
            .content_margin(egui::Margin::symmetric(0, 0))
            .show(ui, |ui| -> Result<(), Box<dyn std::error::Error>> {
                if let Ok(children) = core.tree.get_children(&self.current_task) {
                    for (child_id, child_data) in children {
                        Self::child(self, ui, core, &child_id, &child_data)?;
                    }
                }
                Ok(())
            })
            .inner
    }

    fn child(
        &mut self,
        ui: &mut egui::Ui,
        core: &mut application::Core,
        child_id: &automerge::ObjId,
        child_data: &application::tree::Node,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let collapsing_id = ui.make_persistent_id(("expand", child_id));
        let mut collapsing_state = egui::collapsing_header::CollapsingState::load_with_default_open(
            ui.ctx(),
            collapsing_id,
            false,
        );

        let mut result = Ok(());

        egui::Frame::default()
            .outer_margin(egui::Margin::symmetric(6, 4))
            .corner_radius(0)
            .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.vertical(|ui| {
                    Self::child_progress(ui, child_data);

                    egui::Frame::default()
                        .inner_margin(egui::Margin {
                            left: 12,
                            right: 4,
                            top: 4,
                            bottom: 4,
                        })
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        if let Err(e) = Self::button_section(
                                            self,
                                            ui,
                                            core,
                                            child_id,
                                            &mut collapsing_state,
                                        ) {
                                            result = Err(e);
                                        }

                                        ui.add_space(6.0);

                                        Self::child_label(ui, child_data);
                                    },
                                );
                            });
                        });

                    collapsing_state.show_body_unindented(ui, |ui| {
                        egui::Frame::default()
                            .inner_margin(egui::Margin {
                                top: 2,
                                left: 6,
                                right: 6,
                                bottom: 6,
                            })
                            .show(ui, |ui| {
                                ui.set_width(ui.available_width());
                                ui.small("DDescription or child sub-tasks metadata goes here...");
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new("ID:").weak());
                                    ui.small(format!("{child_id:?}"));
                                });
                            });
                    });
                });
            });

        result
    }

    #[inline]
    fn child_progress(ui: &mut egui::Ui, child_data: &application::tree::Node) {
        ui.add(
            egui::ProgressBar::new(child_data.progress.progress())
                .corner_radius(0)
                .fill(crate::theme::FG)
                .desired_height(17.0)
                .desired_width(ui.available_width())
                .text(
                    egui::RichText::new(format!(
                        " [ {} / {} ]",
                        child_data.progress.completed, child_data.progress.total
                    ))
                    .size(10.0)
                    .color(crate::theme::BORDER)
                    .strong(),
                ),
        );

        ui.add(egui::Separator::default().spacing(0.0));
    }

    #[inline]
    fn child_label(ui: &mut egui::Ui, child_data: &application::tree::Node) {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            ui.add(egui::Label::new(&child_data.name).truncate());
        });
    }

    #[inline]
    fn button_section(
        &mut self,
        ui: &mut egui::Ui,
        core: &mut application::Core,
        child_id: &automerge::ObjId,
        collapsing_state: &mut egui::collapsing_header::CollapsingState,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if ui
            .add_sized(
                crate::theme::CHILD_BUTTON_V2,
                egui::Button::image(crate::icons::right()).corner_radius(0),
            )
            .clicked()
        {
            self.current_task = child_id.clone();
        }

        ui.add_space(6.0);

        let panel_icon = if collapsing_state.is_open() {
            crate::icons::panel_close()
        } else {
            crate::icons::panel_open()
        };

        if ui
            .add_sized(
                crate::theme::CHILD_BUTTON_V2,
                egui::Button::image(panel_icon).corner_radius(0),
            )
            .clicked()
        {
            collapsing_state.toggle(ui);
        }

        ui.add_space(6.0);

        if ui
            .add_sized(
                crate::theme::CHILD_BUTTON_V2,
                egui::Button::image(crate::icons::delete()).corner_radius(0),
            )
            .clicked()
            && let Err(err) = core.tree.remove(child_id)
        {
            eprintln!("{err:?}");
        }

        if core.tree.is_leaf(child_id)? {
            ui.add_space(8.0);

            if ui
                .add_sized(
                    crate::theme::CHILD_BUTTON_V2,
                    egui::Button::image(crate::icons::plus()).corner_radius(0),
                )
                .clicked()
                && let Err(err) = core.tree.change_progress_completed(child_id, 1)
            {
                eprintln!("{err:?}");
            }

            if ui
                .add_sized(
                    crate::theme::CHILD_BUTTON_V2,
                    egui::Button::image(crate::icons::minus()).corner_radius(0),
                )
                .clicked()
                && let Err(err) = core.tree.change_progress_completed(child_id, -1)
            {
                eprintln!("{err:?}");
            }
        }

        Ok(())
    }
}
