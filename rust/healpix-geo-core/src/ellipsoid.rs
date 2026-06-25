use geodesy::authoring::FourierCoefficients;
use geodesy::ellps::{Ellipsoid as GeodesyEllipsoid, EllipsoidBase, GeoCart, Latitudes};
use geodesy::prelude::{Coor3D, CoordinateTuple};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize};
use serde::{de, ser};
use std::collections::HashMap;
use std::fmt;

pub trait ReferenceBody {
    fn latitude_authalic_to_geographic(&self, latitude: f64) -> f64;
    fn latitude_geographic_to_authalic(&self, latitude: f64) -> f64;

    fn to_mapping(&self) -> HashMap<String, f64>;
    fn from_mapping(mapping: &HashMap<String, f64>) -> Self;

    fn ellipsoid(&self) -> &GeodesyEllipsoid;

    fn geographic_to_cartesian(&self, point: &(f64, f64)) -> (f64, f64, f64) {
        let p = Coor3D::raw(point.0, point.1, 0.0);
        self.ellipsoid().cartesian(&p).xyz()
    }

    fn cartesian_to_geographic(&self, point: &(f64, f64, f64)) -> (f64, f64) {
        let p = Coor3D::raw(point.0, point.1, point.2);
        self.ellipsoid().geographic(&p).xy()
    }
}

#[derive(Clone, Debug)]
pub struct ReferenceSphere {
    ellipsoid: GeodesyEllipsoid,
}

impl ReferenceSphere {
    pub fn new(ellipsoid: GeodesyEllipsoid) -> Self {
        Self { ellipsoid }
    }
}

impl ReferenceBody for ReferenceSphere {
    fn latitude_authalic_to_geographic(&self, latitude: f64) -> f64 {
        latitude
    }

    fn latitude_geographic_to_authalic(&self, latitude: f64) -> f64 {
        latitude
    }

    fn ellipsoid(&self) -> &GeodesyEllipsoid {
        &self.ellipsoid
    }

    fn to_mapping(&self) -> HashMap<String, f64> {
        let mut mapping = HashMap::new();
        mapping.insert("radius".to_string(), self.ellipsoid.semimajor_axis());

        mapping
    }

    fn from_mapping(mapping: &HashMap<String, f64>) -> Self {
        let radius = mapping.get("radius").unwrap();

        let ellipsoid = GeodesyEllipsoid::new(*radius, 0.0);

        Self { ellipsoid }
    }
}

impl ser::Serialize for ReferenceSphere {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut state = serializer.serialize_struct("ReferenceSphere", 2)?;

        let radius = self.ellipsoid.semimajor_axis();

        state.serialize_field("radius", &radius)?;

        state.end()
    }
}

impl<'de> de::Deserialize<'de> for ReferenceSphere {
    fn deserialize<D>(deserializer: D) -> Result<ReferenceSphere, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            #[serde(rename = "radius")]
            Radius,
        }

        struct ReferenceSphereVisitor;

        impl<'de> de::Visitor<'de> for ReferenceSphereVisitor {
            type Value = ReferenceSphere;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct ReferenceSphere")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: de::MapAccess<'de>,
            {
                let mut radius = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Radius => {
                            if radius.is_some() {
                                return Err(de::Error::duplicate_field("radius"));
                            }

                            radius = Some(map.next_value()?);
                        }
                    }
                }

                let radius = radius.ok_or_else(|| de::Error::missing_field("radius"))?;

                let ellipsoid = GeodesyEllipsoid::new(radius, 0.0);
                Ok(ReferenceSphere::new(ellipsoid))
            }
        }

        const FIELDS: &[&str] = &["radius"];
        deserializer.deserialize_struct("ReferenceSphere", FIELDS, ReferenceSphereVisitor)
    }
}

impl PartialEq for ReferenceSphere {
    fn eq(&self, other: &Self) -> bool {
        self.ellipsoid == other.ellipsoid
    }
}

