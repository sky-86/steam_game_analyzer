use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AppListResponse {
    pub applist: Apps,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Apps {
    pub apps: Vec<App>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct App {
    pub appid: u32,
    pub name: String,
}

pub async fn fill_game_db() -> Result<AppListResponse> {
    let url = "https://api.steampowered.com/ISteamApps/GetAppList/v2/";

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(reqwest::header::ACCEPT, "application/x-www-form-urlencoded")
        .send()
        .await?
        .json::<AppListResponse>()
        .await?;

    Ok(response)

}
