use crate::cyclone_math::REAL_EPISLON;

#[derive(PartialEq)]
pub struct Quaternion {
    r: f64,
    i: f64,
    j: f64,
    k: f64,
}

impl Quaternion {
    pub fn new(r: f64, i: f64, j: f64, k: f64) -> Self {
        Quaternion { r, i, j, k }
    }

    pub fn normalise(&mut self) {
        let mut d = self.r.powi(2) + self.i.powi(2) + self.j.powi(2) + self.k.powi(2);

        if d < REAL_EPISLON {
            self.r = 1.0;
            return;
        }

        d = 1.0 / d.sqrt();
        self.r *= d;
        self.i *= d;
        self.j *= d;
        self.k *= d;
    }
}

impl Default for Quaternion {
    fn default() -> Self {
        Quaternion {
            r: 1.0,
            i: 0.0,
            j: 0.0,
            k: 0.0,
        }
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