#[derive(Clone, Debug)]
pub struct ReferenceEllipsoid {
    ellipsoid: GeodesyEllipsoid,
    coefficients: FourierCoefficients,
}

impl ReferenceEllipsoid {
    pub fn new(ellipsoid: GeodesyEllipsoid) -> Self {
        let coefficients = ellipsoid.coefficients_for_authalic_latitude_computations();

        Self {
            ellipsoid,
            coefficients,
        }
    }
}

impl ReferenceBody for ReferenceEllipsoid {
    fn latitude_authalic_to_geographic(&self, latitude: f64) -> f64 {
        self.ellipsoid
            .latitude_authalic_to_geographic(latitude, &self.coefficients)
    }

    fn latitude_geographic_to_authalic(&self, latitude: f64) -> f64 {
        self.ellipsoid
            .latitude_geographic_to_authalic(latitude, &self.coefficients)
    }

    fn ellipsoid(&self) -> &GeodesyEllipsoid {
        &self.ellipsoid
    }

    fn to_mapping(&self) -> HashMap<String, f64> {
        let mut mapping = HashMap::new();
        mapping.insert(
            "semimajor_axis".to_string(),
            self.ellipsoid.semimajor_axis(),
        );
        mapping.insert("flattening".to_string(), self.ellipsoid.flattening());

        mapping
    }

    fn from_mapping(mapping: &HashMap<String, f64>) -> Self {
        let semimajor_axis = mapping.get("semimajor_axis").unwrap();
        let flattening = mapping.get("flattening").unwrap();

        let ellipsoid = GeodesyEllipsoid::new(*semimajor_axis, *flattening);
        let coefficients = ellipsoid.coefficients_for_authalic_latitude_computations();

        Self {
            ellipsoid,
            coefficients,
        }
    }
}

impl PartialEq for ReferenceEllipsoid {
    fn eq(&self, other: &Self) -> bool {
        self.ellipsoid == other.ellipsoid
    }
}

impl ser::Serialize for ReferenceEllipsoid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut state = serializer.serialize_struct("ReferenceEllipsoid", 2)?;

        let semimajor_axis = self.ellipsoid.semimajor_axis();
        let flattening = self.ellipsoid.flattening();

        state.serialize_field("semimajor_axis", &semimajor_axis)?;
        state.serialize_field("flattening", &flattening)?;

        state.end()
    }
}

impl<'de> de::Deserialize<'de> for ReferenceEllipsoid {
    fn deserialize<D>(deserializer: D) -> Result<ReferenceEllipsoid, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            #[serde(rename = "semimajor_axis")]
            SemimajorAxis,
            #[serde(rename = "flattening")]
            Flattening,
        }

        struct ReferenceEllipsoidVisitor;

        impl<'de> de::Visitor<'de> for ReferenceEllipsoidVisitor {
            type Value = ReferenceEllipsoid;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct ReferenceEllipsoid")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: de::MapAccess<'de>,
            {
                let mut semimajor_axis = None;
                let mut flattening = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::SemimajorAxis => {
                            if semimajor_axis.is_some() {
                                return Err(de::Error::duplicate_field("semimajor_axis"));
                            }

                            semimajor_axis = Some(map.next_value()?);
                        }
                        Field::Flattening => {
                            if flattening.is_some() {
                                return Err(de::Error::duplicate_field("flattening"));
                            }

                            flattening = Some(map.next_value()?);
                        }
                    }
                }

                let semimajor_axis =
                    semimajor_axis.ok_or_else(|| de::Error::missing_field("semimajor_axis"))?;
                let flattening =
                    flattening.ok_or_else(|| de::Error::missing_field("flattening"))?;

                let ellipsoid = GeodesyEllipsoid::new(semimajor_axis, flattening);
                Ok(ReferenceEllipsoid::new(ellipsoid))
            }
        }

        const FIELDS: &[&str] = &["semimajor_axis", "flattening"];
        deserializer.deserialize_struct("ReferenceEllipsoid", FIELDS, ReferenceEllipsoidVisitor)
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum Ellipsoid {
    Ellipsoid(ReferenceEllipsoid),
    Sphere(ReferenceSphere),
}

