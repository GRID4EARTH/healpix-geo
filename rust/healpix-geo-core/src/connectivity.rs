use cdshealpix::compass_point::MainWind;

const EDGE_DIRECTIONS: &[MainWind] = &[MainWind::SW, MainWind::NW, MainWind::NE, MainWind::SE];
const VERTEX_DIRECTIONS: &[MainWind] = &[MainWind::S, MainWind::W, MainWind::N, MainWind::E];

const ALL_DIRECTIONS: &[MainWind] = &[
    MainWind::S,
    MainWind::SW,
    MainWind::W,
    MainWind::NW,
    MainWind::N,
    MainWind::NE,
    MainWind::E,
    MainWind::SE,
];

#[derive(Debug, Default)]
pub enum Connectivity {
    Edge,
    Vertex,
    #[default]
    All,
}

impl Connectivity {
    pub fn size(&self) -> usize {
        self.directions().len()
    }
    pub fn directions(&self) -> &'static [MainWind] {
        match self {
            Self::Edge => EDGE_DIRECTIONS,
            Self::Vertex => VERTEX_DIRECTIONS,
            Self::All => ALL_DIRECTIONS,
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
