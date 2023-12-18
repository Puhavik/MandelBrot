use std::ops::{Add, Mul};

// Define the Complex struct
#[derive(Clone, Copy, Debug)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

// Implement the Add trait for Complex numbers
impl Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Complex {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

// Implement the Mul trait for Complex numbers
impl Mul for Complex {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Complex {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
}

// Implement a method to calculate the magnitude of the complex number
impl Complex {
    pub fn mag(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }
}