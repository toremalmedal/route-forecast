#!/bin/bash

required_commands=("docker" "cargo" "jq")
missing_commands=()

for cmd in "${required_commands[@]}"; do
  if ! command -v "$cmd" &>/dev/null; then
    missing_commands+=("$cmd")
  fi
done

if [ ${#missing_commands[@]} -gt 0 ]; then
  echo "Error: Missing the following requirement(s): ${missing_commands[*]}"
  exit 1
fi

# Weather forecast API / yr
# Downloads spec, adds host and scheme, creates client and adds it as an local dependency
curl https://api.met.no/weatherapi/locationforecast/2.0/swagger | jq >>location-forecast-swagger.json
echo "$(jq '. += {"host": "api.met.no", "schemes": ["https"]}' location-forecast-swagger.json)" >location-forecast-swagger-edit.json
docker run --rm -v "${PWD}:/local" openapitools/openapi-generator-cli generate -i /local/location-forecast-swagger-edit.json -g rust -o /local/location-forecast-client --package-name=location-forecast-client --additional-properties=supportMiddleware=true
cargo add --path ./location-forecast-client
rm location-forecast-swagger.json location-forecast-swagger-edit.json

curl https://docs.openrouteservice.org/all/docs | jq >>ors-swagger.json
echo "$(jq 'del(.paths["/pois"])' ors-swagger.json)" >ors-swagger-edit.json
docker run --rm -v "${PWD}:/local" openapitools/openapi-generator-cli generate -i /local/ors-swagger-edit.json -g rust -o /local/ors-client --package-name=ors-client --additional-properties=supportMiddleware=true
cargo add --path ./ors-client
rm ors-swagger.json ors-swagger-edit.json

curl https://ws.geonorge.no/stedsnavn/v1/openapi.json | jq >>stedsnavn.json
docker run --rm -v "${PWD}:/local" openapitools/openapi-generator-cli generate -i /local/stedsnavn.json -g rust -o /local/stedsnavn-client --package-name=stedsnavn-client --additional-properties=supportMiddleware=true
cargo add --path ./stedsnavn-client
rm stedsnavn.json
