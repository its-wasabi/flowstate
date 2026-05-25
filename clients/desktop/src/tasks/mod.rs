mod tree;

use crate::appearance::ButtonsExt;

#[derive(Debug)]
pub struct Tasks {
    current_task: automerge::ObjId,
    pub active_name_edit: Option<(egui::Id, automerge::ObjId)>,
    pub active_desc_edit: Option<(egui::Id, automerge::ObjId)>,

    tree_state: tree::TreeState,
}

impl Tasks {
    pub const fn new() -> Self {
        Self {
            current_task: automerge::ROOT,
            active_name_edit: None,
            active_desc_edit: None,

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
        Self::top_bar(self, core, ui);
        Self::parent_task(self, core, ui);
        Self::add_button(self, ui, core);
        Self::children(self, ui, core);

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
    fn top_bar(&self, core: &application::Core, ui: &mut egui::Ui) {
        if let Ok(progress) = core.tree.get_progress(&self.current_task) {
            egui::Panel::top("top_bar_progress")
                .frame(egui::Frame::default())
                .min_size(crate::appearance::TOP_BAR_HEIGHT)
                .show_inside(ui, |ui| {
                    ui.add(
                        egui::ProgressBar::new(progress.procentage() / 100.0)
                            .corner_radius(0)
                            .fill(crate::appearance::FG)
                            .desired_height(ui.available_height())
                            .desired_width(ui.available_width())
                            .text(
                                egui::RichText::new(format!(" {progress}%"))
                                    .color(crate::appearance::BORDER),
                            ),
                    );
                });
        }
    }

    #[inline]
    fn parent_task(&mut self, core: &mut application::Core, ui: &mut egui::Ui) {
        if let Ok(node) = core.tree.get_node(&self.current_task) {
            let name_id = ui.make_persistent_id(("node_name_edit", &self.current_task));
            let desc_id = ui.make_persistent_id(("node_desc_edit", &self.current_task));

            let mut display_name =
                ui.data_mut(|d| d.get_temp::<String>(name_id).unwrap_or_else(|| node.name));

            let mut display_desc =
                ui.data_mut(|d| d.get_temp::<String>(desc_id).unwrap_or_else(|| node.desc));

            ui.horizontal(|ui| {
                egui::Frame::default()
                    .outer_margin(egui::Margin::same(4))
                    .show(ui, |ui| {
                        if ui
                            .icon_button(
                                crate::appearance::PARENT_BUTTON_V2.into(),
                                crate::icons::left(),
                                egui::Color32::WHITE,
                            )
                            .clicked()
                        {
                            if let Ok((id, _)) = core.tree.get_parent(&self.current_task) {
                                self.current_task = id;
                            } else {
                                self.current_task = automerge::ObjId::Root;
                            }
                        }
                    });

                egui::Frame::default()
                    .outer_margin(egui::Margin::symmetric(2, 6))
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            let name_edit = ui.add(
                                egui::TextEdit::singleline(&mut display_name)
                                    .font(egui::TextStyle::Heading)
                                    .frame(egui::Frame::default())
                                    .hint_text("task name")
                                    .desired_width(ui.available_width()),
                            );

                            if name_edit.changed() {
                                self.active_name_edit = Some((name_id, self.current_task.clone()));
                                core.tree.change_node_name_cache(
                                    &self.current_task,
                                    display_name.clone(),
                                );
                            }

                            if name_edit.lost_focus() {
                                if let Err(err) =
                                    core.tree.change_node_name(&self.current_task, display_name)
                                {
                                    eprintln!("FAILED: To commit name {err:?}");
                                }
                                ui.data_mut(|d| d.remove::<String>(name_id));
                                self.active_name_edit = None;
                            }

                            let desc_edit = ui.add(
                                egui::TextEdit::multiline(&mut display_desc)
                                    .font(egui::TextStyle::Body)
                                    .frame(egui::Frame::default())
                                    .hint_text("task description")
                                    .desired_rows(1)
                                    .desired_width(ui.available_width()),
                            );

                            if desc_edit.changed() {
                                println!("set");
                                self.active_desc_edit = Some((desc_id, self.current_task.clone()));
                                core.tree.change_node_desc_cache(
                                    &self.current_task,
                                    display_desc.clone(),
                                );
                            }

                            if desc_edit.lost_focus() {
                                if let Err(err) =
                                    core.tree.change_node_desc(&self.current_task, display_desc)
                                {
                                    eprintln!("FAILED: To commit desc {err:?}");
                                }
                                ui.data_mut(|d| d.remove::<String>(desc_id));
                                self.active_desc_edit = None;
                            }
                        });
                    });
            });

