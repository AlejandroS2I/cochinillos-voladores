use crate::{Error, Result};
use serde::Deserialize;
use axum::{routing::post, Json, Router};
use serde_json::{json, Value};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    // TODO: Añadir autentificación real
    if payload.mail != "prueba@gmail.com" || payload.password != "prueba" {
        return Err(Error::LoginFail);
    }

    // TODO: Añadir cookies

    // Resultado correcto
    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    mail: String,
    password: String
}
