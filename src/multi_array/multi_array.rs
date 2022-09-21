use crate::{dummy_vector::DummyIndex, OutOfShapeError};

use super::{DynShape, MultiArrayView, Shape, Shape1, Shape2, Shape3, Shape4};
use meta_programming::{indexed, next_index, Indexed};
use std::ops::{Deref, Index, IndexMut};

pub struct MultiArray<T: Sized, S: Shape> {
    list: Vec<Option<T>>,
    shape: S,
}

impl<T: Sized, S: Shape> MultiArray<T, S> {
    pub fn new(shape: S) -> Self {
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
                .map(|_| Option::Some(value.clone()))
                .collect(),
            shape: shape,
        }
    }

    fn new_by(shape: S, generator: &dyn Fn(usize) -> T) -> Self {
        Self {
            list: (0..shape.len())
                .map(|index| Option::Some(generator(index)))
                .collect(),
            shape: shape,
        }
    }

    fn get<'a>(
        &'a self,
        vector: &S::DummyVectorType,
    ) -> Result<Vec<Option<&'a T>>, OutOfShapeError> {
        let mut ret = Vec::new();
        let mut now = self.shape.zero();
        for i in 0..self.shape.dimension() {
            if let DummyIndex::Index(index) = vector[i] {
                now[i] = self.shape.actual_index(i, index)
                // todo
            }
        }
        Ok(ret)
    }

    // fn map<'a>(&'a self, vector: &S::MapVectorType) -> MultiArrayView<'a, T, DynShape> {}
}

impl<T: Sized, S: Shape> Index<usize> for MultiArray<T, S> {
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

impl<T: Sized, S: Shape> IndexMut<usize> for MultiArray<T, S> {
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

impl<T: Sized, S: Shape> Index<&S::VectorType> for MultiArray<T, S> {
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

impl<T: Sized, S: Shape> IndexMut<&S::VectorType> for MultiArray<T, S> {
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

macro_rules! vector_index {
    ($x:literal) => {
        $x as usize
    };
    ($x:expr) => {
        $x as usize
    };
}

#[macro_export]
macro_rules! vector {
    ($($x:expr),*) => {
        &[$(vector_index!($x),)*]
    };
}

#[macro_export]
macro_rules! dyn_vector {
    ($($x:expr),*) => {
        vec!($(vector_index!($x),)*)
    };
}
