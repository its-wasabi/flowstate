use iced::advanced::layout::{self, Layout, Node};
use iced::advanced::renderer;
use iced::advanced::widget::{self, Tree, Widget};
use iced::border::Border;
use iced::{Element, Length, Rectangle, Size, Theme};

// ── Public helpers (used by external code) ──

pub fn border_horizontal<M: 'static>(height: impl Into<Length>) -> Element<'static, M> {
    BorderLine::new(Length::Fill, height.into(), false).into()
}

pub fn border_vertical<M: 'static>(width: impl Into<Length>) -> Element<'static, M> {
    BorderLine::new(width.into(), Length::Fill, false).into()
}

pub fn border_interactive_horizontal<M: 'static>(
    height: impl Into<Length>,
    is_active: bool,
) -> Element<'static, M> {
    BorderLine::new(Length::Fill, height.into(), is_active).into()
}

pub fn border_interactive_vertical<M: 'static>(
    width: impl Into<Length>,
    is_active: bool,
) -> Element<'static, M> {
    BorderLine::new(width.into(), Length::Fill, is_active).into()
}

/// Returns a function that produces a `Border` for the split dividers.
/// Used by `SplitVertical`/`SplitHorizontal` to share the same interactive style.
pub fn divider_style(is_active: bool) -> impl Fn(&Theme) -> Border {
    crate::style::border_interactive(is_active)
}

// ── Internal widget (not exported) ──

struct BorderLine {
    width: Length,
    height: Length,
    is_active: bool,
}

impl BorderLine {
    fn new(width: Length, height: Length, is_active: bool) -> Self {
        Self {
            width,
            height,
            is_active,
        }
    }
}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for BorderLine
where
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(&mut self, _tree: &mut Tree, _renderer: &Renderer, limits: &layout::Limits) -> Node {
        let size = limits.resolve(self.width, self.height, Size::ZERO);
        Node::new(size)
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: iced::mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let border = crate::style::border_interactive(self.is_active)(theme);
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border,
                ..Default::default()
            },
            iced::Color::TRANSPARENT,
        );
    }
}

impl<Message, Renderer> From<BorderLine> for Element<'static, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer + 'static,
{
    fn from(line: BorderLine) -> Self {
        Element::new(line)
    }
}
