use jsonwebtoken::decode_header;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JWK {
    pub alg: String,
    pub kty: String,
    #[serde(rename = "use")]
    pub use_as: String,
    pub n: String,
    pub e: String,
    pub kid: String,
    pub x5t: String,
    pub x5c: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JWKS {
    pub keys: Vec<JWK>,
}

pub fn fetch_jwks(uri: &str) -> Result<JWKS, Box<dyn Error>> {
    let res = reqwest::blocking::get(uri)?;
    let val = res.json::<JWKS>()?;
    return Ok(val);
}

pub async fn fetch_jwks_async(uri: &str) -> Result<JWKS, Box<dyn Error>> {
    let res = reqwest::get(uri).await?;
    let val = res.json::<JWKS>().await?;
    return Ok(val);
}

pub fn get_jwk_for_token(token: &str, jwks: &JWKS) -> Option<JWK> {
    let header = decode_header(&token);
    if let Err(err) = header {
        return None;
    }
    let header = header.unwrap();
    let header_kid = header.kid.clone().unwrap_or("".to_string());
    let keys = &jwks.keys;
    let jwk = keys.into_iter().find(|k| k.kid == header_kid);
    if jwk.is_none() {
        return None;
    }
    return Some(jwk.unwrap().clone());
}
