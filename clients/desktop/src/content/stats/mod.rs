use std::cell::RefCell;
use std::rc::Rc;

use iced::mouse;
use iced::widget::canvas::{Action, Frame, Geometry, Path, Program, Stroke};
use iced::{Color, Point, Rectangle, Renderer, Theme};

// ─── Message the chart widget emits ───
#[derive(Debug, Clone)]
pub enum ChartMessage {
    ViewChanged { pan_x: f32, zoom_x: f32 },
}

// ─── Internal drag state (canvas only) ───
#[derive(Default)]
pub struct ChartState {
    dragging: bool,
    drag_start_x: f32,
    start_pan: f32,
}

// ─── Shared mutable chart data ───
#[derive(Clone)]
pub struct ChartData {
    pub line1: Vec<u32>,
    pub line2: Vec<u32>,
    pub zoom_x: f32,
    pub pan_x: f32,
}

impl ChartData {
    pub const fn new(line1: Vec<u32>, line2: Vec<u32>) -> Self {
        Self {
            line1,
            line2,
            zoom_x: 1.0,
            pan_x: 0.0,
        }
    }

    fn data_bounds(&self) -> (f32, f32, f32, f32) {
        let max_len = self.line1.len().max(self.line2.len());
        let min_x = 0.0;
        let max_x = if max_len > 0 {
            (max_len - 1) as f32
        } else {
            0.0
        };

        let mut min_y = u32::MAX;
        let mut max_y = u32::MIN;
        for &y in &self.line1 {
            min_y = min_y.min(y);
            max_y = max_y.max(y);
        }
        for &y in &self.line2 {
            min_y = min_y.min(y);
            max_y = max_y.max(y);
        }
        // If no points, set a default visible range
        let (min_y_f32, max_y_f32) = if min_y > max_y {
            (0.0, 1.0)
        } else {
            (min_y as f32, max_y as f32)
        };
        (min_x, max_x, min_y_f32, max_y_f32)
    }

    pub fn clamp_viewport(&mut self) {
        let (min_x, max_x, _, _) = self.data_bounds();
        let data_width = max_x - min_x;
        self.zoom_x = self.zoom_x.clamp(0.01, data_width);
        self.pan_x = self.pan_x.clamp(min_x, max_x - self.zoom_x);
    }
}

// ─── The chart widget (uses Rc<RefCell<ChartData>>) ───
#[derive(Clone)]
pub struct MinimalChart {
    data: Rc<RefCell<ChartData>>,
}

impl MinimalChart {
    pub const fn new(data: Rc<RefCell<ChartData>>) -> Self {
        Self { data }
    }
}

impl Program<ChartMessage> for MinimalChart {
    type State = ChartState;

