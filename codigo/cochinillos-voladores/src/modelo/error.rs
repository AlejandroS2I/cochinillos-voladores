use std::borrow::Cow;

use derive_more::From;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use sqlx::error::{DatabaseError, ErrorKind};
use uuid::Uuid;

pub type Result<T> = std::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, From, Serialize)]
pub enum Error {
    NoEncontrado {
        id: u32
    },

    NoEncontradoLogin {
        #[serde_as(as = "DisplayFromStr")]
        uuid: Uuid
    },

    UsuarioExiste {
        mail: String
    },

    ViolacionUnico,

    ErrorPasswordHashing,
    ErrorVerificandoPassword{ error: String },

    #[from]
    Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error)
}

impl Error {
    pub fn resolver_unico<F>(self, funcion: Option<F>) -> Self
    where
        F: FnOnce() -> Option<Self>,
    {
        match self.convertir_errorbd().map(|error_bd| {
            error_bd.is_unique_violation()
        }) {
            // Error unico mysql: 23000
            Some(true) => {
                funcion
                    .and_then(|fun| fun())
                    .unwrap_or_else(|| Error::ViolacionUnico)
            }
            _ => self
        }
    }

    pub fn convertir_errorbd(&self) -> Option<&(dyn DatabaseError + 'static)> {
        match self {
            Error::Sqlx(error_sqlx) => {
                error_sqlx.as_database_error()
            }
            _ => None
        }
    }
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
