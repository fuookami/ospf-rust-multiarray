use super::Shape;
use std::cell::RefCell;
use std::ops::{
    Bound, Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
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
    IndexArray(Box<dyn Iterator<Item = isize>>),
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

pub(super) struct DummyAccessPolicy<'a, S: Shape> {
    vector: &'a S::DummyVectorType,
    shape: S,
    now: RefCell<S::VectorType>,
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
        &[$(dummy_index!($x),)*]
    };
}

#[macro_export]
macro_rules! dyn_dummy {
    ($($x:expr),*) => {
        &vec!($(dummy_index!($x),)*)
    };
}
