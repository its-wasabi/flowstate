mod axis;
pub mod line;

pub struct LineChart<X: axis::AxisValue, Y: axis::AxisValue, const N: usize> {
    lines: [line::Line<X, Y>; N],
}

impl<X: axis::AxisValue, Y: axis::AxisValue, const N: usize> LineChart<X, Y, N> {
    pub fn new(lines: [line::Line<X, Y>; N]) -> Self {
        Self { lines }
    }
}
