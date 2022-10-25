use super::{DynShape, Shape};
use meta_programming::GeneratorIterator;
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
    IndexArray(Box<dyn Iterator<Item = isize>>),
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

pub(crate) struct DummyAccessPolicy<'a, T: Sized, S: Shape> {
    pub(self) container: &'a Vec<Option<T>>,
    pub(self) vector: S::DummyVectorType,
    pub(self) shape: &'a S,
}

impl<'a, T: Sized, S: Shape> DummyAccessPolicy<'a, T, S>
where
    S::DummyVectorType: IndexMut<usize, Output = DummyIndex>,
    &'a S::DummyVectorType: IntoIterator<Item = &'a DummyIndex>,
{
    pub(crate) fn new(
        container: &'a Vec<Option<T>>,
        vector: S::DummyVectorType,
        shape: &'a S,
    ) -> Self {
        Self {
            container: container,
            vector: vector,
            shape: shape,
        }
    }

    pub fn iter(&'a self) -> impl Iterator<Item = &'a Option<T>> + 'a {
        GeneratorIterator(move || {
            let mut iterators: Vec<Box<dyn Iterator<Item = usize>>> = self
                .vector
                .into_iter()
                .enumerate()
                .map(|(i, v)| self.shape.iterator_of(i, v))
                .collect();
            let mut now = self.shape.zero();

            while self.next(&mut now, &mut iterators) {
                yield self.container[self.shape.index(&now).unwrap()]
            }

            return;
        })
    }

    pub(self) fn next(
        &self,
        now: &mut S::VectorType,
        iterators: &mut Vec<Box<dyn Iterator<Item = usize>>>,
    ) -> bool {
        for i in (0..iterators.len()).rev() {
            match iterators[i].next() {
                Some(value) => {
                    now[i] = value;
                    return true;
                }
                None => {
                    iterators[i] = self.shape.iterator_of(i, &self.vector[i]);
                    now[i] = iterators[i].next().unwrap();
                }
            }
        }
        return false;
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
