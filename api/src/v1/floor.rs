use std::collections::HashMap;

use actix_web::{web, Responder};
use rust_decimal::Decimal;
use serde::Deserialize;
use sqlx::FromRow;

async fn get_topaz(collection_id: String) -> Option<String> {
    let url = "https://api-v1.topaz.so/api/floor";
    let mut params = HashMap::new();
    params.insert("collection_id", collection_id);
    let client = reqwest::Client::new();
    let resp = client.post(url).json(&params).send().await.unwrap();

    let json = resp.json::<serde_json::Value>().await.unwrap();
    if json["status"].as_i64().unwrap() == 200 {
        let floor = json["data"]["floor"].as_u64().unwrap();
        Some(floor.to_string())
    } else {
        None
    }
}

async fn _get(collection_id: String) -> String {
    let mut is_cache = false;

    // 先检查爬虫缓存数据
    let pool = lib::db::conn_mysql().await.unwrap();
    let sql = "select * from spider_nft_collections_trending where collection_id=?";

    #[derive(Debug, Deserialize, FromRow)]
    struct Row {
        floor: Decimal,
    }

    let res: Result<Row, sqlx::Error> = sqlx::query_as(sql)
        .bind(&collection_id)
        .fetch_one(&pool)
        .await;
    let mut floor = match res {
        Ok(row) => {
            is_cache = true;
            Some(row.floor.to_string())
        },
        _ => None,
    };

    // 请求最新数据
    if floor.is_none() {
        floor = get_topaz(collection_id).await;
    }

    serde_json::json!({
        "code": 0,
        "msg": "OK",
        "result": {
            "is_cache": is_cache,
            "floor": floor,
        }
    })
    .to_string()
}

pub async fn get(collection_id: web::Path<String>) -> impl Responder {
    _get(collection_id.clone()).await
}

pub async fn post(collection_id: web::Json<serde_json::Value>) -> impl Responder {
    let cid = collection_id["collection_id"].as_str().unwrap().to_string();
    _get(cid).await
}
