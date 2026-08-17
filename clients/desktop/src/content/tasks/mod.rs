use application::store;

#[derive(Default)]
pub struct Tasks {
    current_node_uuid: Option<uuid::Uuid>,
    pending_delete_uuid: Option<uuid::Uuid>,
}

#[derive(Debug, Clone)]
pub enum TasksMessage {
    GoNode(Option<uuid::Uuid>),
    GoParent,
    EditStore(application::store::Command),

    _DismissOverlay,
    RequestDelNode(uuid::Uuid),
    ConfirmDelNode(uuid::Uuid),
}

impl crate::Display for Tasks {
    type Message = TasksMessage;

    fn update(&mut self, message: Self::Message, core: &mut application::Core) {
        if !matches!(
            message,
            TasksMessage::RequestDelNode(_) | TasksMessage::ConfirmDelNode(_)
        ) {
            self.pending_delete_uuid = None;
        }

        match message {
            TasksMessage::GoNode(uuid) => self.current_node_uuid = uuid,

            TasksMessage::GoParent => {
                if let Some(current_uuid) = self.current_node_uuid {
                    self.current_node_uuid =
                        core.store.tree.get_parent_uuid(&current_uuid).ok().copied();
                }
            }

            TasksMessage::EditStore(command) => {
                if let Err(error) = core.store.dispatch(command) {
                    eprintln!("{error:?}");
                };
            }

            TasksMessage::_DismissOverlay => {}

            TasksMessage::RequestDelNode(uuid) => self.pending_delete_uuid = Some(uuid),
            TasksMessage::ConfirmDelNode(uuid) => {
                if let Some(pending_delete_uuid) = self.pending_delete_uuid
                    && pending_delete_uuid == uuid
                {
                    if self.current_node_uuid == Some(uuid) {
                        self.current_node_uuid =
                            core.store.tree.get_parent_uuid(&uuid).ok().copied();
                    }

                    core.store.dispatch(application::store::Command::Tree(
                        application::store::tree::Command::DelNode { uuid },
                    ));
                }
            }
        }
    }

    fn subscription(&self) -> iced::Subscription<TasksMessage> {
        if self.pending_delete_uuid.is_some() {
            iced::event::listen_with(|event, status, _window_id| {
                if status == iced::event::Status::Ignored
                    && let iced::Event::Mouse(iced::mouse::Event::ButtonPressed(_)) = event
                {
                    return Some(TasksMessage::_DismissOverlay);
                }
                None
            })
        } else {
            iced::Subscription::none()
        }
    }

    fn view_center<'a>(&'a self, core: &'a application::Core) -> iced::Element<'a, Self::Message> {
        iced::widget::column![self.view_nodes(&core.store), self.add_node()]
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }

    fn view_aside(&self, core: &application::Core) -> iced::Element<'_, Self::Message> {
        iced::widget::text!("Hello (TODO)",)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .align_x(iced::Alignment::Center)
            .align_y(iced::Alignment::Center)
            .into()
    }
}

