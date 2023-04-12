use std::cmp::Ordering;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div};
use crate::algorithm::glicko2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rating {
    pub value: f32,
    pub rd: f32,
    pub volatility: f32,
}

impl Eq for Rating {}

impl Ord for Rating {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.partial_cmp(&other.value).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for Rating {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Default for Rating {
    fn default() -> Self {
        Self {
            value: 2500.0,
            rd: 580.0,
            volatility: 0.06,
        }
    }
}

impl Add for Rating {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            value: self.value + other.value,
            rd: self.rd + other.rd,
            volatility: self.volatility + other.volatility,
        }
    }
}

impl AddAssign for Rating {
    fn add_assign(&mut self, other: Self) {
        self.value += other.value;
        self.rd += other.rd;
        self.volatility += other.volatility;
    }
}

impl Div<f32> for Rating {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            value: self.value / rhs,
            rd: self.rd / rhs,
            volatility: self.volatility / rhs,
        }
    }
}

impl Sum for Rating {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        let mut sum = Self::zero();
        for rating in iter {
            sum += rating;
        }
        sum
    }
}

impl Rating {
    pub fn new(value: f32, rd: f32, sigma: f32) -> Self {
        Self {
            value,
            rd,
            volatility: sigma,
        }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn new_no_sigma(value: f32, rd: f32) -> Self {
        Self::new(value, rd, 0.06)
    }

    pub fn update(&mut self, other: &Self, score: f32) {
        let (value, rd, sigma) = glicko2::update(self.value, self.rd, self.volatility, other.value, other.rd, 5.0/3.0, score);
        self.value = value;
        self.rd = rd;
        self.volatility = sigma;
    }
}