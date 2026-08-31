use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use cdshealpix::compass_point::MainWind;

pub(crate) enum WindRose {
    S,
    SW,
    W,
    NW,
    N,
    NE,
    E,
    SE,
}

impl FromPyObject<'_, '_> for WindRose {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, '_, PyAny>) -> Result<Self, Self::Error> {
        let extracted: String = obj.extract()?;

        match extracted.as_str() {
            "S" => Ok(WindRose::S),
            "SW" => Ok(WindRose::SW),
            "W" => Ok(WindRose::W),
            "NW" => Ok(WindRose::NW),
            "N" => Ok(WindRose::N),
            "NE" => Ok(WindRose::NE),
            "E" => Ok(WindRose::E),
            "SE" => Ok(WindRose::SE),
            _ => Err(PyValueError::new_err(format!(
                "direction must be a cardinal or ordinal direction. Got '{extracted}'."
            ))),
        }
    }
}

impl WindRose {
    pub(crate) fn into_mainwind(self) -> MainWind {
        match self {
            Self::S => MainWind::S,
            Self::SW => MainWind::SW,
            Self::W => MainWind::W,
            Self::NW => MainWind::NW,
            Self::N => MainWind::N,
            Self::NE => MainWind::NE,
            Self::E => MainWind::E,
            Self::SE => MainWind::SE,
        }
    }
}
