#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Point(pub [f64; 2]);

impl Point {
    #[must_use]
    const fn new(x: f64, y: f64) -> Self {
        Self([x, y])
    }

    #[must_use]
    pub const fn x(&self) -> f64 {
        self.0[0]
    }

    #[must_use]
    pub const fn y(&self) -> f64 {
        self.0[1]
    }
}

impl From<Point> for [f64; 2] {
    fn from(p: Point) -> Self {
        p.0
    }
}

impl std::ops::Deref for Point {
    type Target = [f64; 2];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct MinMax<T> {
    pub min: T,
    pub max: T,
}
