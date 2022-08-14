pub mod get_owned_games;
pub mod get_game_reviews;
pub mod get_game_info;

// prelude
pub use get_owned_games::get_owned_games_ids;
pub use get_game_reviews::get_game_review;
pub use get_game_info::get_game_info;

#[derive(Default)]
pub struct SteamInfo {
    api_key: String,
    steamid: u64,
}

impl SteamInfo {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        let api_key = std::env::var("KEY").unwrap();
        let steamid = std::env::var("ID").unwrap().parse::<u64>().unwrap();
        Self {
            api_key,
            steamid,
        }
    }
}
