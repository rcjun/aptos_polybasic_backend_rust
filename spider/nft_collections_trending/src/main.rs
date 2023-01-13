// 获取排名前N条记录
const SPIDER_NFT_COLLECTIONS_TRENDING_NUM: u32 = 100;

use lib::db::conn_mysql;
use rust_decimal::Decimal;
use serde::Deserialize;
use std::env;
use std::error::Error;

#[derive(Debug, Deserialize, sqlx::FromRow)]
struct NftCollectionsTrending {
    collection_id: Option<String>,
    slug: Option<String>,
    num_tokens: Option<Decimal>,
    creator: Option<String>,
    name: Option<String>,
    description: Option<String>,
    max_amount: Option<Decimal>,
    uri: Option<String>,
    logo_uri: Option<String>,
    verified: Option<bool>,
    total_volume: Option<Decimal>,
    floor: Option<Decimal>,
    num_unique_owners: Option<Decimal>,
    volume_24: Option<Decimal>,
    volume_48: Option<Decimal>,
    floor_24: Option<Decimal>,
}

#[derive(Deserialize)]
struct Res<T> {
    data: T,
}

async fn sync_tops(start: u32, to: u32) -> Result<u64, Box<dyn Error>> {
    let url = format!("https://api-v1.topaz.so/api/explore-collections?from={start}&to={to}");
    let c = reqwest::Client::new();
    let resp = c.get(url).send().await?;
    let data = resp.json::<Res<Vec<NftCollectionsTrending>>>().await?;

    let pool = conn_mysql().await?;
    sqlx::migrate!().run(&pool).await?;
    let mut tx = pool.begin().await?;
    sqlx::query!("truncate spider_nft_collections_trending")
        .execute(&mut tx)
        .await?;
    let mut qb = sqlx::QueryBuilder::new(
        r#"
        insert into spider_nft_collections_trending (
            collection_id,
            slug,
            num_tokens,
            creator,
            name,
            description,
            max_amount,
            uri,
            logo_uri,
            verified,
            total_volume,
            floor,
            num_unique_owners,
            volume_24,
            volume_48,
            floor_24
        )
    "#,
    );
    qb.push_values(data.data.iter(), |mut q, i| {
        q.push_bind(i.collection_id.clone())
            .push_bind(i.slug.clone())
            .push_bind(i.num_tokens.clone())
            .push_bind(i.creator.clone())
            .push_bind(i.name.clone())
            .push_bind(i.description.clone())
            .push_bind(i.max_amount.clone())
            .push_bind(i.uri.clone())
            .push_bind(i.logo_uri.clone())
            .push_bind(i.verified)
            .push_bind(i.total_volume.clone())
            .push_bind(i.floor.clone())
            .push_bind(i.num_unique_owners.clone())
            .push_bind(i.volume_24.clone())
            .push_bind(i.volume_48.clone())
            .push_bind(i.floor_24.clone());
    });
    let res = qb.build().execute(&mut tx).await?;
    tx.commit().await?;

    Ok(res.rows_affected())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    let num = match env::var("SPIDER_NFT_COLLECTIONS_TRENDING_NUM") {
        Ok(x) => x.parse::<u32>()?,
        _ => SPIDER_NFT_COLLECTIONS_TRENDING_NUM,
    };
    let n = sync_tops(0, num - 1).await?;
    println!("Successfully updated. rows_affected: {n}");
    Ok(())
}
