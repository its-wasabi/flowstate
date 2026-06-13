use iced::advanced::layout::{self, Layout, Node};
use iced::advanced::renderer;
use iced::advanced::widget::{self, Tree, Widget};
use iced::advanced::{Clipboard, Shell};
use iced::event::{self, Event};
use iced::mouse;
use iced::{Element, Length, Point, Rectangle, Size, Theme};

// ────────────────────── SplitVertical ──────────────────────

pub struct SplitVertical<'a, Message, Renderer> {
    first: Element<'a, Message, Theme, Renderer>,
    second: Element<'a, Message, Theme, Renderer>,
    ratio: f32,
    min_space_first: f32,
    min_space_second: f32,
    on_change: Box<dyn Fn(f32) -> Message + 'a>,
    drag_area: f32,
}

impl<'a, Message, Renderer> SplitVertical<'a, Message, Renderer> {
    pub fn new<F>(
        first: impl Into<Element<'a, Message, Theme, Renderer>>,
        second: impl Into<Element<'a, Message, Theme, Renderer>>,
        ratio: f32,
        min_first: f32,
        min_second: f32,
        on_change: F,
    ) -> Self
    where
        F: Fn(f32) -> Message + 'a,
    {
        Self {
            first: first.into(),
            second: second.into(),
            ratio: ratio.clamp(0.0, 1.0),
            min_space_first: min_first,
            min_space_second: min_second,
            drag_area: 10.0,
            on_change: Box::new(on_change),
        }
    }
}

#[derive(Default, Clone, Copy)]
struct State {
    is_dragging: bool,
}

impl<'a, Message, Renderer> Widget<Message, Theme, Renderer>
    for SplitVertical<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn tag(&self) -> widget::tree::Tag {
        widget::tree::Tag::of::<State>()
    }

    fn state(&self) -> widget::tree::State {
        widget::tree::State::new(State::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.first), Tree::new(&self.second)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[&self.first, &self.second]);
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Fill,
        }
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) -> Node {
        let limits = limits.width(Length::Fill).height(Length::Fill);
        let size = limits.resolve(Length::Fill, Length::Fill, Size::ZERO);

        let left_width = size.width * self.ratio;
        let right_width = size.width - left_width;

        let left_limits = limits.max_width(left_width);
        let right_limits = limits.max_width(right_width);

        let left_node =
            self.first
                .as_widget_mut()
                .layout(&mut tree.children[0], renderer, &left_limits);

        let right_node =
            self.second
                .as_widget_mut()
                .layout(&mut tree.children[1], renderer, &right_limits);

        let right_node = right_node.move_to(Point::new(left_width, 0.0));

        Node::with_children(size, vec![left_node, right_node])
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<State>();
        let bounds = layout.bounds();

        let divider_x = bounds.x + (bounds.width * self.ratio);
        let drag_rect = Rectangle {
            x: divider_x - (self.drag_area / 2.0),
            y: bounds.y,
            width: self.drag_area,
            height: bounds.height,
        };

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if let Some(cursor_pos) = cursor.position() {
                    if drag_rect.contains(cursor_pos) {
                        state.is_dragging = true;
                        shell.capture_event();
                        return;
                    }
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                if state.is_dragging {
                    state.is_dragging = false;
                    shell.capture_event();
                    return;
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { position }) => {
                if state.is_dragging {
                    let min_first_px = self.min_space_first * bounds.width;
                    let min_second_px = self.min_space_second * bounds.width;

                    let min_bound = bounds.x + min_first_px;
                    let max_bound = bounds.x + bounds.width - min_second_px;

                    let clamped_x = position.x.clamp(min_bound, max_bound);
                    let new_ratio = (clamped_x - bounds.x) / bounds.width;

                    shell.publish((self.on_change)(new_ratio));
                    shell.capture_event();
                    return;
                }
            }
            _ => {}
        }

        let mut layouts = layout.children();

        self.first.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layouts.next().unwrap(),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        self.second.as_widget_mut().update(
            &mut tree.children[1],
            event,
            layouts.next().unwrap(),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let state = tree.state.downcast_ref::<State>();
        let bounds = layout.bounds();
        let divider_x = bounds.x + (bounds.width * self.ratio);

        let drag_rect = Rectangle {
            x: divider_x - (self.drag_area / 2.0),
            y: bounds.y,
            width: self.drag_area,
            height: bounds.height,
        };

        let is_hovering = cursor
            .position()
            .map(|pos| drag_rect.contains(pos))
            .unwrap_or(false);

        if state.is_dragging || is_hovering {
            mouse::Interaction::ResizingHorizontally
        } else {
            let mut layouts = layout.children();
            let left_interaction = self.first.as_widget().mouse_interaction(
                &tree.children[0],
                layouts.next().unwrap(),
                cursor,
                viewport,
                renderer,
            );
            let right_interaction = self.second.as_widget().mouse_interaction(
                &tree.children[1],
                layouts.next().unwrap(),
                cursor,
                viewport,
                renderer,
            );

            left_interaction.max(right_interaction)
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let mut layouts = layout.children();
        let left_layout = layouts.next().unwrap();
        let right_layout = layouts.next().unwrap();

        self.first.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            left_layout,
            cursor,
            viewport,
        );

        self.second.as_widget().draw(
            &tree.children[1],
            renderer,
            theme,
            style,
            right_layout,
            cursor,
            viewport,
        );

        let bounds = layout.bounds();
        let divider_x = bounds.x + (bounds.width * self.ratio);

        let is_hovering = cursor
            .position()
            .map(|pos| {
                let drag_rect = Rectangle {
                    x: divider_x - (self.drag_area / 2.0),
                    y: bounds.y,
                    width: self.drag_area,
                    height: bounds.height,
                };
                drag_rect.contains(pos)
            })
            .unwrap_or(false);

        let state = tree.state.downcast_ref::<State>();
        let is_active = state.is_dragging || is_hovering;

        let border = crate::widgets::border::divider_style(is_active)(theme);

        renderer.fill_quad(
            renderer::Quad {
                bounds: Rectangle {
                    x: divider_x - (crate::style::BORDER_WIDTH / 2.0),
                    y: bounds.y,
                    width: crate::style::BORDER_WIDTH,
                    height: bounds.height,
                },
                border,
                ..renderer::Quad::default()
            },
            iced::Color::TRANSPARENT,
        );
    }
}