            ui.add(egui::Separator::default().spacing(0.0));
        }
    }

    #[inline]
    fn add_button(&mut self, ui: &mut egui::Ui, core: &mut application::Core) {
        egui::Panel::bottom("add_button")
            .frame(egui::Frame::default())
            .show_inside(ui, |ui| {
                let button_size = egui::vec2(ui.available_width(), crate::appearance::CHILD_BUTTON);
                if ui
                    .icon_button_borderless(button_size, crate::icons::add(), egui::Color32::WHITE)
                    .clicked()
                    && let Ok(new_node) = core
                        .tree
                        .append_child(&self.current_task, application::tree::node::Node::default())
                {
                    self.current_task = new_node;
                }
            });
    }

    #[inline]
    fn children(&mut self, ui: &mut egui::Ui, core: &mut application::Core) {
        egui::CentralPanel::default()
            .frame(egui::Frame::default())
            .show_inside(ui, |ui| {
                if let Ok(children) = core.tree.get_children(&self.current_task) {
                    if children.is_empty() {
                        Self::leaf_control_panel(ui, core);
                    } else {
                        egui::ScrollArea::vertical()
                            .content_margin(egui::Margin::symmetric(6, 2))
                            .show(ui, |ui| {
                                for (child_id, child_data) in children {
                                    ui.push_id(&child_id, |ui| {
                                        if let Err(err) =
                                            Self::child(self, ui, core, &child_id, child_data)
                                        {
                                            eprintln!("{err:?}");
                                        }
                                    });
                                }
                            });
                    }
                };
            });
    }

    #[inline]
    // TODO: Remove
    #[allow(unused, clippy::unnecessary_wraps)]
    fn leaf_control_panel(
        ui: &mut egui::Ui,
        core: &application::Core,
    ) -> Result<(), Box<dyn std::error::Error>> {
        ui.centered_and_justified(|ui| ui.heading("empty (todo)"));
        Ok(())
    }

    fn child(
        &mut self,
        ui: &mut egui::Ui,
        core: &mut application::Core,
        child_id: &automerge::ObjId,
        child_data: application::tree::Node,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let collapsing_id = ui.make_persistent_id(("expand", child_id));
        let mut collapsing_state = egui::collapsing_header::CollapsingState::load_with_default_open(
            ui.ctx(),
            collapsing_id,
            false,
        );

        let mut result = Ok(());

        egui::Frame::default()
            .outer_margin(egui::Margin::symmetric(0, 4))
            .corner_radius(0)
            .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    Self::child_progress(ui, &child_data);

                    egui::Frame::default()
                        .inner_margin(egui::Margin {
                            left: 4,
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

                                        Self::child_label(self, ui, child_id, core, &child_data);
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
                                ui.set_max_height(90.0);
                                ui.centered_and_justified(|ui| ui.label("(todo)"))
                            });
                    });
                });
            });

        result
    }

    #[inline]
    fn child_progress(ui: &mut egui::Ui, child_data: &application::tree::Node) {
        ui.add_sized(
            [ui.available_width(), 18.0],
            egui::ProgressBar::new(child_data.progress.procentage() / 100.0)
                .corner_radius(0)
                .fill(crate::appearance::FG)
                .desired_height(18.0)
                .text(
                    egui::RichText::new(format!(" {}%", child_data.progress))
                        .size(13.0)
                        .color(crate::appearance::BORDER)
                        .strong(),
                ),
        );

        ui.add(egui::Separator::default().spacing(0.0));
    }

    #[inline]
    fn child_label(
        &mut self,
        ui: &mut egui::Ui,
        id: &automerge::ObjId,
        core: &mut application::Core,
        node: &application::tree::Node,
    ) {
        let name_id = ui.make_persistent_id(("node_edit_name", id));

        let mut display_name = ui.data_mut(|d| {
            d.get_temp::<String>(name_id)
                .unwrap_or_else(|| node.name.clone())
        });

        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            egui::ScrollArea::horizontal()
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                .content_margin(egui::Margin::symmetric(6, 0))
                .show(ui, |ui| {
                    let name_edit = ui.add(
                        egui::TextEdit::singleline(&mut display_name)
                            .font(egui::TextStyle::Button)
                            .frame(egui::Frame::default())
                            .hint_text("task name")
                            .clip_text(false)
                            .desired_width(ui.available_width()),
                    );

                    if name_edit.changed() {
                        self.active_name_edit = Some((name_id, id.clone()));
                        core.tree.change_node_name_cache(id, display_name.clone());
                    }

                    if name_edit.lost_focus() {
                        if let Err(err) = core.tree.change_node_name(id, display_name) {
                            eprintln!("FAILED: To commit child name {err:?}");
                        }
                        ui.data_mut(|d| d.remove::<String>(name_id));
                        self.active_name_edit = None;
                    }
                })
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
            .icon_button(
                crate::appearance::CHILD_BUTTON_V2.into(),
                crate::icons::right(),
                egui::Color32::WHITE,
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
            .icon_button(
                crate::appearance::CHILD_BUTTON_V2.into(),
                panel_icon,
                egui::Color32::WHITE,
            )
            .clicked()
        {
            collapsing_state.toggle(ui);
        }

        ui.add_space(6.0);

        if ui
            .icon_button(
                crate::appearance::CHILD_BUTTON_V2.into(),
                crate::icons::delete(),
                egui::Color32::RED,
            )
            .clicked()
            && let Err(err) = core.tree.remove(child_id)
        {
            eprintln!("{err:?}");
        }

        if core.tree.is_leaf(child_id)? {
            ui.add_space(8.0);

            if ui
                .icon_button(
                    crate::appearance::CHILD_BUTTON_V2.into(),
                    crate::icons::plus(),
                    egui::Color32::GREEN,
                )
                .clicked()
                && let Err(err) = core.tree.change_node_completed(child_id, 1)
            {
                eprintln!("{err:?}");
            }

            if ui
                .icon_button(
                    crate::appearance::CHILD_BUTTON_V2.into(),
                    crate::icons::minus(),
                    egui::Color32::YELLOW,
                )
                .clicked()
                && let Err(err) = core.tree.change_node_completed(child_id, -1)
            {
                eprintln!("{err:?}");
            }
        }

        Ok(())
    }
}
