use crate::DynShape;

use super::Shape;
use std::cell::RefCell;
use std::ops::{
    Bound, IndexMut, Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo,
    RangeToInclusive,
};

pub(super) trait DummyIndexRange {
    fn start_bound(&self) -> Bound<isize>;
    fn end_bound(&self) -> Bound<isize>;
    fn contains(&self, v: isize) -> bool;
}

impl<T> DummyIndexRange for T
    where
        T: RangeBounds<isize>,
{
    fn start_bound(&self) -> Bound<isize> {
        match RangeBounds::start_bound(self) {
            Bound::Included(value) => Bound::Included(*value),
            Bound::Excluded(value) => Bound::Excluded(*value),
            Bound::Unbounded => Bound::Unbounded,
        }
    }

    fn end_bound(&self) -> Bound<isize> {
        match RangeBounds::end_bound(self) {
            Bound::Included(value) => Bound::Included(*value),
            Bound::Excluded(value) => Bound::Excluded(*value),
            Bound::Unbounded => Bound::Unbounded,
        }
    }

    fn contains(&self, value: isize) -> bool {
        RangeBounds::contains(self, &value)
    }
}

pub(super) enum DummyIndex {
    Index(isize),
    Range(Box<dyn DummyIndexRange>),
    IndexArray(Box<dyn Iterator<Item=isize>>),
}

impl DummyIndex {
    fn dummy(&self) -> bool {
        if let DummyIndex::Index(_) = self {
            false
        } else {
            true
        }
    }

    fn continuous(&self) -> bool {
        if let DummyIndex::Range(_) = self {
            true
        } else {
            false
        }
    }

    fn discrete(&self) -> bool {
        if let DummyIndex::IndexArray(_) = self {
            true
        } else {
            false
        }
    }
}

impl From<isize> for DummyIndex {
    fn from(value: isize) -> Self {
        Self::Index(value)
    }
}

impl From<usize> for DummyIndex {
    fn from(value: usize) -> Self {
        Self::Index(value as isize)
    }
}

impl From<Range<isize>> for DummyIndex {
    fn from(range: Range<isize>) -> Self {
        Self::Range(Box::new(range))
    }
}

impl From<RangeFrom<isize>> for DummyIndex {
    fn from(range: RangeFrom<isize>) -> Self {
        Self::Range(Box::new(range))
    }
}

impl From<RangeInclusive<isize>> for DummyIndex {
    fn from(range: RangeInclusive<isize>) -> Self {
        Self::Range(Box::new(range))
    }
}

impl From<RangeTo<isize>> for DummyIndex {
    fn from(range: RangeTo<isize>) -> Self {
        Self::Range(Box::new(range))
    }
}

impl From<RangeToInclusive<isize>> for DummyIndex {
    fn from(range: RangeToInclusive<isize>) -> Self {
        Self::Range(Box::new(range))
    }
}

impl From<RangeFull> for DummyIndex {
    fn from(range: RangeFull) -> Self {
        Self::Range(Box::new(range))
    }
}

impl From<&[isize]> for DummyIndex {
    fn from(indexes: &[isize]) -> Self {
        Self::IndexArray(Box::new(indexes.iter().map(|index| *index)))
    }
}

pub(super) struct DummyAccessPolicy<'a, T: Sized, S: Shape> {
    pub(self) container: &'a Vec<Option<T>>,
    pub(self) vector: &'a S::DummyVectorType,
    pub(self) shape: S,
    pub(self) now: S::VectorType,
    pub(self) continuous_base: Vec<usize>,
    pub(self) continuous_offset_indexes: Vec<usize>,
    pub(self) discrete_offset_indexes: Vec<usize>,
    pub(self) flag: bool,
}

