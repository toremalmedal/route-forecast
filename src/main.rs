struct Position {
    lat: f32,
    lon: f32,
}

use core::panic;

use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};
use location_forecast_client::apis::Error;
use location_forecast_client::apis::configuration::Configuration as LocationForecastConfiguration;
use location_forecast_client::apis::data_api::{CompactGetError, compact_get};
use location_forecast_client::models::MetjsonForecast;

use ors_client::apis::configuration::Configuration as ORSConfiguration;
use ors_client::apis::directions_service_api::get_geo_json_route;
use ors_client::models::DirectionsService;

// The spec we generate our ors_client from lacks the response signature of features from GeoJSON:
mod geo_json_200_response;
use reqwest::Client;
use reqwest::header::HeaderMap;
use reqwest_middleware::ClientBuilder;
use serde::Deserialize;
use tonic::transport::Server;

use crate::geo_json_200_response::{Feature, Step};
use crate::proto::route_forecast_server::RouteForecastServer;

use proto::route_forecast_server::RouteForecast;

mod proto {
    tonic::include_proto!("route_forecast");
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("route-forecast_binary");
}

#[derive(Debug, Default)]
struct RouteForecastService {}

#[tonic::async_trait]
impl RouteForecast for RouteForecastService {
    async fn get_route_with_forecast(
        &self,
        request: tonic::Request<proto::RouteWithForecastRequest>,
    ) -> Result<tonic::Response<proto::RouteWithForecastResponse>, tonic::Status> {
        print!("Got a request: {:?}", request);
        let input = request.get_ref();
        let coords = input
            .coordinates
            .iter()
            .map(|coord| vec![coord.longitude, coord.latitude])
            .collect();
        let forecasts = handle_route_command(coords, input.number_of_forecasts).await;
        let response = proto::RouteWithForecastResponse { forecasts };
        Ok(tonic::Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = ("[::1]:50051").parse()?;
    let route_forecast_service = RouteForecastService::default();
    let route_forecast_reflector = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build_v1()
        .unwrap();

    Server::builder()
        .add_service(route_forecast_reflector)
        .add_service(RouteForecastServer::new(route_forecast_service))
        .serve(addr)
        .await?;

    Ok(())
}

async fn get_forecast(pos: Position) -> Result<MetjsonForecast, Error<CompactGetError>> {
    let mut location_config = LocationForecastConfiguration::new();
    let middleware_client = ClientBuilder::new(Client::new())
        .with(Cache(HttpCache {
            mode: CacheMode::Default,
            manager: CACacheManager::new("./cache/".into(), false),
            options: HttpCacheOptions::default(),
        }))
        .build();
    location_config.user_agent = Some("fredfull.no post@fredfull.no".into());
    location_config.client = middleware_client;
    compact_get(&location_config, pos.lat, pos.lon, None).await
}

async fn handle_route_command(
    coords: Vec<Vec<f64>>,
    number_of_forecasts: f64,
) -> Vec<proto::Forecast> {
    let api_key = match std::env::var("ORS_API_KEY") {
        Ok(val) => val,
        Err(e) => {
            println!("couldn't find env var ORS_API_KEY: {e}");
            return vec![];
        }
    };

    let mut route_config = ORSConfiguration::new();

    let mut headers = HeaderMap::new();
    headers.insert(
        reqwest::header::AUTHORIZATION,
        reqwest::header::HeaderValue::from_str(&api_key).unwrap(),
    );

    let client = Client::builder().default_headers(headers).build().unwrap();

    let middleware_client = ClientBuilder::new(client)
        .with(Cache(HttpCache {
            mode: CacheMode::Default,
            manager: CACacheManager::new("./cache/".into(), false),
            options: HttpCacheOptions::default(),
        }))
        .build();

    route_config.client = middleware_client;
    route_config.user_agent = Some("fredfull.no post@fredfull.no".into());

    route_config.api_key = Some(ors_client::apis::configuration::ApiKey {
        prefix: None,
        key: api_key,
    });

    // OpenRouteService uses vectors of [longitude, latitude] pairs as coords
    let direction_service_options = DirectionsService::new(coords);

    let response =
        get_geo_json_route(&route_config, "driving-car", direction_service_options).await;

    let result = match response {
        Ok(result) => result,
        Err(err) => {
            println!("Route service returned error: {err}");
            return vec![];
        }
    };

    // Attribution is required to use open route services
    // https://openrouteservice.org/terms-of-service/
    let metadata = match result.metadata {
        Some(metadata) => match metadata.attribution {
            Some(attribution) => attribution,
            None => {
                println!("No attribution found in metadata from route service, using default ");
                "© openrouteservice.org by HeiGIT | Map data © OpenStreetMap contributors"
                    .to_string()
            }
        },
        None => {
            println!("No metadata returned by route service, using default");
            "© openrouteservice.org by HeiGIT | Map data © OpenStreetMap contributors".to_string()
        }
    };

    let feature = match result.features {
        Some(features) => features[0].clone(),
        None => {
            println!("No feature returned by route service");
            return vec![];
        }
    };

    // Feature is just a serde "value" at this point (bad/generic spec). We convert it to a Feature:
    let geo_json_feature = <Feature as Deserialize>::deserialize(feature);

    // We want to store some names to add to our weather data, this is not returned by the forecast service:
    let mut names: Vec<String> = vec![];

    // We want positions to use with a weather service. A Feature contains indices
    // (way_points) that points to the index of a vector of coordinates that contains a start and stop
    // coordinate for the feature: We use the start coordinate.
    let coords = match geo_json_feature {
        Ok(geo_json_feature) => {
            let steps = &geo_json_feature.properties.segments[0].steps;
            let full_distance = geo_json_feature.properties.segments[0].distance;
            let sampled_steps =
                sample_steps_from_feature(steps, full_distance, number_of_forecasts);

            for s in &sampled_steps {
                names.push(s.name.clone());
            }

            let all_coordinates = geo_json_feature.geometry.coordinates;
            find_geometry_from_steps(sampled_steps, all_coordinates)
        }
        Err(err) => {
            println!("No route found, got error: {err}");
            vec![]
        }
    };

    let mut names_index: usize = 0;

    let mut forecasts: Vec<proto::Forecast> = vec![];

    for coord in coords {
        let pos = Position {
            // The spec for locationforecast uses f32s as long, lats. coords are f64
            lon: coord[0] as f32,
            lat: coord[1] as f32,
        };

        let coord = proto::Coordinate {
            longitude: pos.lon as f64,
            latitude: pos.lat as f64,
        };

        let result = get_forecast(pos).await;
        let air_temperature = match result {
            Ok(forecast) => {
                let current_hour = forecast.properties.timeseries[0].clone();
                let instant_details = current_hour.data.instant.details;
                let time = current_hour.time;
                println!("Weather for:  {:?} at {}", names.get(names_index), time);
                names_index += 1;
                match instant_details {
                    None => {
                        panic!("No instant details found");
                    }
                    Some(d) => {
                        println!("Temperature: {:?}", d.air_temperature.unwrap());
                        d.air_temperature.unwrap()
                    }
                }
            }
            Err(err) => {
                panic!("{err}");
            }
        };

        let current_forecast = proto::Forecast {
            position: Some(coord),
            air_temperature,
        };
        forecasts.push(current_forecast);
    }
    forecasts
}

fn sample_steps_from_feature(
    steps: &Vec<Step>,
    full_distance: f64,
    number_of_points: f64,
) -> Vec<Step> {
    let mut sample_steps: Vec<Step> = vec![];

    // Add first and last steps. I think it is safe to assume that weather at start and end of travel is of
    // interest:
    sample_steps.push(steps[0].clone());
    sample_steps.push(steps[steps.len() - 1].clone());

    // If we have set number_of_points to 3, we know that we already have sampled 2 steps.'
    // This means that we now want to sample one step about halfway along our route;
    // full_distance / 2
    assert!(number_of_points >= 2.0);
    let step_distance = full_distance / number_of_points - 1.0;

    // We want to sample a step when step_distance has been reached
    let mut current_distance = 0.0;

    for step in steps {
        current_distance += step.distance;
        if current_distance >= step_distance {
            current_distance = 0.0;
            sample_steps.push(step.clone());
        }
        if sample_steps.len() as f64 >= number_of_points {
            break;
        }
    }
    sample_steps
}

fn find_geometry_from_steps(steps: Vec<Step>, coordinates: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut sampled_coords: Vec<Vec<f64>> = vec![];

    // The way_point 'index' from steps must be converted to a usize to be used for indexing
    for step in steps {
        let idx_i64: i64 = step.way_points[0];
        if let Ok(idx) = usize::try_from(idx_i64) {
            sampled_coords.push(coordinates[idx].clone());
        }
    }
    sampled_coords
}
