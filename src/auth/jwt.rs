use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use poem::http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub email: String,
    pub role_id: i32,
    pub exp: usize,
}

pub fn generate_token(claims: Claims, jwt_secret_key: [u8; 32]) -> Result<String, StatusCode> {
    match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(&jwt_secret_key),
    ) {
        Ok(tok) => Ok(tok),
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}

pub fn validate_creds(
    token: &str,
    email: Option<&str>,
    role_id: Option<i32>,
    key: [u8; 32],
) -> Result<(), StatusCode> {
    let claims = decode_token(token, key)?;
    let email_matched = match email {
        Some(val) => val == claims.email,
        None => true,
    };
    let not_expired = claims.exp > Utc::now().timestamp() as usize;
    let correct_role = match role_id {
        Some(val) => val == claims.role_id,
        None => true,
    };
    match email_matched && not_expired && correct_role {
        true => Ok(()),
        false => Err(StatusCode::UNAUTHORIZED),
    }
}

pub fn decode_token(token: &str, key: [u8; 32]) -> Result<Claims, StatusCode> {
    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(&key),
        &Validation::default(),
    ) {
        Ok(val) => Ok(val.claims),
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}
