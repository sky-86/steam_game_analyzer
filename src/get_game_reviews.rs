use anyhow::Result;
use serde::{Deserialize, Serialize};

//https://partner.steamgames.com/doc/store/getreviews
#[derive(Serialize, Deserialize, Debug)]
struct GetUserReviewsRequest {
    filter: String,
    language: String,
    day_range: String,
    cursor: String,
    review_type: String,
    purchase_type: String,
    num_per_page: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct GetUserReviewsResponse {
    success: u8,
    query_summary: QuerySummary,
    reviews: Vec<Review>,
    cursor: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct QuerySummary {
    num_reviews: u32,
    review_score: u8,
    review_score_desc: String,
    total_positive: u32,
    total_negative: u32,
    total_reviews: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Review {
    recommendationid: String,
    author: Author,
    language: String,
    review: String,
    timestamp_created: u64,
    timestamp_updated: u64,
    voted_up: bool,
    votes_up: u32,
    votes_funny: u32,
    weighted_vote_score: String, // float
    comment_count: u32,
    steam_purchase: bool,
    received_for_free: bool,
    written_during_early_access: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Author {
    steamid: String,
    num_games_owned: u32,
    num_reviews: u32,
    playtime_forever: u32,
    playtime_last_two_weeks: u32,
    playtime_at_review: u32,
    last_played: u64,
}

pub async fn get_game_review(id: u64) -> Result<()> {
    //let info = SteamInfo::new();

    let request = serde_json::to_string(&GetUserReviewsRequest {
        filter: "all".into(),
        language: "english".into(),
        day_range: "365".into(),
        cursor: "*".into(),
        review_type: "all".into(),
        purchase_type: "steam".into(),
        num_per_page: "20".into(),
    }).unwrap();

    let url = format!(
        "https://store.steampowered.com/appreviews/{}?json=1?input_json={}",
        id, request
    );

    let client = reqwest::Client::new();
    let request = client
        .get(url)
        .header(reqwest::header::ACCEPT, "application/x-www-form-urlencoded")
        .send()
        .await?
        .json::<GetUserReviewsResponse>()
        .await?;

    let response = &request.query_summary;
    println!("{}", response.review_score);

    Ok(())
}
