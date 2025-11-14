struct Position {
    lat: f32,
    lon: f32,
}

use core::panic;
use std::ops::{Div, Mul};

use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};

use location_forecast_client::apis::Error;
use location_forecast_client::apis::configuration::Configuration as LocationForecastConfiguration;
use location_forecast_client::apis::data_api::{CompactGetError, DataApi, DataApiClient};
use location_forecast_client::models::MetjsonForecast;

use ors_client::apis::Error as ORSError;
use ors_client::apis::configuration::Configuration as ORSConfiguration;
use ors_client::apis::directions_service_api::{
    DirectionsServiceApi, DirectionsServiceApiClient, GetGeoJsonRouteError,
    GetSimpleGeoJsonRouteError,
};

use ors_client::models::{DirectionsService, GetSimpleGeoJsonRoute200Response};

use stedsnavn_client::apis::Error as PlaceError;
use stedsnavn_client::apis::configuration::Configuration as PlaceConfiguration;
use stedsnavn_client::apis::default_api::StedGetError;
use stedsnavn_client::apis::default_api::{DefaultApi, DefaultApiClient as PlaceApiClient};

// The spec we generate our ors_client from lacks the response signature of features from GeoJSON:
mod geo_json_200_response;
use reqwest::header::HeaderMap;
use reqwest::{Client, Method};
use reqwest_middleware::ClientBuilder;
use serde::Deserialize;
use stedsnavn_client::models::ReturSted;
use tonic::transport::{Identity, Server, ServerTlsConfig};
use tonic_web::GrpcWebLayer;
use tower_http::cors::{Any, CorsLayer};

use chrono::Local;

use crate::proto::route_forecast_server::{RouteForecast, RouteForecastServer};
use crate::proto::{
    self, Coordinate, FILE_DESCRIPTOR_SET, Forecast, ForecastNextHour, Place, PlaceRequest,
    PlaceResponse, RouteWithForecastRequest, RouteWithForecastResponse, Step as ResponseStep,
};
use crate::server::geo_json_200_response::Step;
use geo_json_200_response::Feature;

#[derive(Debug, Default)]
struct RouteForecastService {}

#[tonic::async_trait]
impl RouteForecast for RouteForecastService {
    async fn get_route_with_forecast(
        &self,
        request: tonic::Request<RouteWithForecastRequest>,
    ) -> Result<tonic::Response<RouteWithForecastResponse>, tonic::Status> {
        let ors_api_key = match std::env::var("ORS_API_KEY") {
            Ok(val) => val,
            Err(e) => {
                panic!("couldn't find env var ORS_API_KEY: {e}");
            }
        };

        let user_agent = match std::env::var("USER_AGENT") {
            Ok(val) => val,
            Err(e) => {
                panic!("couldn't find env var USER_AGENT: {e}");
            }
        };

        let input = request.get_ref();
        println!("{}: Recieved RouteWithForecastRequest", Local::now());

        println!("number_of_forecasts: {}", input.number_of_forecasts);
        if !(1.0..=20.0).contains(&input.number_of_forecasts) {
            return Err(tonic::Status::invalid_argument(
                "number_of_forecasts should be between 2 and 19",
            ));
        }

        // "Itinerary – A list of a tour’s or entire trip’s schedule and major travel elements."
        // - https://www.travelwta.com/travel-terms-glossary/
        let itinerary_coords = input
            .coordinates
            .iter()
            .map(|coord| {
                println!("longitude: {}", coord.longitude);
                println!("latitude: {}", coord.latitude);
                vec![coord.longitude, coord.latitude]
            })
            .collect();

        let geo_json_route_result = get_route(itinerary_coords, user_agent, ors_api_key).await;
        let geo_json_route = match geo_json_route_result {
            Ok(g) => g,
            Err(e) => {
                println!("{}: Could not find route, got error {}", Local::now(), e);
                return Err(tonic::Status::new(tonic::Code::NotFound, "Route not found"));
            }
        };

        let route_with_forecast_result =
            handle_route_command(geo_json_route, input.number_of_forecasts).await;

        match route_with_forecast_result {
            Ok(r) => {
                println!("{}: Returning RouteWithForecastResponse", Local::now());
                return Ok(tonic::Response::new(r));
            }
            Err(e) => {
                println!(
                    "{}: Could not create RouteWithForecastResponse, got error {}",
                    Local::now(),
                    e
                );
                return Err(tonic::Status::new(
                    tonic::Code::NotFound,
                    "Forecast for route not found",
                ));
            }
        }
    }

