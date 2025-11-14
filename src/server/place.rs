use crate::proto::{Coordinate, Place};
use chrono::Local;
use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};
use reqwest::Client;
use reqwest_middleware::ClientBuilder;
use stedsnavn_client::apis::Error as PlaceError;
use stedsnavn_client::apis::configuration::Configuration as PlaceConfiguration;
use stedsnavn_client::apis::default_api::StedGetError;
use stedsnavn_client::apis::default_api::{DefaultApi, DefaultApiClient as PlaceApiClient};
use stedsnavn_client::models::ReturSted;

pub async fn get_places(search: String, user_agent: String) -> Result<Vec<Place>, tonic::Status> {
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
    Ok(places)
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
