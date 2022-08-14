use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct IdResponse {
    #[serde(rename = "10")] 
    id: GameInfoResponse,
}

#[derive(Serialize, Deserialize, Debug)]
struct GameInfoResponse {
    success: bool,
    data: GameInfo,
}

#[derive(Serialize, Deserialize, Debug)]
struct GameInfo {
    #[serde(rename = "type")] 
    app_type: String,
    name: String,
    steam_appid: u32,
    is_free: bool,
    detailed_description: String,
    about_the_game: String,
    short_description: String,
    developers: Vec<String>,
    publishers: Vec<String>,
    price_overview: PriceOverview,
}

#[derive(Serialize, Deserialize, Debug)]
struct PriceOverview {
    currency: String,
    initial: u32,
    #[serde(rename = "final")] 
    final_price: u32,
    discount_percent: u8,
    initial_formatted: String,
    final_formatted: String,
}

pub async fn get_game_info(id: i32) -> Result<()> {
    let url = format!(
        "https://store.steampowered.com/api/appdetails?appids={}",
        id
    );

    let client = reqwest::Client::new();
    let request = client
        .get(url)
        .header(reqwest::header::ACCEPT, "application/x-www-form-urlencoded")
        .send()
        .await?
        .json::<IdResponse>()
        .await?;

    let response = &request.id.data;
    println!("{}", response.name);

    Ok(())

}
