use axum::{
    extract::Request,
    http::{StatusCode, header::AUTHORIZATION},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{DecodingKey, Validation, decode};

use crate::user::{Claims, CurrentUser, Role};

pub async fn auth_middleware(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req.headers().get(AUTHORIZATION);

    println!("Auth header: {:?}", auth_header);
    if let Some(header) = auth_header {
        let header_str = header.to_str().unwrap_or("");
        let token = header_str.strip_prefix("Bearer ").unwrap_or(header_str);

        println!("Token: {}", token);

        let decoded = decode::<Claims>(
            token,
            &DecodingKey::from_secret("secret".as_ref()),
            &Validation::default(),
        );

        println!("Decoded: {:?}", decoded);
        match decoded {
            Ok(data) => {
                println!("Valid user: {:?}", data);

                let role = data.claims.role;
                if role != Role::ADMIN {
                    return Err(StatusCode::FORBIDDEN);
                }

                let current_user = CurrentUser {
                    username: data.claims.sub,
                    role: role,
                };
                req.extensions_mut().insert(current_user);
                return Ok(next.run(req).await);
            }
            Err(err) => {
                println!("Invalid token: {}", err);
                return Err(StatusCode::UNAUTHORIZED);
            }
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}
