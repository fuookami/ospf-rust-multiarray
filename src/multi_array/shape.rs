use std::fmt;

const DYN_DIMENSION: usize = usize::MAX;

pub struct DimensionError {
    pub dimension: usize,
    pub target_dimension: usize
}

impl fmt::Debug for DimensionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Dimension error: {}, but there is only {}", self.dimension, self.target_dimension)
    }
}

pub struct OutOfIndexError {
    pub dimension: usize,
    pub size_of_dimension: usize,
    pub index_of_dimension: usize
}

impl fmt::Debug for OutOfIndexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Out of index error: {} in dimension {}, but there is only {}", self.index_of_dimension, self.dimension, self.size_of_dimension)
    }
}

pub enum IndexCalculationError {
    DimensionError(DimensionError),
    OutOfIndexError(OutOfIndexError)
}

pub trait Shape {
    const DIMENSION: usize;

    fn len(&self) -> usize;
    fn dimension(&self) -> usize { Self::DIMENSION }
    fn len_of_dimension(&self, dimension: usize) -> Result<usize, DimensionError>;
    fn offset_of_dimension(&self, dimension: usize) -> Result<usize, DimensionError>;

    fn index(&self, vector: &[usize]) -> Result<usize, IndexCalculationError> {
        if vector.len() > self.dimension() {
            Err(IndexCalculationError::DimensionError(
                DimensionError { 
                    dimension: self.dimension(), 
                    target_dimension: vector.len() 
                }))
        } else {
            let mut index = 0;
            for i in 0..self.dimension() {
                if vector[i] > self.len_of_dimension(i).unwrap() {
                    return Err(IndexCalculationError::OutOfIndexError(
                        OutOfIndexError { 
                            dimension: i,
                            size_of_dimension: self.len_of_dimension(i).unwrap(),
                            index_of_dimension: vector[i]
                        }))
                }
            }
            Ok(index)
        }
    }
}

pub struct Shape1 {
    pub(self) shape: [usize; 1],
    pub(self) len: usize
}

impl Shape1 {
    fn new() -> Self {
        let len = Self::len(&[1]);
        Shape1 {
            shape: [0],
            len: len
        }
    }

    fn new_from_array(shape: [usize; 1]) -> Self {
        let len = Self::len(&shape);
        Shape1 {
            shape: shape,
            len: len
        }
    }

    pub(self) fn len(shape: &[usize; 1]) -> usize { shape[0] }
}

impl Shape for Shape1 {
    const DIMENSION: usize = 1;

    fn len(&self) -> usize { self.len }

    fn len_of_dimension(&self, dimension: usize) -> Result<usize, DimensionError> {
        if dimension > Self::DIMENSION {
            Err(DimensionError { 
                dimension: Self::DIMENSION, 
                target_dimension: dimension
            })
        } else {
            Ok(self._shape[dimension])
        }
    }
}

pub struct Shape2 {
    pub(self) _shape: [usize; 2],
    pub(self) _offset: [usize; 2]
}

impl Shape2 {
    fn new() -> Self {
        let offset = Self::offset(&[1, 1]);
        Shape2 {
            _shape: [1, 1],
            _offset: offset
        }
    }

    fn new_from_array(shape: [usize; 2]) -> Self {
        let offset = Self::offset(&shape);
        Shape2 {
            _shape: shape,
            _offset: offset
        }
    }

    fn offset(shape: &[usize; 2]) -> [usize; 2] {
        [shape[1], 1]
    }
}

impl Shape for Shape2 {
    const DIMENSION: usize = 1;

    fn index(&self, vector: &[usize]) -> Result<usize, IndexCalculationError> {
        if vector.len() != Self::DIMENSION {
            Err(IndexCalculationError::DimensionError(
                DimensionError { 
                    dimension: self._shape.len(), 
                    targetDimension: vector.len() 
                }))
        } else if let outOfIndexDimension = Option::Some(self.check(&vector)) {
            // todo: impl it
            Ok(0)
        } else {
            Ok(0)
        }
    }
}

pub struct Shape3 {
    pub(self) shape: [usize; 3],
    pub(self) offset: [usize; 3]
}

impl Shape3 {
    fn new() -> Self {
        return Shape3 {
            _shape: [1, 1, 1]
        }
    }

    fn new_from_array(shape: [usize; 3]) -> Self {
        return Shape3 {
            _shape: shape
        }
    }
}

impl Shape for Shape3 {
    const DIMENSION: usize = 1;

    fn index(&self, vector: &[usize]) -> Result<usize, IndexCalculationError> {
        return if vector.len() != Self::DIMENSION {
            Err(DimensionError { dimension: self._shape.len(), targetDimension: vector.len() })
        } else {
            // todo: impl it
            Ok(0)
        }
    }
}

pub struct Shape4 {
    pub(self) shape: [usize; 4],
    pub(self) offset: [usize; 4]
}

impl Shape4 {
    fn new() -> Self {
        return Shape4 {
            _shape: [1, 1, 1, 1]
        }
    }

    fn new_from_array(shape: [usize; 4]) -> Self {
        return Shape4 {
            _shape: shape
        }
    }
}

impl Shape for Shape4 {
    const DIMENSION: usize = 1;

    fn index(&self, vector: &[usize]) -> Result<usize, DimensionError> {
        return if vector.len() != Self::DIMENSION {
            Err(DimensionError { dimension: self._shape.len(), targetDimension: vector.len() })
        } else {
            // todo: impl it
            Ok(0)
        }
    }
}

pub struct DynShape {
    pub(self) shape: Vec<usize>,
    pub(self) offset: Vec<usize>
}

impl DynShape {
    fn new() -> Self {
        return DynShape {
            _shape: vec![1]
        }
    }

    fn new_from_array(shape: Vec<usize>) -> Self {
        return DynShape {
            _shape: shape
        }
    }
}

impl Shape for DynShape {
    const DIMENSION: usize = 1;

    fn dimension(&self) -> usize { self._shape.len() }

    fn index(&self, vector: &[usize]) -> Result<usize, DimensionError> {
        return if vector.len() != self._shape.len() {
            Err(DimensionError { dimension: self._shape.len(), targetDimension: vector.len() })
        } else {
            // todo: impl it
            Ok(0)
        }
    }
}
