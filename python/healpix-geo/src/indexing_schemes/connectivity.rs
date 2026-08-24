use healpix_geo_core::scalar::connectivity::Connectivity as RustConnectivity;

#[derive(Debug, Default, FromPyObject)]
enum Connectivity {
    Edge,
    Vertex,
    #[default]
    All,
}

impl Connectivity {
    pub fn into_connectivity(self) -> RustConnectivity {
        match self {
            Connectivity::Edge => RustConnectivity::Edge,
            Connectivity::Vertex => RustConnectivity::Vertex,
            Connectivity::All => RustConnectivity::All,
        }
    }
}

#[cfg(tests)]
mod tests {
    #[test]
    fn default() {
        assert!(matches!(Connectivity::default(), Connectivity::All));
    }
}
