use serde::Deserialize;
use time::{Date, Time};
use crate::ctx::Ctx;
use crate::modelo::pwd::hash_password;
use crate::modelo::{Error, Result};
use crate::modelo::ControladorModelo;


#[derive(Clone, Debug, sqlx::FromRow)]
pub struct Evento {
    pub id: u32,
    pub valor: Option<u16>,
    pub minuto: Option<Time>,
    pub idTipoEvento: u32,
    pub idJugador: u32,
    pub idPartido: u32
}

#[derive(Clone, Debug)]
pub struct EstadisticasJugador {
    pub partidos_jugados: u32,
    pub goles: u32,
    pub asistencias: u32,
    pub faltas: u32,
    pub puntos: u32,
    pub goles_contra: u32,
    pub tiros_recibidos: u32,
    pub porcentaje_parada: u32,
    pub minutos_sancion: u32
}

#[derive(Deserialize)]
pub struct EventoCrear {
    pub valor: Option<u16>,
    pub minuto: Option<Time>,
    pub idTipoEvento: u32,
    pub idJugador: u32,
    pub idPartido: u32
}

#[derive(Deserialize)]
pub struct EventoActualizar {
    pub id: u32,
    pub valor: Option<u16>,
    pub minuto: Option<Time>,
    pub idTipoEvento: u32,
    pub idJugador: u32,
    pub idPartido: u32
}

pub struct ControladorEvento;


