use std::str::FromStr;

use crate::{ctx::Ctx, modelo::{login::ControladorLogin, pwd::verificar_password, usuario::{ControladorUsuario, UsuarioCrear}, ControladorModelo}, web::{self, AUTH_TOKEN}, Error, Result};
use axum_htmx::HxRedirect;
use serde::Deserialize;
use axum::{extract::State, http::{StatusCode, Uri}, response::{Html, IntoResponse, Redirect}, routing::{get, post}, Form, Json, Router};
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

pub fn routes(cm: ControladorModelo) -> Router {
    Router::new()
        .route("/login", post(api_login))
        .route("/logout", get(api_logout))
        .route("/registrar", post(api_registrar))
        .with_state(cm)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    mail: String,
    password: String
}

async fn api_login(
    State(cm): State<ControladorModelo>,
    ctx: Option<Ctx>,
    cookies: Cookies, 
    Form(payload): Form<LoginPayload>
) -> Result<impl IntoResponse> {
    if ctx.is_some() {
        return Err(Error::LoginExistente);
    }

    let usuario = ControladorUsuario::usuario_mail(cm.clone(), payload.mail).await?
        .ok_or(Error::ErrorLoginMailNoEncontrado)?;
    if !verificar_password(payload.password, &usuario.password)? {
        return Err(Error::PasswordIncorrecta);
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

#[derive(Debug, Deserialize)]
struct RegistrarPayload {
    nombre: String,
    mail: String,
    password: String,
    passwordRepetir: String
}

async fn api_registrar(
    State(cm): State<ControladorModelo>,
    cookies: Cookies, 
    Form(usuario): Form<RegistrarPayload>
) -> Result<impl IntoResponse> {
    if (usuario.nombre.is_empty() || usuario.mail.is_empty() || usuario.password.is_empty() || usuario.passwordRepetir.is_empty()) {
        let mut campos = Vec::new();
        usuario.nombre.is_empty().then(|| { campos.push(format!("Nombre"))});
        usuario.mail.is_empty().then(|| { campos.push(format!("Mail"))});
        usuario.password.is_empty().then(|| { campos.push(format!("Contraseña"))});
        usuario.passwordRepetir.is_empty().then(|| { campos.push(format!("Confirmación contraseña"))});
        return Err(Error::CamposVacios { campos });
    }

    if usuario.password != usuario.passwordRepetir {
        return Err(Error::PasswordNoCoinciden);
    }

    let usuario = UsuarioCrear { nombre: usuario.nombre, mail: usuario.mail, password: usuario.password };

    let usuario = ControladorUsuario::crear_usuario(cm.clone(), usuario).await?;

    let login = ControladorLogin::crear_login(cm.clone(), usuario.id).await?;

    let mut cookie = Cookie::new(web::AUTH_TOKEN, format!("{}", login.uuid));
    cookie.set_http_only(true);
    cookie.set_path("/");
    cookies.add(cookie);

    // Resultado correcto
    Ok((HxRedirect("/".parse().map_err(|_| Error::UriInvalida)?), ()).into_response())
}
