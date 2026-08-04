use crate::database::connect;
use sqlx::query;

pub async fn pacman_counts(userid: u64) -> i64 {
    let conn = &connect().await;
    if let Ok(conn) = conn {
        let result = query!("SELECT * FROM pacman WHERE userid = ?", userid.to_string())
            .fetch_one(conn)
            .await;
        if result.is_err() {
            return 0;
        }

        return result.unwrap().packages;
    }

    0
}
