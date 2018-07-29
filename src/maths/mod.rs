use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

mod vector;
mod matrix;

//Vector types

pub type Vector2f = vector::Vector2<f32>;

pub type Vector3f = vector::Vector3<f32>;

pub type Vector2i = vector::Vector2<i32>;

//pub type Vector3i = vector::Vector3<i32>;

pub type Vector2u = vector::Vector2<u32>;

//pub type Vector3u = vector::Vector3<u32>;

pub type Matrix4f = matrix::Matrix4;


//Num

///Defines a number that can be a member of a vector.
pub trait Num<Rhs = Self, Output = Self>: Copy +
Add<Rhs, Output=Output> + AddAssign<Rhs> +
Sub<Rhs, Output=Output> + SubAssign<Rhs> +
Mul<Rhs, Output=Output> + MulAssign<Rhs> +
Div<Rhs, Output=Output> + DivAssign<Rhs> {}

//Automatically implement Num for all valid structs
impl<T> Num for T where T: Copy +
Add<Self, Output=Self> + AddAssign<Self> +
Sub<Self, Output=Self> + SubAssign<Self> +
Mul<Self, Output=Self> + MulAssign<Self> +
Div<Self, Output=Self> + DivAssign<Self> {}
