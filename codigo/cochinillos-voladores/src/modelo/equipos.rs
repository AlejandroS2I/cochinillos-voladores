use serde::Deserialize;
use crate::ctx::Ctx;
use crate::modelo::pwd::hash_password;
use crate::modelo::{Error, Result};
use crate::modelo::ControladorModelo;


#[derive(Clone, Debug, sqlx::FromRow)]
pub struct Equipo {
    pub id: u32,
    pub nombre: String,
    pub lugar: String
}

#[derive(Deserialize)]
pub struct EquipoCrear {
    pub nombre: String,
    pub lugar: String
}

#[derive(Deserialize)]
pub struct EquipoActualizar {
    pub id: u32,
    pub nombre: String,
    pub lugar: String
}

pub struct ControladorEquipo;


impl ControladorEquipo {
    pub async fn crear_equipo(
        cm: ControladorModelo, 
        equipo: EquipoCrear
    ) -> Result<Equipo> {
        let pool = cm.conexion;
        let mut txn = pool.begin().await?;

        sqlx::query!("
            INSERT INTO tequipos (nombre, lugar) 
            VALUES (?, ?); 
        ",
            equipo.nombre,
            equipo.lugar,
        )
        .execute(txn.as_mut())
        .await?;

        let equipo = sqlx::query_as!(
        Equipo,
        "
            SELECT id, nombre, lugar FROM tequipos
            WHERE id = LAST_INSERT_ID();
        ")
        .fetch_one(txn.as_mut())
        .await?;

        txn.commit().await?;

        Ok(equipo)
    }

    pub async fn actualizar_equipo(
        cm: ControladorModelo, 
        equipo: EquipoActualizar
    ) -> Result<Equipo> {
        let pool = cm.conexion;
        let mut txn = pool.begin().await?;

        sqlx::query!("
            UPDATE tequipos SET nombre = ?, lugar = ?
            WHERE id = ?
        ",
            equipo.nombre,
            equipo.lugar,
            equipo.id
        )
        .execute(txn.as_mut())
        .await?;

        let equipo = sqlx::query_as!(
        Equipo,
        "
            SELECT id, nombre, lugar FROM tequipos
            WHERE id = ?;
        ",
            equipo.id
        )
        .fetch_one(txn.as_mut())
        .await?;

        txn.commit().await?;

        Ok(equipo)
    }

    pub async fn listar_equipos(
        cm: ControladorModelo
    ) -> Result<Vec<Equipo>> {
        let pool = cm.conexion;

        let equipos: Vec<Equipo> = sqlx::query_as("
            SELECT id, nombre, lugar FROM tequipos
        ")
        .fetch_all(&pool)
        .await?;

        Ok(equipos)
    }

    pub async fn eliminar_equipo(
        ctx: Ctx,
        cm: ControladorModelo, 
        id: u32
    ) -> Result<Equipo> {
        let pool = cm.conexion;
        let mut txn = pool.begin().await?;

        let equipo = sqlx::query_as!(
        Equipo,
        "
            SELECT id, nombre, lugar
            FROM tequipos WHERE id = ?;
        ",
            id
        )
        .fetch_optional(txn.as_mut())
        .await?
        .ok_or(Error::NoEncontrado { id })?;

        sqlx::query!(
        "
            DELETE FROM tequipos WHERE id = ?
        ",
            id
        )
        .execute(txn.as_mut())
        .await?;

        txn.commit().await?;

        Ok(equipo)
    }

    pub async fn equipo_id(cm: ControladorModelo, id: u32) -> Result<Option<Equipo>> {
        let pool = cm.conexion;

        let equipo = sqlx::query_as!(
        Equipo,
        "
            SELECT id, nombre, lugar FROM tequipos
            WHERE id = ?
        ",
            id
        )
        .fetch_optional(&pool)
        .await?;

        Ok(equipo)
    }
}
