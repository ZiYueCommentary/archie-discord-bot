use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{raw_sql, AssertSqlSafe, Error, Pool, Sqlite, SqlitePool};
use std::fs::read_to_string;

pub async fn init() -> Result<(), Error> {
    let query = read_to_string("assets/init.sql")?;
    let assert_safe_query = AssertSqlSafe(query.to_owned());
    raw_sql(assert_safe_query).execute(&connect().await).await?;

    Ok(())
}

pub async fn connect() -> Pool<Sqlite> {
    let options = SqliteConnectOptions::new()
        .filename("database.sqlite")
        .create_if_missing(true);
    let conn = SqlitePool::connect_with(options).await;
    match conn {
        Ok(sql) => sql,
        Err(err) => panic!("cannot connect to database `database.sqlite`: {}", err),
    }
}
