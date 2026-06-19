#[derive(Debug, PartialEq, Clone)]
pub struct BoundingBox {
    pub lon_min: f64,
    pub lat_min: f64,
    pub lon_max: f64,
    pub lat_max: f64,
}

impl BoundingBox {
    pub fn from_tuple(tuple: (f64, f64, f64, f64)) -> Self {
        BoundingBox {
            lon_min: tuple.0,
            lat_min: tuple.1,
            lon_max: tuple.2,
            lat_max: tuple.3,
        }
    }

    pub fn to_tuple(&self) -> (f64, f64, f64, f64) {
        (self.lon_min, self.lat_min, self.lon_max, self.lat_max)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Point(f64, f64);

impl Point {
    pub fn from_tuple(tuple: (f64, f64)) -> Self {
        Self(tuple.0, tuple.1)
    }

    pub fn to_tuple(&self) -> (f64, f64) {
        (self.0, self.1)
    }
}

/// A polygon in two dimensions represented by vertices
///
/// No support for internal rings, yet.
#[derive(Debug, PartialEq, Clone)]
pub struct Polygon {
    pub exterior: Vec<(f64, f64)>,
}

impl Polygon {
    pub fn create(exterior: Vec<(f64, f64)>) -> Self {
        let len = exterior.len();
        Self {
            exterior: if len >= 2 && exterior[0] == exterior[len - 1] {
                exterior.into_iter().take(len - 1).collect()
            } else {
                exterior
            },
        }
    }
}

pub enum Geometry {
    Point(Point),
    BoundingBox(BoundingBox),
    Polygon(Polygon),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bbox_roundtrip_tuple() {
        let bbox = (0.0, 1.0, 0.2, 1.5);

        let bounding_box = BoundingBox::from_tuple(bbox);
        assert_eq!(bounding_box.lon_min, bbox.0);
        assert_eq!(bounding_box.lat_min, bbox.1);
        assert_eq!(bounding_box.lon_max, bbox.2);
        assert_eq!(bounding_box.lat_max, bbox.3);

        let roundtripped = bounding_box.to_tuple();

        assert_eq!(bbox, roundtripped);
    }

    #[test]
    fn test_point_roundtrip_tuple() {
        let point_data = (1.2, 5.1);

        let point = Point::from_tuple(point_data);
        assert_eq!(point.0, point_data.0);
        assert_eq!(point.1, point_data.1);

        let roundtripped = point.to_tuple();
        assert_eq!(roundtripped, point_data);
    }

    #[test]
    fn test_polygon_create() {
        let vertices = vec![(1.0, 0.0), (4.0, 0.0), (2.5, 1.0), (1.0, 0.0)];

        let polygon = Polygon::create(vertices.clone());
        assert_eq!(
            polygon.exterior,
            vertices.iter().take(3).copied().collect::<Vec<_>>()
        );

        let vertices = vertices.into_iter().take(3).collect::<Vec<_>>();
        let polygon = Polygon::create(vertices.clone());
        assert_eq!(polygon.exterior, vertices);
    }

    #[test]
    fn test_geometry() {
        let polygon = Polygon {
            exterior: vec![(1.0, 0.0), (4.0, 0.0), (2.5, 1.0)],
        };
        let point = Point(1.0, 0.0);
        let bbox = BoundingBox::from_tuple((1.0, 2.0, 3.0, 4.0));

        let geom = Geometry::Point(point.clone());
        if let Geometry::Point(p) = geom {
            assert_eq!(p, point);
        }

        let geom = Geometry::Polygon(polygon.clone());
        if let Geometry::Polygon(p) = geom {
            assert_eq!(p, polygon);
        }

        let geom = Geometry::BoundingBox(bbox.clone());
        if let Geometry::BoundingBox(b) = geom {
            assert_eq!(b, bbox);
        }
    }
}
