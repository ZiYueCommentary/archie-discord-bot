use crate::database::connect;
use sqlx::query;

pub async fn pacman_counts(userid: u64) -> i64 {
    let result = query!("SELECT * FROM pacman WHERE userid = ?", userid.to_string())
        .fetch_one(&connect().await)
        .await;
    if result.is_err() {
        return 0;
    }
    result.unwrap().packages
}
