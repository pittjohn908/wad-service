use std::sync::Arc;

use jsonwebtoken::{decode, decode_header, DecodingKey, Validation};
use tonic::{Request, Status};

use super::apple_keys::AppleKeyManager;

pub async fn check_auth(
    req: Request<()>,
    key_manager: Arc<AppleKeyManager>,
) -> Result<Request<()>, Status> {
    match req.metadata().get("authorization") {
        Some(token) => {
            let token_str = token
                .to_str()
                .map_err(|_| Status::unauthenticated("Invalid token format"))?;

            if token_str.starts_with("Bearer ") {
                let token = &token_str["Bearer ".len()..];

                let _ = decode_token(&token, key_manager).await?;

                if !token.is_empty() {
                    Ok(req)
                } else {
                    Err(Status::unauthenticated("Invalid token"))
                }
            } else {
                Err(Status::unauthenticated("Invalid token format"))
            }
        }
        None => Err(Status::unauthenticated("No auth token provided")),
    }
}

async fn decode_token(token: &str, key_manager: Arc<AppleKeyManager>) -> Result<(), Status> {
    let header = decode_header(&token).map_err(|_| Status::unauthenticated("Invalid Header"))?;

    let kid = header
        .kid
        .ok_or_else(|| Status::unauthenticated("No key ID in token"))?;

    let apple_key = key_manager
        .get_key(&kid)
        .await
        .ok_or_else(|| Status::unauthenticated("Invalid key ID"))?;

    // TODO: Cache decoding key
    let decoding_key = DecodingKey::from_rsa_components(&apple_key.n, &apple_key.e)
        .map_err(|_| Status::unauthenticated("Failed to generate decoding key"))?;

    let mut validation = Validation::new(header.alg);
    validation.set_audience(&["com.Sora.WordADay"]);
    validation.set_issuer(&["https://appleid.apple.com"]);

    match decode::<serde_json::Value>(token, &decoding_key, &validation) {
        Ok(val) => {
            println!("{:#?}", val);
            Ok(())
        }
        Err(_) => Err(Status::unauthenticated("Invalid token")),
    }
}
