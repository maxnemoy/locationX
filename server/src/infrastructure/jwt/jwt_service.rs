use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use std::env;
use crate::domain::entities::TokenClaims;
use uuid::Uuid;

#[derive(Clone)]
pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtService {
    pub fn new() -> Self {
        let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret_key".to_string());
        
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
        }
    }

    pub fn generate_access_token(&self, user_id: Uuid, email: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let now = Utc::now();
        let iat = now.timestamp() as usize;
        let exp = (now + Duration::minutes(15)).timestamp() as usize; 

        let claims = TokenClaims {
            sub: user_id.to_string(),
            email: email.to_string(),
            exp,
            iat,
            token_type: "access".to_string(),
        };

        encode(&Header::default(), &claims, &self.encoding_key)
    }

    pub fn generate_refresh_token(&self, user_id: Uuid, email: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let now = Utc::now();
        let iat = now.timestamp() as usize;
        let exp = (now + Duration::days(30)).timestamp() as usize; // 30 дней

        let claims = TokenClaims {
            sub: user_id.to_string(),
            email: email.to_string(),
            exp,
            iat,
            token_type: "refresh".to_string(),
        };

        encode(&Header::default(), &claims, &self.encoding_key)
    }

    pub fn verify_token(&self, token: &str) -> Result<TokenData<TokenClaims>, jsonwebtoken::errors::Error> {
        decode::<TokenClaims>(token, &self.decoding_key, &Validation::default())
    }

    pub fn get_user_id_from_token(&self, token: &str) -> Result<Uuid, Box<dyn std::error::Error>> {
        let token_data = self.verify_token(token)?;
        let user_id = Uuid::parse_str(&token_data.claims.sub)?;
        Ok(user_id)
    }
}
