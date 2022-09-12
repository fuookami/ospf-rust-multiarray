use std::fmt;
use std::ops;

const DYN_DIMENSION: usize = usize::MAX;

pub struct DimensionMismatchingError {
    pub dimension: usize,
    pub vector_dimension: usize,
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

pub struct OutOfShapeError {
    pub dimension: usize,
    pub size: usize,
    pub vector_index: usize,
}

impl fmt::Debug for OutOfShapeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Length of dimension {} is {}, but it get {}.",
            self.dimension, self.size, self.vector_index
        )
    }
}

pub enum IndexCalculationError {
    DimensionMismatching(DimensionMismatchingError),
    OutOfShape(OutOfShapeError),
}

pub trait Shape {
    const DIMENSION: usize;
    type VectorType: ops::IndexMut<usize, Output = usize>;

    fn zero(&self) -> Self::VectorType;

    fn len(&self) -> usize;
    fn dimension(&self) -> usize {
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

    fn index(&self, vector: &[usize]) -> Result<usize, IndexCalculationError> {
        if vector.len() > self.dimension() {
            Err(IndexCalculationError::DimensionMismatching(
                DimensionMismatchingError {
                    dimension: self.dimension(),
                    vector_dimension: vector.len(),
                },
            ))
        } else {
            let mut index = 0;
            for i in 0..self.dimension() {
                if vector[i] > self.len_of_dimension(i).unwrap() {
                    return Err(IndexCalculationError::OutOfShape(OutOfShapeError {
                        dimension: i,
                        size: self.len_of_dimension(i).unwrap(),
                        vector_index: vector[i],
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
            let offest = self.offset_of_dimension(i).unwrap();
            vector[i] = index / offest;
            index = index % offest;
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
}

pub struct Shape1 {
    pub(self) shape: [usize; 1],
}

impl Shape1 {
    fn new(shape: [usize; 1]) -> Self {
        Shape1 { shape: shape }
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

pub struct Shape2 {
    pub(self) shape: [usize; 2],
    pub(self) offset: [usize; 2],
    pub(self) len: usize,
}

impl Shape2 {
    fn new(shape: [usize; 2]) -> Self {
        let (offset, len) = Self::offset(&shape);
        Shape2 {
            shape: shape,
            offset: offset,
            len: len,
        }
    }

    pub(self) fn offset(shape: &[usize; 2]) -> ([usize; 2], usize) {
        ([shape[0], 1], shape[0] * shape[1])
    }
}

impl Shape for Shape2 {
    const DIMENSION: usize = 2;
    type VectorType = [usize; 2];

    fn zero(&self) -> Self::VectorType {
        [0, 0]
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

pub struct Shape3 {
    pub(self) shape: [usize; 3],
    pub(self) offset: [usize; 3],
    pub(self) len: usize,
}

impl Shape3 {
    fn new(shape: [usize; 3]) -> Self {
        let (offset, len) = Self::offset(&shape);
        Shape3 {
            shape: shape,
            offset: offset,
            len: len,
        }
    }

    pub(self) fn offset(shape: &[usize; 3]) -> ([usize; 3], usize) {
        (
            [shape[0] * shape[1], shape[0], 1],
            shape[0] * shape[1] * shape[2],
        )
    }
}

impl Shape for Shape3 {
    const DIMENSION: usize = 3;
    type VectorType = [usize; 3];

    fn zero(&self) -> Self::VectorType {
        [0, 0, 0]
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

pub struct Shape4 {
    pub(self) shape: [usize; 4],
    pub(self) offset: [usize; 4],
    pub(self) len: usize,
}

impl Shape4 {
    fn new(shape: [usize; 4]) -> Self {
        let (offset, len) = Self::offset(&shape);
        Shape4 {
            shape: shape,
            offset: offset,
            len: len,
        }
    }

    pub(self) fn offset(shape: &[usize; 4]) -> ([usize; 4], usize) {
        (
            [
                shape[0] * shape[1] * shape[2],
                shape[0] * shape[1],
                shape[0],
                1,
            ],
            shape[0] * shape[1] * shape[2] * shape[3],
        )
    }
}

impl Shape for Shape4 {
    const DIMENSION: usize = 4;
    type VectorType = [usize; 4];

    fn zero(&self) -> Self::VectorType {
        [0, 0, 0, 0]
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

pub struct DynShape {
    pub(self) shape: Vec<usize>,
    pub(self) offset: Vec<usize>,
    pub(self) len: usize,
}

impl DynShape {
    fn new(shape: Vec<usize>) -> Self {
        let (offset, len) = Self::offset(&shape);
        DynShape {
            shape: shape,
            offset: offset,
            len: len,
        }
    }

    pub(self) fn offset(shape: &Vec<usize>) -> (Vec<usize>, usize) {
        let mut offset: Vec<usize> = (0..shape.len()).map(|_| 0).collect();
        offset[shape.len() - 1] = 1;
        let mut len = 1;
        for i in (0..(shape.len() - 1)).rev() {}
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

    fn shape(&self) -> &[usize] {
        &self.shape
    }

    fn offset(&self) -> &[usize] {
        &self.offset
    }
}
