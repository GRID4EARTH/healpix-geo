mod geometry;
mod indexers;
mod indexing;
mod ops;
mod region;
mod set;

pub use self::geometry::GeometryQuery;
pub use self::indexers::{Array, ConcreteSlice, LabelIndexer, PositionalIndexer, Slice};
pub use self::indexing::{Indexing, LabelIndexing, PositionIndexing};
pub use self::region::CellRegion;
pub use self::set::SetOperations;
