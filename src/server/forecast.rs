use crate::proto::Coordinate;
use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};
use location_forecast_client::apis::Error;
use location_forecast_client::apis::configuration::Configuration as LocationForecastConfiguration;
use location_forecast_client::apis::data_api::{CompactGetError, DataApi};

use location_forecast_client::models::MetjsonForecast;
use reqwest::Client;
use reqwest_middleware::ClientBuilder;
use std::ops::{Div, Mul};

pub async fn get_forecast(
    coord: Coordinate,
    data_api_client: &impl DataApi,
) -> Result<MetjsonForecast, Error<CompactGetError>> {
    // The spec for locationforecast uses f32s as long, lats. coords are f64
    // should be truncated to max 4 dicits of precision because of ToS
    let lon = to_f32_precision_4(coord.longitude);
    let lat = to_f32_precision_4(coord.latitude);
    data_api_client.compact_get(lat, lon, None).await
}

fn to_f32_precision_4(number: f64) -> f32 {
    (number as f32).mul(10_000.0).round().div(10_000.0)
}

pub fn create_forecast_client(user_agent: String) -> LocationForecastConfiguration {
    let mut location_config = LocationForecastConfiguration::new();
    let middleware_client = ClientBuilder::new(Client::new())
        .with(Cache(HttpCache {
            mode: CacheMode::Default,
            manager: CACacheManager::new("./cache/".into(), false),
            options: HttpCacheOptions::default(),
        }))
        .build();
    location_config.user_agent = Some(user_agent);
    location_config.client = middleware_client;
    location_config
}

// Mock DataApiClient
#[cfg(test)]
mod tests {
    use super::*;
    use location_forecast_client::{
        apis::data_api::MockDataApi,
        models::{
            Forecast, PointGeometry, metjson_forecast::Type as ForecastType,
            point_geometry::Type as PointGeometryType,
        },
    };

    #[test]
    fn to_f32_precision_4_works() {
        let f64: f64 = 12.3456789;
        let expected_f32: f32 = 12.3457;

        let three_decimal_f32: f32 = 12.346;
        let five_decimal_f32: f32 = 12.34567;

        assert_eq!(expected_f32, to_f32_precision_4(f64));
        assert_ne!(five_decimal_f32, to_f32_precision_4(f64));
        assert_ne!(three_decimal_f32, to_f32_precision_4(f64));
    }

    #[tokio::test]
    async fn get_client_success() {
        let coordinates = vec![8.0, 60.0];
        let geometry = PointGeometry {
            coordinates,
            r#type: PointGeometryType::Point,
        };

        let ok_forecast = MetjsonForecast {
            geometry: Box::new(geometry),
            properties: Box::new(Forecast::default()),
            r#type: ForecastType::Feature,
        };

        let mut mock_data_api = MockDataApi::new();
        mock_data_api
            .expect_compact_get()
            .returning(move |_a, _b, _c| Ok(ok_forecast.clone()));

        let coord = Coordinate {
            longitude: 10.0,
            latitude: 10.0,
        };
        let result = get_forecast(coord, &mock_data_api).await;
        match result {
            Ok(r) => {
                assert_eq!(r.geometry.coordinates[0], 8.0);
                assert_eq!(r.geometry.coordinates[1], 60.0);
            }
            Err(e) => panic!("{}", e.to_string()),
        };
    }
}
