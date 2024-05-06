use std::sync::Arc;

use axum::{http::{StatusCode, Uri}, response::{
    IntoResponse, Redirect, Response
}};
use axum_htmx::HxRedirect;
use derive_more::From;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

use crate::{modelo, web, web::auth};

pub type Result<T> = std::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, From, strum_macros::AsRefStr, Serialize)]
#[serde(tag = "tipo", content = "datos")]
pub enum Error {
    UriInvalida,

    // Login
    ErrorLogin,
    ErrorLoginMailNoEncontrado,
    ErrorLoginNoCookie,
    LoginExistente,

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
        let mut res: Response = match self {
            Self::ErrorLogin => 
                StatusCode::FORBIDDEN.into_response(),
            Self::LoginExistente =>
                (HxRedirect(Uri::from_static("/")), ()).into_response(),
            Self::CtxExt(auth::mw_auth::CtxExtError::TokenExpirado) =>
                Redirect::temporary("/login").into_response(),
            Self::CtxExt(_) =>
                StatusCode::FORBIDDEN.into_response(),
            Self::Model(modelo::Error::NoEncontrado { id: _ }) =>
                StatusCode::BAD_REQUEST.into_response(),
            _ =>
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
        };

        res.extensions_mut().insert(Arc::new(self));

        res
    }
}
