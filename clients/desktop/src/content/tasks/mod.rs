pub struct Tasks {
    current_node_id: automerge::ObjId,
}

// TODO: Try using ref for id
#[derive(Debug, Clone)]
pub enum TasksMessage {
    GoBack,
    GoNode(automerge::ObjId),
    DelNode(automerge::ObjId),
    AddNode {
        parent: automerge::ObjId,
        node_data: application::tree::node::NodeData,
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
                    self.current_node_id = parent;
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
                    self.current_node_id = parent;
                }
            }

            TasksMessage::AddNode { parent, node_data } => {
                if core.tree.append_child(&parent, &node_data).is_err() {
                    todo!("IMPLEMENT LOGGING WITH BUILD CFG");
                }
            }

            TasksMessage::NodeNameChange { id, content } => {
                // FIX: Change to cache function - now it spams automerge with changes
                core.tree.change_node_name(&id, content);
            }

            TasksMessage::NodeDescChange { id, content } => {
                // FIX: Change to cache function - now it spams automerge with changes
                core.tree.change_node_desc(&id, content);
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
        iced::widget::text("(TODO)")
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .align_x(iced::Alignment::Center)
            .align_y(iced::Alignment::Center)
            .into()
    }
}

impl Tasks {
    fn current_progress<'a>(
        &self,
        tree: &application::tree::Tree,
    ) -> Option<iced::Element<'a, TasksMessage>> {
        tree.get_progress(&self.current_node_id).map_or_else(
            |_| None,
            |progress| {
                Some(
                    iced::widget::column![
                        iced::widget::container(
                            iced::widget::progress_bar(0.0..=100.0, progress.procentage())
                                .style(crate::style::progress)
                        )
                        .width(iced::Length::Fill)
                        .height(iced::Length::Fixed(crate::style::TOP_BAR_HEIGHT)),
                        iced::widget::rule::horizontal(crate::style::BORDER_WIDTH)
                            .style(crate::style::border)
                    ]
                    .width(iced::Length::Fill)
                    .height(iced::Length::Shrink)
                    .into(),
                )
            },
        )
    }

    fn list_nodes<'a>(
        &self,
        tree: &application::tree::Tree,
    ) -> Option<iced::Element<'a, TasksMessage>> {
        tree.get_children(&self.current_node_id).map_or_else(
            |_| None,
            |children| {
                Some(match children {
                    application::tree::NodeContent::Leaf((node_id, node_data)) => {
                        Self::leaf_node(node_id, &node_data)
                    }

                    application::tree::NodeContent::Inner(nodes) => {
                        let mut children_elements: Vec<iced::Element<'a, TasksMessage>> =
                            Vec::new();
                        for (node_id, node_data) in nodes {
                            children_elements.push(Self::inner_node(node_id, &node_data));
                        }

                        iced::widget::column![
                            self.current_node(tree),
                            iced::widget::scrollable(
                                iced::widget::column(children_elements)
                                    .spacing(6)
                                    .padding(6),
                            )
                            .style(scroll_style)
                            .height(iced::Length::Fill)
                        ]
                        .into()
                    }
                })
            },
        )
    }

    fn current_node<'a>(
        &self,
        tree: &application::tree::Tree,
    ) -> Option<iced::Element<'a, TasksMessage>> {
        tree.get_node(&self.current_node_id).map_or_else(
            |_| None,
            |node_data| {
                let (left_btn_style, left_svg_style) =
                    crate::style::button_with_icon(crate::style::Variant::Default, true);
                Some(
                    iced::widget::column![
                        iced::widget::row![
                            iced::widget::button(crate::icon::left(left_svg_style))
                                .width(iced::Length::Fixed(48.0))
                                .height(iced::Length::Fixed(48.0))
                                .padding(4)
                                .style(left_btn_style)
                                .on_press(TasksMessage::GoBack),
                            iced::widget::space()
                                .height(iced::Length::Fill)
                                .width(iced::Length::Fixed(4.0)),
                            iced::widget::column![
                                iced::widget::text_input("NAME", &node_data.name)
                                    .width(iced::Length::Fill)
                                    .line_height(iced::widget::text::LineHeight::Absolute(
                                        iced::Pixels(14.0)
                                    ))
                                    .padding(5)
                                    .align_x(iced::Alignment::Start)
                                    .on_input({
                                        let id = self.current_node_id.clone();
                                        move |content| TasksMessage::NodeNameChange {
                                            id: id.clone(),
                                            content,
                                        }
                                    })
                                    .style(crate::style::text_input),
                                iced::widget::text_input("DESC", &node_data.desc)
                                    .width(iced::Length::Fill)
                                    .line_height(iced::widget::text::LineHeight::Absolute(
                                        iced::Pixels(14.0)
                                    ))
                                    .padding(5)
                                    .align_x(iced::Alignment::Start)
                                    .on_input({
                                        let id = self.current_node_id.clone();
                                        move |content| TasksMessage::NodeDescChange {
                                            id: id.clone(),
                                            content,
                                        }
                                    })
                                    .style(crate::style::text_input),
                            ]
                        ]
                        .padding(4),
                        iced::widget::rule::horizontal(crate::style::BORDER_WIDTH)
                            .style(crate::style::border)
                    ]
                    .height(iced::Length::Shrink)
                    .into(),
                )
            },
        )
    }

    fn leaf_node<'a>(
        id: automerge::ObjId,
        data: &application::tree::node::NodeData,
    ) -> iced::Element<'a, TasksMessage> {
        let (left_btn_style, left_svg_style) =
            crate::style::button_with_icon(crate::style::Variant::Default, true);
        let (minus_btn_style, minus_svg_style) =
            crate::style::button_with_icon(crate::style::Variant::Warn, true);
        let (plus_btn_style, plus_svg_style) =
            crate::style::button_with_icon(crate::style::Variant::Ok, true);
        let (delete_btn_style, delete_svg_style) =
            crate::style::button_with_icon(crate::style::Variant::Danger, true);
        iced::widget::column![
            iced::widget::row![
                iced::widget::button(crate::icon::left(left_svg_style))
                    .width(iced::Length::Fixed(38.0))
                    .height(iced::Length::Fixed(38.0))
                    .padding(4)
                    .style(left_btn_style)
                    .on_press(TasksMessage::GoBack),
                iced::widget::space()
                    .height(iced::Length::Fill)
                    .width(iced::Length::Fixed(4.0)),
                iced::widget::button(crate::icon::delete(delete_svg_style))
                    .width(iced::Length::Fixed(38.0))
                    .height(iced::Length::Fixed(38.0))
                    .padding(4)
                    .style(delete_btn_style)
                    .on_press(TasksMessage::DelNode(id)),
                iced::widget::space()
                    .height(iced::Length::Fill)
                    .width(iced::Length::Fixed(4.0)),
                iced::widget::button(crate::icon::minus(minus_svg_style))
                    .width(iced::Length::Fixed(38.0))
                    .height(iced::Length::Fixed(38.0))
                    .padding(4)
                    .style(minus_btn_style)
                    .on_press(TasksMessage::GoBack),
                iced::widget::button(crate::icon::plus(plus_svg_style))
                    .width(iced::Length::Fixed(38.0))
                    .height(iced::Length::Fixed(38.0))
                    .padding(4)
                    .style(plus_btn_style)
                    .on_press(TasksMessage::GoBack),
            ]
            .height(iced::Length::Shrink)
            .padding(4),
            iced::widget::rule::horizontal(crate::style::BORDER_WIDTH).style(crate::style::border),
            iced::widget::container(
                iced::widget::text(data.desc.clone())
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
        id: automerge::ObjId,
        data: &application::tree::node::NodeData,
    ) -> iced::Element<'a, TasksMessage> {
        let (right_btn_style, right_svg_style) =
            crate::style::button_with_icon(crate::style::Variant::Default, true);
        let (minus_btn_style, minus_svg_style) =
            crate::style::button_with_icon(crate::style::Variant::Warn, true);
        let (plus_btn_style, plus_svg_style) =
            crate::style::button_with_icon(crate::style::Variant::Ok, true);
        let (delete_btn_style, delete_svg_style) =
            crate::style::button_with_icon(crate::style::Variant::Danger, true);

        iced::widget::container(iced::widget::row![
            iced::widget::text_input("NAME", &data.name)
                .width(iced::Length::Fill)
                .line_height(iced::widget::text::LineHeight::Absolute(iced::Pixels(18.0)))
                .padding(5)
                .align_x(iced::Alignment::Start)
                .on_input({
                    let id = id.clone();
                    move |content| TasksMessage::NodeNameChange {
                        id: id.clone(),
                        content,
                    }
                })
                .style(crate::style::text_input),
            iced::widget::button(crate::icon::minus(minus_svg_style))
                .width(iced::Length::Fixed(28.0))
                .height(iced::Length::Fixed(28.0))
                .padding(4)
                .style(minus_btn_style)
                .on_press(TasksMessage::GoBack),
            iced::widget::button(crate::icon::plus(plus_svg_style))
                .width(iced::Length::Fixed(28.0))
                .height(iced::Length::Fixed(28.0))
                .padding(4)
                .style(plus_btn_style)
                .on_press(TasksMessage::GoBack),
            iced::widget::space()
                .height(iced::Length::Fill)
                .width(iced::Length::Fixed(4.0)),
            iced::widget::button(crate::icon::delete(delete_svg_style))
                .width(iced::Length::Fixed(28.0))
                .height(iced::Length::Fixed(28.0))
                .padding(4)
                .style(delete_btn_style)
                .on_press(TasksMessage::DelNode(id.clone())),
            iced::widget::space()
                .height(iced::Length::Fill)
                .width(iced::Length::Fixed(4.0)),
            iced::widget::button(crate::icon::right(right_svg_style))
                .width(iced::Length::Fixed(28.0))
                .height(iced::Length::Fixed(28.0))
                .padding(4)
                .style(right_btn_style)
                .on_press(TasksMessage::GoNode(id))
        ])
        .style(crate::style::container(true))
        .width(iced::Length::Fill)
        .height(iced::Length::Shrink)
        .padding(6)
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

fn scroll_style(
    _theme: &iced::Theme,
    status: iced::widget::scrollable::Status,
) -> iced::widget::scrollable::Style {
    let (width, handle) = match status {
        iced::widget::scrollable::Status::Hovered { .. } => (
            32.0,
            iced::Background::Color(iced::Color::from_rgb8(180, 180, 180)),
        ),
        iced::widget::scrollable::Status::Dragged { .. } => {
            (8.0, iced::Background::Color(iced::Color::WHITE))
        }
        iced::widget::scrollable::Status::Active { .. } => (
            8.0,
            iced::Background::Color(iced::Color::from_rgba8(255, 255, 255, 0.4)),
        ),
    };

    iced::widget::scrollable::Style {
        container: iced::widget::container::Style::default(),

        vertical_rail: iced::widget::scrollable::Rail {
            background: Some(iced::Background::Color(iced::Color::TRANSPARENT)),
            border: iced::Border::default(),
            scroller: iced::widget::scrollable::Scroller {
                background: handle,
                border: iced::Border {
                    radius: 0.0.into(),
                    width,
                    ..Default::default()
                },
            },
        },

        horizontal_rail: iced::widget::scrollable::Rail {
            background: Some(iced::Background::Color(iced::Color::TRANSPARENT)),
            border: iced::Border::default(),
            scroller: iced::widget::scrollable::Scroller {
                background: handle,
                border: iced::Border {
                    radius: 0.0.into(),
                    width,
                    ..Default::default()
                },
            },
        },

        gap: None,

        auto_scroll: iced::widget::scrollable::AutoScroll {
            background: iced::Background::Color(iced::Color::TRANSPARENT),
            border: iced::Border::default(),
            shadow: iced::Shadow::default(),
            icon: iced::Color::TRANSPARENT,
        },
    }
}
