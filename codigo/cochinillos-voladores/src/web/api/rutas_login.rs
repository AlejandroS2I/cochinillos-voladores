use std::str::FromStr;

use crate::{modelo::{login::ControladorLogin, usuario::ControladorUsuario, ControladorModelo}, web::{self, AUTH_TOKEN}, Error, Result};
use axum_htmx::HxRedirect;
use serde::Deserialize;
use axum::{extract::State, http::{StatusCode, Uri}, response::{Html, IntoResponse, Redirect}, routing::{get, post}, Form, Json, Router};
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

pub fn routes(cm: ControladorModelo) -> Router {
    Router::new().route("/login", post(api_login))
        .route("/logout", get(api_logout))
        .with_state(cm)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    mail: String,
    password: String
}

async fn api_login(
    State(cm): State<ControladorModelo>,
    cookies: Cookies, 
    Form(payload): Form<LoginPayload>
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - api_login", "HANDLER");

    let usuario = ControladorUsuario::usuario_mail(cm.clone(), payload.mail).await?
        .ok_or(Error::ErrorLoginMailNoEncontrado)?;
    if usuario.password != payload.password {
        return Err(Error::ErrorLogin);
    }

    let login = ControladorLogin::crear_login(cm.clone(), usuario.id).await?;

    let mut cookie = Cookie::new(web::AUTH_TOKEN, format!("{}", login.uuid));
    cookie.set_http_only(true);
    cookie.set_path("/");
    cookies.add(cookie);

    // Resultado correcto
    Ok((HxRedirect("/".parse().map_err(|_| Error::UriInvalida)?), ()).into_response())
}

async fn api_logout(
    State(cm): State<ControladorModelo>,
    cookies: Cookies, 
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - api_login", "HANDLER");

    let uuid = Uuid::parse_str(cookies.get(AUTH_TOKEN)
        .map(|c| c.value().to_string())
        .ok_or(Error::ErrorLoginNoCookie)?.as_str())?;

    ControladorLogin::eliminar_login(cm.clone(), uuid).await?;

    let mut cookie = Cookie::from(AUTH_TOKEN);
    cookie.set_path("/");
    cookies.remove(cookie);

    // Resultado correcto
    Ok((HxRedirect("/".parse().map_err(|_| Error::UriInvalida)?), ()).into_response())
}
