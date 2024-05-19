use serde::Deserialize;
use time::Date;
use crate::ctx::Ctx;
use crate::modelo::pwd::hash_password;
use crate::modelo::{Error, Result};
use crate::modelo::ControladorModelo;


#[derive(Clone, Debug, sqlx::FromRow)]
pub struct Competicion {
    pub id: u32,
    pub nombre: String,
    pub fechaInicio: Option<Date>,
    pub fechaFin: Option<Date>,
    pub idCategoria: u32
}

#[derive(Deserialize)]
pub struct CompeticionCrear {
    pub nombre: String,
    pub fechaInicio: Option<Date>,
    pub fechaFin: Option<Date>,
    pub idCategoria: u32
}

#[derive(Deserialize)]
pub struct CompeticionActualizar {
    pub id: u32,
    pub nombre: String,
    pub fechaInicio: Date,
    pub fechaFin: Date,
    pub idCategoria: u32
}

pub struct ControladorCompeticion;


impl ControladorCompeticion {
    pub async fn crear_competicion(
        cm: ControladorModelo, 
        competicion: CompeticionCrear
    ) -> Result<Competicion> {
        let pool = cm.conexion;
        let mut txn = pool.begin().await?;

        sqlx::query!("
            INSERT INTO tcompeticiones (nombre, fechaInicio, fechaFin, idCategoria) 
            VALUES (?, ?, ?, ?); 
        ",
            competicion.nombre,
            competicion.fechaInicio,
            competicion.fechaFin,
            competicion.idCategoria
        )
        .execute(txn.as_mut())
        .await?;

        let competicion = sqlx::query_as!(
        Competicion,
        "
            SELECT id, nombre, fechaInicio, fechaFin, idCategoria FROM tcompeticiones
            WHERE id = LAST_INSERT_ID();
        ")
        .fetch_one(txn.as_mut())
        .await?;

        txn.commit().await?;

        Ok(competicion)
    }

    pub async fn actualizar_competicion(
        cm: ControladorModelo, 
        competicion: CompeticionActualizar
    ) -> Result<Competicion> {
        let pool = cm.conexion;
        let mut txn = pool.begin().await?;

        sqlx::query!("
            UPDATE tcompeticiones SET nombre = ?, fechaInicio = ?, fechaFin = ?, idCategoria = ?
            WHERE id = ?
        ",
            competicion.nombre,
            competicion.fechaInicio,
            competicion.fechaFin,
            competicion.idCategoria,
            competicion.id
        )
        .execute(txn.as_mut())
        .await?;

        let competicion = sqlx::query_as!(
        Competicion,
        "
            SELECT id, nombre, fechaInicio, fechaFin, idCategoria FROM tcompeticiones
            WHERE id = ?;
        ",
            competicion.id
        )
        .fetch_one(txn.as_mut())
        .await?;

        txn.commit().await?;

        Ok(competicion)
    }

    pub async fn listar_competiciones(
        cm: ControladorModelo
    ) -> Result<Vec<Competicion>> {
        let pool = cm.conexion;

        let competiciones: Vec<Competicion> = sqlx::query_as("
            SELECT id, nombre, fechaInicio, fechaFin, idCategoria FROM tcompeticiones
        ")
        .fetch_all(&pool)
        .await?;

        Ok(competiciones)
    }

    pub async fn eliminar_competicion(
        ctx: Ctx,
        cm: ControladorModelo, 
        id: u32
    ) -> Result<Competicion> {
        let pool = cm.conexion;
        let mut txn = pool.begin().await?;

        let competicion = sqlx::query_as!(
        Competicion,
        "
            SELECT id, nombre, fechaInicio, fechaFin, idCategoria
            FROM tcompeticiones WHERE id = ?;
        ",
            id
        )
        .fetch_optional(txn.as_mut())
        .await?
        .ok_or(Error::NoEncontrado { id })?;

        sqlx::query!(
        "
            DELETE FROM tcompeticiones WHERE id = ?
        ",
            id
        )
        .execute(txn.as_mut())
        .await?;

        txn.commit().await?;

        Ok(competicion)
    }

    pub async fn competicion_id(cm: ControladorModelo, id: u32) -> Result<Option<Competicion>> {
        let pool = cm.conexion;

        let competicion = sqlx::query_as!(
        Competicion,
        "
            SELECT id, nombre, fechaInicio, fechaFin, idCategoria FROM tcompeticiones
            WHERE id = ?
        ",
            id
        )
        .fetch_optional(&pool)
        .await?;

        Ok(competicion)
    }
}
