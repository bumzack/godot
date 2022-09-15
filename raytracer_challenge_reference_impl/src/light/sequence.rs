use rand::{thread_rng, Rng};

#[derive(Clone, Debug)]
pub struct Sequence {
    data: Vec<f64>,
    idx: usize,
}

impl Sequence {
    pub fn new(data: Vec<f64>) -> Sequence {
        Sequence { data, idx: 0 }
    }

    pub(crate) fn next(&mut self) -> f64 {
        if self.data.len() > 0 {
            let elem = self.data[self.idx % self.data.len()];
            self.idx += 1;
            elem
        } else {
            let mut rng = thread_rng();
            rng.gen::<f64>()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::light::Sequence;

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
