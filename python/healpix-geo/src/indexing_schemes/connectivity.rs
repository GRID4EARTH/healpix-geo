use healpix_geo_core::connectivity::Connectivity as RustConnectivity;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[derive(Debug, Default)]
pub(crate) enum Connectivity {
    Edge,
    Vertex,
    #[default]
    All,
}

impl FromPyObject<'_, '_> for Connectivity {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, '_, PyAny>) -> Result<Self, Self::Error> {
        let extracted: String = obj.extract()?;

        match extracted.as_str() {
            "edge" => Ok(Connectivity::Edge),
            "vertex" => Ok(Connectivity::Vertex),
            "all" => Ok(Connectivity::All),
            _ => Err(PyValueError::new_err(format!(
                "Connectivity must be 'edge', 'vertex', or 'all'. Got '{extracted}'."
            ))),
        }
    }
}

impl Connectivity {
    pub(crate) fn into_connectivity(self) -> RustConnectivity {
        match self {
            Connectivity::Edge => RustConnectivity::Edge,
            Connectivity::Vertex => RustConnectivity::Vertex,
            Connectivity::All => RustConnectivity::All,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        assert!(matches!(Connectivity::default(), Connectivity::All));
    }
}
