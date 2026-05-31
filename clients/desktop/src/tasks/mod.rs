// mod tree;

fn name_edit_id(ui: &egui::Ui, id: &automerge::ObjId) -> egui::Id {
    ui.make_persistent_id(("name_edit", id))
}

fn desc_edit_id(ui: &egui::Ui, id: &automerge::ObjId) -> egui::Id {
    ui.make_persistent_id(("desc_edit", id))
}

fn total_drag_id(ui: &egui::Ui, id: &automerge::ObjId) -> egui::Id {
    ui.make_persistent_id(("total_drag", id))
}

#[derive(Debug)]
pub struct Tasks {
    current_task: automerge::ObjId,
    pub active_name_edit: Option<(egui::Id, automerge::ObjId)>,
    pub active_desc_edit: Option<(egui::Id, automerge::ObjId)>,
    pub active_total_drag: Option<(egui::Id, automerge::ObjId)>,
    // tree_state: tree::TreeState,
}

impl Tasks {
    pub const fn new() -> Self {
        Self {
            current_task: automerge::ROOT,

            active_name_edit: None,
            active_desc_edit: None,
            active_total_drag: None,
            // tree_state: tree::TreeState::new(),
        }
    }
}

impl super::View for Tasks {
    fn main(&mut self, ui: &mut egui::Ui, core: &mut application::Core) {
        Self::top_bar_progress(self, core, ui);
        Self::add_button(self, ui, core);

        Self::children(self, ui, core);
    }

    fn aside(&mut self, ui: &mut egui::Ui, _core: &mut application::Core) {
        // self.tree_state.show(ui, &core.tree, &mut self.current_task);
        ui.centered_and_justified(|ui| ui.heading("(TODO)"));
    }
}

impl Tasks {
    #[inline]
    fn top_bar_progress(&self, core: &application::Core, ui: &mut egui::Ui) {
        if let Ok(progress) = core.tree.get_progress(&self.current_task) {
            egui::Panel::top("CHANGE ME")
                .frame(egui::Frame::default())
                .exact_size(crate::appearance::TOP_BAR_HEIGHT)
                .show_inside(ui, |ui| {
                    ui.add(crate::components::TopProgressBar::new(
                        progress.procentage(),
                    ));
                });
        } else {
            eprintln!("FAILED TO DISPLAY TOP BAR");
        }
    }

    #[inline]
    fn add_button(&mut self, ui: &mut egui::Ui, core: &mut application::Core) {
        egui::Panel::bottom("add_button")
            .frame(egui::Frame::default())
            .exact_size(crate::appearance::BUTTON_MID)
            .resizable(false)
            .show_inside(ui, |ui| {
                if ui
                    .add_sized(
                        egui::Vec2::new(ui.available_width(), ui.available_height()),
                        crate::components::IconButtonBorderless::new(
                            egui::Color32::WHITE,
                            crate::icons::Icon::Add,
                        ),
                    )
                    .clicked()
                    && let Ok(new_node) = core.tree.append_child(
                        &self.current_task,
                        &application::tree::node::Node::default(),
                    )
                {
                    self.current_task = new_node;
                }
            });
    }

