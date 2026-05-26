mod tree;

use crate::appearance::{ButtonsExt, ProgressBarExt};

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
    fn main(&mut self, ui: &mut egui::Ui, core: &mut application::Core) {
        Self::parent_progress(self, core, ui);
        Self::add_button(self, ui, core);
        Self::parent_task(self, core, ui);
        Self::children(self, ui, core);
    }

    fn aside(&mut self, ui: &mut egui::Ui, core: &mut application::Core) {
        self.tree_state.show(ui, &core.tree, &mut self.current_task);
    }
}

impl Tasks {
    #[inline]
    fn parent_progress(&self, core: &application::Core, ui: &mut egui::Ui) {
        if let Ok(progress) = core.tree.get_progress(&self.current_task) {
            ui.top_progress_bar("parent_task_progressbar", progress.procentage());
        } else {
            eprintln!("FAILED TO DISPLAY TOP BAR");
        }
    }

    #[inline]
    fn parent_task(&mut self, core: &mut application::Core, ui: &mut egui::Ui) {
        if let Ok(node) = core.tree.get_node(&self.current_task) {
            let name_id = ui.make_persistent_id(("node_name_edit", &self.current_task));
            let desc_id = ui.make_persistent_id(("node_desc_edit", &self.current_task));

            let mut display_name =
                ui.data_mut(|d| d.get_temp::<String>(name_id).unwrap_or(node.name));
            let mut display_desc =
                ui.data_mut(|d| d.get_temp::<String>(desc_id).unwrap_or(node.desc));

            ui.horizontal(|ui| {
                egui::Frame::default()
                    .outer_margin(egui::Margin::same(4))
                    .show(ui, |ui| {
                        if ui
                            .icon_button(
                                crate::appearance::PARENT_BUTTON_V2.into(),
                                crate::icons::left(crate::icons::IconSize::Big),
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
                                egui::TextEdit::multiline(&mut display_name)
                                    .desired_rows(1)
                                    .desired_width(ui.available_width())
                                    .font(egui::TextStyle::Heading)
                                    .frame(egui::Frame::default())
                                    .hint_text("task name"),
                            );

                            if name_edit.changed() {
                                self.active_name_edit = Some((name_id, self.current_task.clone()));
                                ui.data_mut(|d| d.insert_temp(name_id, display_name.clone()));
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

                            ui.add_space(4.0);

                            let desc_edit = ui.add(
                                egui::TextEdit::multiline(&mut display_desc)
                                    .desired_rows(1)
                                    .desired_width(ui.available_width())
                                    .font(egui::TextStyle::Body)
                                    .frame(egui::Frame::default())
                                    .hint_text("task description"),
                            );

                            if desc_edit.changed() {
                                println!("set");
                                self.active_desc_edit = Some((desc_id, self.current_task.clone()));
                                ui.data_mut(|d| d.insert_temp(desc_id, display_desc.clone()));
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
                    .icon_button_borderless(
                        button_size,
                        crate::icons::add(crate::icons::IconSize::Mid),
                        egui::Color32::WHITE,
                    )
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
                            .content_margin(egui::Margin::symmetric(6, 4))
                            .show(ui, |ui| {
                                for (child_id, child_data) in children {
                                    ui.push_id(&child_id, |ui| {
                                        Self::child(self, ui, core, &child_id, &child_data);
                                    });
                                }
                            });
                    }
                }
            });
    }

    #[inline]
    fn leaf_control_panel(ui: &mut egui::Ui, _core: &application::Core) {
        ui.centered_and_justified(|ui| ui.heading("empty (todo)"));
    }

    fn child(
        &mut self,
        ui: &mut egui::Ui,
        core: &mut application::Core,
        child_id: &automerge::ObjId,
        child_data: &application::tree::Node,
    ) {
        let collapsing_id = ui.make_persistent_id(("collapse_child_menu", child_id));
        let mut collapsing_state = egui::collapsing_header::CollapsingState::load_with_default_open(
            ui.ctx(),
            collapsing_id,
            false,
        );

        egui::Frame::default()
            .outer_margin(egui::Margin::symmetric(0, 4))
            .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.frame_progress_bar(child_data.progress.procentage());

                    egui::Frame::default()
                        .inner_margin(egui::Margin::same(4))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        Self::button_section(
                                            self,
                                            ui,
                                            core,
                                            child_id,
                                            &mut collapsing_state,
                                        );

                                        ui.add_space(6.0);

                                        Self::child_label(self, ui, child_id, core, child_data);
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
                .auto_shrink([false, true])
                .content_margin(egui::Margin::symmetric(6, 0))
                .show(ui, |ui| {
                    let name_edit = ui.add(
                        egui::TextEdit::singleline(&mut display_name)
                            .font(egui::TextStyle::Button)
                            .frame(egui::Frame::default())
                            .hint_text("task name")
                            .clip_text(false),
                    );

                    if name_edit.changed() {
                        self.active_name_edit = Some((name_id, id.clone()));
                        ui.data_mut(|d| d.insert_temp(name_id, display_name.clone()));
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
    ) {
        let right = ui.icon_button(
            crate::appearance::CHILD_BUTTON_V2.into(),
            crate::icons::right(crate::icons::IconSize::Mid),
            egui::Color32::WHITE,
        );

        ui.add_space(6.0);

        let panel_icon = if collapsing_state.is_open() {
            crate::icons::down_close(crate::icons::IconSize::Mid)
        } else {
            crate::icons::down_open(crate::icons::IconSize::Mid)
        };

        let panel_toggle = ui.icon_button(
            crate::appearance::CHILD_BUTTON_V2.into(),
            panel_icon,
            egui::Color32::WHITE,
        );

        ui.add_space(6.0);

        let delete = ui.icon_button(
            crate::appearance::CHILD_BUTTON_V2.into(),
            crate::icons::delete(crate::icons::IconSize::Mid),
            egui::Color32::RED,
        );

        if core.tree.is_leaf(child_id).unwrap_or(false) {
            ui.add_space(8.0);
            Self::leaf_add_min_buttons(ui, core, child_id);
        }

        if right.clicked() {
            self.current_task = child_id.clone();
        }

        if panel_toggle.clicked() {
            collapsing_state.toggle(ui);
        }

        if delete.clicked()
            && let Err(err) = core.tree.remove(child_id)
        {
            eprintln!("{err:?}");
        }
    }

    #[inline]
    fn leaf_add_min_buttons(
        ui: &mut egui::Ui,
        core: &mut application::Core,
        child_id: &automerge::ObjId,
    ) {
        if ui
            .icon_button(
                crate::appearance::CHILD_BUTTON_V2.into(),
                crate::icons::plus(crate::icons::IconSize::Mid),
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
                crate::icons::minus(crate::icons::IconSize::Mid),
                egui::Color32::YELLOW,
            )
            .clicked()
            && let Err(err) = core.tree.change_node_completed(child_id, -1)
        {
            eprintln!("{err:?}");
        }
    }
}
