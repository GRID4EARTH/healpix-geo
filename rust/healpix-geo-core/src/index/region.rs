use super::geometry::GeometryQuery;
use super::indexers::{Array, ConcreteSlice, LabelIndexer, PositionalIndexer, Slice};
use super::indexing::{Indexing, LabelIndexing, PositionIndexing};
use super::ops::{JoinOp, JoinOps};
use super::set::SetOperations;
use crate::ellipsoid::{Ellipsoid, ReferenceBody};
use crate::geometry::Geometry;
use crate::scalar;
use cdshealpix::nested;
use moc::deser::json::from_json_aladin;
use moc::moc::cell::CellMOC;
use moc::moc::range::{CellSelection, RangeMOC};
use moc::moc::{
    CellMOCIntoIterator, CellMOCIterator, HasMaxDepth, RangeMOCIntoIterator, RangeMOCIterator,
};
use moc::qty::Hpx;

#[derive(Debug, Clone, PartialEq)]
pub struct CellRegion {
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

    pub fn empty(depth: u8, ellipsoid: Ellipsoid) -> Self {
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

    pub fn nbytes(&self) -> usize {
        self.moc.len() * 2 * u64::BITS as usize / 8
    }

    pub fn size(&self) -> usize {
        self.moc.n_depth_max_cells() as usize
    }

    pub fn depth(&self) -> u8 {
        self.moc.depth_max()
    }

    pub fn cell_ids(&self) -> Vec<u64> {
        self.moc.flatten_to_fixed_depth_cells().collect()
    }

    pub fn cells_at_depth(&self) -> u64 {
        12 * 4u64.pow(self.depth() as u32)
    }

    pub fn ellipsoid(&self) -> &Ellipsoid {
        &self.ellipsoid
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut moc_bytes: Vec<u8> = Default::default();

        self.moc
            .clone()
            .into_range_moc_iter()
            .cells()
            .to_json_aladin(None, &mut moc_bytes)
            .expect("failed to serialize the moc");

        // serde_json: serialize ellipsoid, chain both together
        let ellipsoid_bytes = serde_json::to_vec(&self.ellipsoid).unwrap();
        let sizes: Vec<usize> = vec![moc_bytes.len(), ellipsoid_bytes.len()];

        sizes
            .into_iter()
            .flat_map(usize::to_le_bytes)
            .chain(moc_bytes)
            .chain(ellipsoid_bytes)
            .collect()
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        // extract the json_aladin and the ellipsoid data, deserialize both
        let n_bytes: usize = usize::BITS as usize / 8;
        let sizes: Vec<usize> = bytes[0..2 * n_bytes]
            .chunks(n_bytes)
            .map(|c| usize::from_le_bytes(c.try_into().unwrap()))
            .collect();

        let cell_moc: CellMOC<u64, Hpx<u64>> = from_json_aladin(
            std::str::from_utf8(&bytes[2 * n_bytes..2 * n_bytes + sizes[0]]).unwrap(),
        )
        .unwrap();
        let reconstructed_moc = RangeMOC::from_cells(
            cell_moc.depth_max(),
            cell_moc
                .into_cell_moc_iter()
                .map(|c| -> (u8, u64) { (c.depth, c.idx) }),
            None,
        );

        let ellipsoid: Ellipsoid =
            serde_json::from_slice(&bytes[2 * n_bytes + sizes[0]..]).unwrap();

        Self {
            moc: reconstructed_moc,
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

impl Indexing for CellRegion {
    fn sel(&self, indexer: &LabelIndexer) -> (Self, PositionalIndexer) {
        let (subset, positional_indexer): (RangeMOC<u64, Hpx<u64>>, PositionalIndexer) =
            match indexer {
                LabelIndexer::Slice(slice) => {
                    let concrete_slice = slice.normalize(self.cells_at_depth());

                    let (subset, positional_slice) = self.moc.label_slice(concrete_slice);

                    (
                        subset,
                        PositionalIndexer::Slice(Slice::create(
                            Some(positional_slice.start as isize),
                            Some(positional_slice.stop as isize),
                            Some(positional_slice.step as isize),
                        )),
                    )
                }
                LabelIndexer::Array(array) => {
                    let (subset, positional_array) = self.moc.label_index(array);

                    (
                        subset,
                        PositionalIndexer::Array(Array {
                            data: positional_array
                                .data
                                .into_iter()
                                .map(|v| v as isize)
                                .collect::<Vec<_>>(),
                        }),
                    )
                }
            };

        let new_region = CellRegion {
            moc: subset,
            ellipsoid: self.ellipsoid.clone(),
        };

        (new_region, positional_indexer)
    }

    fn isel(&self, indexer: &PositionalIndexer) -> Self {
        let subset = match indexer {
            PositionalIndexer::Slice(slice) => {
                let concrete_slice = slice.normalize(self.size());

                self.moc.position_slice(&concrete_slice)
            }
            PositionalIndexer::Array(array) => self.moc.position_index(array),
        };

        CellRegion {
            moc: subset,
            ellipsoid: self.ellipsoid.clone(),
        }
    }
}

impl GeometryQuery for CellRegion {
    fn query(&self, geometry: &Geometry) -> (Vec<ConcreteSlice<isize>>, Self) {
        let depth = self.depth();
        let layer = nested::get(depth);

        let geometry_moc = match geometry {
            Geometry::Point(point) => {
                let (lon, lat) = point.to_tuple();
                let hash = scalar::nested::coordinates::lonlat_to_healpix(
                    &lon,
                    &lat,
                    layer,
                    &self.ellipsoid,
                );

                RangeMOC::from_fixed_depth_cells(depth, vec![hash].into_iter(), None)
            }
            Geometry::BoundingBox(bbox) => {
                let (lon_min, lat_min, lon_max, lat_max) = bbox.to_tuple();

                RangeMOC::from_zone(
                    lon_min.rem_euclid(360.0).to_radians(),
                    self.ellipsoid
                        .latitude_geographic_to_authalic(lat_min.to_radians()),
                    lon_max.rem_euclid(360.0).to_radians(),
                    self.ellipsoid
                        .latitude_geographic_to_authalic(lat_max.to_radians()),
                    depth,
                    CellSelection::All,
                )
            }
            Geometry::Polygon(polygon) => {
                let converted: Vec<(f64, f64)> = polygon
                    .exterior
                    .iter()
                    .map(|(lon, lat)| {
                        (
                            lon.rem_euclid(360.0).to_radians(),
                            self.ellipsoid
                                .latitude_geographic_to_authalic(lat.to_radians()),
                        )
                    })
                    .collect();

                RangeMOC::from_polygon(&converted, false, depth, CellSelection::All)
            }
        };

        let (slices, moc) = self.moc.join(&geometry_moc, JoinOp::Intersection);

        let new_region = CellRegion {
            moc,
            ellipsoid: self.ellipsoid.clone(),
        };

        (slices, new_region)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ellipsoid::ReferenceEllipsoid;
    use geodesy::ellps::Ellipsoid as GeodesyEllipsoid;

    fn named_ellipsoid(name: &str) -> Ellipsoid {
        Ellipsoid::Ellipsoid(ReferenceEllipsoid::new(
            GeodesyEllipsoid::named(name).unwrap(),
        ))
    }

    #[test]
    fn test_full_domain() {
        let depth: u8 = 6;
        let ellipsoid = named_ellipsoid("WGS84");

        let actual = CellRegion::full_domain(depth, ellipsoid.clone());

        assert_eq!(actual.ellipsoid, ellipsoid);
        assert_eq!(actual.moc.depth_max(), depth);
        assert_eq!(actual.moc.n_depth_max_cells(), 12 * 4_u64.pow(depth as u32));
    }

    #[test]
    fn test_empty() {
        let depth: u8 = 5;
        let ellipsoid = named_ellipsoid("WGS84");

        let actual = CellRegion::empty(depth, ellipsoid.clone());

        assert_eq!(actual.moc.n_depth_max_cells(), 0);
        assert_eq!(actual.ellipsoid, ellipsoid);
    }

    #[test]
    fn test_from_cell_ids() {
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

    mod properties {
        use super::*;

        #[test]
        fn test_size() {
            let depth: u8 = 7;
            let region = CellRegion::full_domain(depth, named_ellipsoid("WGS84"));

            assert_eq!(region.size(), 12 * 4_usize.pow(depth as u32));
        }

        #[test]
        fn test_nbytes() {
            let depth: u8 = 7;
            let region = CellRegion::full_domain(depth, named_ellipsoid("WGS84"));

            assert_eq!(region.nbytes(), 16);
        }
    }

    mod set_ops {
        use super::*;

        #[test]
        fn test_set_union() {
            let ellipsoid = named_ellipsoid("WGS84");

            let first = CellRegion::from_cell_ids(
                1,
                vec![1, 2, 3, 18, 20, 21, 39, 40, 41, 42],
                ellipsoid.clone(),
            );
            let second =
                CellRegion::from_cell_ids(1, vec![1, 2, 16, 20, 41, 42], ellipsoid.clone());

            let actual = first.union(&second);
            let expected = CellRegion::from_cell_ids(
                1,
                vec![1, 2, 3, 16, 18, 20, 21, 39, 40, 41, 42],
                ellipsoid.clone(),
            );

            assert_eq!(actual, expected);
        }

        #[test]
        fn test_set_intersection() {
            let ellipsoid = named_ellipsoid("WGS84");

            let first = CellRegion::from_cell_ids(
                1,
                vec![1, 2, 3, 18, 20, 21, 39, 40, 41, 42],
                ellipsoid.clone(),
            );
            let second =
                CellRegion::from_cell_ids(1, vec![1, 2, 16, 20, 41, 42], ellipsoid.clone());

            let actual = first.intersection(&second);
            let expected = CellRegion::from_cell_ids(1, vec![1, 2, 20, 41, 42], ellipsoid.clone());

            assert_eq!(actual, expected);
        }

        #[test]
        fn test_set_difference() {
            let ellipsoid = named_ellipsoid("WGS84");

            let first = CellRegion::from_cell_ids(
                1,
                vec![1, 2, 3, 18, 20, 21, 39, 40, 41, 42],
                ellipsoid.clone(),
            );
            let second =
                CellRegion::from_cell_ids(1, vec![1, 2, 16, 20, 41, 42], ellipsoid.clone());

            let actual = first.difference(&second);
            let expected = CellRegion::from_cell_ids(1, vec![3, 18, 21, 39, 40], ellipsoid.clone());

            assert_eq!(actual, expected);
        }

        #[test]
        fn test_set_symmetric_difference() {
            let ellipsoid = named_ellipsoid("WGS84");

            let first = CellRegion::from_cell_ids(
                1,
                vec![1, 2, 3, 18, 20, 21, 39, 40, 41, 42],
                ellipsoid.clone(),
            );
            let second =
                CellRegion::from_cell_ids(1, vec![1, 2, 16, 20, 41, 42], ellipsoid.clone());

            let actual = first.symmetric_difference(&second);
            let expected =
                CellRegion::from_cell_ids(1, vec![3, 16, 18, 21, 39, 40], ellipsoid.clone());

            assert_eq!(actual, expected);
        }
    }

    mod query {
        use super::*;
        use crate::geometry::{BoundingBox, Geometry, Point, Polygon};

        #[test]
        fn test_query_point_full_domain() {
            let ellipsoid = named_ellipsoid("WGS84");
            let depth: u8 = 6;

            let region = CellRegion::full_domain(depth, ellipsoid.clone());
            let point = Geometry::Point(Point::from_tuple((0.0, 2.0)));

            let (slices, subset) = region.query(&point);

            let expected_slices = vec![ConcreteSlice {
                start: 19459,
                stop: 19460,
                step: 1,
            }];
            let expected_subset = CellRegion::from_cell_ids(depth, vec![19459], ellipsoid);

            assert_eq!(slices, expected_slices);
            assert_eq!(subset, expected_subset);
        }

        #[test]
        fn test_query_bbox_full_domain() {
            let ellipsoid = named_ellipsoid("WGS84");
            let depth: u8 = 1;

            let region = CellRegion::full_domain(depth, ellipsoid.clone());
            let bbox = Geometry::BoundingBox(BoundingBox::from_tuple((-10.0, 0.0, 20.0, 25.0)));

            let (slices, subset) = region.query(&bbox);

            let expected_slices = vec![
                ConcreteSlice {
                    start: 2,
                    stop: 3,
                    step: 1,
                },
                ConcreteSlice {
                    start: 17,
                    stop: 20,
                    step: 1,
                },
            ];
            let expected_subset = CellRegion::from_cell_ids(depth, vec![2, 17, 18, 19], ellipsoid);

            assert_eq!(slices, expected_slices);
            assert_eq!(subset, expected_subset);
        }

        #[test]
        fn test_query_polygon_full_domain() {
            let ellipsoid = named_ellipsoid("WGS84");
            let depth: u8 = 1;

            let region = CellRegion::full_domain(depth, ellipsoid.clone());
            let polygon = Geometry::Polygon(Polygon::create(vec![
                (-10.0, 0.0),
                (10.0, 0.0),
                (0.0, 25.0),
            ]));

            let (slices, subset) = region.query(&polygon);

            let expected_slices = vec![ConcreteSlice {
                start: 17,
                stop: 20,
                step: 1,
            }];
            let expected_subset = CellRegion::from_cell_ids(depth, vec![17, 18, 19], ellipsoid);

            assert_eq!(slices, expected_slices);
            assert_eq!(subset, expected_subset);
        }
    }

    mod io {
        use super::*;

        #[test]
        fn test_roundtrip() {
            let ellipsoid = named_ellipsoid("WGS84");
            let depth: u8 = 10;
            let region = CellRegion::full_domain(depth, ellipsoid.clone());

            let bytes = region.to_bytes();
            let roundtripped = CellRegion::from_bytes(&bytes);

            assert_eq!(region, roundtripped);
        }
    }
}
