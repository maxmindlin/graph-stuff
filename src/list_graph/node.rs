use super::edge::Edge;

pub struct Node<T, W> {
    pub data: T,
    pub edges: Vec<Edge<W>>,
}

impl<T, W> Node<T, W> {
    pub fn new(data: T, edges: Vec<Edge<W>>) -> Self {
        Self { data, edges }
    }
}
