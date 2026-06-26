pub struct Tasks {
    current_task: automerge::ObjId,
}

#[derive(Debug, Clone)]
pub enum TasksMessage {
    ChangeCurrentTab(automerge::ObjId),
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

    fn update(&mut self, message: Self::Message) {
        match message {
            TasksMessage::ChangeCurrentTab(id) => self.current_task = id,
        }
    }

    fn view_center(&self) -> iced::Element<'_, Self::Message> {
        iced::widget::column![self.current_task(), Self::list_tasks()].into()
    }

    fn view_aside(&self) -> iced::Element<'_, Self::Message> {
        iced::widget::text("(TODO)")
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .align_x(iced::Alignment::Center)
            .align_y(iced::Alignment::Center)
            .into()
    }
}

impl Tasks {
    fn current_task<'a>(&self) -> iced::Element<'a, TasksMessage> {
        iced::widget::column![
            iced::widget::row![
                iced::widget::button(
                    iced::widget::text("<")
                        .width(iced::Length::Fill)
                        .height(iced::Length::Fill)
                        .center(),
                )
                .width(iced::Length::Fixed(48.0))
                .height(iced::Length::Fixed(48.0))
                .style(crate::style::button)
                .on_press(TasksMessage::ChangeCurrentTab(automerge::ObjId::Root)),
                iced::widget::space()
                    .height(iced::Length::Fill)
                    .width(iced::Length::Fixed(4.0)),
                iced::widget::column![iced::widget::text("NAME"), iced::widget::text("DESC"),]
            ]
            .padding(4),
            iced::widget::rule::horizontal(crate::style::BORDER_WIDTH).style(crate::style::border)
        ]
        .height(iced::Length::Shrink)
        .into()
    }

    fn list_tasks<'a>() -> iced::Element<'a, TasksMessage> {
        let mut children: Vec<iced::Element<'a, TasksMessage>> = Vec::new();

        for _ in 0..10 {
            children.push(Self::node());
        }

        iced::widget::scrollable(iced::widget::column(children).spacing(4).padding(4))
            .style(scroll_style)
            .into()
    }

    fn node<'a>() -> iced::Element<'a, TasksMessage> {
        iced::widget::container(
            iced::widget::text("(TODO)")
                .width(iced::Length::Fill)
                .height(iced::Length::Fill)
                .align_x(iced::Alignment::Center)
                .align_y(iced::Alignment::Center),
        )
        .style(|theme| iced::widget::container::Style {
            border: iced::Border {
                color: iced::Color::WHITE,
                width: crate::style::BORDER_WIDTH,
                radius: iced::border::radius(0),
            },
            snap: true,
            ..Default::default()
        })
        .width(iced::Length::Fill)
        .height(iced::Length::Fixed(200.0))
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
        _ => (
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
                    width: width,
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
                    width: width,
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
