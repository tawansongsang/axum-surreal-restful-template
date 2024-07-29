use std::io;

use lib_auth::jsonwebtoken::{
    decode, decode_header, encode, get_current_timestamp, DecodingKey, EncodingKey, Header,
    Validation,
};
use lib_surrealdb::{
    ctx::Ctx,
    model::{
        users::{bmc::UsersBmc, Users},
        ModelManager,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[tokio::main]
async fn main() {
    let mut sub = String::new();
    io::stdin().read_line(&mut sub).unwrap();
    let sub = sub.trim_end().to_string();
    println!("{}.", sub);
    let mm = ModelManager::new().await.unwrap();
    let ctx = Ctx::root_ctx();
    let user = UsersBmc::get::<Users>(&ctx, &mm, sub.as_str())
        .await
        .unwrap()
        .unwrap();
    println!("{:?}", user);
    let token_salt = user.token_salt.to_string();
    println!("{token_salt}");
    let current_time = get_current_timestamp() as usize;
    println!("{}", current_time);
    let exp: usize = current_time;
    let claim = Claims {
        sub: sub.clone(),
        exp,
    };
    let mut headers = Header::default();
    headers.kid = Some(sub);
    let token = encode(
        &headers,
        &claim,
        &EncodingKey::from_secret(token_salt.as_ref()),
    )
    .unwrap();
    println!("encoding");
    println!("{token}");

    // let sleep_duration = time::Duration::from_secs(61);
    // thread::sleep(sleep_duration);
    let headers = decode_header(&token).unwrap();
    println!("{:?}", headers);
    let user_id = headers.kid.unwrap();
    let ctx = Ctx::root_ctx();
    let user = UsersBmc::get::<Users>(&ctx, &mm, user_id.as_str())
        .await
        .unwrap()
        .unwrap();
    let token_salt = user.token_salt.to_string();

    let token = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(token_salt.as_ref()),
        &Validation::default(),
    )
    .unwrap();

    let claims = token.claims;

    println!("decoding");
    let current_time = get_current_timestamp() as usize;
    println!("{}", current_time);
    println!("{claims:?}");
}
