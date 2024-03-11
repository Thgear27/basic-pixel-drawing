use crate::vertex::{Vertex3, Vertex4};
use std::ops::Mul;

use lazy_static::lazy_static;

#[derive(Debug, Clone)]
pub struct Matrix {
    pub data: Vec<Vec<f32>>,
    pub rows: usize,
    pub cols: usize,
}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, other: Matrix) -> Matrix {
        self.multiply(&other)
    }
}

impl Mul<Vertex3<f32>> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Vertex3<f32>) -> Self::Output {
        let mut result = Matrix::new(4, 1);
        result.data[0][0] = rhs.x;
        result.data[1][0] = rhs.y;
        result.data[2][0] = rhs.z;
        result.data[3][0] = 1.0;

        self.multiply(&result)
    }
}

impl Mul<Vertex4<f32>> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Vertex4<f32>) -> Self::Output {
        let mut result = Matrix::new(4, 1);
        result.data[0][0] = rhs.x;
        result.data[1][0] = rhs.y;
        result.data[2][0] = rhs.z;
        result.data[3][0] = rhs.w;

        self.multiply(&result)
    }
}

impl Matrix {
    pub fn new(rows: i32, cols: i32) -> Matrix {
        let mut data: Vec<Vec<f32>> = Vec::new();

        for _ in 0..rows {
            let mut row: Vec<f32> = Vec::new();
            for _ in 0..cols {
                row.push(0.0);
            }
            data.push(row);
        }

        Matrix {
            data,
            rows: rows as usize,
            cols: cols as usize,
        }
    }

    pub fn identity(size: i32) -> Matrix {
        let mut data: Vec<Vec<f32>> = Vec::new();

        for i in 0..size {
            let mut row: Vec<f32> = Vec::new();
            for j in 0..size {
                if i == j {
                    row.push(1.0);
                } else {
                    row.push(0.0);
                }
            }
            data.push(row);
        }

        Matrix {
            data,
            rows: size as usize,
            cols: size as usize,
        }
    }

    pub fn rotation_x(angle: f32) -> Matrix {
        let mut result = Matrix::identity(4);
        result.data[1][1] = angle.cos();
        result.data[1][2] = -angle.sin();
        result.data[2][1] = angle.sin();
        result.data[2][2] = angle.cos();

        result
    }

    pub fn rotation_y(angle: f32) -> Matrix {
        let mut result = Matrix::identity(4);
        result.data[0][0] = angle.cos();
        result.data[0][2] = angle.sin();
        result.data[2][0] = -angle.sin();
        result.data[2][2] = angle.cos();

        result
    }

    pub fn rotation_z(angle: f32) -> Matrix {
        let mut result = Matrix::identity(4);
        result.data[0][0] = angle.cos();
        result.data[0][1] = -angle.sin();
        result.data[1][0] = angle.sin();
        result.data[1][1] = angle.cos();

        result
    }

    pub fn projection(aspect_ratio: f32, fov: f32, near: f32, far: f32) -> Matrix {
        let mut result = Matrix::identity(4);

        let fov_rad = 1.0 / (fov * 0.5 / 180.0 * std::f32::consts::PI).tan();
        result.data[0][0] = aspect_ratio * fov_rad;
        result.data[1][1] = fov_rad;
        result.data[2][2] = far / (far - near);
        result.data[3][2] = (-far * near) / (far - near);
        result.data[2][3] = 1.0;
        result.data[3][3] = 0.0;

        result
    }

    // If the projection matrix does not work, try using the following:
    pub fn z_division() -> Matrix {
        let mut result = Matrix::identity(4);
        result.data[3][2] = 1.0;

        result
    }

    pub fn translation(x: f32, y: f32, z: f32) -> Matrix {
        let mut result = Matrix::identity(4);
        result.data[0][3] = x;
        result.data[1][3] = y;
        result.data[2][3] = z;

        result
    }

    pub fn viewport(width: usize, height: usize) -> Matrix {
        let mut result: Matrix = Matrix::new(4, 4);
        result.data[0][0] = (width as f32) / 2.0;
        result.data[1][1] = (height as f32) / 2.0;
        result.data[2][2] = 1.0;
        result.data[3][3] = 1.0;
        result.data[0][3] = (width as f32) / 2.0;
        result.data[1][3] = (height as f32) / 2.0;

        result
    }

    pub fn multiply(&self, other: &Matrix) -> Matrix {
        let mut result = Matrix::new(self.rows as i32, other.cols as i32);

        for i in 0..self.rows {
            for j in 0..other.cols {
                for k in 0..self.cols {
                    result.data[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }

        result
    }
}

lazy_static! {
    pub static ref IDENTITY: Matrix = Matrix::identity(4);
}
