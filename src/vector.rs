#[derive(Debug)]
pub struct Vector {
    values: Vec<f64>,
}

impl Vector {
    pub fn new(lst: Vec<f64>) -> Vector {
        Vector { values: lst }
    }

    pub fn values(&self) -> &Vec<f64> {
        &self.values
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.len() == 0
    }
}

use std::ops::{Add, Mul, Neg, Sub};

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        assert_eq!(
            self.len(),
            other.len(),
            "Error in adding. Length of vectors must be same."
        );
        let values = self
            .values
            .iter()
            .zip(other.values.iter())
            .map(|(a, b)| a + b)
            .collect();
        Vector::new(values)
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        assert_eq!(
            self.len(),
            other.len(),
            "Error in subtracting. Length of vectors must be same."
        );
        let values = self
            .values
            .iter()
            .zip(other.values.iter())
            .map(|(a, b)| a - b)
            .collect();
        Vector::new(values)
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        let values = self.values.iter().map(|e| e * scalar).collect();
        Vector::new(values)
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, vector: Vector) -> Vector {
        vector * self
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

use std::fmt;
impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({})",
            self.values
                .iter()
                .map(|e| format!("{:.1}", e))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl IntoIterator for Vector {
    type Item = f64;
    type IntoIter = std::vec::IntoIter<f64>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

impl std::ops::Index<usize> for Vector {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}
