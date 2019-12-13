#[derive(Debug)]
pub struct IndexedModel {
    vertex_index: usize,
    tex_coord_index: usize,
    normal_index: usize,
}

pub trait IndexedModelOps {
    fn new() -> IndexedModel;
}

impl IndexedModelOps for IndexedModel {
    fn new() -> IndexedModel {
        let o = IndexedModel {
            vertex_index: 0,
            tex_coord_index: 0,
            normal_index: 0
        };
        o
    }
}
