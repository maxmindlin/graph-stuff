use std::ops::{Index, IndexMut};

#[derive(Default, Debug, PartialEq, Eq)]
pub struct TransitiveClosureMtx(Vec<Vec<bool>>);

impl Index<usize> for TransitiveClosureMtx {
    type Output = Vec<bool>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for TransitiveClosureMtx {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl From<Vec<Vec<bool>>> for TransitiveClosureMtx {
    fn from(v: Vec<Vec<bool>>) -> Self {
        Self(v)
    }
}
