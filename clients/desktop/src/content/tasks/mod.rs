pub struct Tasks {
    current_node_id: automerge::ObjId,
    pending_delete_id: Option<automerge::ObjId>,
}

#[derive(Debug, Clone)]
pub enum TasksMessage {
    Background,
    GoBack,
    GoNode(automerge::ObjId),
    ReqDelNode(automerge::ObjId),
    AckDelNode(automerge::ObjId),
    AddNode {
        parent: automerge::ObjId,
        node_data: application::tree::node::NodeData,
    },

    NodeCompletedChange {
        id: automerge::ObjId,
        delta: i64,
    },

    NodeNameChange {
        id: automerge::ObjId,
        content: String,
    },

    NodeDescChange {
        id: automerge::ObjId,
        content: String,
    },
}

impl Tasks {
    pub const fn new() -> Self {
        Self {
            current_node_id: automerge::ObjId::Root,
            pending_delete_id: None,
        }
    }
}

impl crate::Display for Tasks {
    type Message = TasksMessage;

    fn update(&mut self, message: Self::Message, core: &mut application::Core) {
        if !matches!(
            message,
            TasksMessage::ReqDelNode(_) | TasksMessage::AckDelNode(_)
        ) {
            self.pending_delete_id = None;
        }

        match message {
            TasksMessage::Background => {}

            TasksMessage::GoBack => {
                if let Ok(parent) = core.tree.get_parent(&self.current_node_id) {
                    self.current_node_id = parent.clone();
                } else {
                    println!("GO BACK FAIL");
                }
            }

            TasksMessage::GoNode(id) => self.current_node_id = id,

            TasksMessage::ReqDelNode(id) => {
                self.pending_delete_id = Some(id);
            }

            TasksMessage::AckDelNode(id) => {
                if let Some(pending_delete_id) = &self.pending_delete_id
                    && *pending_delete_id == id
                {
                    if id == self.current_node_id {
                        if let Ok(parent) = core.tree.get_parent(&self.current_node_id) {
                            self.current_node_id = parent.clone();
                        } else {
                            eprintln!("GO BACK AFTER DEL FAIL");
                        }
                    }

                    core.tree.delete(&id);
                }
            }

            TasksMessage::AddNode { parent, node_data } => {
                if core.tree.append_child(&parent, &node_data).is_err() {
                    eprintln!("ADD TASK FAIL");
                }
            }

            TasksMessage::NodeCompletedChange { id, delta } => {
                core.tree.change_node_completed(&id, delta);
            }

            TasksMessage::NodeNameChange { id, content } => {
                // FIX: Change to cache function - now it spams automerge with changes
                core.tree.change_node_name(&id, content);
            }

            TasksMessage::NodeDescChange { id, content } => {
                // FIX: Change to cache function - now it spams automerge with changes
                core.tree.projection.update_node_desc(&id, content);
            }
        }
    }

    fn subscription(&self) -> iced::Subscription<TasksMessage> {
        if self.pending_delete_id.is_some() {
            iced::event::listen_with(|event, status, _window_id| {
                if status == iced::event::Status::Ignored
                    && let iced::Event::Mouse(iced::mouse::Event::ButtonPressed(_)) = event
                {
                    return Some(TasksMessage::Background);
                }
                None
            })
        } else {
            iced::Subscription::none()
        }
    }

    fn view_center(&self, core: &application::Core) -> iced::Element<'_, Self::Message> {
        iced::widget::column![self.list_nodes(&core.tree), self.add_node()]
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }

    fn view_aside(&self, core: &application::Core) -> iced::Element<'_, Self::Message> {
        iced::widget::text!(
            "{:?}",
            core.tree
                .analytics
                .change_over_time(application::tree::analytics::TimeScale::Hour),
        )
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .align_x(iced::Alignment::Center)
        .align_y(iced::Alignment::Center)
        .into()
    }
}

