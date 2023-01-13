pub mod v1;

use std::str::FromStr;

use actix_web::{web, App, HttpServer, Responder};
use aptos_sdk::{self, move_types::account_address::AccountAddress};
use aws_sdk_s3 as s3;
use s3::Region;
use url::Url;

async fn hello(name: web::Path<String>) -> impl Responder {
    // Aptos
    let u = Url::from_str("https://fullnode.devnet.aptoslabs.com").unwrap();
    let c = aptos_sdk::rest_client::Client::new(u);
    let c_coin = aptos_sdk::coin_client::CoinClient::new(&c);
    let n = name.into_inner().clone();
    let addr = AccountAddress::from_str(n.as_str()).unwrap();
    let amount = c_coin.get_account_balance(&addr).await.unwrap() as f64 / 1e8;
    let mut ret = format!("Aptos: {n}: {amount} APT");

    // AWS S3
    let regin = Region::new("ap-southeast-1");
    let config = aws_config::from_env().region(regin).load().await;
    let client = s3::Client::new(&config);
    let bucket_name = "paclub-s3-dev";
    let file_key = "banner/20474c18c90c26b91a430230cd96cc02.jpg";
    let resp = client.get_object().bucket(bucket_name).key(file_key).send().await;
    ret += &format!("\nAWS S3: {file_key}: ");
    match resp {
        Ok(x) => {
            ret += &format!(" type: {}", x.content_type().unwrap());
            ret += &format!(" len: {}", x.content_length());
        },
        Err(e) => println!("{:?}", &e),
    }

    ret
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    HttpServer::new(|| App::new()
        .route("/hello/{name}", web::get().to(hello))
        .route("/doc/rcj", web::get().to(v1::doc_rcj::get))
        .route("/floor/{collection_id}", web::get().to(v1::floor::get))
        .route("/floor/", web::post().to(v1::floor::post))
        ).bind(("0.0.0.0", 8080))?.run().await
}
