use super::forecast::{create_forecast_client, get_forecast};
use super::geo_json_200_response::Feature;
use crate::proto::{
    self, Coordinate, Forecast, ForecastNextHour, RouteWithForecastResponse, Step as ResponseStep,
};
use crate::server::geo_json_200_response::Step;
use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};
use ors_client::apis::Error as ORSError;
use ors_client::apis::configuration::Configuration as ORSConfiguration;
use ors_client::apis::directions_service_api::DirectionsServiceApi;
use reqwest::Client;
use reqwest::header::HeaderMap;
use reqwest_middleware::ClientBuilder;
use serde::Deserialize;

use ors_client::models::{DirectionsService, GetSimpleGeoJsonRoute200Response};

use location_forecast_client::apis::data_api::DataApiClient;

pub fn create_ors_client(user_agent: String, api_key: String) -> ORSConfiguration {
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

pub async fn get_route(
    coords: Vec<Vec<f64>>,
    directions_service_api: &impl DirectionsServiceApi,
) -> Result<
    ors_client::models::GetSimpleGeoJsonRoute200Response,
    ORSError<ors_client::apis::directions_service_api::GetGeoJsonRouteError>,
> {
    // OpenRouteService uses vectors of [longitude, latitude] pairs as coords
    let direction_service_options = DirectionsService::new(coords);

    directions_service_api
        .get_geo_json_route("driving-car", direction_service_options)
        .await
}

pub async fn handle_route_command(
    json_route: GetSimpleGeoJsonRoute200Response,
    number_of_forecasts: f64,
    user_agent: String,
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
            instruction: s.instruction.clone(),
            type_field: s.type_field,
            way_points: s.way_points.clone(),
            name: s.name.clone(),
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

    let location_config = create_forecast_client(user_agent);
    let data_api_client = DataApiClient::new(location_config.into());

    for coord in coords_with_duration {
        let duration = coord[2];
        let coord = Coordinate {
            longitude: coord[0],
            latitude: coord[1],
        };

        let result = get_forecast(coord, &data_api_client).await;
        match result {
            Ok(forecast) => {
                //TODO: This assumes every index + 1 -> increases time by an hour, not correct
                //after x hours, should check ForecastTimeStep.time instead
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

    // Always adds first and last steps
    sample_steps.insert(0, steps[0].clone());
    let mut last_step = steps[steps.len() - 1].clone();
    last_step.duration = full_duration;
    sample_steps.push(last_step);

    if number_of_points <= 2.0 {
        return sample_steps;
    }

    if number_of_points >= 2.0 {
        let step_distance = full_distance / (number_of_points - 1.0);

        // We want to sample a step when step_distance has been reached
        let mut current_distance = 0.0;

        for step in steps {
            //TODO: fix 'overshooting' for large step.distance
            if current_distance >= step_distance {
                current_distance = 0.0;

                let mut sampled_step = step.clone();
                sampled_step.duration = current_duration;

                sample_steps.push(sampled_step);
            }
            current_distance += step.distance;
            current_duration += step.duration;
            if sample_steps.len() as f64 >= number_of_points {
                // Move previously added last step to the end of vector
                let last_step = sample_steps.remove(1);
                sample_steps.push(last_step);
                break;
            }
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_steps_from_feature_works() {
        let step_one = Step {
            distance: 100.0,
            duration: 360.0,
            instruction: "Run forest! Run!".to_string(),
            name: "Forestveg 1".to_string(),
            type_field: 10,
            way_points: vec![1, 2],
            exit_number: Some(1),
        };
        let step_two = Step {
            distance: 100.0,
            duration: 360.0,
            instruction: "Run forest! Run!".to_string(),
            name: "Forestveg 2".to_string(),
            type_field: 10,
            way_points: vec![2, 3],
            exit_number: Some(2),
        };
        let step_three = Step {
            distance: 100.0,
            duration: 360.0,
            instruction: "Run forest! Run!".to_string(),
            name: "Forestveg 3".to_string(),
            type_field: 10,
            way_points: vec![3, 4],
            exit_number: Some(3),
        };
        let step_four = Step {
            distance: 100.0,
            duration: 360.0,
            instruction: "Run forest! Run!".to_string(),
            name: "Forestveg 4".to_string(),
            type_field: 10,
            way_points: vec![4, 5],
            exit_number: Some(4),
        };
        let step_five = Step {
            distance: 100.0,
            duration: 360.0,
            instruction: "Run forest! Run!".to_string(),
            name: "Forestveg 5".to_string(),
            type_field: 10,
            way_points: vec![5, 6],
            exit_number: Some(5),
        };
        let steps: Vec<Step> = vec![step_one, step_two, step_three, step_four, step_five];

        let full_distance = steps.iter().fold(0.0, |acc, step| acc + step.distance);
        let full_duration = steps.iter().fold(0.0, |acc, step| acc + step.duration);

        // Sampling two points simply returns start and stop step
        let number_of_points = 2.0;
        let sampled_two_steps =
            sample_steps_from_feature(&steps, full_distance, number_of_points, full_duration);
        assert_eq!(sampled_two_steps.len(), 2);
        assert_eq!(sampled_two_steps[0].duration, 360.0 * 1.0);
        assert_eq!(sampled_two_steps[1].duration, 360.0 * 5.0);

        let number_of_points = 3.0;
        let sampled_three_steps =
            sample_steps_from_feature(&steps, full_distance, number_of_points, full_duration);
        assert_eq!(sampled_three_steps.len(), 3);
        assert_eq!(sampled_three_steps[0].duration, 360.0 * 1.0);
        assert_eq!(sampled_three_steps[1].duration, 360.0 * 3.0);
        assert_eq!(sampled_three_steps[2].duration, 360.0 * 5.0);

        let number_of_points = 4.0;
        let sampled_four_steps =
            sample_steps_from_feature(&steps, full_distance, number_of_points, full_duration);
        assert_eq!(sampled_four_steps.len(), 4);
        assert_eq!(sampled_four_steps[1].duration, 360.0 * 2.0);
        assert_eq!(sampled_four_steps[2].duration, 360.0 * 4.0);
    }

    #[test]
    fn sample_number_of_steps_from_features_containing_large_distance() {
        let step_one = Step {
            distance: 100.0,
            duration: 360.0,
            instruction: "Run forest! Run!".to_string(),
            name: "Forestveg 1".to_string(),
            type_field: 10,
            way_points: vec![1, 2],
            exit_number: Some(1),
        };
        let step_two_long_distance = Step {
            distance: 10_000.0,
            duration: 360.0,
            instruction: "Run forest! Run!".to_string(),
            name: "Forestveg 2".to_string(),
            type_field: 10,
            way_points: vec![2, 3],
            exit_number: Some(2),
        };
        let step_three = Step {
            distance: 100.0,
            duration: 360.0,
            instruction: "Run forest! Run!".to_string(),
            name: "Forestveg 3".to_string(),
            type_field: 10,
            way_points: vec![3, 4],
            exit_number: Some(3),
        };
        let step_four = Step {
            distance: 100.0,
            duration: 360.0,
            instruction: "Run forest! Run!".to_string(),
            name: "Forestveg 4".to_string(),
            type_field: 10,
            way_points: vec![4, 5],
            exit_number: Some(4),
        };
        let step_five = Step {
            distance: 100.0,
            duration: 360.0,
            instruction: "Run forest! Run!".to_string(),
            name: "Forestveg 5".to_string(),
            type_field: 10,
            way_points: vec![5, 6],
            exit_number: Some(5),
        };
        let steps: Vec<Step> = vec![
            step_one,
            step_two_long_distance,
            step_three,
            step_four,
            step_five,
        ];

        let full_distance = steps.iter().fold(0.0, |acc, step| acc + step.distance);
        let full_duration = steps.iter().fold(0.0, |acc, step| acc + step.duration);

        let number_of_points = 4.0;
        let sampled_four_steps =
            sample_steps_from_feature(&steps, full_distance, number_of_points, full_duration);
        // This is the current behaviour, return 3 when 4 is expected
        assert_eq!(sampled_four_steps.len(), 3);

        // This is the wanted behaviour:
        // TODO: fix overshooting
        // assert_eq!(sampled_four_steps.len(), 4);
    }
}
