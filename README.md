# Weather route

Shows the weather conditions along a route.

## features
Features are used to split dependencies when building for different targets:
- server: Used when building and running the server binary
`cargo build --features server`

- wasm: Used when building the client library with target wasm32-unknown-unknown
`cargo build --lib --features wasm --target wasm32-unknow-unknown`

## Starting the server

Requirements:
- To use Open Route Service you will need a API-key from heigit, their [free plan](https://account.heigit.org/info/plans) allows 2k requests for the routing API. Not bad!

- To use location-forecast you must identify your application using an user-agent.

The server expects these env variables:
- ORS_API_KEY - the api key to pass in the authorization header for ORS requests
- USER_AGENT - the user agent used for both ORS and locationforecast requests

You could start the server like this:
```
export ORS_API_KEY="$(pass <your_api_key>)" && export USER_AGENT="mydomain.no/app contact@mydomain.no" cargo run --bin route-forecast-server --features server
```

## Using the client (in yew):
- Using the client on a wasm32 build from a local crate:
```{toml}
# file: Cargo.toml
weather-route = { version = "0.1.0", path = "<path_to_this_crate>", default-features=false, features = ["wasm"] }
```

Minimal example yew component using the client (takes the first temp returned and views it, or shows an error):
```{rust}

use yew::prelude::*;
use weather_route::proto::Coordinate;
use weather_route::wasm::get_route_with_forecast;

#[function_component(DataFetcher)]
pub fn data_fetcher() -> Html {
    let data = use_state(|| None::<String>);
    let error = use_state(|| None::<String>);

    let onclick = {
        let data = data.clone();
        let error = error.clone();
        Callback::from(move |_| {
            let data = data.clone();
            let error = error.clone();
            // Spawn a new asynchronous task for the gRPC call
            wasm_bindgen_futures::spawn_local(async move {
                let coords = vec![
                    Coordinate {
                        longitude: 8.7721,
                        latitude: 58.4617,
                    },
                    Coordinate {
                        longitude: 7.1807,
                        latitude: 62.7403,
                    },
                ];
                let number_of_forecasts = 3.0;
                let result = get_route_with_forecast(coords, number_of_forecasts).await;
                match result {
                    Ok(response) => {
                        let air_temp = response.into_inner().forecasts[0]
                            .air_temperature
                            .to_string();
                        data.set(Some(air_temp));
                        error.set(None);
                    }
                    Err(e) => {
                        error.set(Some(format!("Error fetching data: {}", e)));
                        data.set(None);
                    }
                }
            });
        })
    };

    html! {
        <div>
            <button {onclick}>{ "Fetch Data" }</button>
            if let Some(d) = &*data {
                <p>{ d }</p>
            }
            if let Some(e) = &*error {
                <p style="color: red;">{ e }</p>
            }
        </div>
    }
}
```

## Data sources:

### Weather API:

Docs: https://developer.yr.no/doc/
Swagger: https://api.met.no/weatherapi/locationforecast/2.0/swagger
Terms of Service: https://developer.yr.no/doc/TermsOfService/

- Identification
> All requests must (if possible) include an identifying User Agent-string (UA)
> in the request with the application/domain name, optionally version number.
> You should also include a company email address or a link to the company
> website where we can find contact information. If we cannot contact you in
> case of problems, you risk being blocked without warning.":

"fredfull.no/ post@fredfull.no"

Optionally, use and Origin or Referer header for identification.

- Attribution:

> You must give appropriate credit, provide a link to the license, and indicate
> if changes were made. You may do so in any reasonable manner, but not in any
> way that suggests the licensor endorses you or your use.

Traffic:
> Do not ask too often, and don't repeat requests until the time indicated in the Expires response header.

Bandwith:
> Anything over 20 requests/second per application (total, not per client) requires special agreement


### Routing:

Open Street Map

ToS: https://openrouteservice.org/restrictions/
Free plan: https://account.heigit.org/info/plans

When generating the client using openapi-generator, authorization does not seem to be implemented properly.
I "fixed" this by adding the api_key 'key' property to the authorization header:

```rust
// ors-client/src/apis/directions_service_api.rs:160
if let Some(ref api_key) = configuration.api_key {
    req_builder = req_builder.header(reqwest::header::AUTHORIZATION, api_key.key.clone())
}
```

## Tools

### Generating client APIs from OpenAPI/swagger specs

https://github.com/OpenAPITools/openapi-generator
https://github.com/swagger-api/swagger-codegen

### Generating serde structs from a json blob:
https://transform.tools/json-to-rust-serde

## Development

## Protobuf
A protobuf compiler must be installed for the build script to work. On ubuntu/debian:
```{bash}
apt install protobuf-compiler
```

grpcurl can be installed for testing the server:
```{bash}
snap install --edge grpcurl
grpcurl -plaintext -d '{"coordinates": [{"longitude": 8.7724,"latitude": 58.4618},{"longitude": 7.1808, "latitude": 62.7403}], "number_of_forecasts": 10}' '[::1]:50051' route_forecast.RouteForecast.GetRouteWithForecast
```
