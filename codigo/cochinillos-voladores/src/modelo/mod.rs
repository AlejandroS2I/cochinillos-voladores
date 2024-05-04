mod error;

pub mod usuario;
pub mod login;

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