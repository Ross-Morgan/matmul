use std::{fmt::{Display, Write}, ops::{Add, Mul}};

use num_traits::{Num, Zero};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Matrix<T: Num, const ROW: usize, const COL: usize>([[T; COL]; ROW]);

impl<T: Copy + Num> Matrix<T, 1, 1> {
    pub const fn determinant(&self) -> T {
        self.0[0][0]
    }
}

impl<T: Num, const R: usize, const C: usize> Matrix<T, R, C> {
    pub const fn new(value: [[T; C]; R]) -> Self {
        Self(value)
    }
}

impl<T: Copy + Num> Matrix<T, 2, 2> {
    pub fn determinant(&self) -> T {
        self.0[0][0] * self.0[1][1] - self.0[1][0] * self.0[0][1]
    }
}

impl<T: Copy + Num> Matrix<T, 3, 3> {
    pub fn determinant(&self) -> T {
        T::zero()
        + (Matrix([[self.0[1][1], self.0[1][2]], [self.0[2][1], self.0[2][2]]]).determinant() * self.0[0][0])
        - (Matrix([[self.0[1][0], self.0[1][2]], [self.0[2][0], self.0[2][2]]]).determinant() * self.0[0][1])
        + (Matrix([[self.0[1][0], self.0[1][1]], [self.0[2][0], self.0[2][1]]]).determinant() * self.0[0][2])
    }
}

impl<T: Display + Num + ToString + Copy, const R: usize, const C: usize> Display for Matrix<T, R, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[\n")?;
        
        for row in self.0.iter() {
            let s = row.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", ");
            f.write_str("    ")?;
            f.write_str(&s)?;
            f.write_char('\n')?;
        }

        f.write_str("]\n")?;

        Ok(())
    }
}

impl<T: Copy + Num, const R: usize, const RC: usize, const C: usize> Mul<Matrix<T, RC, C>> for Matrix<T, R, RC> {
    type Output = Matrix<T, R, C>;
    
    fn mul(self, rhs: Matrix<T, RC, C>) -> Self::Output {
        let mut out = Matrix::zero();

        for i in 0..R {
            for j in 0..C {
                let mut sum = T::zero();

                for k in 0..RC {
                    sum = sum + (self.0[i][k] + rhs.0[k][j]);
                }

                out.0[i][j] = sum;
            }
        }

        out
    }
} 

impl<T: Copy + Num, const R: usize, const C: usize> Add<T> for Matrix<T, R, C> {
    type Output = Matrix<T, R, C>;
    
    fn add(mut self, rhs: T) -> Self::Output {
        self.0
            .iter_mut()
            .for_each(|row| {
                row
                    .iter_mut()
                    .for_each(|e| *e = *e * rhs);
            });

        self
    }
}

impl<T: Copy + Num, const R: usize, const C: usize> Add<Matrix<T, R, C>> for Matrix<T, R, C> {
    type Output = Matrix<T, R, C>;

    fn add(self, rhs: Matrix<T, R, C>) -> Self::Output {
        let mut lhs = self.0;

        lhs
            .iter_mut()
            .enumerate()
            .for_each(|(row_idx, row)| {
                row.iter_mut()
                    .enumerate()
                    .for_each(|(col_idx, element)| {
                        *element = *element + rhs.0[row_idx][col_idx];
                    })
            });

        Matrix(lhs)
    }
}

impl<T: Copy + Num, const R: usize, const C: usize> Zero for Matrix<T, R, C> {
    fn zero() -> Self {
        Self([[T::zero(); C]; R])
    }

    fn is_zero(&self) -> bool {
        !self.0
            .iter()
            .any(|r| r.iter().any(|e| !e.is_zero()))
    }

    fn set_zero(&mut self) {
        self.0 = [[T::zero(); C]; R];
    }
}
