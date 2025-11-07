use proto::Coordinate;
use proto::route_forecast_client::RouteForecastClient;
use std::error::Error;

pub mod proto {
    tonic::include_proto!("route_forecast");
}

#[cfg(feature = "wasm")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://[::1]:50051";
    let mut client = RouteForecastClient::connect(url).await?;
    let req = proto::RouteWithForecastRequest {
        number_of_forecasts: 3.0,
        coordinates: vec![
            Coordinate {
                latitude: 62.7,
                longitude: 7.1,
            },
            Coordinate {
                latitude: 58.4,
                longitude: 8.7,
            },
        ],
    };
    let request = tonic::Request::new(req);
    let response = client.get_route_with_forecast(request).await?;
    dbg!(&response);
    Ok(())
}
