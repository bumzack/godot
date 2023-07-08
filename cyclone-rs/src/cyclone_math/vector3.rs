use std::ops::{Add, AddAssign, BitXor, Div, DivAssign, Mul, MulAssign, Not, Sub, SubAssign};

#[derive(PartialEq)]
pub struct CycloneVector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl CycloneVector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        CycloneVector3 { x, y, z }
    }

    pub fn scalar_product(&self, rhs: &CycloneVector3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn add_scaled_vector(&mut self, vector: &CycloneVector3, scale: f64) {
        self.x += vector.x * scale;
        self.y += vector.y * scale;
        self.z += vector.z * scale;
    }

    pub fn magnitude(&self) -> f64 {
        self.square_magnitude().sqrt()
    }

    pub fn square_magnitude(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn trim(&mut self, size: f64) {
        if self.square_magnitude() > size.powi(2) {
            self.normalise();
            self.x += size;
            self.y += size;
            self.z += size;
        }
    }

    pub fn normalise(&mut self) {
        let l = self.magnitude();
        if l > 0.0 {
            self.x *= 1.0 / l;
            self.y *= 1.0 / l;
            self.z *= 1.0 / l;
        }
    }

    pub fn unit(&self) -> CycloneVector3 {
        let mut x = CycloneVector3::new(self.x, self.y, self.z);
        x.normalise();
        x
    }

    pub fn smaller(&self, other: &CycloneVector3) -> bool {
        self.x < other.x && self.y < other.y && self.z < other.z
    }

    pub fn smaller_or_equal(&self, other: &CycloneVector3) -> bool {
        self.x <= other.x && self.y <= other.y && self.z <= other.z
    }

    pub fn bigger(&self, other: &CycloneVector3) -> bool {
        self.x > other.x && self.y > other.y && self.z > other.z
    }

    pub fn bigger_or_equal(&self, other: &CycloneVector3) -> bool {
        self.x >= other.x && self.y >= other.y && self.z >= other.z
    }

    pub fn zero(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
        self.z = 0.0;
    }

    pub fn invert(&mut self) {
        self.x *= -1.0;
        self.y *= -1.0;
        self.z *= -1.0;
    }
}

impl Default for CycloneVector3 {
    fn default() -> Self {
        CycloneVector3 { x: 0.0, y: 0.0, z: 0.0 }
    }
}

impl Add for CycloneVector3 {
    type Output = CycloneVector3;

    fn add(self, rhs: Self) -> Self::Output {
        CycloneVector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for CycloneVector3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for &CycloneVector3 {
    type Output = CycloneVector3;

    fn sub(self, rhs: Self) -> Self::Output {
        CycloneVector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign for CycloneVector3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<f64> for CycloneVector3 {
    type Output = CycloneVector3;

    fn mul(self, rhs: f64) -> Self::Output {
        CycloneVector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl MulAssign<f64> for CycloneVector3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

//         Vector3 componentProduct(const Vector3 &vector) const {
impl Mul for CycloneVector3 {
    type Output = CycloneVector3;
    fn mul(self, rhs: Self) -> Self::Output {
        CycloneVector3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

//         void componentProductUpdate(const Vector3 &vector) {
impl MulAssign for CycloneVector3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl Div for CycloneVector3 {
    type Output = CycloneVector3;

    fn div(self, rhs: Self) -> Self::Output {
        CycloneVector3::new(
            self.y * rhs.z - rhs.z * self.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}

impl DivAssign for CycloneVector3 {
    fn div_assign(&mut self, rhs: Self) {
        let x = self.y * rhs.z - rhs.z * self.y;
        let y = self.z * rhs.x - self.x * rhs.z;
        let z = self.x * rhs.y - self.y * rhs.x;
        self.x = x;
        self.y = y;
        self.z = z;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(true);
    }
}
