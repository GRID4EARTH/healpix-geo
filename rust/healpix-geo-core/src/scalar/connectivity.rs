#[derive(Debug, Default)]
enum Connectivity {
    Edge,
    Vertex,
    #[default]
    All,
}

#[cfg(tests)]
mod tests {
    #[test]
    fn default() {
        assert!(matches!(Connectivity::default(), Connectivity::All));
    }
}
