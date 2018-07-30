use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

///A column-major 4x4 float matrix used for OpenGL calculations.
///Elements can be accessed from a tuple (column, row) or index (0..16)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Matrix4 {
    array: [f32; 16]
}

impl Matrix4 {
    ///Initializes a zero-filled matrix.
    pub fn zero() -> Self {
        Matrix4::from_col([0.0; 16])
    }

    ///Initializes an identity matrix.
    pub fn identity() -> Self {
        let mut mat = Self::zero();

        for i in 0..4 {
            mat[(i, i)] = 1.0;
        }

        mat
    }

    ///Initializes matrix from row-major array
    pub fn new_row(mut arr: [f32; 16]) -> Self {
        //Cells that need to be swapped to go from row to col
        for i in [1, 2, 3, 6, 7, 11].iter() {
            //Get target cell in converted matrix
            let j = (4 * i) % 16 + i / 4;
            arr.swap(*i, j);
        }

        Matrix4 { array: arr }
    }

    ///Initializes matrix from column-major array
    pub fn from_col(arr: [f32; 16]) -> Self {
        Matrix4 { array: arr }
    }

    ///Get reference to the y-th row and x-th column (0-indexed)
    fn get(&self, y: usize, x: usize) -> &f32 {
        &self.array[4 * (x % 4) + (y % 4)]
    }

    ///Get mutable reference to the y-th row and x-th column (0-indexed)
    fn get_mut(&mut self, x: usize, y: usize) -> &mut f32 {
        &mut self.array[4 * (x % 4) + (y % 4)]
    }

    ///Get pointer to the first value, to use with OpenGL.
    pub fn as_ptr(&self) -> *const f32 {
        self.get(0, 0) as *const f32
    }
}

//Access elements by tuple index (row, column)

impl Index<(usize, usize)> for Matrix4 {
    type Output = f32;

    fn index(&self, index: (usize, usize)) -> &f32 {
        self.get(index.0, index.1)
    }
}

impl IndexMut<(usize, usize)> for Matrix4 {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut f32 {
        self.get_mut(index.0, index.1)
    }
}

//Access elements by index in array (0-15)

impl Index<usize> for Matrix4 {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        &self.array[index % 16]
    }
}

impl IndexMut<usize> for Matrix4 {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        &mut self.array[index % 16]
    }
}

//Matrix Addition

impl Add for Matrix4 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut arr = [0.0; 16];

        for i in 0..16 {
            arr[i] = self[i] + other[i];
        }

        Matrix4::from_col(arr)
    }
}

impl AddAssign for Matrix4 {
    fn add_assign(&mut self, other: Self) {
        for i in 0..16 {
            self[i] += other[i]
        }
    }
}

//Matrix Substraction

impl Sub for Matrix4 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut arr = [0.0; 16];

        for i in 0..16 {
            arr[i] = self[i] - other[i];
        }

        Matrix4::from_col(arr)
    }
}

impl SubAssign for Matrix4 {
    fn sub_assign(&mut self, other: Self) {
        for i in 0..16 {
            self[i] -= other[i]
        }
    }
}

//Scalar multiplication

impl Mul<Matrix4> for f32 {
    type Output = Matrix4;

    fn mul(self, other: Matrix4) -> Matrix4 {
        other * self
    }
}

impl Mul<f32> for Matrix4 {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        let mut arr = [0.0; 16];

        for i in 0..16 {
            arr[i] = self[i] * other;
        }

        Matrix4::from_col(arr)
    }
}

impl MulAssign<f32> for Matrix4 {
    fn mul_assign(&mut self, other: f32) {
        for i in 0..16 {
            self[i] *= other
        }
    }
}

//Matrix multiplication

impl Mul for Matrix4 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut mat = Self::zero();

        for y in 0..4 {
            for x in 0..4 {
                for i in 0..4 {
                    mat[(y, x)] += self[(y, i)] * other[(i, x)]
                }
            }
        }

        mat
    }
}
