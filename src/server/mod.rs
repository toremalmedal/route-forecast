use core::panic;

use http::{HeaderName, HeaderValue};
use ors_client::apis::directions_service_api::DirectionsServiceApiClient;
use reqwest::Method;

// The spec we generate our ors_client from lacks the response signature of features from GeoJSON:
mod geo_json_200_response;
use stedsnavn_client::apis::default_api::DefaultApiClient;
use tonic::transport::{Identity, Server, ServerTlsConfig};
use tonic_web::GrpcWebLayer;
use tower_http::cors::CorsLayer;

mod forecast;
mod place;
mod routing;

use chrono::Local;

use crate::proto::route_forecast_server::{RouteForecast, RouteForecastServer};
use crate::proto::{
    self, FILE_DESCRIPTOR_SET, PlaceRequest, PlaceResponse, RouteWithForecastRequest,
    RouteWithForecastResponse,
};

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

        let directions_service_config = routing::create_ors_client(user_agent.clone(), ors_api_key);

        let directions_service_api_client =
            DirectionsServiceApiClient::new(directions_service_config.into());

        let geo_json_route_result =
            routing::get_route(itinerary_coords, &directions_service_api_client).await;

        let geo_json_route = match geo_json_route_result {
            Ok(g) => g,
            Err(e) => {
                println!("{}: Could not find route, got error {}", Local::now(), e);
                return Err(tonic::Status::new(tonic::Code::NotFound, "Route not found"));
            }
        };

        let route_with_forecast_result =
            routing::handle_route_command(geo_json_route, input.number_of_forecasts, user_agent)
                .await;

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
        let municipality_str: Option<&str> = input.municipality.as_deref();

        let user_agent = match std::env::var("USER_AGENT") {
            Ok(val) => val,
            Err(e) => {
                panic!("couldn't find env var USER_AGENT: {e}");
            }
        };
        println!("{}: Recieved PlaceRequest", Local::now());
        println!("name: {}", search);

        let config = place::create_place_config(user_agent);
        let api_place_client = DefaultApiClient::new(config.into());

        let places_result = place::get_places(search, municipality_str, &api_place_client).await;
        match places_result {
            Ok(place) => {
                println!("{}: Returning PlaceResponse", Local::now());
                Ok(tonic::Response::new(proto::PlaceResponse { place }))
            }
            Err(e) => Err(e),
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

    let allow_origin_domain = std::env::var("ALLOW_ORIGIN").unwrap();

    let cors = CorsLayer::new()
        .allow_headers([
            http::header::CONTENT_TYPE,
            HeaderName::from_static("x-grpc-web"),
        ])
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(allow_origin_domain.parse::<HeaderValue>().unwrap());

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
