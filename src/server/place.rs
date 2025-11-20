use crate::proto::{Coordinate, Place};
use chrono::Local;
use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};
use reqwest::Client;
use reqwest_middleware::ClientBuilder;
use stedsnavn_client::apis::Error as PlaceError;
use stedsnavn_client::apis::configuration::Configuration as PlaceConfiguration;
use stedsnavn_client::apis::default_api::DefaultApi as PlaceApi;
use stedsnavn_client::apis::default_api::StedGetError;
use stedsnavn_client::models::ReturSted;

pub async fn get_places(
    search: String,
    municipality: Option<&str>,
    place_api_client: &impl PlaceApi,
) -> Result<Vec<Place>, tonic::Status> {
    let api_response_result = get_place_request(search, municipality, place_api_client).await;
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
            let municipality = n.kommuner.clone().unwrap()[0]
                .kommunenavn
                .as_ref()
                .unwrap()
                .to_string();
            let place = Place {
                name: first_place.skrivemte.unwrap(),
                point: Some(Coordinate {
                    latitude: first_point.nord.unwrap(),
                    longitude: first_point.st.unwrap(),
                }),
                municipality: Some(municipality),
            };
            println!("Found place: {}", place.name);
            place
        })
        .collect();
    Ok(places)
}

async fn get_place_request(
    search: String,
    municipality: Option<&str>,
    place_api_client: &impl PlaceApi,
) -> Result<ReturSted, PlaceError<StedGetError>> {
    place_api_client
        .sted_get(
            Some(&search),
            Some(true),   //fuzzy,
            None,         //fnr,
            None,         //knr,
            municipality, //kommunenavn,
            None,         //fylkesnavn,
            None,         //stedsnummer,
            None,         //navneobjekttype,
            None,         //utkoordsys,
            Some(10),     //treff_per_side,
            Some(1),      //side,
            None,         //filtrer,
        )
        .await
}