impl<'a, Message: 'a, Renderer> From<SplitVertical<'a, Message, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer + 'a,
{
    fn from(split: SplitVertical<'a, Message, Renderer>) -> Self {
        Element::new(split)
    }
}

// ────────────────────── SplitHorizontal ──────────────────────

pub struct SplitHorizontal<'a, Message, Renderer> {
    first: Element<'a, Message, Theme, Renderer>,
    second: Element<'a, Message, Theme, Renderer>,
    ratio: f32,
    min_space_first: f32,
    min_space_second: f32,
    on_change: Box<dyn Fn(f32) -> Message + 'a>,
    drag_area: f32,
}

impl<'a, Message, Renderer> SplitHorizontal<'a, Message, Renderer> {
    pub fn new<F>(
        first: impl Into<Element<'a, Message, Theme, Renderer>>,
        second: impl Into<Element<'a, Message, Theme, Renderer>>,
        ratio: f32,
        min_first: f32,
        min_second: f32,
        on_change: F,
    ) -> Self
    where
        F: Fn(f32) -> Message + 'a,
    {
        Self {
            first: first.into(),
            second: second.into(),
            ratio: ratio.clamp(0.0, 1.0),
            min_space_first: min_first,
            min_space_second: min_second,
            drag_area: 10.0,
            on_change: Box::new(on_change),
        }
    }
}

