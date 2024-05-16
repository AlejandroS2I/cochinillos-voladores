use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{delete, get, post, put};
use axum::{Form, Router};
use axum::response::{IntoResponse, Response};
use axum::debug_handler;
use askama::Template;
use serde::Deserialize;

use crate::ctx::Ctx;
use crate::modelo::pwd::verificar_password;
use crate::modelo::{ControladorModelo};
use crate::modelo::tipos_jugador::{ControladorTipoJugador, TipoJugador, TipoJugadorActualizar, TipoJugadorCrear};
use crate::{Result, Error};

pub fn routes(cm: ControladorModelo) -> Router {
    Router::new()
        .route("/tiposJugador", post(crear_tipo_jugador).put(actualizar_tipo_jugador))
        .route("/tiposJugador/:id", delete(eliminar_tipo_jugador))
        .with_state(cm)
}

async fn crear_tipo_jugador(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
    Form(tipo_jugador_crear): Form<TipoJugadorCrear>
) -> Result<StatusCode> {
    let tipo_jugador = ControladorTipoJugador::crear_tipo_jugador(cm, tipo_jugador_crear).await?;

    Ok(StatusCode::CREATED)
}

async fn actualizar_tipo_jugador(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
    Form(tipo_jugador_actualizar): Form<TipoJugadorActualizar>
) -> Result<StatusCode> {
    let tipo_jugador = ControladorTipoJugador::actualizar_tipo_jugador(cm, tipo_jugador_actualizar).await?;

    Ok(StatusCode::CREATED)
}

async fn eliminar_tipo_jugador(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
    Path(id): Path<u32>
) -> Result<StatusCode> {
    let tipo_jugador = ControladorTipoJugador::eliminar_tipo_jugador(ctx, cm, id).await?;

    Ok(StatusCode::OK)
}
