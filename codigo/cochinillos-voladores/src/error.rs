use std::sync::Arc;

use askama::Template;
use axum::{http::{StatusCode, Uri}, response::{
    IntoResponse, Redirect, Response
}};
use axum_htmx::{HxRedirect, HxReswap, HxRetarget, HxTarget, HX_RESWAP, HX_RETARGET};
use derive_more::From;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

use crate::{modelo, web, web::auth};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Template)]
#[template(path = "componentes/error.html")]
struct ErrorTemplate {
    error: String
}

#[serde_as]
#[derive(Debug, From, strum_macros::AsRefStr, Serialize)]
#[serde(tag = "tipo", content = "datos")]
pub enum Error {
    UriInvalida,
    ErrorTemplate,

    // Login
    ErrorLogin,
    ErrorLoginMailNoEncontrado,
    ErrorLoginNoCookie,
    LoginExistente,
    PasswordIncorrecta,
    PasswordNoCoinciden,

    // Registro
    ErrorRegistro,

    // Errores auth
    TokenFormatoIncorrecto,
    SinPermisos,

    #[from]
    CtxExt(auth::mw_auth::CtxExtError),

    // Errores modelo
    #[from]
    Model(modelo::Error),

    #[from]
    Uuid(#[serde_as(as = "DisplayFromStr")] uuid::Error),
}

impl std::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut std::fmt::Formatter,
	) -> std::result::Result<(), std::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let mut res: Response = match &self {
            Self::ErrorLogin => 
                StatusCode::FORBIDDEN.into_response(),
            Self::LoginExistente =>
                (HxRedirect(Uri::from_static("/")), ()).into_response(),
            Self::ErrorLoginMailNoEncontrado | Self::PasswordIncorrecta => (
                StatusCode::BAD_REQUEST,
                ErrorTemplate { error: format!("Mail o contraseña incorrecta") }
            ).into_response(),
            Self::CtxExt(auth::mw_auth::CtxExtError::TokenExpirado) =>
                Redirect::temporary("/login").into_response(),
            Self::CtxExt(_) =>
                StatusCode::FORBIDDEN.into_response(),
            Self::Model(modelo::Error::NoEncontrado { id: _ }) =>
                StatusCode::BAD_REQUEST.into_response(),
            Self::Model(modelo::Error::UsuarioExiste { mail }) => (
                StatusCode::BAD_REQUEST,
                ErrorTemplate { error: format!("Usuario con mail \"{}\" ya existe", mail) }
            ).into_response(),
            _ =>
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
        };

       res.extensions_mut().insert(Arc::new(self));

       res
    }
}
