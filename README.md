# Weather route

Shows the weather conditions along a route.

## features
Features are used to split dependencies when building for different targets:
- server: Used when building and running the server binary
`cargo build --features server`

- wasm: Used when building the client library with target wasm32-unknown-unknown
`cargo build --lib --features wasm --target wasm32-unknow-unknown`

## Development

Requirements:

- A protobuf compiler must be installed for the build script to work. On ubuntu/debian:
```{bash}
apt install protobuf-compiler
```

- An API-key from heigit to use Open Route Service. Their [free plan](https://account.heigit.org/info/plans) allows 2k requests for the routing API. 

- Setting an User Agent to use location-forecast. ToS says you must identify your application.

- The server expects these env variables:
- ORS_API_KEY - the api key to pass in the authorization header for ORS requests
- USER_AGENT - the user agent used for both ORS and locationforecast requests
- GRCP_SERVER_URL - the url for the GRCP server

- Build it 
```
./create-api-clients.sh && cargo build --features server
```

- Start the server on your host:
```
export ORS_API_KEY="$(pass <your_api_key>)" && \
export USER_AGENT="mydomain.no/app contact@mydomain.no" && \
export GRPC_SERVER_URL="[::1]:50051" && \
cargo run --bin route-forecast-server --features server
```

- Start by using docker:

```{bash}
docker build . -t route-api
docker run -e ORS_API_KEY="$(pass ors-api-key)" -e USER_AGENT="mydomain.no/app contact@mydomain.no" -e GRPC_SERVER_URL="0.0.0.0:50051" -p 50051:50051 --name route-api -t route-api:latest
```

- Test connection with grpcurl:

```{bash}
grpcurl -plaintext -d '{"coordinates": [{"longitude": 10.7335,"latitude": 59.9119},{"longitude": 10.7413, "latitude": 59.921}], "number_of_forecasts": 3}' '[::1]:50051' route_forecast.RouteForecast.GetRouteWithForecast
{
  "forecasts": [
    {
      "position": {
        "longitude": 10.732999801635742,
        "latitude": 59.911399841308594
      },
      "airTemperature": 10.3,
      "symbolCode": "heavyrain",
      "windSpeed": 8.9,
      "nextHour": {
        "precipitationAmount": 1.3
      },
      "time": "2025-11-12T10:00:00Z",
      "duration": 26.1
    },
    ....
  ]
  "steps": [
    {
      "distance": 435.5,
      "dration": 26.1,
      "typeField": "11",
      "instruction": "Head northwest on Operatunnelen / Festningstunnelen, E 18",
      "name": "Operatunnelen / Festningstunnelen, E 18",
      "wayPoints": [
        "0",
        "6"
      ]
    },
    ....
   ]
    }
  ],
  "coords": [
    {
      "longitude": 10.733041,
      "latitude": 59.91139
    },
    ....
  ]
}
```

## Using the web client (in yew):
Env variables at compile time:
- GRCP_SERVER_URL - the server url ("http://<my-service>:port") for the web grpcs client to connect to

- Using the client for a wasm32 target build (note only 'wasm' feature is enabled):
```{toml}
# file: Cargo.toml
weather-route = { version = "0.1.0", path = "<path_to_this_crate>", default-features=false, features = ["wasm"] }
```

<! --TODO: Minimal yew project with example using the client (takes the first temp returned and views it, or shows an error): -->


## Tools

### Generating client APIs from OpenAPI/swagger specs

https://github.com/OpenAPITools/openapi-generator
https://github.com/swagger-api/swagger-codegen

### Generating serde structs from a json blob:
https://transform.tools/json-to-rust-serde

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

Docs: https://openrouteservice.org/dev/#/api-docs/v2/directions

Terms of Service: https://openrouteservice.org/restrictions/

Free heigit plan: https://account.heigit.org/info/plans


### Name lookup, coords from name:

Docs: https://api.kartverket.no/stedsnavn/v1/

OpenApi: https://api.kartverket.no/stedsnavn/v1/openapi.json

