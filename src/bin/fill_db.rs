use anyhow::Result;
use sqlx::{postgres::PgConnection, Connection};
use sqlx::query;
use steam_game_analyzer::fill_game_db;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let url = std::env::var("DATABASE_URL").unwrap();
    let mut conn = PgConnection::connect(&url).await?;

    let data = fill_game_db().await?;

    for (i, game) in data.applist.apps.iter().enumerate() {
        let _ = query!(
            "INSERT INTO games (appid,name) VALUES ($1,$2) RETURNING id",
            game.appid as i64,
            game.name
        ).fetch_all(&mut conn).await?;
        
        println!("{}", i);
    }

    Ok(())
}
