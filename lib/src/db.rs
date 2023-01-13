use sqlx::{MySql, MySqlPool, Pool};
use std::env;
use std::error::Error;

pub async fn conn_mysql() -> Result<Pool<MySql>, Box<dyn Error>> {
    let db_url = env::var("PB_DATABASE_URL")?;
    let pool = MySqlPool::connect(&db_url).await?;
    Ok(pool)
}
