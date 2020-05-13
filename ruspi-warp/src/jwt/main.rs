use argon2::{self, Config};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, errors, Header, Validation};
use rand::Rng;
use serde::{Deserialize, Serialize};

pub fn hash(password: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}

pub fn verify(hash: &str, password: &[u8]) -> bool {
    argon2::verify_encoded(hash, password).unwrap_or(false)
}

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
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    ).map(|token_data| token_data.claims)
}

pub fn login(username: String, password: String, jwt_secret: String) -> String {
    let hashed_password = "$argon2i$v=19$m=4096,t=3,p=1$wBNL0yEpl0+lx5jp28bwsYwQVJ5oMBVm8Yi5WFXtguo$Cz0CHGSf8rzg9y/Rq6pMOzT5o82UZqIixyPyKf2R54A";

    if username != "admin".to_string() {
        panic!("Invalid username");
    }

    if !verify(hashed_password, password.as_bytes()) {
        panic!("Invalid password");
    }

    let rights = vec!["admin".to_string(), "user".to_string()];
    let token = encode_token(&jwt_secret, username, rights);

    token
}


fn main() {
    let jwt_secret = String::from("d302886ecca83222a392f6549e520dacbd95dc65f1a303f8c571ca3bb9b8e196");

    let username = String::from("admin");
    let password = String::from("Admin!");

    let token = login(username, password, jwt_secret.clone());

    println!("Token: {}", token);

    let claims = decode_token(&jwt_secret, &token).unwrap();

    println!("Claims: {:?}", claims);
}