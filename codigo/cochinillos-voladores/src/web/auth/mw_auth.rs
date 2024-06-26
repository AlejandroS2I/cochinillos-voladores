use async_trait::async_trait;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::RequestPartsExt;
use axum::{
    http::Request,
    middleware::Next,
    response::Response,
    body::Body
};
use lazy_regex::regex_captures;
use serde::Serialize;
use time::macros::format_description;
use time::{Date, OffsetDateTime};
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

use crate::ctx::Ctx;
use crate::modelo::login::ControladorLogin;
use crate::modelo::usuario::ControladorUsuario;
use crate::modelo::ControladorModelo;
use crate::web::AUTH_TOKEN;
use crate::{Error, Result};

pub async fn mw_requerir_auth(
    ctx: Result<Ctx>,
    req: Request<Body>, 
    next: Next
) -> Result<Response> {
    ctx?;

    Ok(next.run(req).await)
}

pub async fn mw_requerir_admin(
    ctx: Result<Ctx>,
    req: Request<Body>, 
    next: Next
) -> Result<Response> {
    if !ctx?.usuario().esAdministrador {
        return Err(Error::SinPermisos);
    };

    Ok(next.run(req).await)
}

pub async fn mw_resolvedor_ctx(
    State(cm): State<ControladorModelo>,
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Response {
    let result_ctx = resolver_ctx(cm, &cookies).await;

    if result_ctx.is_err()
        && !matches!(result_ctx, Err(CtxExtError::NoTokenEnCookies))
    {
        cookies.remove(Cookie::named(AUTH_TOKEN));
    }

    req.extensions_mut().insert(result_ctx);

    next.run(req).await
}

async fn resolver_ctx(cm: ControladorModelo, cookies: &Cookies) -> CtxExtResult {
    let uuid = Uuid::parse_str(cookies
        .get(AUTH_TOKEN)
        .map(|c| c.value().to_string())
        .ok_or(CtxExtError::NoTokenEnCookies)?.as_str())
        .map_err(|_|CtxExtError::TokenFormatoIncorrecto)?;

    let login = ControladorLogin::login_uuid(cm.clone(), uuid).await
        .map_err(|err| CtxExtError::ErrorModelo(err.to_string()))?
        .ok_or(CtxExtError::LoginNoEncontrado)?;

    // Validar token
    if login.fechaCaducidad <= OffsetDateTime::now_utc().date() {
        ControladorLogin::eliminar_login(cm.clone(), uuid).await
            .map_err(|err| CtxExtError::ErrorModelo(err.to_string()))?;

        let mut cookie = Cookie::from(AUTH_TOKEN);
        cookie.set_path("/");
        cookies.remove(cookie);

        return Err(CtxExtError::TokenExpirado);
    }

    let usuario = ControladorUsuario::usuario_id(cm, login.idUsuario).await
        .map_err(|err| CtxExtError::ErrorModelo(err.to_string()))?
        .ok_or(CtxExtError::UsuarioNoEncontrado)?;
    
    Ok(Ctx::new(usuario))
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        parts
            .extensions
            .get::<CtxExtResult>()
            .ok_or(Error::CtxExt(CtxExtError::NoCtxEnExtension))?
            .clone()
            .map_err(Error::CtxExt)
    }
}

type CtxExtResult = std::result::Result<Ctx, CtxExtError>;

#[derive(Clone, Debug, Serialize)]
pub enum CtxExtError {
    NoCtxEnExtension,
    NoTokenEnCookies,
    TokenFormatoIncorrecto,
    TokenExpirado,
    ErrorModelo(String),
    LoginNoEncontrado,
    UsuarioNoEncontrado
}
