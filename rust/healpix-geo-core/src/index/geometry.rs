use super::indexers::ConcreteSlice;
use crate::geometry::Geometry;

pub trait GeometryQuery {
    fn query(&self, geometry: &Geometry) -> (Vec<ConcreteSlice<isize>>, Self);
}
