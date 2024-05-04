use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{delete, get, post};
use axum::{Form, Router};
use axum::response::{IntoResponse, Response};
use axum::debug_handler;
use askama::Template;

use crate::ctx::Ctx;
use crate::modelo::{ControladorModelo};
use crate::modelo::usuario::{Usuario, UsuarioCrear, ControladorUsuario};
use crate::{Result, Error};

pub fn routes(cm: ControladorModelo) -> Router {
    Router::new()
        .route("/", get(inicio))
        .with_state(cm)
}

#[derive(Template)]
#[template(path = "inicio.html")]
struct InicioTemplate;

async fn inicio() -> InicioTemplate {
    return InicioTemplate;
}
