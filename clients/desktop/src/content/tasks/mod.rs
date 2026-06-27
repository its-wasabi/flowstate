pub struct Tasks {
    current_task: automerge::ObjId,
}

#[derive(Debug, Clone)]
pub enum TasksMessage {
    GoBack,
    GoNode(automerge::ObjId),
    AddNode {
        parent: automerge::ObjId,
        node_data: application::tree::node::NodeData,
    },
}

impl Tasks {
    pub const fn new() -> Self {
        Self {
            current_task: automerge::ObjId::Root,
        }
    }
}

impl crate::Display for Tasks {
    type Message = TasksMessage;

    fn update(&mut self, message: Self::Message, core: &mut application::Core) {
        match message {
            TasksMessage::GoBack => {
                println!("GoBack");
                if let Ok(parent) = core.tree.get_parent(&self.current_task) {
                    self.current_task = parent;
                } else {
                    println!("FAIL");
                }
            }

            TasksMessage::GoNode(id) => self.current_task = id,

            TasksMessage::AddNode { parent, node_data } => {
                core.tree
                    .append_child(&parent, &node_data)
                    .expect("Failed to add child");
            }
        }
    }

    fn view_center(&self, core: &application::Core) -> iced::Element<'_, Self::Message> {
        iced::widget::column![
            self.current_progress(&core.tree),
            self.list_tasks(&core.tree),
            self.add_task()
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
        tree.get_progress(&self.current_task).map_or_else(
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

    fn list_tasks<'a>(
        &self,
        tree: &application::tree::Tree,
    ) -> Option<iced::Element<'a, TasksMessage>> {
        tree.get_children(&self.current_task).map_or_else(
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
                            self.current_task(tree),
                            iced::widget::scrollable(
                                iced::widget::column(children_elements)
                                    .spacing(4)
                                    .padding(4),
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

    fn current_task<'a>(
        &self,
        tree: &application::tree::Tree,
    ) -> Option<iced::Element<'a, TasksMessage>> {
        tree.get_node(&self.current_task).map_or_else(
            |_| None,
            |node_data| {
                Some(
                    iced::widget::column![
                        iced::widget::row![
                            iced::widget::button(crate::icon::left())
                                .width(iced::Length::Fixed(48.0))
                                .height(iced::Length::Fixed(48.0))
                                .padding(4)
                                .style(crate::style::button(true))
                                .on_press(TasksMessage::GoBack),
                            iced::widget::space()
                                .height(iced::Length::Fill)
                                .width(iced::Length::Fixed(4.0)),
                            iced::widget::column![
                                iced::widget::text(node_data.name),
                                iced::widget::text(node_data.desc),
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
        iced::widget::column![
            iced::widget::row![
                iced::widget::button(crate::icon::left())
                    .width(iced::Length::Fixed(34.0))
                    .height(iced::Length::Fixed(34.0))
                    .padding(4)
                    .style(crate::style::button(true))
                    .on_press(TasksMessage::GoBack)
            ]
            .height(iced::Length::Shrink)
            .padding(4),
            iced::widget::rule::horizontal(crate::style::BORDER_WIDTH).style(crate::style::border),
            iced::widget::container(
                iced::widget::text("(TODO)")
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
        iced::widget::container(iced::widget::row![
            iced::widget::text("(TODO)")
                .width(iced::Length::Fill)
                .height(iced::Length::Fill)
                .align_x(iced::Alignment::Start)
                .align_y(iced::Alignment::Center),
            iced::widget::button(crate::icon::minus())
                .width(iced::Length::Fixed(24.0))
                .height(iced::Length::Fixed(24.0))
                .padding(4)
                .style(crate::style::button(true))
                .on_press(TasksMessage::GoBack),
            iced::widget::button(crate::icon::plus())
                .width(iced::Length::Fixed(24.0))
                .height(iced::Length::Fixed(24.0))
                .padding(4)
                .style(crate::style::button(true))
                .on_press(TasksMessage::GoBack),
            iced::widget::space()
                .height(iced::Length::Fill)
                .width(iced::Length::Fixed(4.0)),
            iced::widget::button(crate::icon::right())
                .width(iced::Length::Fixed(24.0))
                .height(iced::Length::Fixed(24.0))
                .padding(4)
                .style(crate::style::button(true))
                .on_press(TasksMessage::GoNode(id))
        ])
        .style(crate::style::container(true))
        .width(iced::Length::Fill)
        .height(iced::Length::Shrink)
        .padding(4)
        .into()
    }

    fn add_task<'a>(&self) -> iced::Element<'a, TasksMessage> {
        iced::widget::column![
            iced::widget::rule::horizontal(crate::style::BORDER_WIDTH).style(crate::style::border),
            iced::widget::button(crate::icon::plus())
                .width(iced::Length::Fill)
                .height(iced::Length::Fill)
                .padding(4)
                .style(crate::style::button(false))
                .on_press(TasksMessage::AddNode {
                    parent: self.current_task.clone(),
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
