use serde::Deserialize;
use crate::ctx::Ctx;
use crate::modelo::pwd::hash_password;
use crate::modelo::{Error, Result};
use crate::modelo::ControladorModelo;


#[derive(Clone, Debug, sqlx::FromRow)]
pub struct TipoJugador {
    pub id: u32,
    pub nombre: String,
}

#[derive(Deserialize)]
pub struct TipoJugadorCrear {
    pub nombre: String,
}

#[derive(Deserialize)]
pub struct TipoJugadorActualizar {
    pub id: u32,
    pub nombre: String,
}

pub struct ControladorTipoJugador;


impl ControladorTipoJugador {
    pub async fn crear_tipo_jugador(
        cm: ControladorModelo, 
        tipo_jugador: TipoJugadorCrear
    ) -> Result<TipoJugador> {
        let pool = cm.conexion;
        let mut txn = pool.begin().await?;

        sqlx::query!("
            INSERT INTO ltiposjugador (nombre) 
            VALUES (?); 
        ",
            tipo_jugador.nombre,
        )
        .execute(txn.as_mut())
        .await?;

        let tipo_jugador = sqlx::query_as!(
        TipoJugador,
        "
            SELECT id, nombre FROM ltiposjugador
            WHERE id = LAST_INSERT_ID();
        ")
        .fetch_one(txn.as_mut())
        .await?;

        txn.commit().await?;

        Ok(tipo_jugador)
    }

    pub async fn actualizar_tipo_jugador(
        cm: ControladorModelo, 
        tipo_jugador: TipoJugadorActualizar
    ) -> Result<TipoJugador> {
        let pool = cm.conexion;
        let mut txn = pool.begin().await?;

        sqlx::query!("
            UPDATE ltiposjugador SET nombre = ?
            WHERE id = ?
        ",
            tipo_jugador.nombre,
            tipo_jugador.id
        )
        .execute(txn.as_mut())
        .await?;

        let tipo_jugador = sqlx::query_as!(
        TipoJugador,
        "
            SELECT id, nombre FROM ltiposjugador
            WHERE id = ?;
        ",
            tipo_jugador.id
        )
        .fetch_one(txn.as_mut())
        .await?;

        txn.commit().await?;

        Ok(tipo_jugador)
    }

    pub async fn listar_tipos_jugador(
        ctx: Ctx,
        cm: ControladorModelo
    ) -> Result<Vec<TipoJugador>> {
        let pool = cm.conexion;

        let tipo_jugador: Vec<TipoJugador> = sqlx::query_as("
            SELECT id, nombre FROM ltiposjugador
        ")
        .fetch_all(&pool)
        .await?;

        Ok(tipo_jugador)
    }

    pub async fn eliminar_tipo_jugador(
        ctx: Ctx,
        cm: ControladorModelo, 
        id: u32
    ) -> Result<TipoJugador> {
        let pool = cm.conexion;
        let mut txn = pool.begin().await?;

        let tipo_jugador = sqlx::query_as!(
        TipoJugador,
        "
            SELECT id, nombre
            FROM ltiposjugador WHERE id = ?;
        ",
            id
        )
        .fetch_optional(txn.as_mut())
        .await?
        .ok_or(Error::NoEncontrado { id })?;

        sqlx::query!(
        "
            DELETE FROM ltiposjugador WHERE id = ?
        ",
            id
        )
        .execute(txn.as_mut())
        .await?;

        txn.commit().await?;

        Ok(tipo_jugador)
    }

    pub async fn tipo_jugador_id(cm: ControladorModelo, id: u32) -> Result<Option<TipoJugador>> {
        let pool = cm.conexion;

        let tipo_jugador = sqlx::query_as!(
        TipoJugador,
        "
            SELECT id, nombre FROM ltiposjugador
            WHERE id = ?
        ",
            id
        )
        .fetch_optional(&pool)
        .await?;

        Ok(tipo_jugador)
    }
}
