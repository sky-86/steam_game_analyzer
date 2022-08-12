use crate::SteamInfo;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct StoreServiceRequest {
    max_results: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct StoreServiceResponse {
    response: Apps,
}

#[derive(Serialize, Deserialize, Debug)]
struct Apps {
    apps: Vec<App>,
    have_more_results: bool,
    last_appid: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct App {
    appid: u64,
    name: String,
    last_modified: u64,
    price_change_number: u64,
}

pub async fn get_game_info() -> Result<()> {
    let info = SteamInfo::new();
    let store_request = serde_json::to_string(&StoreServiceRequest {
        max_results: 3,
    }).unwrap();

    let url = format!(
        "https://api.steampowered.com/IStoreService/GetAppList/v1/?key={}&input_json={}",
        info.api_key, store_request
    );

    let client = reqwest::Client::new();
    let request = client
        .get(url)
        .header(reqwest::header::ACCEPT, "application/x-www-form-urlencoded")
        .send()
        .await?
        .json::<StoreServiceResponse>()
        .await?;

    let response = &request.response;
    for app in response.apps.iter() {
        println!("{}", app.name);
    }

    Ok(())
}
