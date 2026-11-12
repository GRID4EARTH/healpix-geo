//! The `morton` scheme: HEALPix cells as packed-`u64` Morton words.
//!
//! Backed by the [`mortie-core`](https://crates.io/crates/mortie-core) codec.
//! Like `zuniq`, a Morton word encodes its own refinement level, so most
//! functions take no `depth` parameter. Unlike `zuniq`, a raw unsigned sort of
//! *area* words is a Z-order traversal with parents sorting immediately before
//! their children, and containment testing is plain prefix truncation (see
//! [`hierarchy::contains`]).
//!
//! That ordering holds over area words only: a max-encoded point word lives in
//! the suffix region above the whole depth-28/29 area region of its body, so it
//! sorts after every area cell there and is not part of any subtree run.
pub mod conversion;
pub mod hierarchy;
