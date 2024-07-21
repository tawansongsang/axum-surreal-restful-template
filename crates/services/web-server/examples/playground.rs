use std::{thread, time};

use jsonwebtoken::{
    decode, encode, get_current_timestamp, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

fn main() {
    let sub = String::from("iR1f8i7Wg7jipR3uhDhJ");
    let current_time = get_current_timestamp() as usize;
    println!("{}", current_time);
    let exp: usize = current_time;
    let claim = Claims { sub, exp };
    let token = encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret("36ee060e-20a7-4a42-8bd1-0cbd704a29a2".as_ref()),
    )
    .unwrap();
    println!("encoding");
    println!("{token}");

    let sleep_duration = time::Duration::from_secs(61);
    thread::sleep(sleep_duration);

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