impl Tasks {
    fn view_nodes<'a>(
        &'a self,
        store: &'a application::store::Store,
    ) -> iced::Element<'a, TasksMessage> {
        match store.tree.view(self.current_node_uuid.as_ref()) {
            Err(error) => {
                eprintln!("{error:?}");
                iced::widget::space()
                    .width(iced::Length::Fill)
                    .height(iced::Length::Fill)
                    .into()
            }
            Ok(view) => match view {
                application::store::tree::View::RootList { children } => {
                    self.list_root(store, children)
                }

                application::store::tree::View::InnerList {
                    current_uuid,
                    current_node,
                    children,
                } => self.list_inner(store, current_uuid, current_node, children),

                application::store::tree::View::Leaf {
                    current_uuid,
                    current_node,
                } => self.leaf_node(current_uuid, current_node),
            },
        }
    }

    fn list_root<'a>(
        &'a self,
        store: &'a application::store::Store,
        uuids: &'a [uuid::Uuid],
    ) -> iced::Element<'a, TasksMessage> {
        iced::widget::scrollable(
            iced::widget::column(
                uuids
                    .iter()
                    .copied()
                    .map(|child| self.inner_node(store, child).unwrap()),
            )
            .spacing(6)
            .padding(6),
        )
        .style(crate::style::scroll)
        .height(iced::Length::Fill)
        .auto_scroll(true)
        .direction(iced::widget::scrollable::Direction::Vertical(
            iced::widget::scrollable::Scrollbar::hidden(),
        ))
        .into()
    }

    fn list_inner<'a>(
        &'a self,
        store: &'a application::store::Store,
        uuid: uuid::Uuid,
        node: &'a application::store::tree::NodeData,
        children: &'a [uuid::Uuid],
    ) -> iced::Element<'a, TasksMessage> {
        iced::widget::column![
            self.current_node(uuid, node),
            iced::widget::scrollable(
                iced::widget::column(
                    children
                        .iter()
                        .copied()
                        .map(|child| { self.inner_node(store, child).unwrap() })
                )
                .spacing(6)
                .padding(6),
            )
            .style(crate::style::scroll)
            .height(iced::Length::Fill)
            .auto_scroll(true)
            .direction(iced::widget::scrollable::Direction::Vertical(
                iced::widget::scrollable::Scrollbar::hidden(),
            ))
        ]
        .into()
    }

    fn current_node<'a>(
        &'a self,
        uuid: uuid::Uuid,
        node: &'a application::store::tree::NodeData,
    ) -> iced::Element<'a, TasksMessage> {
        let (left_btn_style, left_svg_style) =
            crate::style::button_with_icon(crate::style::Variant::Default, true);

        let progress_procentage = node.progress.procentage();

        iced::widget::column![
            iced::widget::stack![
                iced::widget::progress_bar(0.0..=100.0, progress_procentage)
                    .style(crate::style::progress),
                iced::widget::text!(" {progress_procentage:.0}%")
                    .width(iced::Length::Fill)
                    .height(iced::Length::Fill)
                    .align_x(iced::Alignment::Start)
                    .align_y(iced::Alignment::Center)
                    .style(crate::style::text(true))
                    .size(crate::style::SMALL_TEXT_SIZE)
            ]
            .width(iced::Length::Fill)
            .height(iced::Length::Fixed(crate::style::TOP_BAR_HEIGHT)),
            iced::widget::rule::horizontal(crate::style::BORDER_WIDTH).style(crate::style::border),
            iced::widget::row![
                iced::widget::button(crate::icon::left(left_svg_style))
                    .width(iced::Length::Fixed(crate::style::BIG_BUTTON_SIZE))
                    .height(iced::Length::Fixed(crate::style::BIG_BUTTON_SIZE))
                    .padding(crate::style::PADDING)
                    .style(left_btn_style)
                    .on_press(TasksMessage::GoParent),
                iced::widget::space()
                    .height(iced::Length::Fill)
                    .width(iced::Length::Fixed(crate::style::PADDING)),
                iced::widget::column![
                    iced::widget::text_input("NAME", &node.name)
                        .width(iced::Length::Fill)
                        .line_height(iced::widget::text::LineHeight::Absolute(iced::Pixels(
                            crate::style::BIG_BUTTON_SIZE / 2.0
                        )))
                        .size(crate::style::BIG_TEXT_SIZE)
                        .padding(0)
                        .align_x(iced::Alignment::Start)
                        .on_input(move |content| {
                            let (index, delete, insert) = compute_splice(&node.name, &content);
                            TasksMessage::EditStore(application::store::Command::Tree(
                                application::store::tree::Command::SpliceNodeName {
                                    uuid,
                                    index,
                                    delete,
                                    insert,
                                },
                            ))
                        })
                        .style(crate::style::text_input),
                    iced::widget::text_input("DESC", &node.desc)
                        .width(iced::Length::Fill)
                        .line_height(iced::widget::text::LineHeight::Absolute(iced::Pixels(
                            crate::style::BIG_BUTTON_SIZE / 2.0
                        )))
                        .size(crate::style::BIG_TEXT_SIZE)
                        .padding(0)
                        .align_x(iced::Alignment::Start)
                        .on_input(move |content| {
                            let (index, delete, insert) = compute_splice(&node.desc, &content);
                            TasksMessage::EditStore(application::store::Command::Tree(
                                application::store::tree::Command::SpliceNodeDesc {
                                    uuid,
                                    index,
                                    delete,
                                    insert,
                                },
                            ))
                        })
                        .style(crate::style::text_input),
                ]
            ]
            .padding(crate::style::PADDING),
            iced::widget::rule::horizontal(crate::style::BORDER_WIDTH).style(crate::style::border)
        ]
        .height(iced::Length::Shrink)
        .into()
    }

    fn leaf_node<'a>(
        &self,
        uuid: uuid::Uuid,
        node: &application::store::tree::NodeData,
    ) -> iced::Element<'a, TasksMessage> {
        let (left_btn_style, left_svg_style) =
            crate::style::button_with_icon(crate::style::Variant::Default, true);
        let (minus_btn_style, minus_svg_style) =
            crate::style::button_with_icon(crate::style::Variant::Warn, true);
        let (plus_btn_style, plus_svg_style) =
            crate::style::button_with_icon(crate::style::Variant::Ok, true);
        let (delete_btn_style, delete_svg_style) =
            crate::style::button_with_icon(crate::style::Variant::Danger, true);

        let progress = node.progress;

        iced::widget::column![
            iced::widget::stack![
                iced::widget::progress_bar(0.0..=100.0, progress.procentage())
                    .style(crate::style::progress),
                iced::widget::text!(" {progress:.0}%")
                    .width(iced::Length::Fill)
                    .height(iced::Length::Fill)
                    .align_x(iced::Alignment::Start)
                    .align_y(iced::Alignment::Center)
                    .style(crate::style::text(true))
                    .size(crate::style::SMALL_TEXT_SIZE)
            ]
            .width(iced::Length::Fill)
            .height(iced::Length::Fixed(crate::style::TOP_BAR_HEIGHT)),
            iced::widget::rule::horizontal(crate::style::BORDER_WIDTH).style(crate::style::border),
            iced::widget::row![
                iced::widget::button(crate::icon::left(left_svg_style))
                    .width(iced::Length::Fixed(crate::style::SMALL_BUTTON_SIZE))
                    .height(iced::Length::Fixed(crate::style::SMALL_BUTTON_SIZE))
                    .padding(crate::style::PADDING)
                    .style(left_btn_style)
                    .on_press(TasksMessage::GoParent),
                iced::widget::space()
                    .height(iced::Length::Fill)
                    .width(iced::Length::Fixed(crate::style::PADDING)),
                iced::widget::button(crate::icon::minus(minus_svg_style))
                    .width(iced::Length::Fixed(crate::style::SMALL_BUTTON_SIZE))
                    .height(iced::Length::Fixed(crate::style::SMALL_BUTTON_SIZE))
                    .padding(crate::style::PADDING)
                    .style(minus_btn_style)
                    .on_press(TasksMessage::EditStore(application::store::Command::Tree(
                        application::store::tree::Command::UpdateNodeCompleted { uuid, by: -1 },
                    ))),
                iced::widget::button(crate::icon::plus(plus_svg_style))
                    .width(iced::Length::Fixed(crate::style::SMALL_BUTTON_SIZE))
                    .height(iced::Length::Fixed(crate::style::SMALL_BUTTON_SIZE))
                    .padding(crate::style::PADDING)
                    .style(plus_btn_style)
                    .on_press(TasksMessage::EditStore(application::store::Command::Tree(
                        application::store::tree::Command::UpdateNodeCompleted { uuid, by: 1 },
                    ))),
                iced::widget::space()
                    .height(iced::Length::Fill)
                    .width(iced::Length::Fixed(crate::style::PADDING)),
                if let Some(pending_delete_uuid) = self.pending_delete_uuid
                    && pending_delete_uuid == uuid
                {
                    iced::widget::button(
                        iced::widget::text("CONFIRM").style(crate::style::text(false)),
                    )
                    .width(iced::Length::Shrink)
                    .height(iced::Length::Fixed(crate::style::SMALL_BUTTON_SIZE))
                    .padding(crate::style::PADDING)
                    .style(delete_btn_style)
                    .on_press(TasksMessage::ConfirmDelNode(uuid))
                } else {
                    iced::widget::button(crate::icon::delete(delete_svg_style))
                        .width(iced::Length::Fixed(crate::style::SMALL_BUTTON_SIZE))
                        .height(iced::Length::Fixed(crate::style::SMALL_BUTTON_SIZE))
                        .padding(crate::style::PADDING)
                        .style(delete_btn_style)
                        .on_press(TasksMessage::RequestDelNode(uuid))
                }
            ]
            .height(iced::Length::Shrink)
            .padding(crate::style::PADDING),
            iced::widget::rule::horizontal(crate::style::BORDER_WIDTH).style(crate::style::border),
            iced::widget::container(
                iced::widget::text(node.desc.clone())
                    .width(iced::Length::Fill)
                    .height(iced::Length::Fill)
                    .align_x(iced::Alignment::Center)
                    .align_y(iced::Alignment::Center),
            )
            .height(iced::Length::Fill)
        ]
        .height(iced::Length::Fill)
        .into()
    }

    fn inner_node<'a>(
        &'a self,
        store: &'a application::store::Store,
        uuid: uuid::Uuid,
    ) -> Result<iced::Element<'a, TasksMessage>, Box<dyn std::error::Error>> {
        let (right_btn_style, right_svg_style) =
            crate::style::button_with_icon(crate::style::Variant::Default, true);
        let (minus_btn_style, minus_svg_style) =
            crate::style::button_with_icon(crate::style::Variant::Warn, true);
        let (plus_btn_style, plus_svg_style) =
            crate::style::button_with_icon(crate::style::Variant::Ok, true);
        let (delete_btn_style, delete_svg_style) =
            crate::style::button_with_icon(crate::style::Variant::Danger, true);

        let node = store.tree.get_node(&uuid)?;
        let progress = store.tree.get_progress(&uuid)?;
        let is_leaf = !store.tree.has_children(&uuid);

        Ok(iced::widget::container(iced::widget::column![
            iced::widget::stack![
                iced::widget::progress_bar(0.0..=100.0, progress.procentage())
                    .style(crate::style::progress),
                iced::widget::text!(" {progress:.0}%")
                    .width(iced::Length::Fill)
                    .height(iced::Length::Fill)
                    .align_x(iced::Alignment::Start)
                    .align_y(iced::Alignment::Center)
                    .style(crate::style::text(true))
                    .size(crate::style::SMALL_TEXT_SIZE)
            ]
            .width(iced::Length::Fill)
            .height(iced::Length::Fixed(crate::style::SMALL_BAR_HEIGHT)),
            iced::widget::rule::horizontal(crate::style::BORDER_WIDTH).style(crate::style::border),
            iced::widget::row![
                iced::widget::text_input("NAME", &node.name)
                    .width(iced::Length::Fill)
                    .line_height(iced::widget::text::LineHeight::Absolute(iced::Pixels(
                        crate::style::SMALL_BUTTON_SIZE
                    )))
                    .size(crate::style::BIG_TEXT_SIZE)
                    .padding(0)
                    .align_x(iced::Alignment::Start)
                    .on_input(move |content| {
                        let (index, delete, insert) = compute_splice(&node.name, &content);
                        TasksMessage::EditStore(application::store::Command::Tree(
                            application::store::tree::Command::SpliceNodeName {
                                uuid,
                                index,
                                delete,
                                insert,
                            },
                        ))
                    })
                    .style(crate::style::text_input),
                is_leaf.then(|| {
                    let (minus_btn_style, minus_svg_style) =
                        crate::style::button_with_icon(crate::style::Variant::Warn, true);
                    iced::widget::button(crate::icon::minus(minus_svg_style))
                        .width(iced::Length::Fixed(crate::style::SMALL_BUTTON_SIZE))
                        .height(iced::Length::Fixed(crate::style::SMALL_BUTTON_SIZE))
                        .padding(crate::style::PADDING)
                        .style(minus_btn_style)
                        .on_press(TasksMessage::EditStore(application::store::Command::Tree(
                            application::store::tree::Command::UpdateNodeCompleted { uuid, by: -1 },
                        )))
                }),
                is_leaf.then(|| {
                    let (plus_btn_style, plus_svg_style) =
                        crate::style::button_with_icon(crate::style::Variant::Ok, true);
                    iced::widget::button(crate::icon::plus(plus_svg_style))
                        .width(iced::Length::Fixed(crate::style::SMALL_BUTTON_SIZE))
                        .height(iced::Length::Fixed(crate::style::SMALL_BUTTON_SIZE))
                        .padding(crate::style::PADDING)
                        .style(plus_btn_style)
                        .on_press(TasksMessage::EditStore(application::store::Command::Tree(
                            application::store::tree::Command::UpdateNodeCompleted { uuid, by: 1 },
                        )))
                }),
                iced::widget::space()
                    .height(iced::Length::Fill)
                    .width(iced::Length::Fixed(crate::style::PADDING)),
                if let Some(pending_delete_uuid) = self.pending_delete_uuid
                    && pending_delete_uuid == uuid
                {
                    iced::widget::button(
                        iced::widget::text("CONFIRM").style(crate::style::text(false)),
                    )
                    .width(iced::Length::Shrink)
                    .height(iced::Length::Fixed(crate::style::SMALL_BUTTON_SIZE))
                    .padding(crate::style::PADDING)
                    .style(delete_btn_style)
                    .on_press(TasksMessage::ConfirmDelNode(uuid))
                } else {
                    iced::widget::button(crate::icon::delete(delete_svg_style))
                        .width(iced::Length::Fixed(crate::style::SMALL_BUTTON_SIZE))
                        .height(iced::Length::Fixed(crate::style::SMALL_BUTTON_SIZE))
                        .padding(crate::style::PADDING)
                        .style(delete_btn_style)
                        .on_press(TasksMessage::RequestDelNode(uuid))
                },
                iced::widget::space()
                    .height(iced::Length::Fill)
                    .width(iced::Length::Fixed(crate::style::PADDING)),
                iced::widget::button(crate::icon::right(right_svg_style))
                    .width(iced::Length::Fixed(crate::style::SMALL_BUTTON_SIZE))
                    .height(iced::Length::Fixed(crate::style::SMALL_BUTTON_SIZE))
                    .padding(crate::style::PADDING)
                    .style(right_btn_style)
                    .on_press(TasksMessage::GoNode(Some(uuid)))
            ]
            .padding(6)
        ])
        .style(crate::style::container(true))
        .width(iced::Length::Fill)
        .height(iced::Length::Shrink)
        .into())
    }

    fn add_node<'a>(&self) -> iced::Element<'a, TasksMessage> {
        let (plus_btn_style, plus_svg_style) =
            crate::style::button_with_icon(crate::style::Variant::Default, false);

        iced::widget::column![
            iced::widget::rule::horizontal(crate::style::BORDER_WIDTH).style(crate::style::border),
            iced::widget::button(crate::icon::plus(plus_svg_style))
                .width(iced::Length::Fill)
                .height(iced::Length::Fill)
                .padding(4)
                .style(plus_btn_style)
                .on_press(TasksMessage::EditStore(application::store::Command::Tree(
                    application::store::tree::Command::AddNode {
                        parent_uuid: self.current_node_uuid,
                        node_data: application::store::tree::NodeData::default()
                    }
                )))
        ]
        .width(iced::Length::Fill)
        .height(iced::Length::Fixed(26.0))
        .into()
    }
}

fn compute_splice(old: &str, new: &str) -> (usize, isize, String) {
    let old_chars: Vec<char> = old.chars().collect();
    let new_chars: Vec<char> = new.chars().collect();

    let mut prefix_len = 0;
    let min_len = std::cmp::min(old_chars.len(), new_chars.len());
    while prefix_len < min_len && old_chars[prefix_len] == new_chars[prefix_len] {
        prefix_len += 1;
    }

    let mut suffix_len = 0;
    let max_suffix = std::cmp::min(old_chars.len() - prefix_len, new_chars.len() - prefix_len);
    while suffix_len < max_suffix
        && old_chars[old_chars.len() - 1 - suffix_len]
            == new_chars[new_chars.len() - 1 - suffix_len]
    {
        suffix_len += 1;
    }

    let index = prefix_len;
    let delete = old_chars.len() - prefix_len - suffix_len;
    let insert: String = new_chars[prefix_len..new_chars.len() - suffix_len]
        .iter()
        .collect();

    (index, delete as isize, insert)
}