impl ControladorEvento {
    pub async fn crear_evento(
        cm: ControladorModelo, 
        evento: EventoCrear
    ) -> Result<Evento> {
        let pool = cm.conexion;
        let mut txn = pool.begin().await?;

        sqlx::query!("
            INSERT INTO teventos (valor, minuto, idTipoEvento, idJugador, idPartido) 
            VALUES (?, ?, ?, ?, ?); 
        ",
            evento.valor,
            evento.minuto,
            evento.idTipoEvento,
            evento.idJugador,
            evento.idPartido,
        )
        .execute(txn.as_mut())
        .await?;

        let evento = sqlx::query_as!(
        Evento,
        "
            SELECT id, valor, minuto, idTipoEvento, idJugador, idPartido FROM teventos
            WHERE id = LAST_INSERT_ID();
        ")
        .fetch_one(txn.as_mut())
        .await?;

        txn.commit().await?;

        Ok(evento)
    }

    pub async fn actualizar_evento(
        cm: ControladorModelo, 
        evento: EventoActualizar
    ) -> Result<Evento> {
        let pool = cm.conexion;
        let mut txn = pool.begin().await?;

        sqlx::query!("
            UPDATE teventos SET valor = ?, minuto = ?, idTipoEvento = ?, idJugador = ?, idPartido = ?
            WHERE id = ?
        ",
            evento.valor,
            evento.minuto,
            evento.idTipoEvento,
            evento.idJugador,
            evento.idPartido,
            evento.id
        )
        .execute(txn.as_mut())
        .await?;

        let evento = sqlx::query_as!(
        Evento,
        "
            SELECT id, valor, minuto, idTipoEvento, idJugador, idPartido FROM teventos
            WHERE id = ?;
        ",
            evento.id
        )
        .fetch_one(txn.as_mut())
        .await?;

        txn.commit().await?;

        Ok(evento)
    }

    pub async fn listar_eventos(
        ctx: Ctx,
        cm: ControladorModelo
    ) -> Result<Vec<Evento>> {
        let pool = cm.conexion;

        let eventos: Vec<Evento> = sqlx::query_as("
            SELECT id, valor, minuto, idTipoEvento, idJugador, idPartido FROM teventos
        ")
        .fetch_all(&pool)
        .await?;

        Ok(eventos)
    }

    pub async fn eliminar_evento(
        ctx: Ctx,
        cm: ControladorModelo, 
        id: u32
    ) -> Result<Evento> {
        let pool = cm.conexion;
        let mut txn = pool.begin().await?;

        let evento = sqlx::query_as!(
        Evento,
        "
            SELECT id, valor, minuto, idTipoEvento, idJugador, idPartido FROM teventos
            WHERE id = ?;
        ",
            id
        )
        .fetch_optional(txn.as_mut())
        .await?
        .ok_or(Error::NoEncontrado { id })?;

        sqlx::query!(
        "
            DELETE FROM teventos WHERE id = ?
        ",
            id
        )
        .execute(txn.as_mut())
        .await?;

        txn.commit().await?;

        Ok(evento)
    }

    pub async fn evento_id(cm: ControladorModelo, id: u32) -> Result<Option<Evento>> {
        let pool = cm.conexion;

        let evento = sqlx::query_as!(
        Evento,
        "
            SELECT id, valor, minuto, idTipoEvento, idJugador, idPartido FROM teventos
            WHERE id = ?
        ",
            id
        )
        .fetch_optional(&pool)
        .await?;

        Ok(evento)
    }

    pub async fn estadisticas_jugador_id(
        cm: ControladorModelo, 
        id: u32
    ) -> Result<EstadisticasJugador> {
        let pool = cm.conexion;

        // Partido jugado = 1
        let partidos_jugados = sqlx::query!(
        "
            SELECT COUNT(*) as `partidos_jugados: u32` FROM teventos
            WHERE idTipoEvento = 1 AND idJugador = ?
        ",
            id
        )
        .fetch_one(&pool)
        .await?.partidos_jugados;

        // Gol = 2
        let goles = sqlx::query!(
        "
            SELECT COUNT(*) as `goles: u32` FROM teventos
            WHERE idTipoEvento = 2 AND idJugador = ?
        ",
            id
        )
        .fetch_one(&pool)
        .await?.goles;

        // Tiros recibidos = 3
        let tiros_recibidos = sqlx::query!(
        "
            SELECT CAST(SUM(valor) AS UNSIGNED) as `tiros_recibidos: u32` FROM teventos
            WHERE idTipoEvento = 3 AND idJugador = ?
        ",
            id
        )
        .fetch_one(&pool)
        .await?.tiros_recibidos;

        // Asistencia = 4
        let asistencias = sqlx::query!(
        "
            SELECT COUNT(*) as `asistencias: u32` FROM teventos
            WHERE idTipoEvento = 4 AND idJugador = ?
        ",
            id
        )
        .fetch_one(&pool)
        .await?.asistencias;

        // Faltas = 5
        let faltas = sqlx::query!(
        "
            SELECT COUNT(*) as `faltas: u32` FROM teventos
            WHERE idTipoEvento = 5 AND idJugador = ?
        ",
            id
        )
        .fetch_one(&pool)
        .await?.faltas;

        // Goles en contra
        let goles_contra = sqlx::query!(
        "
            SELECT COUNT(*) as `goles_contra: u32` FROM teventos
            WHERE teventos.id = 5 AND 
                idPartido IN (SELECT id FROM tpartidos WHERE 
                    idEquipoCasa=(SELECT idEquipo FROM tjugadores WHERE id=?) OR idEquipoVisitante=(SELECT idEquipo FROM tjugadores WHERE id=?))
                AND idJugador != ?
        ",
            id,
            id,
            id
        )
        .fetch_one(&pool)
        .await?.goles_contra;

        // Faltas = 5
        let minutos_sancion = sqlx::query!(
        "
            SELECT CAST(SUM(valor) AS UNSIGNED) as `minutos_sancion: u32` FROM teventos
            WHERE idTipoEvento = 5 AND idJugador = ?;
        ",
            id
        )
        .fetch_one(&pool)
        .await?.minutos_sancion;

        let estadisticas_jugador = EstadisticasJugador {
            partidos_jugados,
            goles,
            tiros_recibidos: match tiros_recibidos {
                Some(n) => n,
                None => 0
           },
            asistencias,
            faltas,
            puntos: goles + asistencias,
            goles_contra,
            porcentaje_parada: match tiros_recibidos {
                Some(0) => 0,
                Some(tiros) =>100-((f64::from(goles_contra)/f64::from(tiros))*100f64) as u32,
                None => 0
            },
            minutos_sancion: match minutos_sancion {
                Some(min) => min,
                None => 0
            }
        };

        Ok(estadisticas_jugador)
    }

    pub async fn resultado_partido_id(
        cm: ControladorModelo, 
        id: u32
    ) -> Result<(i64, i64)> {
        let pool = cm.conexion;

        let goles_casa = sqlx::query!(
        "
            SELECT COUNT(*) as goles_casa FROM teventos
            WHERE idTipoEvento = 2 AND idPartido = ? AND idJugador IN (SELECT id FROM tjugadores WHERE idEquipo = (SELECT idEquipoCasa FROM tpartidos WHERE id = teventos.idPartido))
        ",
            id
        )
        .fetch_one(&pool)
        .await?.goles_casa;

        let goles_visitante = sqlx::query!(
        "
            SELECT COUNT(*) as goles_visitante FROM teventos
            WHERE idTipoEvento = 2 AND idPartido = ? AND idJugador IN (SELECT id FROM tjugadores WHERE idEquipo = (SELECT idEquipoVisitante FROM tpartidos WHERE id = teventos.idPartido))
        ",
            id
        )
        .fetch_one(&pool)
        .await?.goles_visitante;

        Ok((goles_casa, goles_visitante))
    }
}
