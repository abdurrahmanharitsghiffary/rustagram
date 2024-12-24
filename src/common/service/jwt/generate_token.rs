use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

use super::claims::Claims;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

pub fn generate_token(sub: String, exp: u64) -> Result<String, jsonwebtoken::errors::Error> {
    let header = Header::new(Algorithm::HS256);

    let claims = Claims {
        exp: (SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + exp) as usize,
        iat: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize,
        sub,
    };

    let encoding_key = EncodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_ref());

    encode(&header, &claims, &encoding_key)
}
