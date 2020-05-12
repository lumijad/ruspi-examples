use argon2::{self, Config};
use rand::Rng;
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, errors, Header, Validation};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};

pub fn hash(password: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}

pub fn verify(hash: &str, password: &[u8]) -> bool {
    argon2::verify_encoded(hash, password).unwrap_or(false)
}

pub const TOKEN_PREFIX: &str = "Bearer ";

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Claims {
    sub: String,
    exp: u64,
    // seconds since the epoch
    auth: Vec<String>,
}

impl Claims {
    fn new(user_id: String, auth: Vec<String>) -> Self {
        Self {
            sub: user_id,
            exp: (Utc::now() + Duration::hours(12)).timestamp() as u64,
            auth,
        }
    }

    pub fn user_id(&self) -> String {
        self.sub.to_string()
    }
}

pub fn encode_token(secret: &str, sub: String, auth: Vec<String>) -> String {
    encode(
        &Header::default(),
        &Claims::new(sub, auth),
        &EncodingKey::from_secret(secret.as_ref()),
    ).unwrap()
}

pub fn decode_token(secret: &str, token: &str) -> errors::Result<Claims> {
    decode::<Claims>(
        token.trim_start_matches(TOKEN_PREFIX),
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    ).map(|token_data| token_data.claims)
}



fn main() {
}