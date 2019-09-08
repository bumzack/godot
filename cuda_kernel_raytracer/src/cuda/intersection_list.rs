// TODO: intersectionList is identical for CPU and CUDA, but the Intersection it uses is different
// how can this be merged into 1 file?
// use #!cfg target = ... or something ?
//

use core::fmt;

use crate::cuda::intersection::Intersection;
use crate::cuda::intersection::IntersectionOps;
use raytracer_lib_no_std::ShapeIdx;

pub const MAX_INTERSECTIONLIST_LEN: usize = 100;

type IntersectionContainer = [Intersection; MAX_INTERSECTIONLIST_LEN];

#[derive(Clone)]
pub struct IntersectionList {
    list_of_intersections: [Intersection; MAX_INTERSECTIONLIST_LEN],
    // idx: usize,
    len: usize,
    capacity: usize,
}

pub trait IntersectionListOps {
    fn new() -> IntersectionList;
    fn push(&mut self, i: Intersection);

    fn hit(&self) -> (&Intersection, bool);

    fn get_intersections(&self) -> &IntersectionContainer;
    fn get_intersections_mut(&mut self) -> &mut IntersectionContainer;

    fn sort_intersections(&mut self);

    fn len(&self) -> usize;

    // TODO this is always a  copy, dont know how to borrow ?!
    fn at(&self, idx: usize) -> &Intersection;

    //    fn is_empty(&self) -> bool;
    //
    //    fn last(&self) -> &Intersection;
    //
    //    fn contains(&self, shape_idx: ShapeIdx) -> bool;
    //
    //    fn get_position(&self, shape_idx: ShapeIdx) -> usize;
    //
    //    fn remove(&mut self, elem_idx: usize);
}

impl IntersectionListOps for IntersectionList {
    fn new() -> IntersectionList {
        IntersectionList {
            list_of_intersections: [Intersection::new_empty(); MAX_INTERSECTIONLIST_LEN],
            //    idx: 0,
            capacity: MAX_INTERSECTIONLIST_LEN,
            len: 0,
        }
    }

    fn push(&mut self, i: Intersection) {
        if !(self.len < self.capacity) {
            panic!("IntersectionListOps::push  array is full. try increasing MAX_INTERSECTIONLIST_LEN");
        }
        self.list_of_intersections[self.len] = i;
        self.len += 1;
        self.sort_intersections();
    }

    fn hit(&self) -> (&Intersection, bool) {
        let mut found = false;
        let mut idx = 0;
        for i in 0..self.len {
            if self.list_of_intersections[i].get_t() >= 0.0 {
                found = true;
                idx = i;
                break;
            }
        }
        (&self.list_of_intersections[idx], found)
    }

    fn get_intersections(&self) -> &IntersectionContainer {
        &self.list_of_intersections
    }

    fn get_intersections_mut(&mut self) -> &mut IntersectionContainer {
        &mut self.list_of_intersections
    }

    fn sort_intersections(&mut self) {
        // there you go BubbleSort :-)
        for n in (1..self.len).rev() {
            for i in 0..n - 1 {
                if self.list_of_intersections[i].get_t() > self.list_of_intersections[i + 1].get_t() {
                    let tmp = self.list_of_intersections[i];
                    self.list_of_intersections[i] = self.list_of_intersections[i + 1];
                    self.list_of_intersections[i + 1] = tmp;
                }
            }
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    fn at(&self, idx: usize) -> &Intersection {
        if !(idx < self.len) {
            panic!("IntersectionListOps::at  idx is out of range . try increasing MAX_INTERSECTIONLIST_LEN");
        }
        &self.list_of_intersections[idx]
    }

    //    fn is_empty(&self) -> bool {
    //        self.len == 0
    //    }
    //
    //    fn last(&self) -> &Intersection {
    //        if self.len <= 0 {
    //            panic!("IntersectionListOps::last  idx is out of range . no elements in list ");
    //        }
    //        &self.list_of_intersections[self.len - 1]
    //    }
    //
    //    fn contains(&self, shape_idx: usize) -> bool {
    //        for i in 0..self.len {
    //            if self.list_of_intersections[i].get_shape() == shape_idx {
    //                return true;
    //            }
    //        }
    //        false
    //    }
    //
    //    fn get_position(&self, shape_idx: usize) -> usize {
    //        for i in 0..self.len {
    //            if self.list_of_intersections[i].get_shape() == shape_idx {
    //                return i;
    //            }
    //        }
    //        panic!("IntersectionListOps::get_position  idx  not found in array   shape_idx = {}", shape_idx);
    //    }
    //
    //    fn remove(&mut self, elem_idx: usize) {
    //        for i in elem_idx..self.len - 1 {
    //            self.list_of_intersections[i] = self.list_of_intersections[i + 1];
    //        }
    //        self.len -= 1;
    //    }
}

impl fmt::Debug for IntersectionList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in self.list_of_intersections.iter().take(10) {
            writeln!(f, "isl  {:?}", i)?;
        }
        writeln!(f, "")
    }
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn i_am_no_test_just_help_compile() {
//        let xs = IntersectionList::new();
//        assert_eq!(i.get_t(), 3.5);
//    }
//}
