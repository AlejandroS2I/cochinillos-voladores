use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{delete, get, post};
use axum::{Form, Router};
use axum::response::{IntoResponse, Response};
use axum::debug_handler;
use askama::Template;

use crate::ctx::Ctx;
use crate::modelo::{ControladorModelo};
use crate::modelo::noticias::{ControladorNoticia, Noticia};
use crate::{Result, Error};

pub fn routes(cm: ControladorModelo) -> Router {
    Router::new()
        .route("/periodicos", get(lista_periodicos))
        .with_state(cm)
}

#[derive(Template)]
#[template(path = "componentes/periodicos.html")]
struct PeriodicosTemplate {
    periodicos: Vec<Noticia>
}

async fn lista_periodicos(
    State(cm): State<ControladorModelo>,
    ctx: Option<Ctx>,
) -> Result<PeriodicosTemplate> {
    let periodicos = ControladorNoticia::listar_noticias(cm, Some(5)).await?;
    Ok(PeriodicosTemplate { periodicos })
}
