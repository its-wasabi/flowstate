pub struct Tasks {
    current_node_id: automerge::ObjId,
}

#[derive(Debug, Clone)]
pub enum TasksMessage {
    GoBack,
    GoNode(automerge::ObjId),
    DelNode(automerge::ObjId),
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
        }
    }
}

impl crate::Display for Tasks {
    type Message = TasksMessage;

    fn update(&mut self, message: Self::Message, core: &mut application::Core) {
        match message {
            TasksMessage::GoBack => {
                println!("GoBack");
                if let Ok(parent) = core.tree.get_parent(&self.current_node_id) {
                    self.current_node_id = parent.clone();
                } else {
                    println!("FAIL");
                }
            }

            TasksMessage::GoNode(id) => self.current_node_id = id,

            TasksMessage::DelNode(id) => {
                core.tree.delete(&id);
                if id == self.current_node_id
                    && let Ok(parent) = core.tree.get_parent(&self.current_node_id)
                {
                    self.current_node_id = parent.clone();
                }
            }

            TasksMessage::AddNode { parent, node_data } => {
                if core.tree.append_child(&parent, &node_data).is_err() {
                    unimplemented!("Logging with build cfg")
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

    fn view_center(&self, core: &application::Core) -> iced::Element<'_, Self::Message> {
        iced::widget::column![
            self.current_progress(&core.tree),
            self.list_nodes(&core.tree),
            self.add_node()
        ]
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
    fn current_progress(&self, tree: &application::tree::Tree) -> iced::Element<'_, TasksMessage> {
        if let Ok(progress) = tree.get_progress(&self.current_node_id) {
            let procentage = progress.procentage();
            iced::widget::column![
                iced::widget::container(
                    iced::widget::stack![
                        iced::widget::progress_bar(0.0..=100.0, procentage)
                            .style(crate::style::progress),
                        iced::widget::text!(" [{procentage:.2}%]")
                            .width(iced::Length::Fill)
                            .height(iced::Length::Fill)
                            .align_x(iced::Alignment::Start)
                            .align_y(iced::Alignment::Center)
                            .style(crate::style::text(true))
                            .size(14),
                    ]
                    .width(iced::Length::Fill)
                    .height(iced::Length::Shrink),
                )
                .width(iced::Length::Fill)
                .height(iced::Length::Fixed(crate::style::TOP_BAR_HEIGHT)),
                iced::widget::rule::horizontal(crate::style::BORDER_WIDTH)
                    .style(crate::style::border)
            ]
            .width(iced::Length::Fill)
            .height(iced::Length::Shrink)
            .into()
        } else {
            iced::widget::space()
                .width(iced::Length::Fill)
                .height(iced::Length::Fixed(
                    crate::style::TOP_BAR_HEIGHT + crate::style::BORDER_WIDTH,
                ))
                .into()
        }
    }

    fn list_nodes<'a>(&self, tree: &application::tree::Tree) -> iced::Element<'a, TasksMessage> {
        match tree.get_node(&self.current_node_id) {
            Ok(application::tree::NodeContent::Root { children }) => iced::widget::scrollable(
                iced::widget::column(
                    children
                        .iter()
                        .map(|children| Self::inner_node(&children.0, &children.1)),
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

            Ok(application::tree::NodeContent::Leaf { id, node }) => Self::leaf_node(id, &node),

            Ok(application::tree::NodeContent::Inner { id, node, children }) => {
                iced::widget::column![
                    self.current_node(id, node),
                    iced::widget::scrollable(
                        iced::widget::column(
                            children
                                .iter()
                                .map(|children| { Self::inner_node(&children.0, &children.1) })
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
                .into()
            }

            Err(_) => iced::widget::space()
                .width(iced::Length::Fill)
                .height(iced::Length::Fill)
                .into(),
        }
    }

    fn current_node<'a>(
        &self,
        id: automerge::ObjId,
        node: application::tree::node::NodeData,
    ) -> iced::Element<'a, TasksMessage> {
        let name_id = self.current_node_id.clone();
        let desc_id = self.current_node_id.clone();

        let (left_btn_style, left_svg_style) =
            crate::style::button_with_icon(crate::style::Variant::Default, true);

        iced::widget::column![
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

        iced::widget::column![
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
                iced::widget::button(crate::icon::delete(delete_svg_style))
                    .width(iced::Length::Fixed(crate::style::SMALL_BUTTON_SIZE))
                    .height(iced::Length::Fixed(crate::style::SMALL_BUTTON_SIZE))
                    .padding(crate::style::PADDING)
                    .style(delete_btn_style)
                    .on_press(TasksMessage::DelNode(id.clone())),
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
                    .on_press(TasksMessage::NodeCompletedChange { id, delta: 1 }),
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
        id: &automerge::ObjId,
        node: &application::tree::node::NodeData,
    ) -> iced::Element<'a, TasksMessage> {
        let (right_btn_style, right_svg_style) =
            crate::style::button_with_icon(crate::style::Variant::Default, true);
        let (minus_btn_style, minus_svg_style) =
            crate::style::button_with_icon(crate::style::Variant::Warn, true);
        let (plus_btn_style, plus_svg_style) =
            crate::style::button_with_icon(crate::style::Variant::Ok, true);
        let (delete_btn_style, delete_svg_style) =
            crate::style::button_with_icon(crate::style::Variant::Danger, true);

        let name_id = id.clone();

        iced::widget::container(iced::widget::column![
            iced::widget::container(
                iced::widget::progress_bar(0.0..=100.0, node.progress.procentage())
                    .style(crate::style::progress)
            )
            .width(iced::Length::Fill)
            .height(iced::Length::Fixed(crate::style::TOP_BAR_HEIGHT / 2.0)),
            iced::widget::rule::horizontal(crate::style::BORDER_WIDTH).style(crate::style::border),
            iced::widget::row![
                iced::widget::text_input("NAME", &node.name)
                    .width(iced::Length::Fill)
                    .line_height(iced::widget::text::LineHeight::Absolute(iced::Pixels(
                        crate::style::SMALL_BUTTON_SIZE
                    )))
                    .padding(0)
                    .align_x(iced::Alignment::Start)
                    .on_input(move |content| TasksMessage::NodeNameChange {
                        id: name_id.clone(),
                        content,
                    })
                    .style(crate::style::text_input),
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
                    .on_press(TasksMessage::DelNode(id.clone())),
                iced::widget::space()
                    .height(iced::Length::Fill)
                    .width(iced::Length::Fixed(crate::style::PADDING)),
                iced::widget::button(crate::icon::right(right_svg_style))
                    .width(iced::Length::Fixed(crate::style::SMALL_BUTTON_SIZE))
                    .height(iced::Length::Fixed(crate::style::SMALL_BUTTON_SIZE))
                    .padding(crate::style::PADDING)
                    .style(right_btn_style)
                    .on_press(TasksMessage::GoNode(id.clone()))
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
        .height(iced::Length::Fixed(24.0))
        .into()
    }
}
