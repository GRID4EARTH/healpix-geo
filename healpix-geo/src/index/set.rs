pub trait SetOperations {
    fn union(&self, other: &Self) -> Self;
    fn intersection(&self, other: &Self) -> Self;
    fn difference(&self, other: &Self) -> Self;
    fn symmetric_difference(&self, other: &Self) -> Self;
}
