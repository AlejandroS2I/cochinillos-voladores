use axum::extract::{Path, State};
use axum::http::{StatusCode, Uri};
use axum::routing::{delete, get, post};
use axum::{Form, Router};
use axum::response::{IntoResponse, Redirect, Response};
use axum::debug_handler;
use askama::Template;
use axum_htmx::HxRedirect;
use serde::Deserialize;

use crate::ctx::Ctx;
use crate::modelo::usuario::{ControladorUsuario, Usuario};
use crate::modelo::ControladorModelo;
use crate::{Result, Error};

pub fn routes(cm: ControladorModelo) -> Router {
    let gestiones = Router::new()
        .route("/menu", get(menu))
        .route("/usuarios", get(lista_usuarios))
        .route("/usuarios/:id", get(gestion_usuarios))
        .with_state(cm.clone());
    
    Router::new()
        .route("/panelControl", get(panelControl))
        .nest("/gestion", gestiones)
        .with_state(cm)
}

pub const SECCIONES_CONTROL: &'static [&'static Seccion] = &[
    &Seccion {
        url: "usuarios",
        titulo: "Usuarios"
    },
    &Seccion {
        url: "blogs",
        titulo: "Blogs"
    }
];

#[derive(Debug, Deserialize)]
pub struct Seccion<'a> {
    url: &'a str,
    titulo: &'a str,
}

#[derive(Template)]
#[template(path = "panelControl.html")]
struct PanelControlTemplate;

async fn panelControl(
    ctx: Option<Ctx>,
) -> impl IntoResponse {
    match ctx {
        Some(_) => PanelControlTemplate.into_response(),
        None => (HxRedirect(Uri::from_static("/login")), Redirect::to("/login")).into_response()
    }
}

// Gestiones

#[derive(Debug, Deserialize)]
pub struct RegistroListaGestion {
    id: u32,
    titulo: String,
    valores: Vec<(String, String)>
}

#[derive(Template)]
#[template(path = "componentes/listaGestion.html")]
struct ListaGestionTemplate {
    url: String,
    lista: Vec<RegistroListaGestion>
}

#[derive(Debug, Deserialize)]
pub struct CampoGestion {
    titulo: String,
    nombre: String,
    valor: String
}

#[derive(Template)]
#[template(path = "componentes/gestion.html")]
struct GestionTemplate {
    id: u32,
    url: String,
    campos: Vec<CampoGestion>
}

#[derive(Template)]
#[template(path = "componentes/menuGestion.html")]
struct MenuGestionTemplate<'a> {
    secciones: Vec<&'a Seccion<'a>>
}

async fn menu<'a>(
    State(_cm): State<ControladorModelo>,
    ctx: Option<Ctx>,
) -> impl IntoResponse {
    MenuGestionTemplate { secciones: SECCIONES_CONTROL.to_vec() }
}

async fn lista_usuarios(
    State(cm): State<ControladorModelo>,
    ctx: Option<Ctx>,
) -> Result<ListaGestionTemplate> {
    let usuarios = ControladorUsuario::listar_usuarios(ctx.ok_or(Error::SinPermisos)?, cm).await?;

    Ok(ListaGestionTemplate {
        url: format!("usuarios"),
        lista: usuarios.iter().map(|usuario| {
            RegistroListaGestion {
                id: usuario.id.clone(),
                titulo: usuario.mail.clone(),
                valores: vec![
                    (format!("Nombre"), usuario.nombre.clone()),
                    match usuario.esAdministrador {
                        true => (format!("Es administrador"), format!("Si")),
                        false => (format!("Es administrador"), format!("No"))
                    }
                ]
            }
        }).collect()
    })
}

async fn gestion_usuarios(
    State(cm): State<ControladorModelo>,
    ctx: Option<Ctx>,
    Path(id): Path<u32>
) -> Result<GestionTemplate> {
    let usuario = ControladorUsuario::usuario_id(cm, id).await?.ok_or(Error::NoEncontradoPorId)?;

    Ok(GestionTemplate {
        id,
        url: format!("usuarios"),
        campos: vec![
            CampoGestion {
                titulo: format!("Nombre"),
                nombre: format!("nombre"),
                valor: usuario.nombre
            },
            CampoGestion {
                titulo: format!("Mail"),
                nombre: format!("mail"),
                valor: usuario.mail
            },
        ]
    })
}
