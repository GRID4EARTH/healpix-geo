use crate::ellipsoid::{Ellipsoid, ReferenceBody, ReferenceEllipsoid};
use moc::{moc::range::RangeMOC, qty::Hpx};

#[derive(Debug, Clone)]
struct CellRegion {
    moc: RangeMOC<u64, Hpx<u64>>,
    ellipsoid: Ellipsoid,
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

#[cfg(test)]
mod test {
    use super::*;
    use geodesy::ellps::Ellipsoid as GeodesyEllipsoid;

    #[test]
    fn full_domain() {
        let depth: u8 = 6;
        let ellipsoid = Ellipsoid::Ellipsoid(ReferenceEllipsoid::new(
            GeodesyEllipsoid::named("WGS84").unwrap(),
        ));

        let actual = CellRegion::full_domain(depth, ellipsoid.clone());

        assert_eq!(actual.ellipsoid, ellipsoid);
        assert_eq!(actual.moc.depth_max(), depth);
        assert_eq!(actual.moc.n_depth_max_cells(), 12 * 4_u64.pow(depth as u32));
    }

    #[test]
    fn create_empty() {
        let depth: u8 = 5;

        let ellipsoid = Ellipsoid::Ellipsoid(ReferenceEllipsoid::new(
            GeodesyEllipsoid::named("WGS84").unwrap(),
        ));

        let actual = CellRegion::create_empty(depth, ellipsoid.clone());

        assert_eq!(actual.moc.n_depth_max_cells(), 0);
        assert_eq!(actual.ellipsoid, ellipsoid);
    }

    #[test]
    fn from_cell_ids() {
        let depth: u8 = 3;
        let cell_ids: Vec<u64> = vec![2, 3, 4, 5, 23, 24, 25, 79, 80, 102, 103, 106];
        let ellipsoid = Ellipsoid::Ellipsoid(ReferenceEllipsoid::new(
            GeodesyEllipsoid::named("WGS84").unwrap(),
        ));

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
}
