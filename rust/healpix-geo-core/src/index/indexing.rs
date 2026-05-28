use super::indexers::{LabelIndexer, PositionalIndexer};

pub(crate) trait Indexing {
    fn sel(&self, indexer: LabelIndexer) -> Self;
    fn isel(&self, indexer: PositionalIndexer) -> Self;
}
