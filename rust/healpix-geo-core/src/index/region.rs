use crate::ellipsoid::{Ellipsoid, ReferenceBody, ReferenceEllipsoid};
use moc::{moc::range::RangeMOC, qty::Hpx};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CellRegion {
    moc: RangeMOC<u64, Hpx<u64>>,
    ellipsoid: Ellipsoid,
}

pub(crate) trait SetOperations {
    fn union(&self, other: &Self) -> Self;
    fn intersection(&self, other: &Self) -> Self;
    fn difference(&self, other: &Self) -> Self;
    fn symmetric_difference(&self, other: &Self) -> Self;
}

impl CellRegion {
    pub fn full_domain(depth: u8, ellipsoid: Ellipsoid) -> Self {
        Self {
            moc: RangeMOC::new_full_domain(depth),
            ellipsoid,
        }
    }

    pub fn create_empty(depth: u8, ellipsoid: Ellipsoid) -> Self {
        Self {
            moc: RangeMOC::new_empty(depth),
            ellipsoid,
        }
    }

    pub fn from_cell_ids(depth: u8, cell_ids: Vec<u64>, ellipsoid: Ellipsoid) -> Self {
        Self {
            moc: RangeMOC::from_fixed_depth_cells(depth, cell_ids.into_iter(), None),
            ellipsoid,
        }
    }
}

impl SetOperations for CellRegion {
    fn union(&self, other: &Self) -> Self {
        if other.ellipsoid != self.ellipsoid {
            // TODO: custom error type
            panic!("ellipsoids don't match");
        }

        Self {
            moc: self.moc.union(&other.moc),
            ellipsoid: self.ellipsoid.clone(),
        }
    }

    fn intersection(&self, other: &Self) -> Self {
        if other.ellipsoid != self.ellipsoid {
            // TODO: custom error type
            panic!("ellipsoids don't match");
        }

        Self {
            moc: self.moc.intersection(&other.moc),
            ellipsoid: self.ellipsoid.clone(),
        }
    }

    fn difference(&self, other: &Self) -> Self {
        if other.ellipsoid != self.ellipsoid {
            // TODO: custom error type
            panic!("ellipsoids don't match");
        }

        Self {
            moc: self.moc.minus(&other.moc),
            ellipsoid: self.ellipsoid.clone(),
        }
    }

    fn symmetric_difference(&self, other: &Self) -> Self {
        if other.ellipsoid != self.ellipsoid {
            // TODO: custom error type
            panic!("ellipsoids don't match");
        }

        Self {
            moc: self.moc.xor(&other.moc),
            ellipsoid: self.ellipsoid.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use geodesy::ellps::Ellipsoid as GeodesyEllipsoid;

    fn named_ellipsoid(name: &str) -> Ellipsoid {
        Ellipsoid::Ellipsoid(ReferenceEllipsoid::new(
            GeodesyEllipsoid::named(name).unwrap(),
        ))
    }

    #[test]
    fn full_domain() {
        let depth: u8 = 6;
        let ellipsoid = named_ellipsoid("WGS84");

        let actual = CellRegion::full_domain(depth, ellipsoid.clone());

        assert_eq!(actual.ellipsoid, ellipsoid);
        assert_eq!(actual.moc.depth_max(), depth);
        assert_eq!(actual.moc.n_depth_max_cells(), 12 * 4_u64.pow(depth as u32));
    }

    #[test]
    fn create_empty() {
        let depth: u8 = 5;
        let ellipsoid = named_ellipsoid("WGS84");

        let actual = CellRegion::create_empty(depth, ellipsoid.clone());

        assert_eq!(actual.moc.n_depth_max_cells(), 0);
        assert_eq!(actual.ellipsoid, ellipsoid);
    }

    #[test]
    fn from_cell_ids() {
        let depth: u8 = 3;
        let cell_ids: Vec<u64> = vec![2, 3, 4, 5, 23, 24, 25, 79, 80, 102, 103, 106];
        let ellipsoid = named_ellipsoid("WGS84");

        let actual = CellRegion::from_cell_ids(depth, cell_ids.clone(), ellipsoid.clone());

        assert_eq!(actual.moc.depth_max(), depth);
        assert_eq!(actual.ellipsoid, ellipsoid);

        assert_eq!(
            actual
                .moc
                .flatten_to_fixed_depth_cells()
                .collect::<Vec<u64>>(),
            cell_ids
        );
    }

    #[test]
    fn set_union() {
        let ellipsoid = named_ellipsoid("WGS84");

        let first = CellRegion::from_cell_ids(
            1,
            vec![1, 2, 3, 18, 20, 21, 39, 40, 41, 42],
            ellipsoid.clone(),
        );
        let second = CellRegion::from_cell_ids(1, vec![1, 2, 16, 20, 41, 42], ellipsoid.clone());

        let actual = first.union(&second);
        let expected = CellRegion::from_cell_ids(
            1,
            vec![1, 2, 3, 16, 18, 20, 21, 39, 40, 41, 42],
            ellipsoid.clone(),
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn set_intersection() {
        let ellipsoid = named_ellipsoid("WGS84");

        let first = CellRegion::from_cell_ids(
            1,
            vec![1, 2, 3, 18, 20, 21, 39, 40, 41, 42],
            ellipsoid.clone(),
        );
        let second = CellRegion::from_cell_ids(1, vec![1, 2, 16, 20, 41, 42], ellipsoid.clone());

        let actual = first.intersection(&second);
        let expected = CellRegion::from_cell_ids(1, vec![1, 2, 20, 41, 42], ellipsoid.clone());

        assert_eq!(actual, expected);
    }

    #[test]
    fn set_difference() {
        let ellipsoid = named_ellipsoid("WGS84");

        let first = CellRegion::from_cell_ids(
            1,
            vec![1, 2, 3, 18, 20, 21, 39, 40, 41, 42],
            ellipsoid.clone(),
        );
        let second = CellRegion::from_cell_ids(1, vec![1, 2, 16, 20, 41, 42], ellipsoid.clone());

        let actual = first.difference(&second);
        let expected = CellRegion::from_cell_ids(1, vec![3, 18, 21, 39, 40], ellipsoid.clone());

        assert_eq!(actual, expected);
    }

    #[test]
    fn set_symmetric_difference() {
        let ellipsoid = named_ellipsoid("WGS84");

        let first = CellRegion::from_cell_ids(
            1,
            vec![1, 2, 3, 18, 20, 21, 39, 40, 41, 42],
            ellipsoid.clone(),
        );
        let second = CellRegion::from_cell_ids(1, vec![1, 2, 16, 20, 41, 42], ellipsoid.clone());

        let actual = first.symmetric_difference(&second);
        let expected = CellRegion::from_cell_ids(1, vec![3, 16, 18, 21, 39, 40], ellipsoid.clone());

        assert_eq!(actual, expected);
    }
}
