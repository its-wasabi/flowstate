#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Total(u32);

impl Total {
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    pub fn value(self) -> u32 {
        self.0
    }

    pub fn change_by(self, by: i64) -> Self {
        Self((self.value() as i64).saturating_add(by).max(0) as u32)
    }
}

impl Default for Total {
    fn default() -> Self {
        Self(10)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Completed(u32);

impl Completed {
    pub fn new(value: u32, total: Total) -> Self {
        Self(value.min(total.0))
    }

    pub fn value(self) -> u32 {
        self.0
    }

    pub fn change_by(self, by: i64, total: Total) -> Self {
        Self(
            (self.value() as i64)
                .saturating_add(by)
                .clamp(0, total.0 as i64) as u32,
        )
    }
}

impl Default for Completed {
    fn default() -> Self {
        Self(0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Progress {
    total: Total,
    completed: Completed,
}

impl Progress {
    pub fn new(total: Total, completed: Completed) -> Self {
        Self { total, completed }
    }

    pub fn zero() -> Self {
        Self::from_values(0, 0)
    }

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

            let c_num = child.completed.0 as u128;
            let c_den = child.total.0 as u128;

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

    pub fn completed(&self) -> u32 {
        self.completed.value()
    }

    pub fn total(&self) -> u32 {
        self.total.value()
    }

    pub fn procentage(&self) -> f32 {
        if self.total.value().eq(&0) {
            return 0.0;
        }

        ((self.completed.value() as f64 * 100.0) / self.total.value() as f64) as f32
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
            write!(f, "{:.0}", pct)
        } else {
            write!(f, "{:.2}", pct)
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
