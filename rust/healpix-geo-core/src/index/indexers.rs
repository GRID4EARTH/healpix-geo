/// Slice object with the semantics of python's slice object
pub(crate) struct Slice {
    pub start: Option<isize>,
    pub stop: Option<isize>,
    pub step: isize,
}

pub(crate) struct ConcreteSlice {
    pub start: usize,
    pub stop: isize,
    pub step: isize,
}

impl Slice {
    pub fn create(start: Option<isize>, stop: Option<isize>, step: Option<isize>) -> Self {
        Slice {
            start,
            stop,
            step: step.unwrap_or(1),
        }
    }

    pub fn normalize(&self, size: usize) -> ConcreteSlice {
        let step = self.step;
        let size_ = size as isize;

        let start = self.start.map_or_else(
            || if step < 0 { size_ - 1 } else { 0 },
            |v| if v < 0 { size_ + v } else { v },
        ) as usize;
        let stop = self
            .stop
            .map_or_else(
                || if step < 0 { -1 } else { size_ },
                |v| if v < 0 { size_ + v } else { v },
            )
            .min(size_);

        ConcreteSlice { start, stop, step }
    }
}

pub(crate) struct Array {
    data: Vec<isize>,
}

pub(crate) struct ConcreteArray {
    data: Vec<usize>,
}

impl Array {
    pub fn create(values: Vec<isize>) -> Self {
        Self { data: values }
    }

    pub fn normalize(self, size: usize) -> ConcreteArray {
        ConcreteArray::create(
            self.data
                .into_iter()
                .map(|v| if v < 0 { v + size as isize } else { v } as usize)
                .collect(),
        )
    }
}

impl ConcreteArray {
    pub fn create(values: Vec<usize>) -> Self {
        Self { data: values }
    }
}

pub(crate) enum Indexers {
    Slice(Slice),
    Array(Array),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slice_create() {
        let slice = Slice::create(None, Some(5), None);

        assert_eq!(slice.start, None);
        assert_eq!(slice.stop, Some(5));
        assert_eq!(slice.step, 1);
    }

    #[test]
    fn slice_normalize_positive_full() {
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
    fn slice_normalize_negative_full() {
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
    fn slice_normalize_positive_limit() {
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
}
