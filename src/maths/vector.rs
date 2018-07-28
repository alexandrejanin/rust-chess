use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub, SubAssign};
use super::Num;

//=========
// Vector2
//=========
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Vector2<T: Num> {
    pub x: T,
    pub y: T,
}

impl<T: Num> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn as_ptr(&self) -> *const T {
        &self.x as *const T
    }
}

//Conversion from tuple
impl<T: Num> From<(T, T)> for Vector2<T> {
    fn from(tup: (T, T)) -> Self {
        Self { x: tup.0, y: tup.1 }
    }
}

//Access members by index (0-1)
impl<T: Num> Index<usize> for Vector2<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Vector index out of range: {}", index)
        }
    }
}

//Add norm for float vectors

impl Vector2<f32> {
    pub fn norm(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl Vector2<f64> {
    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

//Operator implementations

impl<T: Num> Add for Vector2<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Num> AddAssign for Vector2<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T: Num> Sub for Vector2<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: Num> SubAssign for Vector2<T> {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl<T: Num> Mul<T> for Vector2<T> {
    type Output = Self;

    fn mul(self, other: T) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl<T: Num> MulAssign<T> for Vector2<T> {
    fn mul_assign(&mut self, other: T) {
        self.x *= other;
        self.y *= other;
    }
}

impl<T: Num> Div<T> for Vector2<T> {
    type Output = Self;

    fn div(self, other: T) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl<T: Num> DivAssign<T> for Vector2<T> {
    fn div_assign(&mut self, other: T) {
        self.x /= other;
        self.y /= other;
    }
}

//If scalar can be negative, add it to vector as well

impl<T: Num + Neg<Output=T>> Neg for Vector2<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}


//=========
// Vector3
//=========
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Vector3<T: Num> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Num> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn as_ptr(&self) -> *const T {
        &self.x as *const T
    }
}

//Conversion from tuple
impl<T: Num> From<(T, T, T)> for Vector3<T> {
    fn from(tup: (T, T, T)) -> Self {
        Self { x: tup.0, y: tup.1, z: tup.2 }
    }
}

//Access members by index (0-2)
impl<T: Num> Index<usize> for Vector3<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Vector index out of range: {}", index)
        }
    }
}

//Add norm for float vectors

impl Vector3<f32> {
    pub fn norm(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

impl Vector3<f64> {
    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

//Operator implementations

impl<T: Num> Add for Vector3<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Num> AddAssign for Vector3<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<T: Num> Sub for Vector3<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T: Num> SubAssign for Vector3<T> {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<T: Num> Mul<T> for Vector3<T> {
    type Output = Self;

    fn mul(self, other: T) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl<T: Num> MulAssign<T> for Vector3<T> {
    fn mul_assign(&mut self, other: T) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl<T: Num> Div<T> for Vector3<T> {
    type Output = Self;

    fn div(self, other: T) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl<T: Num> DivAssign<T> for Vector3<T> {
    fn div_assign(&mut self, other: T) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

//If scalar can be negative, add it to vector as well

impl<T: Num + Neg<Output=T>> Neg for Vector3<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
