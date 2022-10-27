use super::dummy_vector::DummyIndex;
use super::map_vector::MapIndex;
use std::fmt;
use std::mem;
use std::ops;

const DYN_DIMENSION: usize = usize::MAX;

#[derive(Clone, Copy)]
pub struct DimensionMismatchingError {
    pub dimension: usize,
    pub vector_dimension: usize,
}

impl fmt::Display for DimensionMismatchingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Dimension should be {}, not {}.",
            self.dimension, self.vector_dimension
        )
    }
}

impl fmt::Debug for DimensionMismatchingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Dimension should be {}, not {}.",
            self.dimension, self.vector_dimension
        )
    }
}

#[derive(Clone, Copy)]
pub struct OutOfShapeError {
    pub dimension: usize,
    pub len: usize,
    pub vector_index: isize,
}

impl fmt::Display for OutOfShapeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Length of dimension {} is {}, but it get {}.",
            self.dimension, self.len, self.vector_index
        )
    }
}

impl fmt::Debug for OutOfShapeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Length of dimension {} is {}, but it get {}.",
            self.dimension, self.len, self.vector_index
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum IndexCalculationError {
    DimensionMismatching(DimensionMismatchingError),
    OutOfShape(OutOfShapeError),
}

impl fmt::Display for IndexCalculationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IndexCalculationError::DimensionMismatching(err) => {
                write!(f, "{}", err)
            }
            IndexCalculationError::OutOfShape(err) => {
                write!(f, "{}", err)
            }
        }
    }
}

pub trait Shape {
    const DIMENSION: usize;
    type VectorType: ops::IndexMut<usize, Output = usize>;

    fn zero(&self) -> Self::VectorType;

    fn len(&self) -> usize;
    fn dimension(&self) -> usize {
        Self::DIMENSION
    }
    fn dimension_of(vector: &Self::VectorType) -> usize {
        Self::DIMENSION
    }

    fn shape(&self) -> &[usize];
    fn offset(&self) -> &[usize];

    fn len_of_dimension(&self, dimension: usize) -> Result<usize, DimensionMismatchingError> {
        if dimension > Self::DIMENSION {
            Err(DimensionMismatchingError {
                dimension: Self::DIMENSION,
                vector_dimension: dimension,
            })
        } else {
            Ok(self.shape()[dimension])
        }
    }

    fn offset_of_dimension(&self, dimension: usize) -> Result<usize, DimensionMismatchingError> {
        if dimension > Self::DIMENSION {
            Err(DimensionMismatchingError {
                dimension: Self::DIMENSION,
                vector_dimension: dimension,
            })
        } else {
            Ok(self.offset()[dimension])
        }
    }

    fn index(&self, vector: &Self::VectorType) -> Result<usize, IndexCalculationError> {
        if Self::dimension_of(vector) > self.dimension() {
            Err(IndexCalculationError::DimensionMismatching(
                DimensionMismatchingError {
                    dimension: self.dimension(),
                    vector_dimension: Self::dimension_of(vector),
                },
            ))
        } else {
            let mut index = 0;
            for i in 0..self.dimension() {
                if vector[i] > self.len_of_dimension(i).unwrap() {
                    return Err(IndexCalculationError::OutOfShape(OutOfShapeError {
                        dimension: i,
                        len: self.len_of_dimension(i).unwrap(),
                        vector_index: vector[i] as isize,
                    }));
                }
                index += vector[i] * self.offset_of_dimension(i).unwrap();
            }
            Ok(index)
        }
    }

    fn vector(&self, mut index: usize) -> Self::VectorType {
        let mut vector = self.zero();
        for i in 0..self.dimension() {
            let offset = self.offset_of_dimension(i).unwrap();
            vector[i] = index / offset;
            index = index % offset;
        }
        vector
    }

    fn next_vector(&self, vector: &mut Self::VectorType) -> bool {
        let mut carry = false;
        vector[self.dimension() - 1] += 1;

        for i in (0..self.dimension()).rev() {
            if carry {
                vector[i] += 1;
                carry = false;
            }
            if vector[i] == self.len_of_dimension(i).unwrap() {
                vector[i] = 0;
                carry = true;
            }
        }
        !carry
    }

    fn actual_index(&self, dimension: usize, index: isize) -> Option<usize> {
        let len = self.len_of_dimension(dimension).unwrap();
        if index >= (len as isize) || index < -(len as isize) {
            None
        } else {
            Some((index % (len as isize)) as usize)
        }
    }
}

pub trait DummyShape<'a>: Shape {
    type DummyVectorType: ops::IndexMut<usize, Output = DummyIndex<'a>>;

    fn iterator_of(
        &'a self,
        dimension: usize,
        dummy_index: &DummyIndex<'a>,
    ) -> Box<dyn Iterator<Item = usize> + 'a> {
        match dummy_index {
            DummyIndex::Index(index) => match self.actual_index(dimension, *index) {
                Some(value) => Box::new(ops::Range {
                    start: value,
                    end: value + 1,
                }),
                None => Box::new(ops::Range { start: 0, end: 1 }),
            },
            DummyIndex::Range(range) => {
                let lower_bound = match range.start_bound() {
                    ops::Bound::Included(value) => self.actual_index(dimension, value),
                    ops::Bound::Excluded(value) => self.actual_index(dimension, value - 1),
                    ops::Bound::Unbounded => Some(0),
                };
                let upper_bound = match range.end_bound() {
                    ops::Bound::Included(value) => self.actual_index(dimension, value + 1),
                    ops::Bound::Excluded(value) => self.actual_index(dimension, value),
                    ops::Bound::Unbounded => Some(self.len_of_dimension(dimension).unwrap()),
                };
                if lower_bound.is_some() && upper_bound.is_some() {
                    Box::new(ops::Range {
                        start: lower_bound.unwrap(),
                        end: upper_bound.unwrap(),
                    })
                } else {
                    Box::new(ops::Range { start: 0, end: 1 })
                }
            }
            DummyIndex::IndexArray(indexes) => Box::new(
                indexes
                    .clone()
                    .filter_map(move |index| self.actual_index(dimension, index)),
            ),
        }
    }
}

