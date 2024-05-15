use axum::extract::{Path, State};
use axum::routing::{delete, get, post};
use axum::{Form, Router};
use axum::response::{IntoResponse, Response};
use askama::Template;

use crate::ctx::Ctx;
use crate::modelo::{ControladorModelo};
use crate::{Result, Error};

pub fn routes(cm: ControladorModelo) -> Router {
    Router::new()
        .route("/formulario", get(formulario))
        .with_state(cm)
}

#[derive(Template)]
#[template(path = "formulario.html")]
struct FormularioTemplate {
    iniciado: bool
}

async fn formulario(
    State(cm): State<ControladorModelo>,
    ctx: Option<Ctx>,
) -> Result<FormularioTemplate> {
    Ok(FormularioTemplate { iniciado: ctx.is_some() })
}
