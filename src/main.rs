struct Position {
    lat: String,
    lon: String,
}

use location_forecast_client::apis::configuration::Configuration;
use location_forecast_client::apis::data_api::compact_get;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let lat = if let Some(lat) = std::env::args().nth(1) {
        lat.parse::<f32>().unwrap()
    } else {
        println!("No lat provided, using default for Arendal.");
        58.4618
    };

    let long = if let Some(long) = std::env::args().nth(2) {
        long.parse::<f32>().unwrap()
    } else {
        println!("No long provided, using default for Arendal.");
        8.7724
    };

    let mut conf = Configuration::new();
    conf.user_agent = Some("fredfull.no post@fredfull.no".into());
    let result = compact_get(&conf, lat, long, None).await;
    let forecast = result.unwrap().properties.timeseries;
    println!("{:?}", forecast);

    //     let params = [("lat", lat), ("lon", long)];
    //     let get_request = client.get(url).query(&params);
    //     println!("{get_request:?}");
    //
    //     let res = get_request.send().await?;
    //
    //
    //
    //     eprintln!("Request URL: {}", res.url());
    //     eprintln!("Response: {:?} {}", res.version(), res.status());
    //     eprintln!("Headers: {:#?}\n", res.headers());
    //
    //     let body = res.text().await?;
    // println!("{body}");

    Ok(())
}
