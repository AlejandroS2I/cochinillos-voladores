use serde::Deserialize;
use time::Date;
use crate::ctx::Ctx;
use crate::modelo::{Error, Result};
use crate::modelo::ControladorModelo;


#[derive(Clone, Debug, sqlx::FromRow)]
pub struct Jugador {
    pub id: u32,
    pub numero: u32,
    pub nombre: String,
    pub apellido1: String,
    pub apellido2: String,
    pub nacimiento: Date,
    pub fotoURL: Option<String>,
    pub idTipoJugador: u32,
    pub idEquipo: u32,
}

#[derive(Deserialize)]
pub struct JugadorCrear {
    pub numero: u32,
    pub nombre: String,
    pub apellido1: String,
    pub apellido2: String,
    pub nacimiento: Date,
    pub fotoURL: Option<String>,
    pub idTipoJugador: u32,
    pub idEquipo: u32,
}

#[derive(Deserialize)]
pub struct JugadorActualizar {
    pub id: u32,
    pub numero: u32,
    pub nombre: String,
    pub apellido1: String,
    pub apellido2: String,
    pub nacimiento: Date,
    pub fotoURL: Option<String>,
    pub idTipoJugador: u32,
    pub idEquipo: u32,
}

pub struct ControladorJugador;


impl ControladorJugador {
    pub async fn crear_jugador(
        cm: ControladorModelo, 
        jugador: JugadorCrear
    ) -> Result<Jugador> {
        let pool = cm.conexion;
        let mut txn = pool.begin().await?;

        sqlx::query!("
            INSERT INTO tjugadores (numero, nombre, apellido1, apellido2, nacimiento, fotoURL, idTipoJugador, idEquipo) 
            VALUES (?, ?, ?, ?, ?, ? ,? ,?); 
        ",
            jugador.numero,
            jugador.nombre,
            jugador.apellido1,
            jugador.apellido2,
            jugador.nacimiento,
            jugador.fotoURL,
            jugador.idTipoJugador,
            jugador.idEquipo,
        )
        .execute(txn.as_mut())
        .await?;

        let jugador = sqlx::query_as!(
        Jugador,
        "
            SELECT id, numero, nombre, apellido1, apellido2, nacimiento, fotoURL, idTipoJugador, idEquipo FROM tjugadores
            WHERE id = LAST_INSERT_ID();
        ")
        .fetch_one(txn.as_mut())
        .await?;

        txn.commit().await?;

        Ok(jugador)
    }

    pub async fn actualizar_jugador(
        cm: ControladorModelo, 
        jugador: JugadorActualizar
    ) -> Result<Jugador> {
        let pool = cm.conexion;
        let mut txn = pool.begin().await?;

        match jugador.fotoURL {
            Some(url) => {
                sqlx::query!("
                    UPDATE tjugadores SET numero = ?, nombre = ?, apellido1 = ?, apellido2 = ?, nacimiento = ?, fotoURL = ?, idTipoJugador = ?, idEquipo = ?
                    WHERE id = ?
                ",
                    jugador.numero,
                    jugador.nombre,
                    jugador.apellido1,
                    jugador.apellido2,
                    jugador.nacimiento,
                    url,
                    jugador.idTipoJugador,
                    jugador.idEquipo,
                    jugador.id
                )
                .execute(txn.as_mut())
                .await?;
            },
            None => {
                sqlx::query!("
                    UPDATE tjugadores SET numero = ?, nombre = ?, apellido1 = ?, apellido2 = ?, nacimiento = ?, idTipoJugador = ?, idEquipo = ?
                    WHERE id = ?
                ",
                    jugador.numero,
                    jugador.nombre,
                    jugador.apellido1,
                    jugador.apellido2,
                    jugador.nacimiento,
                    jugador.idTipoJugador,
                    jugador.idEquipo,
                    jugador.id
                )
                .execute(txn.as_mut())
                .await?;
            }
        }

        let jugador = sqlx::query_as!(
        Jugador,
        "
            SELECT id, numero, nombre, apellido1, apellido2, nacimiento, fotoURL, idTipoJugador, idEquipo FROM tjugadores
            WHERE id = ?;
        ",
            jugador.id
        )
        .fetch_one(txn.as_mut())
        .await?;

        txn.commit().await?;

        Ok(jugador)
    }

    pub async fn listar_jugadores(
        cm: ControladorModelo
    ) -> Result<Vec<Jugador>> {
        let pool = cm.conexion;

        let jugadores = sqlx::query_as!(
            Jugador,
            "
                SELECT id, numero, nombre, apellido1, apellido2, nacimiento, fotoURL, idTipoJugador, idEquipo FROM tjugadores
            ",
        )
        .fetch_all(&pool)
        .await?;

        Ok(jugadores)
    }

    pub async fn eliminar_jugador(
        ctx: Ctx,
        cm: ControladorModelo, 
        id: u32
    ) -> Result<Jugador> {
        let pool = cm.conexion;
        let mut txn = pool.begin().await?;

        let jugador = sqlx::query_as!(
        Jugador,
        "
            SELECT id, numero, nombre, apellido1, apellido2, nacimiento, fotoURL, idTipoJugador, idEquipo
            FROM tjugadores WHERE id = ?;
        ",
            id
        )
        .fetch_optional(txn.as_mut())
        .await?
        .ok_or(Error::NoEncontrado { id })?;

        sqlx::query!(
        "
            DELETE FROM tjugadores WHERE id = ?
        ",
            id
        )
        .execute(txn.as_mut())
        .await?;

        txn.commit().await?;

        Ok(jugador)
    }

    pub async fn jugador_id(cm: ControladorModelo, id: u32) -> Result<Option<Jugador>> {
        let pool = cm.conexion;

        let jugador = sqlx::query_as!(
        Jugador,
        "
            SELECT id, numero, nombre, apellido1, apellido2, nacimiento, fotoURL, idTipoJugador, idEquipo FROM tjugadores
            WHERE id = ?
        ",
            id
        )
        .fetch_optional(&pool)
        .await?;

        Ok(jugador)
    }
}
