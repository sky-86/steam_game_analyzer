use crate::SteamInfo;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct PlayerServiceRequest {
    steamid: u64,
    include_appinfo: bool,
    include_played_free_games: bool,
    appids_filter: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct PlayerServiceResponse {
    response: Games,
}

#[derive(Serialize, Deserialize, Debug)]
struct Games {
    game_count: u64,
    games: Vec<Game>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Game {
    appid: u64,
    playtime_forever: u32,
    playtime_linux_forever: u32,
    playtime_mac_forever: u32,
    playtime_windows_forever: u32,
    rtime_last_played: u64,
}

pub async fn get_owned_games_ids() -> Result<Vec<u64>> {
    let info = SteamInfo::new();

    let player_service = serde_json::to_string(&PlayerServiceRequest {
        steamid: info.steamid,
        include_appinfo: false,
        include_played_free_games: true,
        appids_filter: 0,
    })
    .unwrap();

    let url = format!(
        "https://api.steampowered.com/IPlayerService/GetOwnedGames/v1/?key={}&input_json={}",
        info.api_key, player_service
    );

    let client = reqwest::Client::new();
    let request = client
        .get(url)
        .header(reqwest::header::ACCEPT, "application/x-www-form-urlencoded")
        .send()
        .await?
        .json::<PlayerServiceResponse>()
        .await?;

    let mut ids = Vec::new();
    let response = &request.response;
    for game in response.games.iter() {
        ids.push(game.appid);
    }

    Ok(ids)
}
