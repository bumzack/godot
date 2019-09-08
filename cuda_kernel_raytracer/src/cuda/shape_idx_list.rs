use core::fmt;

use crate::cuda::intersection::Intersection;
use crate::cuda::intersection::IntersectionOps;
use crate::cuda::intersection_list::MAX_INTERSECTIONLIST_LEN;
use raytracer_lib_no_std::ShapeIdx;

pub const MAX_SHAPE_IDX_LIST_LEN: usize = MAX_INTERSECTIONLIST_LEN;

type ShapeIdxContainer = [ShapeIdx; MAX_SHAPE_IDX_LIST_LEN];

#[derive(Clone)]
pub struct ShapeIdxList {
    list_of_shape_idx: ShapeIdxContainer,
    len: usize,
    capacity: usize,
}

pub trait ShapeIdxListOps {
    fn new() -> ShapeIdxList;
    fn push(&mut self, i: ShapeIdx);
    fn len(&self) -> usize;
    fn at(&self, idx: usize) -> ShapeIdx;
    fn is_empty(&self) -> bool;
    fn last(&self) -> ShapeIdx;
    fn contains(&self, shape_idx: ShapeIdx) -> bool;
    fn get_position(&self, shape_idx: ShapeIdx) -> usize;
    fn remove(&mut self, elem_idx: usize);
}

impl ShapeIdxListOps for ShapeIdxList {
    fn new() -> ShapeIdxList {
        ShapeIdxList {
            list_of_shape_idx: [0; MAX_SHAPE_IDX_LIST_LEN],
            capacity: MAX_SHAPE_IDX_LIST_LEN,
            len: 0,
        }
    }

    fn push(&mut self, i: ShapeIdx) {
        if !(self.len < self.capacity) {
            panic!("ShapeIdxListOps::push  array is full. try increasing MAX_SHAPE_IDX_LIST_LEN");
        }
        self.list_of_shape_idx[self.len] = i;
        self.len += 1;
    }

    fn len(&self) -> usize {
        self.len
    }

    fn at(&self, idx: usize) -> ShapeIdx {
        if !(idx < self.len) {
            panic!("ShapeIdxListOps::at  idx is out of range . try increasing MAX_SHAPE_IDX_LIST_LEN");
        }
        self.list_of_shape_idx[idx]
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn last(&self) -> ShapeIdx {
        if self.len <= 0 {
            panic!("ShapeIdxListOps::last  idx is out of range . no elements in list ");
        }
        self.list_of_shape_idx[self.len - 1]
    }

    fn contains(&self, shape_idx: usize) -> bool {
        for i in 0..self.len {
            if self.list_of_shape_idx[i] == shape_idx {
                return true;
            }
        }
        false
    }

    fn get_position(&self, shape_idx: usize) -> usize {
        for i in 0..self.len {
            if self.list_of_shape_idx[i] == shape_idx {
                return i;
            }
        }
        panic!(
            "ShapeIdxListOps::get_position  idx  not found in array   shape_idx = {}",
            shape_idx
        );
    }

    fn remove(&mut self, elem_idx: usize) {
        for i in elem_idx..self.len - 1 {
            self.list_of_shape_idx[i] = self.list_of_shape_idx[i + 1];
        }
        self.len -= 1;
    }
}

impl fmt::Debug for ShapeIdxList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in self.list_of_shape_idx.iter().take(10) {
            writeln!(f, "shape_idx  {:?}", i)?;
        }
        writeln!(f, "")
    }
}
