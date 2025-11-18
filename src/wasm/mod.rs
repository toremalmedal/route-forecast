use super::proto::route_forecast_client::RouteForecastClient;
use super::proto::{
    Coordinate, PlaceRequest, PlaceResponse, RouteWithForecastRequest, RouteWithForecastResponse,
};

use std::error::Error;
use tonic_web_wasm_client::Client;

#[cfg(feature = "wasm")]
pub async fn get_route_with_forecast(
    coords: Vec<Coordinate>,
    number_of_forecasts: f64,
    server_url: String,
) -> Result<tonic::Response<RouteWithForecastResponse>, Box<dyn Error>> {
    let wasm_client = Client::new(server_url);
    let mut client = RouteForecastClient::new(wasm_client);
    let req = RouteWithForecastRequest {
        number_of_forecasts,
        coordinates: coords,
    };
    let request = tonic::Request::new(req);
    let response = client.get_route_with_forecast(request).await?;
    Ok(response)
}

pub async fn get_place(
    name: String,
    municipality: Option<String>,
    server_url: String,
) -> Result<tonic::Response<PlaceResponse>, Box<dyn Error>> {
    let wasm_client = Client::new(server_url);
    let mut client = RouteForecastClient::new(wasm_client);
    let req = PlaceRequest { name, municipality };
    let request = tonic::Request::new(req);
    let response = client.get_place(request).await?;
    dbg!(&response);
    Ok(response)
}