    fn update(
        &self,
        state: &mut Self::State,
        event: &iced::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<Action<ChartMessage>> {
        let width = bounds.width;
        if width <= 0.0 {
            return None;
        }
        let data = self.data.borrow();

        if let iced::Event::Mouse(mouse_event) = event {
            match mouse_event {
                mouse::Event::ButtonPressed(mouse::Button::Left) => {
                    if let Some(pos) = cursor.position_in(bounds) {
                        state.dragging = true;
                        state.drag_start_x = pos.x;
                        state.start_pan = data.pan_x;
                        return Some(Action::capture());
                    }
                }
                mouse::Event::ButtonReleased(mouse::Button::Left) => {
                    state.dragging = false;
                    return Some(Action::capture());
                }
                mouse::Event::CursorMoved { .. } if state.dragging => {
                    if let Some(pos) = cursor.position_in(bounds) {
                        let delta_x = pos.x - state.drag_start_x;
                        let scale = data.zoom_x / width;
                        let data_delta = delta_x * scale;
                        let (min_x, max_x, _, _) = data.data_bounds();
                        let new_pan =
                            (state.start_pan - data_delta).clamp(min_x, max_x - data.zoom_x);
                        return Some(Action::publish(ChartMessage::ViewChanged {
                            pan_x: new_pan,
                            zoom_x: data.zoom_x,
                        }));
                    }
                }
                mouse::Event::WheelScrolled { delta } => {
                    if let Some(pos) = cursor.position_in(bounds) {
                        let dy = match delta {
                            mouse::ScrollDelta::Lines { y, .. }
                            | mouse::ScrollDelta::Pixels { y, .. } => *y,
                        };
                        let cursor_ratio = pos.x / width;
                        let zoom_factor = dy.mul_add(0.1, 1.0);
                        let old_zoom = data.zoom_x;
                        let new_zoom_raw = (old_zoom * zoom_factor).max(0.01);
                        let cursor_data_x = cursor_ratio.mul_add(old_zoom, data.pan_x);
                        let tentative_pan = cursor_ratio.mul_add(-new_zoom_raw, cursor_data_x);
                        let (min_x, max_x, _, _) = data.data_bounds();
                        let data_width = max_x - min_x;
                        let final_zoom = new_zoom_raw.min(data_width).max(0.01);
                        let final_pan = tentative_pan.clamp(min_x, max_x - final_zoom);
                        return Some(Action::publish(ChartMessage::ViewChanged {
                            pan_x: final_pan,
                            zoom_x: final_zoom,
                        }));
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let data = self.data.borrow();
        let mut frame = Frame::new(renderer, bounds.size());
        let (data_min_x, data_max_x, data_min_y, data_max_y) = data.data_bounds();

        if data_max_y == data_min_y || data_max_x == data_min_x {
            return vec![frame.into_geometry()];
        }

        let view_left = data.pan_x;
        let view_right = data.pan_x + data.zoom_x;
        if view_right <= view_left {
            return vec![frame.into_geometry()];
        }

        // Get theme colors
        let palette = theme.palette();
        let line1_color = palette.warning; // green
        let line2_color = palette.success; // orange

        let x_scale = bounds.width / (view_right - view_left);
        let y_scale = bounds.height / (data_max_y - data_min_y);
        let to_canvas = |x: f32, y: f32| -> Point {
            Point::new(
                (x - view_left) * x_scale,
                (y - data_min_y).mul_add(-y_scale, bounds.height),
            )
        };

        // Line 1 (using success color)
        if data.line1.len() > 1 {
            let path = Path::new(|builder| {
                let mut it = data.line1.iter().enumerate();
                if let Some((i, &y)) = it.next() {
                    builder.move_to(to_canvas(i as f32, y as f32));
                    for (i, &y) in it {
                        builder.line_to(to_canvas(i as f32, y as f32));
                    }
                }
            });
            frame.stroke(
                &path,
                //TODO: Extract width to some setting
                Stroke::default().with_width(2.8).with_color(line1_color),
            );
        }

        // Line 2 (using warning color)
        if data.line2.len() > 1 {
            let path = Path::new(|builder| {
                let mut it = data.line2.iter().enumerate();
                if let Some((i, &y)) = it.next() {
                    builder.move_to(to_canvas(i as f32, y as f32));
                    for (i, &y) in it {
                        builder.line_to(to_canvas(i as f32, y as f32));
                    }
                }
            });
            frame.stroke(
                &path,
                //TODO: Extract width to some setting
                Stroke::default().with_width(2.8).with_color(line2_color),
            );
        }

        vec![frame.into_geometry()]
    }
}

// ─── Stats message ───
#[derive(Debug, Clone)]
pub enum StatsMessage {
    Chart1Message(ChartMessage),
    Chart2Message(ChartMessage),
    SplitResized(iced_resizable_split::State),
}

// ─── The Stats view ───
pub struct Stats {
    pub plot1: Rc<RefCell<ChartData>>,
    pub plot2: Rc<RefCell<ChartData>>,
    split_ratio: iced_resizable_split::State,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            plot1: Rc::new(RefCell::new(ChartData::new(
                vec![18, 20, 21, 9, 2],
                vec![12, 10, 4, 2, 14],
            ))),
            plot2: Rc::new(RefCell::new(ChartData::new(
                vec![18, 20, 21, 9, 2],
                vec![12, 10, 4, 2, 14],
            ))),
            split_ratio: iced_resizable_split::State::new(0.5, 0.1, 0.9),
        }
    }
}

impl crate::Display for Stats {
    type Message = StatsMessage;

    fn update(&mut self, message: Self::Message) {
        match message {
            StatsMessage::SplitResized(new_state) => self.split_ratio.update(new_state),
            StatsMessage::Chart1Message(ChartMessage::ViewChanged { pan_x, zoom_x }) => {
                let mut data = self.plot1.borrow_mut();
                data.pan_x = pan_x;
                data.zoom_x = zoom_x;
                data.clamp_viewport();
            }
            StatsMessage::Chart1Message(chart_msg) => {}
            StatsMessage::Chart2Message(ChartMessage::ViewChanged { pan_x, zoom_x }) => {
                let mut data = self.plot2.borrow_mut();
                data.pan_x = pan_x;
                data.zoom_x = zoom_x;
                data.clamp_viewport();
            }
            StatsMessage::Chart2Message(chart_msg) => {}
        }
    }

    fn view_center(&self) -> iced::Element<'_, Self::Message> {
        let top: iced::Element<'_, ChartMessage> =
            iced::widget::canvas::Canvas::new(MinimalChart::new(self.plot1.clone()))
                .width(iced::Length::Fill)
                .height(iced::Length::Fill)
                .into();
        let top = top.map(StatsMessage::Chart1Message);

        let bottom: iced::Element<'_, ChartMessage> =
            iced::widget::canvas::Canvas::new(MinimalChart::new(self.plot2.clone()))
                .width(iced::Length::Fill)
                .height(iced::Length::Fill)
                .into();
        let bottom = bottom.map(StatsMessage::Chart2Message);

        iced_resizable_split::split_horizontal(
            top,
            bottom,
            self.split_ratio,
            StatsMessage::SplitResized,
        )
        .style(crate::style::split_border)
        .into()
    }

    fn view_aside(&self) -> iced::Element<'_, Self::Message> {
        iced::widget::text("HELLO ASIDE").into()
    }

    // fn subscription(&self) -> iced::Subscription<Self::Message> {
    //     self.split_ratio
    //         .subscription_horizontal()
    //         .map(StatsMessage::SplitResized)
    // }
}
