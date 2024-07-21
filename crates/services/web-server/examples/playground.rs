use std::{thread, time};

use jsonwebtoken::{
    decode, encode, get_current_timestamp, DecodingKey, EncodingKey, Header, Validation,
};
use lib_surrealdb::{
    ctx::Ctx,
    model::{
        users::{bmc::UsersBmc, UsersGet},
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
    let mm = ModelManager::new().await.unwrap();
    let sub = String::from("iR1f8i7Wg7jipR3uhDhJ");
    let ctx = Ctx::root_ctx();
    let user = UsersBmc::first_by_id::<UsersGet>(&ctx, &mm, sub.as_str())
        .await
        .unwrap()
        .unwrap();
    println!("{:?}", user);
    let token_salt = user.token_salt.to_string();
    println!("{token_salt}");
    let current_time = get_current_timestamp() as usize;
    println!("{}", current_time);
    let exp: usize = current_time;
    let claim = Claims { sub, exp };
    let token = encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(token_salt.as_ref()),
    )
    .unwrap();
    println!("encoding");
    println!("{token}");

    // let sleep_duration = time::Duration::from_secs(61);
    // thread::sleep(sleep_duration);

    let token = decode::<Claims>(
        &token,
        &DecodingKey::from_secret("36ee060e-20a7-4a42-8bd1-0cbd704a29a2".as_ref()),
        &Validation::default(),
    )
    .unwrap();

    let claims = token.claims;

    println!("decoding");
    let current_time = get_current_timestamp() as usize;
    println!("{}", current_time);
    println!("{claims:?}");
}
