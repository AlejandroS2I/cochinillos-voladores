use serde::Deserialize;
use time::Date;
use crate::ctx::Ctx;
use crate::modelo::pwd::hash_password;
use crate::modelo::{Error, Result};
use crate::modelo::ControladorModelo;


#[derive(Clone, Debug, sqlx::FromRow)]
pub struct Partido {
    pub id: u32,
    pub fecha: Date,
    pub lugar: String,
    pub idCompeticion: u32,
    pub idEquipoCasa: u32,
    pub idEquipoVisitante: u32
}

#[derive(Deserialize)]
pub struct PartidoCrear {
    pub fecha: Date,
    pub lugar: String,
    pub idCompeticion: u32,
    pub idEquipoCasa: u32,
    pub idEquipoVisitante: u32
}

#[derive(Deserialize)]
pub struct PartidoActualizar {
    pub id: u32,
    pub fecha: Date,
    pub lugar: String,
    pub idCompeticion: u32,
    pub idEquipoCasa: u32,
    pub idEquipoVisitante: u32
}

pub struct ControladorPartido;


impl ControladorPartido {
    pub async fn crear_partido(
        cm: ControladorModelo, 
        partido: PartidoCrear
    ) -> Result<Partido> {
        let pool = cm.conexion;
        let mut txn = pool.begin().await?;

        sqlx::query!("
            INSERT INTO tpartidos (fecha, lugar, idCompeticion, idEquipoCasa, idEquipoVisitante) 
            VALUES (?, ?, ?, ?, ?); 
        ",
            partido.fecha,
            partido.lugar,
            partido.idCompeticion,
            partido.idEquipoCasa,
            partido.idEquipoVisitante,
        )
        .execute(txn.as_mut())
        .await?;

        let partido = sqlx::query_as!(
        Partido,
        "
            SELECT id, fecha, lugar, idCompeticion, idEquipoCasa, idEquipoVisitante FROM tpartidos
            WHERE id = LAST_INSERT_ID();
        ")
        .fetch_one(txn.as_mut())
        .await?;

        txn.commit().await?;

        Ok(partido)
    }

    pub async fn actualizar_partido(
        cm: ControladorModelo, 
        partido: PartidoActualizar
    ) -> Result<Partido> {
        let pool = cm.conexion;
        let mut txn = pool.begin().await?;

        sqlx::query!("
            UPDATE tpartidos SET fecha = ?, lugar = ?, idCompeticion = ?, idEquipoCasa = ?, idEquipoVisitante = ?
            WHERE id = ?
        ",
            partido.fecha,
            partido.lugar,
            partido.idCompeticion,
            partido.idEquipoCasa,
            partido.idEquipoVisitante,
            partido.id
        )
        .execute(txn.as_mut())
        .await?;

        let partido = sqlx::query_as!(
        Partido,
        "
            SELECT id, fecha, lugar, idCompeticion, idEquipoCasa, idEquipoVisitante FROM tpartidos
            WHERE id = ?;
        ",
            partido.id
        )
        .fetch_one(txn.as_mut())
        .await?;

        txn.commit().await?;

        Ok(partido)
    }

    pub async fn listar_partidos(
        ctx: Ctx,
        cm: ControladorModelo
    ) -> Result<Vec<Partido>> {
        let pool = cm.conexion;

        let partidos: Vec<Partido> = sqlx::query_as("
            SELECT id, fecha, lugar, idCompeticion, idEquipoCasa, idEquipoVisitante FROM tpartidos
        ")
        .fetch_all(&pool)
        .await?;

        Ok(partidos)
    }

    pub async fn eliminar_partido(
        ctx: Ctx,
        cm: ControladorModelo, 
        id: u32
    ) -> Result<Partido> {
        let pool = cm.conexion;
        let mut txn = pool.begin().await?;

        let partido = sqlx::query_as!(
        Partido,
        "
            SELECT id, fecha, lugar, idCompeticion, idEquipoCasa, idEquipoVisitante FROM tpartidos
            WHERE id = ?;
        ",
            id
        )
        .fetch_optional(txn.as_mut())
        .await?
        .ok_or(Error::NoEncontrado { id })?;

        sqlx::query!(
        "
            DELETE FROM tpartidos WHERE id = ?
        ",
            id
        )
        .execute(txn.as_mut())
        .await?;

        txn.commit().await?;

        Ok(partido)
    }

    pub async fn partido_id(cm: ControladorModelo, id: u32) -> Result<Option<Partido>> {
        let pool = cm.conexion;

        let partido = sqlx::query_as!(
        Partido,
        "
            SELECT id, fecha, lugar, idCompeticion, idEquipoCasa, idEquipoVisitante FROM tpartidos
            WHERE id = ?
        ",
            id
        )
        .fetch_optional(&pool)
        .await?;

        Ok(partido)
    }
}
