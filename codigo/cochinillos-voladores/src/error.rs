use std::sync::Arc;

use axum::{http::StatusCode, response::{
    IntoResponse,
    Response
}};
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
        println!("->> {:<12} - {self:?}", "INTO_RES");

        let mut res = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        res.extensions_mut().insert(Arc::new(self));

        res
    }
}

impl Error {
    pub fn estado_error_cliente(&self) -> (StatusCode, ClientError) {
        match self {
            Self::ErrorLogin => (
                StatusCode::FORBIDDEN,
                ClientError::ERROR_LOGIN
            ),
            Self::CtxExt(_) => (
                StatusCode::FORBIDDEN,
                ClientError::NO_AUTH
            ),
            Self::Model(modelo::Error::NoEncontrado { id }) => (
                StatusCode::BAD_REQUEST,
                ClientError::NO_ENCONTRADO{ id: *id }
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::ERROR_SERVICIO
            )
        }
    }
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    ERROR_LOGIN,
    NO_AUTH,
    NO_ENCONTRADO { id: u32 },
    PARAMS_INVALIDOS,
    ERROR_SERVICIO
}
