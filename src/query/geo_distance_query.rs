use serde_json::{json, Value};
use crate::query::QueryTrait;

#[derive(Default)]
pub struct GeoDistanceQuery {
    field: String,
    geo: Geo,
    distance: String,
}

#[derive(Default)]
pub struct Geo {
    pub lat: f64,
    pub lon: f64,
}

impl Geo {
    pub fn new(lat: f64, lon: f64) -> Geo {
        Geo {
            lat,
            lon,
        }
    }
}

impl GeoDistanceQuery {
    pub fn new(field: &str, geo: Geo, distance: &str) -> GeoDistanceQuery {
        let mut value = GeoDistanceQuery::default();
        value.field = field.to_string();
        value.distance = distance.to_string();
        value.geo = geo;
        return value;
    }
}

impl QueryTrait for GeoDistanceQuery {
    fn build(&self) -> Value {
        let name = self.query_name();
        let field = self.field.to_string();
        let distance = self.distance.to_string();
        let lat = self.geo.lat.to_string();
        let lon = self.geo.lon.to_string();
        json!({
            name: {
                "distance":distance,
                "field":{
                        "lat" : lat,
                        "lon" : lon,
                    }
            }
        })
    }

    fn query_name(&self) -> String {
        return "geo_distance".to_string();
    }
}


#[cfg(test)]
mod tests {
    use crate::query::geo_distance_query::{Geo, GeoDistanceQuery};
    use crate::query::QueryTrait;

    #[test]
    fn build() {
        assert_eq!(
            GeoDistanceQuery::new("id", Geo::new(35.68944, 139.69167), "10km").build(),
            "{\"geo_distance\":{\"distance\":\"10km\",\"field\":{\"lat\":\"35.68944\",\"lon\":\"139.69167\"}}}"
        );
    }
}
