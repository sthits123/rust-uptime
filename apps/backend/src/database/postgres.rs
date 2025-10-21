use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::time::Duration;
use dotenvy::var;

#[derive(Clone)]
pub struct ReadOnlyPgPool(pub PgPool);

pub async fn connect_all() -> Result<(PgPool, ReadOnlyPgPool), sqlx::Error> {
    let database_url = var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = PgPoolOptions::new()
        .min_connections(3)
        .max_connections(20)
        .connect(&database_url)
        .await?;

    let ro_pool = if let Ok(url) = var("READONLY_DATABASE_URL") {
        PgPoolOptions::new()
            .min_connections(var("READONLY_DATABASE_MIN_CONNECTIONS").ok().and_then(|x| x.parse().ok()).unwrap_or(0))
            .max_connections(var("READONLY_DATABASE_MAX_CONNECTIONS").ok().and_then(|x| x.parse().ok()).unwrap_or(1))
            .max_lifetime(Some(Duration::from_secs(3600)))
            .connect(&url)
            .await?
    } else {
        pool.clone()
    };

    Ok((pool, ReadOnlyPgPool(ro_pool)))
}

