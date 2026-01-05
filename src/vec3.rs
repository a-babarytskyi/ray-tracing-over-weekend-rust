use core::fmt;
use rand::random_range;
use std::ops;
use std::simd::{f64x4, num::SimdFloat, u8x4};

pub struct Vec3 {
    e: f64x4,
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

impl Vec3 {
    pub fn clamp(&mut self, min: f64, max: f64) -> Self {
        self.e = self.e.simd_clamp(f64x4::splat(min), f64x4::splat(max));
        *self
    }

    pub fn random() -> Vec3 {
        return Vec3::from_values(
            random_range(0.0..1.0),
            random_range(0.0..1.0),
            random_range(0.0..1.0),
        );
    }

    pub fn random_ranged(min: f64, max: f64) -> Vec3 {
        return Vec3::from_values(
            random_range(min..max),
            random_range(min..max),
            random_range(min..max),
        );
    }

    pub fn zero() -> Vec3 {
        Vec3 {
            e: f64x4::splat(0.),
        }
    }

    pub fn rounded(&self) -> u8x4 {
        let res: u8x4 = self.e.cast();
        res
    }

    pub fn from_values(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 {
            e: f64x4::from_array([e0, e1, e2, 0.]),
        }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length_squared(&self) -> f64 {
        let res = (self.e * self.e).reduce_sum();
        res
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(vec: &Self) -> Vec3 {
        *vec / vec.length()
    }

    pub fn random_unit_vector() -> Vec3 {
        loop {
            let p = Self::random_ranged(-1.0, 1.0);
            let lensq = p.length_squared();
            if 1e-160 < lensq && lensq <= 1.0 {
                return p / lensq.sqrt();
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Self::random_unit_vector();
        if Self::dot(&on_unit_sphere, normal) > 0.0 {
            // In the same hemisphere as the normal
            return on_unit_sphere;
        } else {
            return -on_unit_sphere;
        }
    }

    pub fn dot(vec1: &Self, vec2: &Self) -> f64 {
        return (vec1.e * vec2.e).reduce_sum();
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        let rhs_s = f64x4::splat(rhs);
        self.e *= rhs_s;
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e += rhs.e;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 {
            e: self.e * f64x4::splat(-1.),
        }
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        return Vec3 { e: self.e + rhs.e };
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 { e: self.e - rhs.e }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            e: self.e * f64x4::splat(rhs),
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl Clone for Vec3 {
    fn clone(&self) -> Self {
        Vec3 { e: self.e }
    }
}

impl Copy for Vec3 {}
