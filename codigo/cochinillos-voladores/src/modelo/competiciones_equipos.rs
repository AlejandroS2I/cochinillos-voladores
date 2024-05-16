use serde::Deserialize;
use crate::ctx::Ctx;
use crate::modelo::pwd::hash_password;
use crate::modelo::{Error, Result};
use crate::modelo::ControladorModelo;


#[derive(Clone, Debug, Deserialize, sqlx::FromRow)]
pub struct CompeticionEquipo {
    pub idCompeticion: u32,
    pub idEquipo: u32,
}

pub struct ControladorCompeticionEquipo;

impl ControladorCompeticionEquipo {
    pub async fn crear_competicion_equipo(
        cm: ControladorModelo, 
        competicion_equipo: CompeticionEquipo
    ) -> Result<CompeticionEquipo> {
        let pool = cm.conexion;
        let mut txn = pool.begin().await?;

        sqlx::query!("
            INSERT INTO rcompeticionesequipos (idCompeticion, idEquipo) 
            VALUES (?, ?); 
        ",
            competicion_equipo.idCompeticion,
            competicion_equipo.idEquipo
        )
        .execute(txn.as_mut())
        .await?;

        txn.commit().await?;

        Ok(competicion_equipo)
    }

    pub async fn actualizar_competicion_equipo(
        cm: ControladorModelo, 
        competicion_equipo: CompeticionEquipo
    ) -> Result<CompeticionEquipo> {
        let pool = cm.conexion;
        let mut txn = pool.begin().await?;

        sqlx::query!("
            UPDATE rcompeticionesequipos SET idCompeticion = ?, idEquipo = ?
            WHERE idCompeticion = ? AND idEquipo = ?
        ",
            competicion_equipo.idCompeticion,
            competicion_equipo.idEquipo,
            competicion_equipo.idCompeticion,
            competicion_equipo.idEquipo,
        )
        .execute(txn.as_mut())
        .await?;

        txn.commit().await?;

        Ok(competicion_equipo)
    }

    pub async fn listar_competiciones_equipos(
        ctx: Ctx,
        cm: ControladorModelo
    ) -> Result<Vec<CompeticionEquipo>> {
        let pool = cm.conexion;

        let competiciones_equipos: Vec<CompeticionEquipo> = sqlx::query_as("
            SELECT idCompeticion, idEquipo FROM rcompeticionesequipos
        ")
        .fetch_all(&pool)
        .await?;

        Ok(competiciones_equipos)
    }

    pub async fn eliminar_competicion_equipo(
        ctx: Ctx,
        cm: ControladorModelo, 
        idCompeticion: u32,
        idEquipo: u32
    ) -> Result<CompeticionEquipo> {
        let pool = cm.conexion;
        let mut txn = pool.begin().await?;

        sqlx::query!(
        "
            DELETE FROM rcompeticionesequipos WHERE idCompeticion = ? AND idEquipo = ?
        ",
            idCompeticion,
            idEquipo
        )
        .execute(txn.as_mut())
        .await?;

        txn.commit().await?;

        Ok(CompeticionEquipo{idCompeticion, idEquipo})
    }
}
