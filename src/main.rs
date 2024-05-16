use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use envy;
use md5;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use url::Url;

#[derive(Deserialize, Clone)]
struct EnvConfig {
    private_key: String,
    uid: String,
    valid_duration: u64,
}

#[derive(Deserialize)]
struct SignInfo {
    origin_url: String,
}

#[derive(Serialize)]
struct SignedUrl {
    signed_url: String,
}

async fn sign_url(config: web::Data<EnvConfig>, info: web::Json<SignInfo>) -> impl Responder {
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let expire_time = current_time + config.valid_duration / 1000;
    let random_int: u32 = rand::thread_rng().gen_range(0..1_000_000);
    let url = Url::parse(&info.origin_url).expect("Failed to parse URL");

    let auth_key = format!(
        "{}-{}-{}-{:?}",
        expire_time,
        random_int,
        config.uid,
        md5::compute(format!(
            "{}-{}-{}-{}",
            url.path(),
            expire_time,
            random_int,
            config.private_key
        ))
    );

    let mut url = url.clone();
    url.query_pairs_mut().append_pair("auth_key", &auth_key);

    HttpResponse::Ok().json(SignedUrl {
        signed_url: url.to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config: EnvConfig = envy::from_env().expect("Failed to load env vars");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .route("/", web::post().to(sign_url))
    })
    .bind("127.0.0.1:7878")?
    .run()
    .await
}