impl ReferenceBody for Ellipsoid {
    fn latitude_authalic_to_geographic(&self, latitude: f64) -> f64 {
        match self {
            Self::Ellipsoid(wrapped) => wrapped.latitude_authalic_to_geographic(latitude),
            Self::Sphere(wrapped) => wrapped.latitude_authalic_to_geographic(latitude),
        }
    }

    fn latitude_geographic_to_authalic(&self, latitude: f64) -> f64 {
        match self {
            Self::Ellipsoid(wrapped) => wrapped.latitude_geographic_to_authalic(latitude),
            Self::Sphere(wrapped) => wrapped.latitude_geographic_to_authalic(latitude),
        }
    }

    fn ellipsoid(&self) -> &GeodesyEllipsoid {
        match self {
            Self::Ellipsoid(wrapped) => wrapped.ellipsoid(),
            Self::Sphere(wrapped) => wrapped.ellipsoid(),
        }
    }

    fn to_mapping(&self) -> HashMap<String, f64> {
        match self {
            Self::Ellipsoid(wrapped) => wrapped.to_mapping(),
            Self::Sphere(wrapped) => wrapped.to_mapping(),
        }
    }

    fn from_mapping(mapping: &HashMap<String, f64>) -> Self {
        if mapping.contains_key("flattening") {
            Self::Ellipsoid(ReferenceEllipsoid::from_mapping(mapping))
        } else {
            Self::Sphere(ReferenceSphere::from_mapping(mapping))
        }
    }
}

impl Default for Ellipsoid {
    fn default() -> Self {
        Self::Sphere(ReferenceSphere {
            ellipsoid: GeodesyEllipsoid::named("sphere").unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reference_ellipsoid_new() {
        let ellipsoid = GeodesyEllipsoid::named("WGS84").unwrap();

        let obj = ReferenceEllipsoid::new(ellipsoid);

        assert_eq!(obj.ellipsoid, ellipsoid);
    }

    #[test]
    fn test_reference_sphere_new() {
        let ellipsoid = GeodesyEllipsoid::named("unitsphere").unwrap();

        let obj = ReferenceSphere::new(ellipsoid);

        assert_eq!(obj.ellipsoid, ellipsoid);
    }

    #[test]
    fn test_reference_sphere_conversions() {
        let lat: f64 = 88.0;

        let ellipsoid = ReferenceSphere::new(GeodesyEllipsoid::named("sphere").unwrap());

        let actual1 = ellipsoid.latitude_geographic_to_authalic(lat);
        assert_eq!(actual1, lat);

        let actual2 = ellipsoid.latitude_authalic_to_geographic(lat);
        assert_eq!(actual2, lat);
    }

    #[test]
    fn test_reference_ellipsoid_conversions_wgs84() {
        let ellipsoid = ReferenceEllipsoid::new(GeodesyEllipsoid::named("WGS84").unwrap());

        let lat: f64 = 45.0;
        let authalic = ellipsoid.latitude_geographic_to_authalic(lat);
        let geographic = ellipsoid.latitude_authalic_to_geographic(authalic);

        assert_eq!(geographic, lat);
    }

    #[test]
    fn test_reference_ellipsoid_conversions_bessel() {
        let ellipsoid = ReferenceEllipsoid::new(GeodesyEllipsoid::named("bessel").unwrap());

        let lat: f64 = 45.0;
        let authalic = ellipsoid.latitude_geographic_to_authalic(lat);
        let geographic = ellipsoid.latitude_authalic_to_geographic(authalic);

        assert_eq!(geographic, lat);
    }
}
