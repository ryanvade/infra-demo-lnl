use crate::auth::jwks::{fetch_jwks_async, get_jwk_for_token};
use actix_web::dev::ServiceRequest;
use actix_web::error::{ErrorForbidden, ErrorInternalServerError};
use actix_web::Error;
use actix_web::HttpMessage;
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use actix_web::http::Method;

pub mod claims;
pub mod jwks;

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    // Don't Require Auth for OPTIONS requests
    if req.method() == Method::OPTIONS {
        return Ok(req);
    }

    let config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default);
    match validate_token(credentials.token()).await {
        Ok(res) => {
            if res == true {
                Ok(req)
            } else {
                Err(AuthenticationError::from(config).into())
            }
        }
        Err(_) => Err(AuthenticationError::from(config).into()),
    }
}

pub async fn validate_token(token: &str) -> Result<bool, Error> {
    let authority = std::env::var("AUTHORITY").expect("AUTHORITY must be set");
    let jwks = fetch_jwks_async(&format!(
        "{}{}",
        authority.as_str(),
        ".well-known/jwks.json"
    ))
    .await;
    if jwks.is_err() {
        return Err(ErrorInternalServerError(""));
    }
    let jwks = jwks.unwrap();
    let jwk = get_jwk_for_token(token, &jwks);
    if jwk.is_none() {
        return Err(ErrorForbidden("Invalid JWT"));
    }
    let jwk = jwk.unwrap();
    let token = decode::<claims::Claims>(
        &token,
        &DecodingKey::from_rsa_components(&jwk.n, &jwk.e),
        &Validation::new(Algorithm::RS256),
    );
    return Ok(token.is_ok());
}