    async fn get_place(
        &self,
        request: tonic::Request<PlaceRequest>,
    ) -> Result<tonic::Response<PlaceResponse>, tonic::Status> {
        let input = request.get_ref();
        let search = input.name.clone();

        let user_agent = match std::env::var("USER_AGENT") {
            Ok(val) => val,
            Err(e) => {
                panic!("couldn't find env var USER_AGENT: {e}");
            }
        };
        println!("{}: Recieved PlaceRequest", Local::now());
        println!("name: {}", search);

        let api_response_result = get_place_request(search, user_agent).await;
        let api_response = match api_response_result {
            Ok(r) => {
                // Metadata is technically optional
                let metadata = match r.metadata.clone() {
                    Some(m) => m,
                    None => {
                        println!("{}: No place_result.metadata", Local::now());
                        return Err(tonic::Status::not_found("No place found"));
                    }
                };
                // ReturSted can return no matches for a name
                match metadata.totalt_antall_treff {
                    Some(t) => {
                        if t < 1 {
                            println!("{}: Total number of places retrieved 0", Local::now());
                            return Err(tonic::Status::not_found("No place found"));
                        }
                    }
                    None => {
                        println!(
                            "{}: place_result.metadata.totalt_antall_treff does not exist",
                            Local::now()
                        );
                        return Err(tonic::Status::not_found("No place found"));
                    }
                }
                r
            }
            Err(e) => {
                eprintln!("{}: Error from 'stedsnavn' API: {e}", Local::now());
                return Err(tonic::Status::internal("Error from 'stedsnavn' API"));
            }
        };

        let name_search_response = match api_response.navn {
            Some(n) => n,
            None => {
                println!("{}: Total number of places retrieved 0", Local::now());
                return Err(tonic::Status::internal("Error from 'stedsnavn' API"));
            }
        };
        let places = name_search_response
            .iter()
            .map(|n| {
                let first_place = n.stedsnavn.clone().unwrap()[0].clone();
                let first_point = n.representasjonspunkt.clone().unwrap();
                let place = Place {
                    name: first_place.skrivemte.unwrap(),
                    point: Some(Coordinate {
                        latitude: first_point.nord.unwrap(),
                        longitude: first_point.st.unwrap(),
                    }),
                };
                print!("Found place: {}", place.name);
                place
            })
            .collect();

        println!("{}: Returning PlaceResponse", Local::now());
        Ok(tonic::Response::new(proto::PlaceResponse { place: places }))
    }
}

pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_url = match std::env::var("GRPC_SERVER_URL") {
        Ok(url) => url,
        Err(e) => {
            panic!("Could not retrieve runtime env variable GRPC_SERVER_URL, got error:{e}");
        }
    };

    let addr = server_url.parse()?;
    let route_forecast_service = RouteForecastService::default();
    let route_forecast_reflector = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build_v1()
        .unwrap();

    let cors = CorsLayer::new()
        .allow_headers(Any)
        .allow_methods([Method::GET])
        .allow_origin(Any);

    let cert_path = std::env::var("CERT_PATH").unwrap();
    let key_path = std::env::var("KEY_PATH").unwrap();

    let cert = std::fs::read_to_string(cert_path)?;
    let key = std::fs::read_to_string(key_path)?;

    let tls_config = ServerTlsConfig::new().identity(Identity::from_pem(&cert, &key));

    println!("Starting server at {}", addr);

    let server = Server::builder()
        .accept_http1(true)
        .tls_config(tls_config)?
        .layer(cors)
        .layer(GrpcWebLayer::new())
        .add_service(route_forecast_reflector)
        .add_service(RouteForecastServer::new(route_forecast_service))
        .serve(addr)
        .await;

    match server {
        Ok(_) => print!("Server started at {}", addr),
        Err(e) => eprintln!("Server could not start, got error: {}", e),
    }

    Ok(())
}

