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
        .route("/perfil", get(perfil))
        .route("/login", get(login))
        .route("/registrar", get(registrar))
        .with_state(cm)
}

#[derive(Template)]
#[template(path = "componentes/perfil.html")]
struct PerfilTemplate{
    tieneCuenta: bool
}

async fn perfil(
    State(_cm): State<ControladorModelo>,
    ctx: Option<Ctx>,
) -> PerfilTemplate {
    match ctx {
        Some(_) => PerfilTemplate { tieneCuenta: true },
        None => PerfilTemplate { tieneCuenta: false }
    }
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate;

async fn login(
    State(cm): State<ControladorModelo>,
    ctx: Option<Ctx>,
) -> LoginTemplate {
    LoginTemplate
}

#[derive(Template)]
#[template(path = "registrar.html")]
struct RegistrarTemplate;

async fn registrar(
    State(cm): State<ControladorModelo>,
    ctx: Option<Ctx>,
) -> RegistrarTemplate {
    RegistrarTemplate
}
