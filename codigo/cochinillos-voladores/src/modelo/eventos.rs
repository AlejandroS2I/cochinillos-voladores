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
}
