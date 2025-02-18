use crate::dto::UserInfo;
use jsonwebtoken::{decode, Algorithm, Validation};
use anyhow::Result;
use std::time::{SystemTime, UNIX_EPOCH};



impl UserInfo {
    pub fn from_header_str(key: &[u8], header: &str) -> Result<Self> {
        let token = header.strip_prefix("Bearer ").unwrap_or(header);
        let decoding_key = jsonwebtoken::DecodingKey::from_secret(key);
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;  // Enable expiration time validation
        
        let token_data = match decode::<UserInfo>(&token, &decoding_key, &validation) {
            Ok(data) => data,
            Err(e) => return Err(e.into()),
        };

		let mut claims = token_data.claims;

		claims.last_login = None;
		claims.exp = None;
		claims.iat = None;


        Ok(claims)
    }
}

pub fn new_token(user: UserInfo, key: &[u8], exp: u64) -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    
    let user = UserInfo {
        iat: Some(now),
        exp: Some(now + exp as i64),
        ..user
    };

    let encoding_key = jsonwebtoken::EncodingKey::from_secret(key);
    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &user,
        &encoding_key
    ).unwrap()
}