pub fn create_place_config(user_agent: String) -> PlaceConfiguration {
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

#[cfg(test)]
mod tests {
    use super::*;
    use stedsnavn_client::apis::default_api::MockDefaultApi;

    #[tokio::test]
    async fn get_places_works() {
        let stedsnavn_api_response_json = "{
  \"metadata\": {
    \"side\": 1,
    \"sokeStreng\": \"sok=arendal&fuzzy=true&utkoordsys=4258&treffPerSide=10&side=1\",
    \"totaltAntallTreff\": 147,
    \"treffPerSide\": 10,
    \"viserFra\": 1,
    \"viserTil\": 10
  },
  \"navn\": [
    {
      \"fylker\": [
        {
          \"fylkesnavn\": \"Agder\",
          \"fylkesnummer\": \"42\"
        }
      ],
      \"geojson\": {
        \"geometry\": {
          \"coordinates\": [
            [
              8.76695,
              58.46121
            ],
            [
              8.77253,
              58.46151
            ]
          ],
          \"type\": \"MultiPoint\"
        }
      },
      \"kommuner\": [
        {
          \"kommunenavn\": \"Arendal\",
          \"kommunenummer\": \"4203\"
        }
      ],
      \"navneobjekttype\": \"By\",
      \"oppdateringsdato\": \"2020-02-28T12:56:56\",
      \"representasjonspunkt\": {
        \"nord\": 58.46121,
        \"øst\": 8.76695
      },
      \"stedsnavn\": [
        {
          \"navnestatus\": \"hovednavn\",
          \"skrivemåte\": \"Arendal\",
          \"skrivemåtestatus\": \"godkjent og prioritert\",
          \"språk\": \"Norsk\",
          \"stedsnavnnummer\": 1
        }
      ],
      \"stedsnummer\": 667594,
      \"stedstatus\": \"aktiv\"
    },
    {
      \"fylker\": [
        {
          \"fylkesnavn\": \"Agder\",
          \"fylkesnummer\": \"42\"
        }
      ],
      \"geojson\": {
        \"geometry\": {
          \"coordinates\": [
            [
              8.76503,
              58.46044
            ],
            [
              8.76503,
              58.46044
            ]
          ],
          \"type\": \"MultiPoint\"
        }
      },
      \"kommuner\": [
        {
          \"kommunenavn\": \"Arendal\",
          \"kommunenummer\": \"4203\"
        }
      ],
      \"navneobjekttype\": \"Kommune\",
      \"oppdateringsdato\": \"2020-02-24T09:28:28\",
      \"representasjonspunkt\": {
        \"nord\": 58.46044,
        \"øst\": 8.76503
      },
      \"stedsnavn\": [
        {
          \"navnestatus\": \"hovednavn\",
          \"skrivemåte\": \"Arendal kommune\",
          \"skrivemåtestatus\": \"godkjent og prioritert\",
          \"språk\": \"Norsk\",
          \"stedsnavnnummer\": 1
        },
        {
          \"navnestatus\": \"undernavn\",
          \"skrivemåte\": \"Arendal\",
          \"skrivemåtestatus\": \"godkjent og prioritert\",
          \"språk\": \"Norsk\",
          \"stedsnavnnummer\": 2
        }
      ],
      \"stedsnummer\": 267383,
      \"stedstatus\": \"aktiv\"
    },
    {
      \"fylker\": [
        {
          \"fylkesnavn\": \"Innlandet\",
          \"fylkesnummer\": \"34\"
        }
      ],
      \"geojson\": {
        \"geometry\": {
          \"coordinates\": [
            10.86939,
            60.78571
          ],
          \"type\": \"Point\"
        }
      },
      \"kommuner\": [
        {
          \"kommunenavn\": \"Ringsaker\",
          \"kommunenummer\": \"3411\"
        }
      ],
      \"navneobjekttype\": \"Dal\",
      \"oppdateringsdato\": \"2020-02-14T23:38:38\",
      \"representasjonspunkt\": {
        \"nord\": 60.78571,
        \"øst\": 10.86939
      },
      \"stedsnavn\": [
        {
          \"navnestatus\": \"hovednavn\",
          \"skrivemåte\": \"Arendal\",
          \"skrivemåtestatus\": \"godkjent og prioritert\",
          \"språk\": \"Norsk\",
          \"stedsnavnnummer\": 1
        }
      ],
      \"stedsnummer\": 224879,
      \"stedstatus\": \"aktiv\"
    },
    {
      \"fylker\": [
        {
          \"fylkesnavn\": \"Trøndelag - Trööndelage\",
          \"fylkesnummer\": \"50\"
        }
      ],
      \"geojson\": {
        \"geometry\": {
          \"coordinates\": [
            11.0491,
            63.67894
          ],
          \"type\": \"Point\"
        }
      },
      \"kommuner\": [
        {
          \"kommunenavn\": \"Levanger\",
          \"kommunenummer\": \"5037\"
        }
      ],
      \"navneobjekttype\": \"Bruk\",
      \"oppdateringsdato\": \"2020-02-14T23:52:52\",
      \"representasjonspunkt\": {
        \"nord\": 63.67894,
        \"øst\": 11.0491
      },
      \"stedsnavn\": [
        {
          \"navnestatus\": \"hovednavn\",
          \"skrivemåte\": \"Arendal\",
          \"skrivemåtestatus\": \"godkjent og prioritert\",
          \"språk\": \"Norsk\",
          \"stedsnavnnummer\": 1
        }
      ],
      \"stedsnummer\": 775172,
      \"stedstatus\": \"aktiv\"
    },
    {
      \"fylker\": [
        {
          \"fylkesnavn\": \"Agder\",
          \"fylkesnummer\": \"42\"
        }
      ],
      \"geojson\": {
        \"geometry\": {
          \"coordinates\": [
            [
              8.82918,
              58.61765
            ],
            [
              8.82047,
              58.61746
            ],
            [
              8.82048,
              58.61751
            ]
          ],
          \"type\": \"MultiPoint\"
        }
      },
      \"kommuner\": [
        {
          \"kommunenavn\": \"Tvedestrand\",
          \"kommunenummer\": \"4213\"
        }
      ],
      \"navneobjekttype\": \"Bruk\",
      \"oppdateringsdato\": \"2020-02-14T22:59:59\",
      \"representasjonspunkt\": {
        \"nord\": 58.61765,
        \"øst\": 8.82918
      },
      \"stedsnavn\": [
        {
          \"navnestatus\": \"hovednavn\",
          \"skrivemåte\": \"Marendal\",
          \"skrivemåtestatus\": \"godkjent og prioritert\",
          \"språk\": \"Norsk\",
          \"stedsnavnnummer\": 1
        }
      ],
      \"stedsnummer\": 486802,
      \"stedstatus\": \"aktiv\"
    },
    {
      \"fylker\": [
        {
          \"fylkesnavn\": \"Nordland\",
          \"fylkesnummer\": \"18\"
        }
      ],
      \"geojson\": {
        \"geometry\": {
          \"coordinates\": [
            15.99538,
            68.43864
          ],
          \"type\": \"Point\"
        }
      },
      \"kommuner\": [
        {
          \"kommunenavn\": \"Lødingen\",
          \"kommunenummer\": \"1851\"
        }
      ],
      \"navneobjekttype\": \"Elv\",
      \"oppdateringsdato\": \"2020-02-14T23:12:12\",
      \"representasjonspunkt\": {
        \"nord\": 68.43864,
        \"øst\": 15.99538
      },
      \"stedsnavn\": [
        {
          \"navnestatus\": \"hovednavn\",
          \"skrivemåte\": \"Arendalselva\",
          \"skrivemåtestatus\": \"godkjent og prioritert\",
          \"språk\": \"Norsk\",
          \"stedsnavnnummer\": 1
        }
      ],
      \"stedsnummer\": 692752,
      \"stedstatus\": \"aktiv\"
    },
    {
      \"fylker\": [
        {
          \"fylkesnavn\": \"Finnmark - Finnmárku - Finmarkku\",
          \"fylkesnummer\": \"56\"
        }
      ],
      \"geojson\": {
        \"geometry\": {
          \"coordinates\": [
            21.2558,
            70.24197
          ],
          \"type\": \"Point\"
        }
      },
      \"kommuner\": [
        {
          \"kommunenavn\": \"Loppa\",
          \"kommunenummer\": \"5614\"
        }
      ],
      \"navneobjekttype\": \"Dal\",
      \"oppdateringsdato\": \"2023-12-29T10:30:30\",
      \"representasjonspunkt\": {
        \"nord\": 70.24197,
        \"øst\": 21.2558
      },
      \"stedsnavn\": [
        {
          \"navnestatus\": \"hovednavn\",
          \"skrivemåte\": \"Arndal\",
          \"skrivemåtestatus\": \"godkjent og prioritert\",
          \"språk\": \"Norsk\",
          \"stedsnavnnummer\": 1
        }
      ],
      \"stedsnummer\": 288188,
      \"stedstatus\": \"aktiv\"
    },
    {
      \"fylker\": [
        {
          \"fylkesnavn\": \"Agder\",
          \"fylkesnummer\": \"42\"
        }
      ],
      \"geojson\": {
        \"geometry\": {
          \"coordinates\": [
            8.0173,
            58.39892
          ],
          \"type\": \"Point\"
        }
      },
      \"kommuner\": [
        {
          \"kommunenavn\": \"Iveland\",
          \"kommunenummer\": \"4218\"
        }
      ],
      \"navneobjekttype\": \"Myr\",
      \"oppdateringsdato\": \"2020-02-14T23:33:33\",
      \"representasjonspunkt\": {
        \"nord\": 58.39892,
        \"øst\": 8.0173
      },
      \"stedsnavn\": [
        {
          \"navnestatus\": \"hovednavn\",
          \"skrivemåte\": \"Arendalsmyra\",
          \"skrivemåtestatus\": \"godkjent og prioritert\",
          \"språk\": \"Norsk\",
          \"stedsnavnnummer\": 1
        }
      ],
      \"stedsnummer\": 212957,
      \"stedstatus\": \"aktiv\"
    },
    {
      \"fylker\": [
        {
          \"fylkesnavn\": \"Agder\",
          \"fylkesnummer\": \"42\"
        }
      ],
      \"geojson\": {
        \"geometry\": {
          \"coordinates\": [
            8.77358,
            58.45628
          ],
          \"type\": \"Point\"
        }
      },
      \"kommuner\": [
        {
          \"kommunenavn\": \"Arendal\",
          \"kommunenummer\": \"4203\"
        }
      ],
      \"navneobjekttype\": \"Grunne i sjø\",
      \"oppdateringsdato\": \"2020-02-14T22:53:53\",
      \"representasjonspunkt\": {
        \"nord\": 58.45628,
        \"øst\": 8.77358
      },
      \"stedsnavn\": [
        {
          \"navnestatus\": \"hovednavn\",
          \"skrivemåte\": \"Arendalsbåen\",
          \"skrivemåtestatus\": \"godkjent og prioritert\",
          \"språk\": \"Norsk\",
          \"stedsnavnnummer\": 1
        }
      ],
      \"stedsnummer\": 299781,
      \"stedstatus\": \"aktiv\"
    },
    {
      \"fylker\": [
        {
          \"fylkesnavn\": \"Nordland\",
          \"fylkesnummer\": \"18\"
        }
      ],
      \"geojson\": {
        \"geometry\": {
          \"coordinates\": [
            13.76085,
            66.80328
          ],
          \"type\": \"Point\"
        }
      },
      \"kommuner\": [
        {
          \"kommunenavn\": \"Meløy\",
          \"kommunenummer\": \"1837\"
        }
      ],
      \"navneobjekttype\": \"Bruk\",
      \"oppdateringsdato\": \"2024-11-27T11:06:06\",
      \"representasjonspunkt\": {
        \"nord\": 66.80328,
        \"øst\": 13.76085
      },
      \"stedsnavn\": [
        {
          \"navnestatus\": \"hovednavn\",
          \"skrivemåte\": \"Rendal\",
          \"skrivemåtestatus\": \"vedtatt\",
          \"språk\": \"Norsk\",
          \"stedsnavnnummer\": 1
        }
      ],
      \"stedsnummer\": 981221,
      \"stedstatus\": \"aktiv\"
    }
  ]
}";

        let upstream_response: ReturSted = match serde_json::from_str(stedsnavn_api_response_json) {
            Ok(r) => r,
            Err(e) => {
                panic!(
                    "Test failed while deserializing json to ReturSted, error: {}",
                    e
                );
            }
        };

        let mut mock_places_api = MockDefaultApi::new();
        mock_places_api
            .expect_sted_get()
            .returning(move |_, _, _, _, _, _, _, _, _, _, _, _| Ok(upstream_response.clone()));

        let actual_response = get_places("Arendal".to_string(), None, &mock_places_api)
            .await
            .unwrap();

        let one = Place {
            point: Some(Coordinate {
                longitude: 8.76695,
                latitude: 58.46121,
            }),
            name: "Arendal".to_string(),
            municipality: Some("Arendal".to_string()),
        };
        let two = Place {
            point: Some(Coordinate {
                longitude: 8.76503,
                latitude: 58.46044,
            }),
            name: "Arendal kommune".to_string(),
            municipality: Some("Arendal".to_string()),
        };
        let last = Place {
            point: Some(Coordinate {
                longitude: 13.76085,
                latitude: 66.80328,
            }),
            name: "Rendal".to_string(),
            municipality: Some("Meløy".to_string()),
        };

        assert_eq!(one, actual_response[0]);
        assert_eq!(two, actual_response[1]);
        assert_eq!(Some(&last), actual_response.last());
    }
}
