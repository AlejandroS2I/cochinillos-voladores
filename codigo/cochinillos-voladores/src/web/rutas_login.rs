use crate::{modelo::{login::{ControladorLogin}, usuario::ControladorUsuario, ControladorModelo}, web, Error, Result};
use serde::Deserialize;
use axum::{extract::State, routing::post, Json, Router, Form};
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

pub fn routes(cm: ControladorModelo) -> Router {
    Router::new().route("/login", post(api_login))
        .with_state(cm)
}

async fn api_login(
    State(cm): State<ControladorModelo>,
    cookies: Cookies, 
    Form(payload): Form<LoginPayload>
) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    let usuario = ControladorUsuario::usuario_mail(cm.clone(), payload.mail).await?
        .ok_or(Error::ErrorLoginMailNoEncontrado)?;
    if usuario.password != payload.password {
        return Err(Error::ErrorLogin);
    }

    let login = ControladorLogin::crear_login(cm.clone(), usuario.id).await?;

    let mut cookie = Cookie::new(web::AUTH_TOKEN, format!("{}.{}.{}", usuario.id, login.fechaCaducidad, login.uuid));
    cookie.set_http_only(true);
    cookie.set_path("/");
    cookies.add(cookie);

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
