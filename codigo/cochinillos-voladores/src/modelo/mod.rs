mod error;

pub mod usuario;
pub mod noticias;
pub mod categorias;
pub mod competiciones;
pub mod partidos;
pub mod eventos;
pub mod equipos;
pub mod jugadores;
pub mod tipos_jugador;
pub mod tipos_evento;
pub mod login;
pub mod pwd;

use serde::Deserialize;
use sqlx::{Pool, MySql, Row, Transaction, error::DatabaseError};
pub use error::{Error, Result};

#[derive(Clone)]
pub struct ControladorModelo {
    conexion: Pool<MySql>
}

impl ControladorModelo {
    pub async fn new(conexion: Pool<MySql>) -> Result<Self> {
        Ok(Self {
            conexion
        })
    }
}