async fn get_place_request(
    search: String,
    user_agent: String,
) -> Result<ReturSted, PlaceError<StedGetError>> {
    let place_config = create_place_config(user_agent);
    let place_api_client = PlaceApiClient::new(place_config.into());
    place_api_client
        .sted_get(
            Some(&search),
            Some(true), //fuzzy,
            None,       //fnr,
            None,       //knr,
            None,       //kommunenavn,
            None,       //fylkesnavn,
            None,       //stedsnummer,
            None,       //Some(vec!["By".to_string()]), //navneobjekttype,
            None,       //utkoordsys,
            Some(10),   //treff_per_side,
            Some(1),    //side,
            None,       //filtrer,
        )
        .await
}

fn create_place_config(user_agent: String) -> PlaceConfiguration {
    let mut place_config = PlaceConfiguration::new();
    let middleware_client = ClientBuilder::new(Client::new())
        .with(Cache(HttpCache {
            mode: CacheMode::Default,
            manager: CACacheManager::new("./cache/".into(), false),
            options: HttpCacheOptions::default(),
        }))
        .build();
    place_config.user_agent = Some(user_agent);
    place_config.client = middleware_client;
    place_config
}

async fn get_forecast(
    pos: Position,
    user_agent: String,
) -> Result<MetjsonForecast, Error<CompactGetError>> {
    let location_config = create_forecast_client(user_agent);
    let data_api_client = DataApiClient::new(location_config.into());
    data_api_client.compact_get(pos.lat, pos.lon, None).await
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

fn create_ors_client(user_agent: String, api_key: String) -> ORSConfiguration {
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
    route_config.user_agent = Some(user_agent);

    route_config.api_key = Some(ors_client::apis::configuration::ApiKey {
        prefix: None,
        key: api_key,
    });
    route_config
}

async fn get_route(
    coords: Vec<Vec<f64>>,
    user_agent: String,
    api_key: String,
) -> Result<
    ors_client::models::GetSimpleGeoJsonRoute200Response,
    ORSError<ors_client::apis::directions_service_api::GetGeoJsonRouteError>,
> {
    let directions_service_config = create_ors_client(user_agent.clone(), api_key);

    let directions_service_api_client =
        DirectionsServiceApiClient::new(directions_service_config.into());

    // OpenRouteService uses vectors of [longitude, latitude] pairs as coords
    let direction_service_options = DirectionsService::new(coords);

    directions_service_api_client
        .get_geo_json_route("driving-car", direction_service_options)
        .await
}

async fn handle_route_command(
    json_route: GetSimpleGeoJsonRoute200Response,
    number_of_forecasts: f64,
) -> Result<RouteWithForecastResponse, String> {
    let features = match json_route.features {
        Some(features) => features[0].clone(),
        None => {
            println!("No feature found in geo_json_route");
            return Err("No features found".to_string());
        }
    };

    // Feature is just a serde "value" at this point (bad/generic spec). We convert it to a Feature:
    let geo_json_features_result = <Feature as Deserialize>::deserialize(features);
    let geo_json_features = match geo_json_features_result {
        Ok(f) => f,
        Err(e) => {
            println!("Could not convert serde feature to geo_json_feature");
            return Err(format!(
                "Could not convert serde feature to geo_json_feature, got error: {}",
                e,
            ));
        }
    };

    // We want positions to use with a weather service. A Feature contains indices
    // (way_points) that points to the index of a vector of coordinates that contains a start and stop
    // coordinate for the feature: We use the start coordinate.
    let steps = &geo_json_features.properties.segments[0].steps;

    let response_steps: Vec<ResponseStep> = steps
        .iter()
        .map(|s| ResponseStep {
            distance: s.distance,
            dration: s.duration,
            instruction: s.instruction,
            type_field: s.type_field,
            way_points: s.way_points,
            name: s.name,
        })
        .collect();

    let full_distance = geo_json_features.properties.segments[0].distance;
    let full_duration = geo_json_features.properties.segments[0].duration;
    let sampled_steps =
        sample_steps_from_feature(steps, full_distance, number_of_forecasts, full_duration);

    let all_coordinates = geo_json_features.geometry.coordinates;

    let response_coords: Vec<Coordinate> = all_coordinates
        .iter()
        .map(|c| Coordinate {
            longitude: c[0],
            latitude: c[1],
        })
        .collect();

    let coords_with_duration = find_geometry_from_steps(sampled_steps, all_coordinates);

    let mut forecasts: Vec<Forecast> = vec![];

    for coord in coords_with_duration {
        let pos = Position {
            // The spec for locationforecast uses f32s as long, lats. coords are f64
            // should be truncated to max 4 dicits of precision because of ToS
            lon: (coord[0] as f32).mul(10_000.0).round().div(10_000.0),
            lat: (coord[1] as f32).mul(10_000.0).round().div(10_000.0),
        };

        let duration = coord[2];

        let coord = Coordinate {
            longitude: pos.lon as f64,
            latitude: pos.lat as f64,
        };

        let result = get_forecast(pos, user_agent.clone()).await;
        match result {
            Ok(forecast) => {
                //TODO: This assumes every index + 1 -> increases time by an hour, not correct
                //after x hours
                let duration_int = (duration / 3600.0) as usize;
                let current_hour = forecast.properties.timeseries[duration_int].clone();
                let instant_details = current_hour.data.instant.details;
                let next_hour = current_hour.data.next_1_hours.unwrap();
                let next_hour_details = next_hour.details;
                let next_hour_response: proto::ForecastNextHour = ForecastNextHour {
                    air_temperature_min: next_hour_details.air_temperature_min,
                    air_temperature_max: next_hour_details.air_temperature_max,
                    precipitation_amount_min: next_hour_details.precipitation_amount_min,
                    precipitation_amount_max: next_hour_details.precipitation_amount_max,
                    precipitation_amount: next_hour_details.precipitation_amount,
                    probability_of_precipitation: next_hour_details.probability_of_precipitation,
                };
                let symbol_code = next_hour.summary.symbol_code.to_string();

                let time = current_hour.time;
                match instant_details {
                    None => {
                        panic!("No instant details found");
                    }
                    Some(d) => {
                        let current_forecast = Forecast {
                            time,
                            position: Some(coord),
                            air_temperature: d.air_temperature.unwrap(),
                            symbol_code,
                            next_hour: Some(next_hour_response),
                            wind_speed: d.wind_speed,
                            wind_speed_of_gust: d.wind_speed_of_gust,
                            duration,
                        };
                        forecasts.push(current_forecast);
                    }
                }
            }
            Err(e) => {
                eprintln!("Could not retrieve forecast: {e}");
            }
        };
    }
    Ok(RouteWithForecastResponse {
        forecasts,
        steps: response_steps,
        coords: response_coords,
    })
}

fn sample_steps_from_feature(
    steps: &Vec<Step>,
    full_distance: f64,
    number_of_points: f64,
    full_duration: f64,
) -> Vec<Step> {
    let mut sample_steps: Vec<Step> = vec![];
    let mut current_duration = 0.0;

    // We will add the first and last step after sampling the intermittent steps.
    if number_of_points >= 2.0 {
        let step_distance = full_distance / number_of_points - 1.0;

        // We want to sample a step when step_distance has been reached
        let mut current_distance = 0.0;

        for step in steps {
            //TODO: This works quite bad for steps with long distances.
            //step will overshoot the distance we want and we end up with fewer forecasts than
            //expected
            if current_distance >= step_distance {
                current_distance = 0.0;

                let mut sampled_step = step.clone();
                sampled_step.duration = current_duration;

                sample_steps.push(sampled_step);
            }
            current_distance += step.distance;
            current_duration += step.duration;
            if sample_steps.len() as f64 >= number_of_points - 2.0 {
                break;
            }
        }
    }
    // Add first and last steps. I think it is safe to assume that weather at start and end of travel is of
    // interest:
    sample_steps.insert(0, steps[0].clone());
    let mut last_step = steps[steps.len() - 1].clone();
    last_step.duration = full_duration;
    sample_steps.push(last_step);

    sample_steps
}

fn find_geometry_from_steps(steps: Vec<Step>, coordinates: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut sampled_coords: Vec<Vec<f64>> = vec![];

    // The way_point 'index' from steps must be converted to a usize to be used for indexing
    for step in steps {
        let idx_i64: i64 = step.way_points[0];
        let duration = step.duration;
        if let Ok(idx) = usize::try_from(idx_i64) {
            let mut coordinate_with_duration = coordinates[idx].clone();
            coordinate_with_duration.push(duration);
            sampled_coords.push(coordinate_with_duration);
        }
    }
    sampled_coords
}
