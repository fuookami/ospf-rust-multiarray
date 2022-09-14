use super::{MultiArray, Shape};

pub struct MultiArrayView<'a, T: Sized, S: Shape> {
    pub(self) parent: &'a MultiArray<T, S>,
    pub(self) list: Vec<Vec<Option<&'a Box<T>>>>,
    pub(self) shape: S,
}
