use num_traits::{FromPrimitive, PrimInt};

/// Slice object with the semantics of python's slice object
#[derive(Debug, Clone, PartialEq)]
pub struct Slice<T: PrimInt> {
    pub start: Option<T>,
    pub stop: Option<T>,
    pub step: T,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConcreteSlice<T: PrimInt> {
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

impl Slice<u64> {
    pub fn normalize(&self, size: u64) -> ConcreteSlice<u64> {
        let step = self.step;

        let start = self.start.unwrap_or(0);
        let stop = self.stop.unwrap_or(size);

        ConcreteSlice { start, stop, step }
    }
}

impl<T: PrimInt> ConcreteSlice<T> {
    pub fn join(slices: Vec<Self>) -> Self {
        if slices.is_empty() {
            panic!("no slices given");
        } else if slices.len() == 1 {
            slices[0].clone()
        } else if !slices
            .windows(2)
            .map(|w| w.first().unwrap().step == w.last().unwrap().step)
            .reduce(|a, b| a & b)
            .unwrap()
        {
            panic!("step sizes are not equal");
        } else if !slices
            .windows(2)
            .map(|w| w.first().unwrap().stop == w.last().unwrap().start)
            .reduce(|a, b| a & b)
            .unwrap()
        {
            panic!("slices are not contiguous");
        } else {
            let first = slices.first().unwrap();
            let last = slices.last().unwrap();

            Self {
                start: first.start,
                stop: last.stop,
                step: first.step,
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Array<T> {
    pub data: Vec<T>,
}

impl<T> Array<T> {
    pub fn create(values: Vec<T>) -> Self {
        Self { data: values }
    }
}

impl Array<isize> {
    pub fn normalize(&self, size: usize) -> Array<usize> {
        Array::<usize>::create(
            self.data
                .iter()
                .copied()
                .map(|v| if v < 0 { v + size as isize } else { v } as usize)
                .collect(),
        )
    }
}

pub enum PositionalIndexer {
    Slice(Slice<isize>),
    Array(Array<isize>),
}

pub enum LabelIndexer {
    Slice(Slice<u64>),
    Array(Array<u64>),
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
        let slice: Slice<isize> = Slice {
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
        let slice: Slice<isize> = Slice {
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
        let slice: Slice<isize> = Slice {
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