pub(self) fn offset<const DIMENSION: usize>(
    shape: &[usize; DIMENSION],
) -> ([usize; DIMENSION], usize) {
    let mut offset: [usize; DIMENSION] = unsafe { mem::zeroed() };

    offset[shape.len() - 1] = 1;
    let mut len = 1;
    for i in (0..(shape.len() - 1)).rev() {
        offset[i] = offset[i + 1] * shape[i + 1];
        len *= shape[i + 1];
    }
    len *= shape[0];
    (offset, len)
}

#[derive(Clone, Copy)]
pub struct Shape1 {
    pub(self) shape: [usize; 1],
}

impl Shape1 {
    pub fn new(shape: [usize; 1]) -> Self {
        Self { shape: shape }
    }
}

impl Shape for Shape1 {
    const DIMENSION: usize = 1;
    type VectorType = [usize; 1];

    fn zero(&self) -> Self::VectorType {
        [0]
    }

    fn len(&self) -> usize {
        self.shape[0]
    }

    fn shape(&self) -> &[usize] {
        &self.shape
    }

    fn offset(&self) -> &[usize] {
        &self.shape
    }
}

impl<'a> DummyShape<'a> for Shape1 {
    type DummyVectorType = [DummyIndex<'a>; 1];
}

macro_rules! shape {
    ($type:ident, $dim:expr) => {
        #[derive(Clone, Copy)]
        pub struct $type {
            pub(self) shape: [usize; $dim],
            pub(self) offset: [usize; $dim],
            pub(self) len: usize,
        }

        impl $type {
            pub fn new(shape: [usize; $dim]) -> Self {
                let (offset, len) = offset(&shape);
                Self {
                    shape: shape,
                    offset: offset,
                    len: len,
                }
            }
        }

        impl Shape for $type {
            const DIMENSION: usize = $dim;
            type VectorType = [usize; $dim];

            fn zero(&self) -> Self::VectorType {
                unsafe { mem::zeroed() }
            }

            fn len(&self) -> usize {
                self.len
            }

            fn shape(&self) -> &[usize] {
                &self.shape
            }

            fn offset(&self) -> &[usize] {
                &self.offset
            }
        }

        impl<'a> DummyShape<'a> for $type {
            type DummyVectorType = [DummyIndex<'a>; $dim];
        }
    };
}

shape!(Shape2, 2);
shape!(Shape3, 3);
shape!(Shape4, 4);
shape!(Shape5, 5);
shape!(Shape6, 6);
shape!(Shape7, 7);
shape!(Shape8, 8);
shape!(Shape9, 9);
shape!(Shape10, 10);
shape!(Shape11, 11);
shape!(Shape12, 12);
shape!(Shape13, 13);
shape!(Shape14, 14);
shape!(Shape15, 15);
shape!(Shape16, 16);
shape!(Shape17, 17);
shape!(Shape18, 18);
shape!(Shape19, 19);
shape!(Shape20, 20);

pub struct DynShape {
    pub(self) shape: Vec<usize>,
    pub(self) offset: Vec<usize>,
    pub(self) len: usize,
}

impl DynShape {
    pub fn new(shape: Vec<usize>) -> Self {
        let (offset, len) = Self::offset(&shape);
        Self {
            shape: shape,
            offset: offset,
            len: len,
        }
    }

    pub(self) fn offset(shape: &Vec<usize>) -> (Vec<usize>, usize) {
        let mut offset: Vec<usize> = (0..shape.len()).map(|_| 0).collect();
        offset[shape.len() - 1] = 1;
        let mut len = 1;
        for i in (0..(shape.len() - 1)).rev() {
            offset[i] = offset[i + 1] * shape[i + 1];
            len *= shape[i + 1];
        }
        len *= shape[0];
        (offset, len)
    }
}

impl Shape for DynShape {
    const DIMENSION: usize = DYN_DIMENSION;
    type VectorType = Vec<usize>;

    fn zero(&self) -> Self::VectorType {
        (0..self.shape.len()).map(|_| 0).collect()
    }

    fn len(&self) -> usize {
        self.len
    }

    fn dimension(&self) -> usize {
        self.shape.len()
    }

    fn dimension_of(vector: &Self::VectorType) -> usize {
        vector.len()
    }

    fn shape(&self) -> &[usize] {
        &self.shape
    }

    fn offset(&self) -> &[usize] {
        &self.offset
    }
}

impl<'a> DummyShape<'a> for DynShape {
    type DummyVectorType = Vec<DummyIndex<'a>>;
}
