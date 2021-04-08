use super::jwks::{get_jwk_for_token, JWKS};
use crate::AppState;
use actix_web::dev::Payload;
use actix_web::error::{ErrorBadRequest, ErrorForbidden, ErrorInternalServerError};
use actix_web::web::Data;
use actix_web::{Error, FromRequest, HttpRequest};
use futures_util::future::{err, ok, Ready};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::future::Future;
use tokio::runtime::Handle;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub iss: String,
    pub sub: String,
    pub aud: String,
    pub iat: i64,
    pub exp: i64,
    pub azp: Option<String>,
    pub gty: Option<String>,
}

impl FromRequest for Claims {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let header = req.headers().get("Authorization");
        if header.is_none() {
            return err(ErrorForbidden("Missing Authorization Header"));
        }
        let header = header.unwrap();
        if header.is_empty() {
            return err(ErrorForbidden("Missing Authorization Header"));
        }
        let header = header.to_str();
        if header.is_err() {
            return err(ErrorBadRequest("Malformed Authorization Header"));
        }
        let header = header.unwrap();
        let header = header.split("Bearer ");

        if header.clone().count() != 2 {
            return err(ErrorBadRequest("Malformed Authorization Header"));
        }
        let header: Vec<&str> = header.clone().into_iter().collect();
        let token = header[1];
        let app_state = req.app_data::<Data<AppState>>();
        if app_state.is_none() {
            return err(ErrorInternalServerError(""));
        }
        let app_state = app_state.unwrap();
        let jwks = &app_state.jwks;
        let jwk = get_jwk_for_token(token, jwks);
        if jwk.is_none() {
            return err(ErrorForbidden("Invalid JWT"));
        }
        let jwk = jwk.unwrap();
        let token = decode::<Claims>(
            &token,
            &DecodingKey::from_rsa_components(&jwk.n, &jwk.e),
            &Validation::new(Algorithm::RS256),
        );
        if token.is_err() {
            return err(ErrorForbidden("Invalid JWT"));
        }

        return ok(token.unwrap().claims);
    }
}
