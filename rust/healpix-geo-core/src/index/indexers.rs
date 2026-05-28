use num_traits::{FromPrimitive, PrimInt};

/// Slice object with the semantics of python's slice object
#[derive(Debug, Clone, PartialEq)]
struct Slice<T: PrimInt> {
    pub start: Option<T>,
    pub stop: Option<T>,
    pub step: T,
}

struct ConcreteSlice<T: PrimInt> {
    pub start: T,
    pub stop: T,
    pub step: T,
}

impl<T: PrimInt + FromPrimitive> Slice<T> {
    pub fn create(start: Option<T>, stop: Option<T>, step: Option<T>) -> Self {
        Self {
            start,
            stop,
            step: step.unwrap_or(FromPrimitive::from_usize(1).unwrap()),
        }
    }
}

impl Slice<isize> {
    pub fn normalize(&self, size: usize) -> ConcreteSlice<isize> {
        let step = self.step;
        let size_ = size as isize;

        let start: isize = self.start.map_or_else(
            || if step < 0 { size_ - 1 } else { 0 },
            |v| if v < 0 { size_ + v } else { v },
        );
        let stop: isize = self
            .stop
            .map_or_else(
                || if step < 0 { -1 } else { size_ },
                |v| if v < 0 { size_ + v } else { v },
            )
            .min(size_);

        ConcreteSlice { start, stop, step }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Array<T> {
    pub data: Vec<T>,
}

impl<T> Array<T> {
    pub fn create(values: Vec<T>) -> Self {
        Self { data: values }
    }
}

impl Array<isize> {
    pub fn normalize(self, size: usize) -> Array<usize> {
        Array::<usize>::create(
            self.data
                .into_iter()
                .map(|v| if v < 0 { v + size as isize } else { v } as usize)
                .collect(),
        )
    }
}

pub(crate) enum PositionalIndexer {
    Slice(Slice<isize>),
    Array(Array<isize>),
}

pub(crate) enum LabelIndexer {
    Slice(Slice<usize>),
    Array(Array<usize>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slice_create() {
        let slice = Slice::create(None, Some(5), None);

        assert_eq!(slice.start, None);
        assert_eq!(slice.stop, Some(5));
        assert_eq!(slice.step, 1);
    }

    #[test]
    fn test_slice_normalize_positive_full() {
        let slice = Slice {
            start: None,
            stop: None,
            step: 1,
        };
        let actual = slice.normalize(5);

        assert_eq!(actual.start, 0);
        assert_eq!(actual.stop, 5);
        assert_eq!(actual.step, 1);
    }

    #[test]
    fn test_slice_normalize_negative_full() {
        let slice = Slice {
            start: None,
            stop: None,
            step: -1,
        };
        let actual = slice.normalize(5);

        assert_eq!(actual.start, 4);
        assert_eq!(actual.stop, -1isize);
        assert_eq!(actual.step, -1);
    }

    #[test]
    fn test_slice_normalize_positive_limit() {
        let slice = Slice {
            start: None,
            stop: Some(5),
            step: 1,
        };

        let actual = slice.normalize(10);
        assert_eq!(actual.start, 0);
        assert_eq!(actual.stop, 5);
        assert_eq!(actual.step, 1);

        let actual = slice.normalize(4);
        assert_eq!(actual.start, 0);
        assert_eq!(actual.stop, 4);
        assert_eq!(actual.step, 1);
    }

    #[test]
    fn test_array_create() {
        let data: Vec<isize> = vec![4, -1, -2, 7];
        let actual = Array::create(data.clone());

        assert_eq!(actual.data, data);
    }

    #[test]
    fn test_array_normalize() {
        let data: Vec<isize> = vec![4, -2, -3, 5];
        let arr = Array::create(data);

        let actual = arr.normalize(10);
        assert_eq!(actual.data, vec![4, 8, 7, 5]);
    }

    #[test]
    fn test_positional_indexer() {
        let slice = Slice::<isize>::create(None, None, Some(1));
        let array = Array::create(vec![1, 2]);

        let slice_enum = PositionalIndexer::Slice(slice.clone());
        match slice_enum {
            PositionalIndexer::Slice(s) => assert_eq!(slice, s),
            _ => unreachable!(),
        }

        let array_enum = PositionalIndexer::Array(array.clone());
        match array_enum {
            PositionalIndexer::Array(a) => assert_eq!(array, a),
            _ => unreachable!(),
        }
    }
}
