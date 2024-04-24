use axum::{http::StatusCode, response::{
    IntoResponse,
    Response
}};
use derive_more::From;

use crate::modelo;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    ErrorLogin,
    ErrorLoginMailNoEncontrado,

    // Errores auth
    NoAuthTokenCookie,
    TokenFormatoIncorrecto,

    #[from]
    Uuid(uuid::Error),

    // Errores modelo
    #[from]
    Model(modelo::Error)
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

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}
