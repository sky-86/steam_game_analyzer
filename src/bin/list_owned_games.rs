use anyhow::Result;
use steam_game_analyzer::*;

#[tokio::main]
async fn main() -> Result<()> {
    // let ids = get_owned_games_ids().await?;
    // for id in ids {
    //     println!("{}", id);
    // }

    //get_game_review(10).await?;
    get_game_info(10).await?;

    Ok(())
}
