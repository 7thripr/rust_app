use axum::http::StatusCode;
use jsonwebtoken::{Header, encode, EncodingKey, TokenData, decode, DecodingKey, Validation};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use serde_json::de;
use tracing_subscriber::util;

use crate::utils;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub iat: usize,
    pub exp: usize,
    pub email: String,
}

pub fn encode_token(email: String) -> Result<String, StatusCode>{

    let now = Utc::now();
    let exp_dur = Duration::hours(24);
    
    let claim = Claims {
        iat: now.timestamp() as usize,
        exp: (now + exp_dur).timestamp() as usize,
        email: email,
    };

    let secret = (*utils::consts::TOKEN).clone();
    return encode(&Header::default(), &claim, &EncodingKey::from_secret(secret.as_ref()))
    .map_err(|_| { StatusCode::INTERNAL_SERVER_ERROR });
}

pub fn decode_token(jwt: String) ->Result<TokenData<Claims>, StatusCode>{
    let secret = (*utils::consts::TOKEN).clone();
    let result = decode(&jwt, &DecodingKey::from_secret(secret.as_ref()), &Validation::default())
    .map_err(|_| { StatusCode::INTERNAL_SERVER_ERROR });

    return result;

}