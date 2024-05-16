use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{delete, post};
use axum::{Form, Router};
use serde::Deserialize;
use time::macros::format_description;

use crate::ctx::Ctx;
use crate::modelo::ControladorModelo;
use crate::modelo::eventos::{ControladorEvento, EventoActualizar, EventoCrear};
use crate::{Result,Error};

pub fn routes(cm: ControladorModelo) -> Router {
    Router::new()
        .route("/eventos", post(crear_evento).put(actualizar_evento))
        .route("/eventos/:id", delete(eliminar_evento))
        .with_state(cm)
}

#[derive(Deserialize)]
struct PayloadCrear {
    valor: Option<u16>,
    minuto: Option<u32>,
    segundo: Option<u32>,
    idTipoEvento: u32,
    idJugador: u32,
    idPartido: u32
}

async fn crear_evento(
    State(cm): State<ControladorModelo>,
    _ctx: Ctx,
    Form(evento_crear): Form<PayloadCrear>
) -> Result<StatusCode> {
    let evento = EventoCrear {
        valor: evento_crear.valor,
        minuto: match (evento_crear.minuto, evento_crear.segundo) {
            (Some(minuto), Some(segundo)) => Some(time::Time::parse(
                format!("00:{:0>2}:{:0>2}", minuto, segundo).as_str(), 
                format_description!("[hour]:[minute]:[second]"))
                .map_err(|err| Error::Generico{error:format!("Error parseando: {}",err)})?),
            _ => None,
        },
        idTipoEvento: evento_crear.idTipoEvento,
        idJugador: evento_crear.idJugador,
        idPartido: evento_crear.idPartido
    };

    let _evento = ControladorEvento::crear_evento(cm, evento).await?;

    Ok(StatusCode::CREATED)
}

#[derive(Deserialize)]
struct PayloadActualizar {
    id: u32,
    valor: Option<u16>,
    minuto: Option<u32>,
    segundo: Option<u32>,
    idTipoEvento: u32,
    idJugador: u32,
    idPartido: u32
}

async fn actualizar_evento(
    State(cm): State<ControladorModelo>,
    _ctx: Ctx,
    Form(evento_actualizar): Form<PayloadActualizar>
) -> Result<StatusCode> {
    let evento = EventoActualizar {
        id: evento_actualizar.id,
        valor: evento_actualizar.valor,
        minuto: match (evento_actualizar.minuto, evento_actualizar.segundo) {
            (Some(minuto), Some(segundo)) => Some(time::Time::parse(
                format!("00:{:0>2}:{:0>2}", minuto, segundo).as_str(), 
                format_description!("[hour]:[minute]:[second]"))
                .map_err(|err| Error::Generico{error:format!("Error parseando: {}",err)})?),
            _ => None,
        },
        idTipoEvento: evento_actualizar.idTipoEvento,
        idJugador: evento_actualizar.idJugador,
        idPartido: evento_actualizar.idPartido
    };

    let _evento = ControladorEvento::actualizar_evento(cm, evento).await?;

    Ok(StatusCode::CREATED)
}

async fn eliminar_evento(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
    Path(id): Path<u32>
) -> Result<StatusCode> {
    let _evento = ControladorEvento::eliminar_evento(ctx, cm, id).await?;

    Ok(StatusCode::OK)
}
