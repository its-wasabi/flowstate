pub trait AxisValue: std::fmt::Display + Clone {
    type Value: std::fmt::Display;

    fn to(self) -> Self::Value;
    fn from(val: Self::Value) -> Option<Self>;
}

impl AxisValue for f32 {
    type Value = f32;
    fn to(self) -> Self::Value {
        self
    }

    fn from(val: Self::Value) -> Option<Self> {
        Some(val)
    }
}

impl AxisValue for u32 {
    type Value = u32;
    fn to(self) -> Self::Value {
        self
    }

    fn from(val: Self::Value) -> Option<Self> {
        Some(val)
    }
}

impl AxisValue for chrono::NaiveDate {
    type Value = u32;
    fn to(self) -> Self::Value {
        todo!()
    }

    fn from(val: Self::Value) -> Option<Self> {
        todo!()
    }
}
