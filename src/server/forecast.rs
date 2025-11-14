use crate::proto::Coordinate;
use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};
use location_forecast_client::apis::Error;
use location_forecast_client::apis::configuration::Configuration as LocationForecastConfiguration;
use location_forecast_client::apis::data_api::{CompactGetError, DataApi, DataApiClient};
use location_forecast_client::models::MetjsonForecast;
use reqwest::Client;
use reqwest_middleware::ClientBuilder;
use std::ops::{Div, Mul};

pub async fn get_forecast(
    coord: Coordinate,
    user_agent: String,
) -> Result<MetjsonForecast, Error<CompactGetError>> {
    let location_config = create_forecast_client(user_agent);
    let data_api_client = DataApiClient::new(location_config.into());

    // The spec for locationforecast uses f32s as long, lats. coords are f64
    // should be truncated to max 4 dicits of precision because of ToS
    let lon = (coord.longitude as f32).mul(10_000.0).round().div(10_000.0);
    let lat = (coord.latitude as f32).mul(10_000.0).round().div(10_000.0);
    data_api_client.compact_get(lat, lon, None).await
}

fn create_forecast_client(user_agent: String) -> LocationForecastConfiguration {
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
