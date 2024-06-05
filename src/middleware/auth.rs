use crate::constant::KEYS;
use crate::helper::error::ResponseError;
use crate::interface::auth::Claims;
use axum::{extract::Request, http::header, middleware::Next, response::Response};
use jsonwebtoken::{decode, Algorithm, Validation};

async fn authorize_current_user(auth_token: &str) -> Option<Claims> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.sub = Some("b@b.com".to_string());
    validation.set_audience(&["me"]);
    validation.set_required_spec_claims(&["exp", "sub", "aud"]);
    match decode::<Claims>(&auth_token, &KEYS.decoding, &validation) {
        Ok(c) => Some(c.claims),
        Err(_err) => None,
    }
}

pub async fn auth(mut req: Request, next: Next) -> Result<Response, ResponseError> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(ResponseError::Unauthorized);
    };

    if let Some(current_user) = authorize_current_user(auth_header).await {
        // insert the current user into a request extension so the handler can
        // extract it
        req.extensions_mut().insert(current_user);
        Ok(next.run(req).await)
    } else {
        Err(ResponseError::Unauthorized)
    }
}
