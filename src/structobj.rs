use std::ops::Add;

#[derive(Debug)]
pub(crate) struct Complex {
    real: f64,
    imagine: f64,
}

impl Complex {
    pub fn new (real: f64, imagine: f64) -> Self {
        Self {
            real, imagine
        }
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let real = self.real + rhs.real;
        let imagine =self.imagine + rhs.imagine;
        Self::new(real,imagine)
    }
}