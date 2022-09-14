use std::ops::{
    Bound, Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};

pub(self) trait DummyIndexRange {
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

enum DummyIndex<'a> {
    Index(isize),
    Range(Box<dyn DummyIndexRange>),
    IndexArray(&'a [isize]),
}

impl<'a> From<isize> for DummyIndex<'a> {
    fn from(value: isize) -> Self {
        Self::Index(value)
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

impl<'a> From<&[isize]> for DummyIndex<'a> {
    fn from(indexes: &[isize]) -> Self {
        Self::IndexArray(indexes)
    }
}

macro_rules! dummy {
    ($($x:expr),*) => {
        [$(DummyIndex::from($x),)*]
    };
}

macro_rules! dyn_dummy {
    ($($x:expr),*) => {
        vec!($(DummyIndex::from($x),)*)
    };
}
