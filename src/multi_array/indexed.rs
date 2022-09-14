pub trait Indexed {
    fn index(&self) -> usize;
}

impl Into<usize> for &dyn Indexed {
    fn into(self) -> usize {
        self.index()
    }
}
