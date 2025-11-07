use super::proto::route_forecast_client::RouteForecastClient;
use super::proto::{Coordinate, RouteWithForecastRequest, RouteWithForecastResponse};
use tonic_web_wasm_client::Client;

use std::error::Error;

#[cfg(feature = "wasm")]
pub async fn get_route_with_forecast(
    coords: Vec<Coordinate>,
    number_of_forecasts: f64,
) -> Result<tonic::Response<RouteWithForecastResponse>, Box<dyn Error>> {
    let url = String::from("http://[::1]:50051");
    let wasm_client = Client::new(url);
    let mut client = RouteForecastClient::new(wasm_client);
    let req = RouteWithForecastRequest {
        number_of_forecasts,
        coordinates: coords,
    };
    let request = tonic::Request::new(req);
    let response = client.get_route_with_forecast(request).await?;
    dbg!(&response);
    Ok(response)
}
