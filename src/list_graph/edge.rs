pub struct Edge<W> {
    pub weight: W,
    pub next: usize,
}

impl<W> Edge<W> {
    pub fn new(weight: W, next: usize) -> Self {
        Self { weight, next }
    }
}