impl Tasks {
    fn list_nodes<'a>(&self, tree: &application::tree::Tree) -> iced::Element<'a, TasksMessage> {
        match tree.view(&self.current_node_id) {
            Ok(application::tree::View::RootList { children }) => iced::widget::scrollable(
                iced::widget::column(
                    children
                        .iter()
                        .map(|children| Self::inner_node(self, children)),
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
            .into(),

            Ok(application::tree::View::InnerList {
                current_id,
                current_node,
                children,
            }) => iced::widget::column![
                self.current_node(&current_id, &current_node),
                iced::widget::scrollable(
                    iced::widget::column(
                        children
                            .iter()
                            .map(|children| Self::inner_node(self, children))
                    )
                    .spacing(6)
                    .padding(6),
                )
                .style(crate::style::scroll)
                .height(iced::Length::Fill)
                .auto_scroll(true)
                .direction(iced::widget::scrollable::Direction::Vertical(
                    iced::widget::scrollable::Scrollbar::hidden()
                ))
            ]
            .into(),

            Ok(application::tree::View::Leaf { id, node }) => Self::leaf_node(id, &node),

            Err(_) => iced::widget::space()
                .width(iced::Length::Fill)
                .height(iced::Length::Fill)
                .into(),
        }
    }

    fn current_node<'a>(
        &self,
        id: &automerge::ObjId,
        node: &application::tree::node::NodeData,
    ) -> iced::Element<'a, TasksMessage> {
        let name_id = self.current_node_id.clone();
        let desc_id = self.current_node_id.clone();

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
                    .on_press(TasksMessage::GoBack),
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
                        .on_input(move |content| TasksMessage::NodeNameChange {
                            id: name_id.clone(),
                            content,
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
                        .on_input(move |content| TasksMessage::NodeDescChange {
                            id: desc_id.clone(),
                            content,
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
        id: automerge::ObjId,
        node: &application::tree::node::NodeData,
    ) -> iced::Element<'a, TasksMessage> {
        let (left_btn_style, left_svg_style) =
            crate::style::button_with_icon(crate::style::Variant::Default, true);
        let (minus_btn_style, minus_svg_style) =
            crate::style::button_with_icon(crate::style::Variant::Warn, true);
        let (plus_btn_style, plus_svg_style) =
            crate::style::button_with_icon(crate::style::Variant::Ok, true);
        let (delete_btn_style, delete_svg_style) =
            crate::style::button_with_icon(crate::style::Variant::Danger, true);

        let name_id = id.clone();
        let desc_id = id.clone();

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
                    .width(iced::Length::Fixed(crate::style::SMALL_BUTTON_SIZE))
                    .height(iced::Length::Fixed(crate::style::SMALL_BUTTON_SIZE))
                    .padding(crate::style::PADDING)
                    .style(left_btn_style)
                    .on_press(TasksMessage::GoBack),
                iced::widget::space()
                    .height(iced::Length::Fill)
                    .width(iced::Length::Fixed(crate::style::PADDING)),
                iced::widget::button(crate::icon::minus(minus_svg_style))
                    .width(iced::Length::Fixed(crate::style::SMALL_BUTTON_SIZE))
                    .height(iced::Length::Fixed(crate::style::SMALL_BUTTON_SIZE))
                    .padding(crate::style::PADDING)
                    .style(minus_btn_style)
                    .on_press(TasksMessage::NodeCompletedChange {
                        id: id.clone(),
                        delta: -1
                    }),
                iced::widget::button(crate::icon::plus(plus_svg_style))
                    .width(iced::Length::Fixed(crate::style::SMALL_BUTTON_SIZE))
                    .height(iced::Length::Fixed(crate::style::SMALL_BUTTON_SIZE))
                    .padding(crate::style::PADDING)
                    .style(plus_btn_style)
                    .on_press(TasksMessage::NodeCompletedChange {
                        id: id.clone(),
                        delta: 1
                    }),
                iced::widget::space()
                    .height(iced::Length::Fill)
                    .width(iced::Length::Fixed(crate::style::PADDING)),
                iced::widget::button(crate::icon::delete(delete_svg_style))
                    .width(iced::Length::Fixed(crate::style::SMALL_BUTTON_SIZE))
                    .height(iced::Length::Fixed(crate::style::SMALL_BUTTON_SIZE))
                    .padding(crate::style::PADDING)
                    .style(delete_btn_style)
                    .on_press(TasksMessage::ReqDelNode(id)),
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
        &self,
        entry: &application::tree::ChildEntry,
    ) -> iced::Element<'a, TasksMessage> {
        let (right_btn_style, right_svg_style) =
            crate::style::button_with_icon(crate::style::Variant::Default, true);
        let (minus_btn_style, minus_svg_style) =
            crate::style::button_with_icon(crate::style::Variant::Warn, true);
        let (plus_btn_style, plus_svg_style) =
            crate::style::button_with_icon(crate::style::Variant::Ok, true);
        let (delete_btn_style, delete_svg_style) =
            crate::style::button_with_icon(crate::style::Variant::Danger, true);

        let progress_procentage = entry.node.progress.procentage();
        let name_id = entry.id.clone();
        iced::widget::container(iced::widget::column![
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
            .height(iced::Length::Fixed(crate::style::SMALL_BAR_HEIGHT)),
            iced::widget::rule::horizontal(crate::style::BORDER_WIDTH).style(crate::style::border),
            iced::widget::row![
                iced::widget::text_input("NAME", &entry.node.name)
                    .width(iced::Length::Fill)
                    .line_height(iced::widget::text::LineHeight::Absolute(iced::Pixels(
                        crate::style::SMALL_BUTTON_SIZE
                    )))
                    .size(crate::style::BIG_TEXT_SIZE)
                    .padding(0)
                    .align_x(iced::Alignment::Start)
                    .on_input(move |content| TasksMessage::NodeNameChange {
                        id: name_id.clone(),
                        content,
                    })
                    .style(crate::style::text_input),
                entry.is_leaf.then(|| {
                    let (minus_btn_style, minus_svg_style) =
                        crate::style::button_with_icon(crate::style::Variant::Warn, true);
                    iced::widget::button(crate::icon::minus(minus_svg_style))
                        .width(iced::Length::Fixed(crate::style::SMALL_BUTTON_SIZE))
                        .height(iced::Length::Fixed(crate::style::SMALL_BUTTON_SIZE))
                        .padding(crate::style::PADDING)
                        .style(minus_btn_style)
                        .on_press(TasksMessage::NodeCompletedChange {
                            id: entry.id.clone(),
                            delta: -1,
                        })
                }),
                entry.is_leaf.then(|| {
                    let (plus_btn_style, plus_svg_style) =
                        crate::style::button_with_icon(crate::style::Variant::Ok, true);
                    iced::widget::button(crate::icon::plus(plus_svg_style))
                        .width(iced::Length::Fixed(crate::style::SMALL_BUTTON_SIZE))
                        .height(iced::Length::Fixed(crate::style::SMALL_BUTTON_SIZE))
                        .padding(crate::style::PADDING)
                        .style(plus_btn_style)
                        .on_press(TasksMessage::NodeCompletedChange {
                            id: entry.id.clone(),
                            delta: 1,
                        })
                }),
                iced::widget::space()
                    .height(iced::Length::Fill)
                    .width(iced::Length::Fixed(crate::style::PADDING)),
                if let Some(pending_delete_id) = &self.pending_delete_id
                    && *pending_delete_id == entry.id
                {
                    iced::widget::button(
                        iced::widget::text("CONFIRM").style(crate::style::text(false)),
                    )
                    .width(iced::Length::Shrink)
                    .height(iced::Length::Fixed(crate::style::SMALL_BUTTON_SIZE))
                    .padding(crate::style::PADDING)
                    .style(delete_btn_style)
                    .on_press(TasksMessage::AckDelNode(entry.id.clone()))
                } else {
                    iced::widget::button(crate::icon::delete(delete_svg_style))
                        .width(iced::Length::Fixed(crate::style::SMALL_BUTTON_SIZE))
                        .height(iced::Length::Fixed(crate::style::SMALL_BUTTON_SIZE))
                        .padding(crate::style::PADDING)
                        .style(delete_btn_style)
                        .on_press(TasksMessage::ReqDelNode(entry.id.clone()))
                },
                iced::widget::space()
                    .height(iced::Length::Fill)
                    .width(iced::Length::Fixed(crate::style::PADDING)),
                iced::widget::button(crate::icon::right(right_svg_style))
                    .width(iced::Length::Fixed(crate::style::SMALL_BUTTON_SIZE))
                    .height(iced::Length::Fixed(crate::style::SMALL_BUTTON_SIZE))
                    .padding(crate::style::PADDING)
                    .style(right_btn_style)
                    .on_press(TasksMessage::GoNode(entry.id.clone()))
            ]
            .padding(6)
        ])
        .style(crate::style::container(true))
        .width(iced::Length::Fill)
        .height(iced::Length::Shrink)
        .into()
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
                .on_press(TasksMessage::AddNode {
                    parent: self.current_node_id.clone(),
                    node_data: application::tree::node::NodeData::default(),
                })
        ]
        .width(iced::Length::Fill)
        .height(iced::Length::Fixed(26.0))
        .into()
    }
}
