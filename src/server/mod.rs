struct Position {
    lat: f32,
    lon: f32,
}

use core::panic;
use std::ops::{Div, Mul};

use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};
use location_forecast_client::apis::Error;
use location_forecast_client::apis::configuration::Configuration as LocationForecastConfiguration;
use location_forecast_client::apis::data_api::{CompactGetError, compact_get};
use location_forecast_client::models::MetjsonForecast;

use ors_client::apis::configuration::Configuration as ORSConfiguration;
use ors_client::apis::directions_service_api::get_geo_json_route;
use ors_client::models::DirectionsService;

use stedsnavn_client::apis::Error as PlaceError;
use stedsnavn_client::apis::configuration::Configuration as PlaceConfiguration;
use stedsnavn_client::apis::default_api::StedGetError;
use stedsnavn_client::apis::default_api::sted_get;

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
        for coord in input.coordinates.clone() {
            println!("longitude: {}", coord.longitude);
            println!("latitude: {}", coord.latitude);
        }
        println!("number_of_forecasts: {}", input.number_of_forecasts);

        if input.number_of_forecasts < 2.0 {
            return Err(tonic::Status::invalid_argument(
                "number_of_forecasts should be between 2 and 10",
            ));
        }

        if input.number_of_forecasts > 20.0 {
            return Err(tonic::Status::invalid_argument(
                "number_of_forecasts should be between 2 and 10",
            ));
        }

        let coords = input
            .coordinates
            .iter()
            .map(|coord| vec![coord.longitude, coord.latitude])
            .collect();
        let response =
            handle_route_command(coords, input.number_of_forecasts, user_agent, ors_api_key).await;
        println!("{}: Returning RouteWithForecastResponse", Local::now());
        Ok(tonic::Response::new(response))
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

        let response = get_place_request(search, user_agent).await;
        match response {
            Ok(r) => {
                if r.metadata.unwrap().totalt_antall_treff.unwrap() > 0 {
                    let places = r
                        .navn
                        .unwrap()
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
                } else {
                    println!("{}: Found no place", Local::now());
                    Err(tonic::Status::not_found("No place found"))
                }
            }
            Err(e) => {
                eprintln!("Error from 'stedsnavn' API: {e}");
                Err(tonic::Status::internal("Error from 'stedsnavn' API"))
            }
        }
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
    dbg!(&search);
    sted_get(
        &place_config,
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
    compact_get(&location_config, pos.lat, pos.lon, None).await
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

async fn handle_route_command(
    coords: Vec<Vec<f64>>,
    number_of_forecasts: f64,
    user_agent: String,
    api_key: String,
) -> RouteWithForecastResponse {
    let default_response = RouteWithForecastResponse {
        forecasts: vec![],
        steps: vec![],
        coords: vec![],
    };
    let route_config = create_ors_client(user_agent.clone(), api_key);

    // OpenRouteService uses vectors of [longitude, latitude] pairs as coords
    let direction_service_options = DirectionsService::new(coords);

    let response =
        get_geo_json_route(&route_config, "driving-car", direction_service_options).await;

    let result = match response {
        Ok(result) => result,
        Err(err) => {
            println!("Route service returned error: {err}");
            return default_response;
        }
    };

    // Attribution is required to use open route services
    // https://openrouteservice.org/terms-of-service/
    //let metadata = match result.metadata {
    //    Some(metadata) => match metadata.attribution {
    //        Some(attribution) => attribution,
    //        None => {
    //            println!("No attribution found in metadata from route service, using default ");
    //            "© openrouteservice.org by HeiGIT | Map data © OpenStreetMap contributors"
    //                .to_string()
    //        }
    //    },
    //    None => {
    //        println!("No metadata returned by route service, using default");
    //        "© openrouteservice.org by HeiGIT | Map data © OpenStreetMap contributors".to_string()
    //    }
    //};

    let feature = match result.features {
        Some(features) => features[0].clone(),
        None => {
            println!("No feature returned by route service");
            return default_response;
        }
    };

    // Feature is just a serde "value" at this point (bad/generic spec). We convert it to a Feature:
    let geo_json_feature = <Feature as Deserialize>::deserialize(feature);

    // We want to store some names to add to our weather data, this is not returned by the forecast service:
    let mut names: Vec<String> = vec![];

    let mut response_steps: Vec<ResponseStep> = vec![];
    let mut response_coords: Vec<Coordinate> = vec![];

    // We want positions to use with a weather service. A Feature contains indices
    // (way_points) that points to the index of a vector of coordinates that contains a start and stop
    // coordinate for the feature: We use the start coordinate.
    let coords_with_duration = match geo_json_feature {
        Ok(geo_json_feature) => {
            let steps = &geo_json_feature.properties.segments[0].steps;

            for s in steps {
                let new_response_step = ResponseStep {
                    distance: s.distance,
                    dration: s.duration,
                    instruction: s.instruction.clone(),
                    type_field: s.type_field,
                    way_points: s.way_points.clone(),
                    name: s.name.clone(),
                };
                response_steps.push(new_response_step);
            }

            let full_distance = geo_json_feature.properties.segments[0].distance;
            let full_duration = geo_json_feature.properties.segments[0].duration;
            let sampled_steps =
                sample_steps_from_feature(steps, full_distance, number_of_forecasts, full_duration);

            for s in &sampled_steps {
                names.push(s.name.clone());
            }

            let all_coordinates = geo_json_feature.geometry.coordinates;
            for c in all_coordinates.clone() {
                response_coords.push(Coordinate {
                    longitude: c[0],
                    latitude: c[1],
                });
            }
            find_geometry_from_steps(sampled_steps, all_coordinates)
        }
        Err(err) => {
            println!("No route found, got error: {err}");
            vec![]
        }
    };

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
                //TODO:_ This assumes every index + 1 -> increases time by an hour
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
    RouteWithForecastResponse {
        forecasts,
        steps: response_steps,
        coords: response_coords,
    }
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
