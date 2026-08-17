#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Total(u32);

impl Total {
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn value(self) -> u32 {
        self.0
    }

    #[must_use]
    pub fn change_by(self, by: i64) -> Self {
        let new_val = i64::from(self.0).saturating_add(by).max(0);
        Self(new_val.try_into().unwrap_or(u32::MAX))
    }
}

impl From<i64> for Total {
    fn from(value: i64) -> Self {
        Self(value.clamp(0, u32::MAX as i64) as u32)
    }
}

impl Default for Total {
    fn default() -> Self {
        Self(10)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Completed(u32);

impl Completed {
    #[must_use]
    pub fn new(value: u32, total: Total) -> Self {
        Self(value.min(total.0))
    }

    pub(super) fn from_i64(value: i64, total: Total) -> Self {
        let safe_val: u32 = value
            .clamp(0, i64::from(u32::MAX))
            .try_into()
            .unwrap_or(u32::MAX);
        Self::new(safe_val, total)
    }

    #[must_use]
    pub const fn value(self) -> u32 {
        self.0
    }

    #[must_use]
    pub fn change_by(self, by: i64, total: Total) -> Self {
        let new_value = i64::from(self.0)
            .saturating_add(by)
            .clamp(0, i64::from(total.0));

        Self(new_value.try_into().unwrap_or(total.0))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Progress {
    total: Total,
    completed: Completed,
}

impl Progress {
    #[must_use]
    pub const fn new(total: Total, completed: Completed) -> Self {
        Self { total, completed }
    }

    #[must_use]
    pub fn zero() -> Self {
        Self::from_values(0, 0)
    }

    #[must_use]
    pub fn from_values(total: u32, completed: u32) -> Self {
        let total = Total::new(total);
        let completed = Completed::new(completed, total);
        Self::new(total, completed)
    }

    pub fn from_many<'a>(children: impl Iterator<Item = &'a Self>) -> Self {
        let mut num: u128 = 0;
        let mut den: u128 = 1;
        let mut count: u128 = 0;

        for child in children {
            count += 1;

            if child.total.0 == 0 {
                continue;
            }

            let c_num = u128::from(child.completed.0);
            let c_den = u128::from(child.total.0);

            let new_num = num * c_den + c_num * den;
            let new_den = den * c_den;

            let g = gcd(new_num, new_den);
            num = new_num / g;
            den = new_den / g;
        }

        if count == 0 {
            return Self::default();
        }

        let final_den = den * count;
        let g = gcd(num, final_den);

        Self::from_values((final_den / g) as u32, (num / g) as u32)
    }

    #[must_use]
    pub const fn completed(&self) -> u32 {
        self.completed.value()
    }

    #[must_use]
    pub const fn total(&self) -> u32 {
        self.total.value()
    }

    #[must_use]
    pub fn procentage(&self) -> f32 {
        if self.total.value().eq(&0) {
            return 0.0;
        }

        (self.completed.value() as f32 * 100.0) / (self.total.value() as f32)
    }
}

impl Default for Progress {
    fn default() -> Self {
        Self::from_values(10, 0)
    }
}

impl std::fmt::Display for Progress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pct = self.procentage();
        if pct.fract() == 0.0 {
            write!(f, "{pct:.0}")
        } else {
            write!(f, "{pct:.2}")
        }
    }
}

const fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
