use num_traits::Num;

#[derive(Debug, Clone, Copy)]
pub struct Vertex3<T: Num + Copy> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Num + Copy> Vertex3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn add(&self, other: &Vertex3<T>) -> Vertex3<T> {
        Vertex3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn sub(&self, other: &Vertex3<T>) -> Vertex3<T> {
        Vertex3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn dot(&self, other: &Vertex3<T>) -> Vertex3<T> {
        Vertex3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }

    pub fn cross(&self, other: &Vertex3<T>) -> Vertex3<T> {
        Vertex3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

pub struct Vertex4<T: Num + Copy> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T: Num + Copy> Vertex4<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    pub fn add(&self, other: &Vertex4<T>) -> Vertex4<T> {
        Vertex4 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }

    pub fn sub(&self, other: &Vertex4<T>) -> Vertex4<T> {
        Vertex4 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }

    pub fn dot(&self, other: &Vertex4<T>) -> Vertex4<T> {
        Vertex4 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w,
        }
    }
}
