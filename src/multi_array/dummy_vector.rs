use super::DummyShape;
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

pub(super) enum DummyIndex<'a> {
    Index(isize),
    Range(Box<dyn DummyIndexRange>),
    IndexArray(Box<dyn Iterator<Item = isize> + 'a>),
}

impl<'a> DummyIndex<'a> {
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

impl<'a> From<isize> for DummyIndex<'a> {
    fn from(value: isize) -> Self {
        Self::Index(value)
    }
}

impl<'a> From<usize> for DummyIndex<'a> {
    fn from(value: usize) -> Self {
        Self::Index(value as isize)
    }
}

impl<'a> From<Range<isize>> for DummyIndex<'a> {
    fn from(range: Range<isize>) -> Self {
        Self::Range(Box::new(range))
    }
}

impl<'a> From<RangeFrom<isize>> for DummyIndex<'a> {
    fn from(range: RangeFrom<isize>) -> Self {
        Self::Range(Box::new(range))
    }
}

impl<'a> From<RangeInclusive<isize>> for DummyIndex<'a> {
    fn from(range: RangeInclusive<isize>) -> Self {
        Self::Range(Box::new(range))
    }
}

impl<'a> From<RangeTo<isize>> for DummyIndex<'a> {
    fn from(range: RangeTo<isize>) -> Self {
        Self::Range(Box::new(range))
    }
}

impl<'a> From<RangeToInclusive<isize>> for DummyIndex<'a> {
    fn from(range: RangeToInclusive<isize>) -> Self {
        Self::Range(Box::new(range))
    }
}

impl<'a> From<RangeFull> for DummyIndex<'a> {
    fn from(range: RangeFull) -> Self {
        Self::Range(Box::new(range))
    }
}

impl<'a> From<&'a [isize]> for DummyIndex<'a> {
    fn from(indexes: &'a [isize]) -> Self {
        Self::IndexArray(Box::new(indexes.iter().map(|index| *index)))
    }
}

pub(crate) struct DummyAccessPolicy<'a, T: Sized, S: DummyShape<'a>> {
    pub(self) container: &'a Vec<Option<T>>,
    pub(self) vector: S::DummyVectorType,
    pub(self) shape: &'a S,
}

struct DummyAccessIterator<'a> {
    iterators: Vec<Box<dyn Iterator<Item = usize> + 'a>>,
}

impl<'a> DummyAccessIterator<'a> {
    pub(self) fn next<S: DummyShape<'a>>(
        &mut self,
        now: &mut S::VectorType,
        shape: &'a S,
        vector: &S::DummyVectorType,
    ) -> bool {
        for i in (0..self.iterators.len()).rev() {
            match self.iterators[i].next() {
                Some(value) => {
                    now[i] = value;
                    return true;
                }
                None => {
                    self.iterators[i] = shape.iterator_of(i, &vector[i]);
                    now[i] = self.iterators[i].next().unwrap();
                }
            }
        }
        return false;
    }
}

impl<'a, T: Sized, S: DummyShape<'a>> DummyAccessPolicy<'a, T, S>
where
    S::DummyVectorType: IndexMut<usize, Output = DummyIndex<'a>>,
    &'a S::DummyVectorType: IntoIterator<Item = &'a DummyIndex<'a>>,
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

    pub fn iter<'b>(&'a self) -> impl Iterator<Item = &'a Option<T>> + 'b
    where
        'a: 'b,
    {
        GeneratorIterator(move || {
            let mut iter = DummyAccessIterator {
                iterators: self
                    .vector
                    .into_iter()
                    .enumerate()
                    .map(|(i, v)| self.shape.iterator_of(i, v))
                    .collect(),
            };
            let mut now = self.shape.zero();

            while iter.next(&mut now, self.shape, &self.vector) {
                yield &self.container[self.shape.index(&now).unwrap()]
            }

            return;
        })
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