impl<'a, Message, Renderer> Widget<Message, Theme, Renderer>
    for SplitHorizontal<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn tag(&self) -> widget::tree::Tag {
        widget::tree::Tag::of::<State>()
    }

    fn state(&self) -> widget::tree::State {
        widget::tree::State::new(State::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.first), Tree::new(&self.second)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[&self.first, &self.second]);
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Fill,
        }
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) -> Node {
        let limits = limits.width(Length::Fill).height(Length::Fill);
        let size = limits.resolve(Length::Fill, Length::Fill, Size::ZERO);

        let top_height = size.height * self.ratio;
        let bottom_height = size.height - top_height;

        let top_limits = limits.max_height(top_height);
        let bottom_limits = limits.max_height(bottom_height);

        let top_node =
            self.first
                .as_widget_mut()
                .layout(&mut tree.children[0], renderer, &top_limits);

        let bottom_node =
            self.second
                .as_widget_mut()
                .layout(&mut tree.children[1], renderer, &bottom_limits);

        let bottom_node = bottom_node.move_to(Point::new(0.0, top_height));

        Node::with_children(size, vec![top_node, bottom_node])
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<State>();
        let bounds = layout.bounds();

        let divider_y = bounds.y + (bounds.height * self.ratio);
        let drag_rect = Rectangle {
            x: bounds.x,
            y: divider_y - (self.drag_area / 2.0),
            width: bounds.width,
            height: self.drag_area,
        };

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if let Some(cursor_pos) = cursor.position() {
                    if drag_rect.contains(cursor_pos) {
                        state.is_dragging = true;
                        shell.capture_event();
                        return;
                    }
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                if state.is_dragging {
                    state.is_dragging = false;
                    shell.capture_event();
                    return;
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { position }) => {
                if state.is_dragging {
                    let min_first_px = self.min_space_first * bounds.height;
                    let min_second_px = self.min_space_second * bounds.height;

                    let min_y = bounds.y + min_first_px;
                    let max_y = bounds.y + bounds.height - min_second_px;

                    let clamped_y = position.y.clamp(min_y, max_y);
                    let new_ratio = (clamped_y - bounds.y) / bounds.height;

                    shell.publish((self.on_change)(new_ratio));
                    shell.capture_event();
                    return;
                }
            }
            _ => {}
        }

        let mut layouts = layout.children();

        self.first.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layouts.next().unwrap(),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        self.second.as_widget_mut().update(
            &mut tree.children[1],
            event,
            layouts.next().unwrap(),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let state = tree.state.downcast_ref::<State>();
        let bounds = layout.bounds();
        let divider_y = bounds.y + (bounds.height * self.ratio);

        let drag_rect = Rectangle {
            x: bounds.x,
            y: divider_y - (self.drag_area / 2.0),
            width: bounds.width,
            height: self.drag_area,
        };

        let is_hovering = cursor
            .position()
            .map(|pos| drag_rect.contains(pos))
            .unwrap_or(false);

        if state.is_dragging || is_hovering {
            mouse::Interaction::ResizingVertically
        } else {
            let mut layouts = layout.children();
            let top_interaction = self.first.as_widget().mouse_interaction(
                &tree.children[0],
                layouts.next().unwrap(),
                cursor,
                viewport,
                renderer,
            );
            let bottom_interaction = self.second.as_widget().mouse_interaction(
                &tree.children[1],
                layouts.next().unwrap(),
                cursor,
                viewport,
                renderer,
            );

            top_interaction.max(bottom_interaction)
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let mut layouts = layout.children();
        let top_layout = layouts.next().unwrap();
        let bottom_layout = layouts.next().unwrap();

        self.first.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            top_layout,
            cursor,
            viewport,
        );

        self.second.as_widget().draw(
            &tree.children[1],
            renderer,
            theme,
            style,
            bottom_layout,
            cursor,
            viewport,
        );

        let bounds = layout.bounds();
        let divider_y = bounds.y + (bounds.height * self.ratio);

        let is_hovering = cursor
            .position()
            .map(|pos| {
                let drag_rect = Rectangle {
                    x: bounds.x,
                    y: divider_y - (self.drag_area / 2.0),
                    width: bounds.width,
                    height: self.drag_area,
                };
                drag_rect.contains(pos)
            })
            .unwrap_or(false);

        let state = tree.state.downcast_ref::<State>();
        let is_active = state.is_dragging || is_hovering;

        let border = crate::widgets::border::divider_style(is_active)(theme);

        renderer.fill_quad(
            renderer::Quad {
                bounds: Rectangle {
                    x: bounds.x,
                    y: divider_y - (crate::style::BORDER_WIDTH / 2.0),
                    width: bounds.width,
                    height: crate::style::BORDER_WIDTH,
                },
                border,
                ..renderer::Quad::default()
            },
            iced::Color::TRANSPARENT,
        );
    }
}

impl<'a, Message: 'a, Renderer> From<SplitHorizontal<'a, Message, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer + 'a,
{
    fn from(split: SplitHorizontal<'a, Message, Renderer>) -> Self {
        Element::new(split)
    }
}
