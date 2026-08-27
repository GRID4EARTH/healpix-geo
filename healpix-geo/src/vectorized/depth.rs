pub enum Depth<'a> {
    Scalar(&'a u8),
    Array(&'a [u8]),
}
