use derive_more::From;
use sqlx::error::{DatabaseError, ErrorKind};
use uuid::Uuid;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    NoEncontrado {
        id: u32
    },

    NoEncontradoLogin {
        uuid: Uuid
    },

    UsuarioExiste {
        mail: String
    },

    ViolacionUnico {
        tabla: String,
        regla: String
    },

    #[from]
    Sqlx(sqlx::Error)
}

impl Error {
    pub fn resolver_unico<F>(self, funcion: Option<F>) -> Self
    where
        F: FnOnce(&str, &str) -> Option<Self>
    {
        match self.convertir_errorbd().map(|error_bd| {
            (error_bd.kind(), error_bd.table(), error_bd.constraint())
        }) {
            Some((ErrorKind::UniqueViolation, Some(tabla), Some(regla))) => {
                funcion
                    .and_then(|fun| fun(tabla, regla))
                    .unwrap_or_else(|| Error::ViolacionUnico { 
                        tabla: tabla.to_string(), 
                        regla: regla.to_string() 
                    })
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
