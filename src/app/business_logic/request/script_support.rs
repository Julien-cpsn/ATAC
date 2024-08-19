use boa_engine::{JsValue, JsResult, JsString, Context};
use jsonwebtoken::Algorithm;

use super::jwt::generate_jwt_token;

pub fn generate_signed_jwt(_this: &JsValue, args: &[JsValue], _ctx: &mut Context) -> JsResult<JsValue> {
    let private_key_string: &str = &args.get(0).unwrap().as_string().unwrap().to_std_string().unwrap();
    let claims: &str = &args.get(1).unwrap().as_string().unwrap().to_std_string().unwrap();
    let alg = args.get(2).map(|arg| arg.as_string().unwrap());
    let kid = args.get(3).map(|arg| arg.as_string().unwrap()).unwrap().to_std_string().unwrap();

    let rust_alg = match alg {
        Some(alg) => {
            let alg_str: &str = &alg.to_std_string().unwrap();
            match alg_str {
                "HS256" => Some(Algorithm::HS256),
                "HS384" => Some(Algorithm::HS384),
                "HS512" => Some(Algorithm::HS512),
                "RS256" => Some(Algorithm::RS256),
                "RS384" => Some(Algorithm::RS384),
                "RS512" => Some(Algorithm::RS512),
                "ES256" => Some(Algorithm::ES256),
                "ES384" => Some(Algorithm::ES384),
                "PS256" => Some(Algorithm::PS256),
                "PS384" => Some(Algorithm::PS384),
                "PS512" => Some(Algorithm::PS512),
                _ => None
            }
        },
        None => None
    };

    let token = generate_jwt_token(private_key_string, claims, rust_alg, Some(kid));
    Ok(JsValue::String(JsString::from(token)))
}