    #[inline]
    fn parent_task(&mut self, core: &mut application::Core, ui: &mut egui::Ui) {
        if let Ok(node) = core.tree.get_node(&self.current_task) {
            let name_id = name_edit_id(ui, &self.current_task);
            let desc_id = desc_edit_id(ui, &self.current_task);

            let mut display_name =
                ui.data_mut(|d| d.get_temp::<String>(name_id).unwrap_or(node.name));
            let mut display_desc =
                ui.data_mut(|d| d.get_temp::<String>(desc_id).unwrap_or(node.desc));

            ui.horizontal(|ui| {
                egui::Frame::default()
                    .outer_margin(egui::Margin::same(4))
                    .show(ui, |ui| {
                        if ui
                            .add_sized(
                                crate::appearance::BUTTON_BIG_V2,
                                crate::components::IconButton::new(
                                    crate::appearance::FG,
                                    crate::icons::Icon::Left,
                                ),
                            )
                            .clicked()
                            && let Ok(id) = core.tree.get_parent(&self.current_task)
                        {
                            self.current_task = id;
                        }
                    });

                egui::Frame::default()
                    .outer_margin(egui::Margin::symmetric(2, 6))
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            let name_edit = ui.add(crate::components::TextEditSingleLine::new(
                                "task name",
                                &mut display_name,
                            ));

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

                            let desc_edit = ui.add(crate::components::TextEditSingleLine::new(
                                "task description",
                                &mut display_desc,
                            ));

                            if desc_edit.changed() {
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
        }
    }

    #[inline]
    fn children(&mut self, ui: &mut egui::Ui, core: &mut application::Core) {
        if let Ok(children) = core.tree.get_children(&self.current_task) {
            match children {
                application::tree::NodeContent::Leaf((_id, node)) => {
                    Self::leaf_control_panel(self, ui, core, &node);
                }
                application::tree::NodeContent::Inner(nodes) => {
                    Self::parent_task(self, core, ui);
                    ui.add(egui::Separator::default().spacing(0.0));
                    egui::ScrollArea::vertical()
                        .content_margin(egui::Margin::symmetric(6, 4))
                        .auto_shrink([false, true])
                        .show(ui, |ui| {
                            for (id, node) in nodes {
                                ui.push_id(&id, |ui| {
                                    Self::child(self, ui, core, &id, &node);
                                });
                            }
                            ui.add_space(6.0);
                        });
                }
            }
        }
    }

    fn child(
        &mut self,
        ui: &mut egui::Ui,
        core: &mut application::Core,
        id: &automerge::ObjId,
        node: &application::tree::Node,
    ) {
        egui::Frame::default()
            .outer_margin(egui::Margin::symmetric(0, 4))
            .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.add_sized(
                        egui::Vec2::new(ui.available_width(), 18.0),
                        crate::components::TopProgressBar::new(node.progress.procentage()),
                    );

                    ui.add(egui::Separator::default().spacing(0.0));

                    egui::Frame::default()
                        .inner_margin(egui::Margin::same(4))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        Self::button_section(self, ui, core, id);

                                        ui.add_space(6.0);

                                        Self::child_label(self, ui, id, core, node);
                                    },
                                );
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
        let name_id = name_edit_id(ui, id);

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
                    let name_edit = ui.add(crate::components::TextEditSingleLine::new(
                        "task name",
                        &mut display_name,
                    ));

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
    fn leaf_control_panel(
        &mut self,
        ui: &mut egui::Ui,
        core: &mut application::Core,
        node: &application::tree::Node,
    ) {
        let total_id = total_drag_id(ui, &self.current_task);

        let mut display_total = ui.data_mut(|d| {
            d.get_temp::<u32>(total_id)
                .unwrap_or_else(|| node.progress.total())
        });

        egui::Frame::default()
            .inner_margin(egui::Margin::same(4))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    if ui
                        .add_sized(
                            crate::appearance::BUTTON_MID_V2,
                            crate::components::IconButton::new(
                                crate::appearance::FG,
                                crate::icons::Icon::Left,
                            ),
                        )
                        .clicked()
                        && let Ok(id) = core.tree.get_parent(&self.current_task)
                    {
                        self.current_task = id;
                    }

                    ui.add_space(4.0);

                    if ui
                        .add_sized(
                            crate::appearance::BUTTON_MID_V2,
                            crate::components::IconButton::new(
                                egui::Color32::YELLOW,
                                crate::icons::Icon::Minus,
                            ),
                        )
                        .clicked()
                        && let Err(err) = core.tree.change_node_completed(&self.current_task, -1)
                    {
                        eprintln!("{err:?}");
                    }

                    if ui
                        .add_sized(
                            crate::appearance::BUTTON_MID_V2,
                            crate::components::IconButton::new(
                                egui::Color32::GREEN,
                                crate::icons::Icon::Plus,
                            ),
                        )
                        .clicked()
                        && let Err(err) = core.tree.change_node_completed(&self.current_task, 1)
                    {
                        eprintln!("{err:?}");
                    }

                    ui.add_space(4.0);

                    let total_edit = ui.add_sized(
                        egui::Vec2::new(ui.available_height() * 2.0, ui.available_height()),
                        crate::components::DragValueEdit::new(
                            crate::appearance::FG,
                            &mut display_total,
                        ),
                    );

                    if total_edit.changed() {
                        self.active_total_drag = Some((total_id, self.current_task.clone()));
                        ui.data_mut(|d| d.insert_temp(total_id, display_total));
                        core.tree
                            .change_node_total_cache(&self.current_task, display_total);
                    }
                    if total_edit.drag_stopped() || total_edit.lost_focus() {
                        if let Err(err) = core
                            .tree
                            .change_node_total(&self.current_task, display_total)
                        {
                            eprintln!("FAILED: To commit total {err:?}");
                        }

                        ui.data_mut(|d| d.remove::<u32>(total_id));
                    }

                    ui.add_space(4.0);

                    let delete = ui.add_sized(
                        crate::appearance::BUTTON_MID_V2,
                        crate::components::IconButton::new(
                            egui::Color32::RED,
                            crate::icons::Icon::Delete,
                        ),
                    );

                    if delete.clicked()
                        && let Err(err) = core.tree.remove(&self.current_task)
                    {
                        eprintln!("{err:?}");
                    }
                });
            });

        ui.add(egui::Separator::default().spacing(0.0));

        let name_id = name_edit_id(ui, &self.current_task);
        let desc_id = desc_edit_id(ui, &self.current_task);

        let mut display_name =
            ui.data_mut(|d| d.get_temp::<String>(name_id).unwrap_or(node.name.clone()));
        let mut display_desc =
            ui.data_mut(|d| d.get_temp::<String>(desc_id).unwrap_or(node.desc.clone()));

        egui::Frame::default()
            .outer_margin(egui::Margin::symmetric(6, 6))
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    let name_edit = ui.add(crate::components::TextEditMultiLine::new(
                        "task name",
                        &mut display_name,
                    ));

                    if name_edit.changed() {
                        self.active_name_edit = Some((name_id, self.current_task.clone()));
                        ui.data_mut(|d| d.insert_temp(name_id, display_name.clone()));
                        core.tree
                            .change_node_name_cache(&self.current_task, display_name.clone());
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

                    let desc_edit = ui.add(crate::components::TextEditMultiLine::new(
                        "task description",
                        &mut display_desc,
                    ));

                    if desc_edit.changed() {
                        self.active_desc_edit = Some((desc_id, self.current_task.clone()));
                        ui.data_mut(|d| d.insert_temp(desc_id, display_desc.clone()));
                        core.tree
                            .change_node_desc_cache(&self.current_task, display_desc.clone());
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
    }

    #[inline]
    fn button_section(
        &mut self,
        ui: &mut egui::Ui,
        core: &mut application::Core,
        child_id: &automerge::ObjId,
    ) {
        let right = ui.add_sized(
            crate::appearance::BUTTON_MID_V2,
            crate::components::IconButton::new(egui::Color32::WHITE, crate::icons::Icon::Right),
        );

        ui.add_space(4.0);

        let delete = ui.add(crate::components::IconButton::new(
            egui::Color32::RED,
            crate::icons::Icon::Delete,
        ));

        if core.tree.is_leaf(child_id).unwrap_or(false) {
            ui.add_space(4.0);
            Self::leaf_add_min_buttons(ui, core, child_id);
        }

        if right.clicked() {
            self.current_task = child_id.clone();
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
            .add_sized(
                crate::appearance::BUTTON_MID_V2,
                crate::components::IconButton::new(egui::Color32::GREEN, crate::icons::Icon::Plus),
            )
            .clicked()
            && let Err(err) = core.tree.change_node_completed(child_id, 1)
        {
            eprintln!("{err:?}");
        }

        if ui
            .add_sized(
                crate::appearance::BUTTON_MID_V2,
                crate::components::IconButton::new(
                    egui::Color32::YELLOW,
                    crate::icons::Icon::Minus,
                ),
            )
            .clicked()
            && let Err(err) = core.tree.change_node_completed(child_id, -1)
        {
            eprintln!("{err:?}");
        }
    }
}
