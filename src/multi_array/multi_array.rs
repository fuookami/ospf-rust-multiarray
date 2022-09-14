use super::{DynShape, MultiArrayView, Shape, Shape1, Shape2, Shape3, Shape4};
use std::ops;

pub struct MultiArray<T: Sized, S: Shape> {
    list: Vec<Option<Box<T>>>,
    shape: S,
}

impl<T: Sized, S: Shape> MultiArray<T, S> {
    fn new(shape: S) -> Self {
        Self {
            list: (0..shape.len()).map(|_| Option::None).collect(),
            shape: shape,
        }
    }

    fn new_with(shape: S, value: T) -> Self
    where
        T: Clone,
    {
        Self {
            list: (0..shape.len())
                .map(|_| Option::Some(Box::new(value.clone())))
                .collect(),
            shape: shape,
        }
    }

    fn new_by(shape: S, generator: &dyn Fn(usize) -> T) -> Self {
        Self {
            list: (0..shape.len())
                .map(|index| Option::Some(Box::new(generator(index))))
                .collect(),
            shape: shape,
        }
    }

    fn get<'a>(&'a self, vector: &S::DummyVectorType) -> Vec<Option<&'a Box<T>>> {}
    fn map<'a>(&'a self, vector: &S::MapVectorType) -> MultiArrayView<'a, T, DynShape> {}
}

impl<T: Sized, S: Shape> ops::Index<usize> for MultiArray<T, S> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match &self.list[index] {
            Option::Some(value) => value,
            Option::None => {
                panic!(
                    "Element with index {} in the multi-array is not initialized",
                    index
                )
            }
        }
    }
}

impl<T: Sized, S: Shape> ops::IndexMut<usize> for MultiArray<T, S> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match &mut self.list[index] {
            Option::Some(value) => value,
            Option::None => {
                panic!(
                    "Element with index {} in the multi-array is not initialized",
                    index
                )
            }
        }
    }
}

impl<T: Sized, S: Shape> ops::Index<&S::VectorType> for MultiArray<T, S> {
    type Output = T;

    fn index(&self, vector: &S::VectorType) -> &Self::Output {
        match self.shape.index(vector) {
            Result::Ok(index) => match &self.list[index] {
                Option::Some(value) => value,
                Option::None => {
                    panic!(
                        "Element with index {} in the multi-array is not initialized",
                        index
                    )
                }
            },
            Result::Err(err) => panic!("{}", err),
        }
    }
}

impl<T: Sized, S: Shape> ops::IndexMut<&S::VectorType> for MultiArray<T, S> {
    fn index_mut(&mut self, vector: &S::VectorType) -> &mut Self::Output {
        match self.shape.index(vector) {
            Result::Ok(index) => match &mut self.list[index] {
                Option::Some(value) => value,
                Option::None => {
                    panic!(
                        "Element with index {} in the multi-array is not initialized",
                        index
                    )
                }
            },
            Result::Err(err) => panic!("{}", err),
        }
    }
}

type MultiArray1<T> = MultiArray<T, Shape1>;
type MultiArray2<T> = MultiArray<T, Shape2>;
type MultiArray3<T> = MultiArray<T, Shape3>;
type MultiArray4<T> = MultiArray<T, Shape4>;
type DynMultiArray<T> = MultiArray<T, DynShape>;
