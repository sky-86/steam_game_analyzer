use anyhow::Result;
use sqlx::query;
use sqlx::{postgres::PgConnection, Connection};
use steam_game_analyzer::*;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let url = std::env::var("DATABASE_URL").unwrap();
    let mut conn = PgConnection::connect(&url).await?;

    let ids = get_owned_games_ids().await?;
    for id in ids {
        let q = query!("SELECT name FROM games WHERE appid=$1", id as i32)
            .fetch_all(&mut conn)
            .await?;
        println!("{:?}", q);
    }

    //get_game_review(10).await?;

    Ok(())
}
