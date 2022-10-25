use crate::{dummy_vector::{DummyIndex, DummyAccessPolicy}, OutOfShapeError};
use super::{DynShape, Shape, Shape1, Shape2, Shape3, Shape4};
use std::ops::{Index, IndexMut};

pub struct MultiArray<T: Sized, S: Shape> {
    pub(self) list: Vec<Option<T>>,
    pub(self) shape: S,
}

impl<T: Sized, S: Shape> MultiArray<T, S> {
    pub fn new(shape: S) -> Self {
        Self {
            list: (0..shape.len()).map(|_| Option::None).collect(),
            shape: shape,
        }
    }

    pub fn new_with(shape: S, value: T) -> Self
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

    pub fn new_by<G>(shape: S, generator: G) -> Self
        where
            G: Fn(usize) -> T,
    {
        Self {
            list: (0..shape.len())
                .map(|index| Option::Some(generator(index)))
                .collect(),
            shape: shape,
        }
    }

    pub fn get<'a>(
        &'a self,
        vector: S::DummyVectorType,
    ) -> Result<Vec<&Option<T>>, OutOfShapeError>
        where
            S::DummyVectorType: IndexMut<usize, Output=DummyIndex>,
            &'a S::DummyVectorType: IntoIterator<Item=&'a DummyIndex>
    {
        let mut policy = DummyAccessPolicy::new(&self.list, vector, &self.shape);
        Ok(policy.iter().collect())
    }

    // fn map<'a>(&'a self, vector: &S::MapVectorType) -> MultiArrayView<'a, T, DynShape> {}
}

impl<T: Sized, S: Shape> Index<usize> for MultiArray<T, S> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match &self.list[index] {
            Some(value) => value,
            None => {
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
            Some(value) => value,
            None => {
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
            Ok(index) => match &self.list[index] {
                Some(value) => value,
                None => {
                    panic!(
                        "Element with index {} in the multi-array is not initialized",
                        index
                    )
                }
            },
            Err(err) => panic!("{}", err),
        }
    }
}

impl<T: Sized, S: Shape> IndexMut<&S::VectorType> for MultiArray<T, S> {
    fn index_mut(&mut self, vector: &S::VectorType) -> &mut Self::Output {
        match self.shape.index(vector) {
            Ok(index) => match &mut self.list[index] {
                Some(value) => value,
                None => {
                    panic!(
                        "Element with index {} in the multi-array is not initialized",
                        index
                    )
                }
            },
            Err(err) => panic!("{}", err),
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

#[test]
fn fuck() {
    let vector = MultiArray2::<u64>::new_by(Shape2::new([10, 11]), |i: usize| i as u64);
    for val in vector.get(dummy!(0, ..)).unwrap() {
        print!("{}, ", val.unwrap())
    }
}
