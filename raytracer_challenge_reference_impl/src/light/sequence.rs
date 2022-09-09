#[derive(Clone, Debug)]
pub struct Sequence {
    data: Vec<f32>,
    idx: usize,
}

impl Sequence {
    pub fn new(data: Vec<f32>) -> Sequence {
        Sequence { data, idx: 0 }
    }

    pub(crate) fn next(&mut self) -> f32 {
        let elem = self.data[self.idx % self.data.len()];
        self.idx += 1;
        elem
    }

    fn clear(&mut self) {
        self.data.clear();
    }
}

#[cfg(test)]
mod tests {
    use crate::light::Sequence;
    use crate::math::tuple4d::Tuple;

    // bonus chapter:  A number generator returns a cyclic sequence of numbers
    #[test]
    fn test_sequence() {
        let mut sequence = Sequence::new(vec![0.1, 0.5, 1.0]);

        assert_eq!(0.1, sequence.next());
        assert_eq!(0.5, sequence.next());
        assert_eq!(1.0, sequence.next());
        assert_eq!(0.1, sequence.next());
    }
}
