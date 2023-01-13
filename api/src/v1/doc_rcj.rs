use actix_web::Responder;


pub async fn get() -> impl Responder {
r###"

API v1 说明文档

地板价
GET floor/:collection_id:
POST floor/

POST请求参数用JSON形式
collection_id: 集合ID

例子：
GET floor/0xb915216799ca659c9dff2c891049d5da0291e71d0c530c3595af68cf0f57ea0a::Aptos Skull
返回：
{"code":0,"msg":"OK","result":{
"is_cache":false,   // 是否缓存的数据
"floor":"11000000"   // 地板价
}}

"###
}
