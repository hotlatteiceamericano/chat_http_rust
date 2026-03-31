use chat_common::claim::Claims;
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};

pub fn create_token(email: &str, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let exp = now + Duration::days(7);

    let claims = Claims {
        sub: email.to_string(),
        iat: now.timestamp() as usize,
        exp: exp.timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}
