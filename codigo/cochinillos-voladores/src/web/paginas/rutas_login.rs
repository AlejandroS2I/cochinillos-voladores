use axum::extract::{Path, State};
use axum::http::{StatusCode, Uri};
use axum::routing::{delete, get, post};
use axum::{Form, Router};
use axum::response::{IntoResponse, Redirect, Response};
use axum::debug_handler;
use askama::Template;
use axum_htmx::HxRedirect;

use crate::ctx::Ctx;
use crate::modelo::usuario::Usuario;
use crate::modelo::ControladorModelo;
use crate::{Result, Error};

pub fn routes(cm: ControladorModelo) -> Router {
    Router::new()
        .route("/perfil", get(perfil))
        .route("/login", get(login))
        .route("/registrar", get(registrar))
        .with_state(cm)
}

#[derive(Template)]
#[template(path = "perfil.html")]
struct PerfilTemplate {
    usuario: Usuario
}

async fn perfil(
    State(_cm): State<ControladorModelo>,
    ctx: Option<Ctx>,
) -> impl IntoResponse {
    match ctx {
        Some(ctx) => PerfilTemplate { usuario: ctx.usuario() }.into_response(),
        None => (HxRedirect(Uri::from_static("/login")), Redirect::to("/login")).into_response()
    }
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate;

async fn login(
    State(_cm): State<ControladorModelo>,
    ctx: Option<Ctx>,
) -> impl IntoResponse {
    if let Some(_) = ctx { return (HxRedirect(Uri::from_static("/perfil")), Redirect::to("/perfil")).into_response(); }
    LoginTemplate.into_response()
}

#[derive(Template)]
#[template(path = "registrar.html")]
struct RegistrarTemplate;

async fn registrar(
    State(_cm): State<ControladorModelo>,
    ctx: Option<Ctx>,
) -> impl IntoResponse {
    if let Some(_) = ctx { return (HxRedirect(Uri::from_static("/perfil")), Redirect::to("/perfil")).into_response(); }
    RegistrarTemplate.into_response()
}
