pub struct Line<X, Y> {
    pub points: Vec<(X, Y)>,
}

impl<X: super::axis::AxisValue, Y: super::axis::AxisValue> Line<X, Y> {
    pub fn new(points: impl IntoIterator<Item = (X, Y)>) -> Self {
        Self {
            points: points.into_iter().collect(),
        }
    }
}