impl<'a, T: Sized, S: Shape> DummyAccessPolicy<'a, T, S>
    where
        &'a S::DummyVectorType: IndexMut<usize, Output=&'a DummyIndex>,
        &'a S::DummyVectorType: IntoIterator<Item=&'a DummyIndex>,
{
    fn new(container: &'a Vec<Option<T>>, vector: &'a S::DummyVectorType, shape: S) -> Self {
        let continuous_base = Self::continuous_dummy_base(&shape, vector);
        let continuous_offset_indexes = Self::continuous_dummy_indexes(&shape, vector);
        let discrete_offset_indexes = Self::discrete_dummy_indexes(&shape, vector);
        Self {
            container: container,
            vector: vector,
            shape: shape,
            now: shape.zero(),
            continuous_base: continuous_base,
            continuous_offset_indexes: continuous_offset_indexes,
            discrete_offset_indexes: discrete_offset_indexes
        }
    }

    pub(self) fn dummy_dimension(
        dummy_vector: &S::DummyVectorType,
        predicate: &dyn Fn(&DummyIndex) -> bool,
    ) -> usize {
        dummy_vector.into_iter().filter(|x| predicate(x)).count()
    }

    pub(self) fn continuous_dummy_dimension(dummy_vector: &S::DummyVectorType) -> usize {
        Self::dummy_dimension(dummy_vector, &DummyIndex::continuous)
    }

    pub(self) fn discrete_dummy_dimension(dummy_vector: &S::DummyVectorType) -> usize {
        Self::dummy_dimension(dummy_vector, &DummyIndex::discrete)
    }

    pub(self) fn dummy_indexes(
        shape: &S,
        dummy_vector: &S::DummyVectorType,
        predicate: &dyn Fn(&DummyIndex) -> bool,
    ) -> Vec<usize> {
        let dimension = Self::dummy_dimension(dummy_vector, predicate);
        let mut ret: Vec<usize> = (0..dimension).map(|_| 0).collect();
        if dimension != 0 {
            let mut j = 0;
            for i in 0..shape.dimension() {
                if predicate(dummy_vector[i]) {
                    ret[j] = i;
                    j += 1;
                }
            }
        }
        ret
    }

    pub(self) fn continuous_dummy_indexes(
        shape: &S,
        dummy_vector: &S::DummyVectorType,
    ) -> Vec<usize> {
        Self::dummy_indexes(shape, dummy_vector, &DummyIndex::continuous)
    }

    pub(self) fn discrete_dummy_indexes(shape: &S, dummy_vector: &S::DummyVectorType) -> Vec<usize> {
        Self::dummy_indexes(shape, dummy_vector, &DummyIndex::discrete)
    }

    pub(self) fn continuous_dummy_base(shape: &S, dummy_vector: &S::DummyVectorType) -> Vec<usize> {
        let dimension = Self::continuous_dummy_dimension(dummy_vector);
        let mut ret = Vec::with_capacity(dimension);
        ret.resize(dimension, 0);
        if dimension != 0 {
            let mut j = 0;
            for i in 0..shape.dimension() {
                if dummy_vector[i].continuous() {
                    ret[j] = i;
                    j += 1;
                }
            }
        }
        ret
    }
}

impl<'a, T: Sized, S: Shape> Iterator for DummyAccessPolicy<'a, T, S> {
    type Item = Vec<&'a Option<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.flag {
            None
        }
        else {
            let mut ret = Vec::new();
            for i in 0..self.continuous_offset_indexes.len() {
                self.now[self.continuous_offset_indexes[i]] = self.continuous_base[i] + self.continuous_offset_indexes[i]
            }
            // todo: impl it
            Some(ret)
        }
    }
}

macro_rules! dummy_index {
    ($x:literal) => {
        DummyIndex::from($x as isize)
    };
    ($x:expr) => {
        DummyIndex::from($x)
    };
}

#[macro_export]
macro_rules! dummy {
    ($($x:expr),*) => {
        [$(dummy_index!($x),)*]
    };
}

#[macro_export]
macro_rules! dyn_dummy {
    ($($x:expr),*) => {
        vec!($(dummy_index!($x),)*)
    };
}
