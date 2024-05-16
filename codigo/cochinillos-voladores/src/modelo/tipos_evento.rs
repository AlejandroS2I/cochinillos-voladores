use serde::Deserialize;
use crate::ctx::Ctx;
use crate::modelo::pwd::hash_password;
use crate::modelo::{Error, Result};
use crate::modelo::ControladorModelo;


#[derive(Clone, Debug, sqlx::FromRow)]
pub struct TipoEvento {
    pub id: u32,
    pub nombre: String,
}

#[derive(Deserialize)]
pub struct TipoEventoCrear {
    pub nombre: String,
}

#[derive(Deserialize)]
pub struct TipoEventoActualizar {
    pub id: u32,
    pub nombre: String,
}

pub struct ControladorTipoEvento;


impl ControladorTipoEvento {
    pub async fn crear_tipo_evento(
        cm: ControladorModelo, 
        tipo_evento: TipoEventoCrear
    ) -> Result<TipoEvento> {
        let pool = cm.conexion;
        let mut txn = pool.begin().await?;

        sqlx::query!("
            INSERT INTO ltiposevento (nombre) 
            VALUES (?); 
        ",
            tipo_evento.nombre,
        )
        .execute(txn.as_mut())
        .await?;

        let tipo_evento = sqlx::query_as!(
        TipoEvento,
        "
            SELECT id, nombre FROM ltiposevento
            WHERE id = LAST_INSERT_ID();
        ")
        .fetch_one(txn.as_mut())
        .await?;

        txn.commit().await?;

        Ok(tipo_evento)
    }

    pub async fn actualizar_tipo_evento(
        cm: ControladorModelo, 
        tipo_evento: TipoEventoActualizar
    ) -> Result<TipoEvento> {
        let pool = cm.conexion;
        let mut txn = pool.begin().await?;

        sqlx::query!("
            UPDATE ltiposevento SET nombre = ?
            WHERE id = ?
        ",
            tipo_evento.nombre,
            tipo_evento.id
        )
        .execute(txn.as_mut())
        .await?;

        let tipo_evento = sqlx::query_as!(
        TipoEvento,
        "
            SELECT id, nombre FROM ltiposevento
            WHERE id = ?;
        ",
            tipo_evento.id
        )
        .fetch_one(txn.as_mut())
        .await?;

        txn.commit().await?;

        Ok(tipo_evento)
    }

    pub async fn listar_tipos_evento(
        ctx: Ctx,
        cm: ControladorModelo
    ) -> Result<Vec<TipoEvento>> {
        let pool = cm.conexion;

        let tipo_evento: Vec<TipoEvento> = sqlx::query_as("
            SELECT id, nombre FROM ltiposevento
        ")
        .fetch_all(&pool)
        .await?;

        Ok(tipo_evento)
    }

    pub async fn eliminar_tipo_evento(
        ctx: Ctx,
        cm: ControladorModelo, 
        id: u32
    ) -> Result<TipoEvento> {
        let pool = cm.conexion;
        let mut txn = pool.begin().await?;

        let tipo_evento = sqlx::query_as!(
        TipoEvento,
        "
            SELECT id, nombre
            FROM ltiposevento WHERE id = ?;
        ",
            id
        )
        .fetch_optional(txn.as_mut())
        .await?
        .ok_or(Error::NoEncontrado { id })?;

        sqlx::query!(
        "
            DELETE FROM ltiposevento WHERE id = ?
        ",
            id
        )
        .execute(txn.as_mut())
        .await?;

        txn.commit().await?;

        Ok(tipo_evento)
    }

    pub async fn tipo_evento_id(cm: ControladorModelo, id: u32) -> Result<Option<TipoEvento>> {
        let pool = cm.conexion;

        let tipo_evento = sqlx::query_as!(
        TipoEvento,
        "
            SELECT id, nombre FROM ltiposevento
            WHERE id = ?
        ",
            id
        )
        .fetch_optional(&pool)
        .await?;

        Ok(tipo_evento)
    }
}
