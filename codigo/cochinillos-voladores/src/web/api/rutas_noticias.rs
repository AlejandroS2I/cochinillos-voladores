use axum::body::Bytes;
use axum::extract::{Multipart, Path, State};
use axum::http::StatusCode;
use axum::routing::{delete, get, post, put};
use axum::{Form, Router};
use axum::response::{IntoResponse, Response};
use axum::debug_handler;
use askama::Template;
use serde::Deserialize;
use time::macros::format_description;
use time::{Date, OffsetDateTime};

use crate::ctx::Ctx;
use crate::modelo::pwd::verificar_password;
use crate::modelo::{ControladorModelo};
use crate::modelo::noticias::{ControladorNoticia, Noticia, NoticiaActualizar, NoticiaCrear};
use crate::uploads::{eliminar_archivo, subir_archivo};
use crate::{Result, Error};

pub fn routes(cm: ControladorModelo) -> Router {
    Router::new()
        .route("/noticias", post(crear_noticia).put(actualizar_noticia))
        .route("/noticias/:id", delete(eliminar_noticia))
        .with_state(cm)
}

struct PayloadCrear {
    titulo: String,
    descripcion: String,
    fecha: Date,
    nombre_archivo: Option<String>,
    archivo: Option<Bytes>
}

async fn crear_noticia(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
    mut multipart: Multipart,
) -> Result<StatusCode> {
    let mut payload_crear = PayloadCrear {
        titulo: "".to_string(),
        descripcion: "".to_string(),
        fecha: OffsetDateTime::now_utc().date(),
        nombre_archivo: None,
        archivo: None
    };
    let mut campos_vacios = Vec::new();

    while let Some(campo) = multipart.next_field().await? {
        let nombre = campo.name().ok_or(Error::FormularioInvalido { error: format!("No se ha enviado el campo: {:?}", campo) })?;

        match nombre {
            "imagen" => {
                 payload_crear.nombre_archivo = campo
                    .file_name()
                    .map(|nombre| nombre.to_string());

                if payload_crear.nombre_archivo.is_some() {
                    payload_crear.archivo = Some(campo.bytes().await?);
                }
            },
            "titulo" => {
                payload_crear.titulo = campo.text().await.map_err(|e| {
                    Error::FormularioInvalido{error: format!("No se ha encontrado el campo titulo: {}", e)}
                })?;
            },
            "descripcion" => {
                payload_crear.descripcion = campo.text().await.map_err(|e| {
                    Error::FormularioInvalido{error: format!("No se ha encontrado el campo descripcion: {}", e)}
                })?;
            },
            "fecha" => {
                let fecha = campo.text().await.map_err(|e| {
                    Error::FormularioInvalido{error: format!("No se ha encontrado el campo fecha: {}", e)}
                })?;

                fecha.is_empty().then(|| { campos_vacios.push(format!("Fecha"))});

                payload_crear.fecha = Date::parse(&fecha, format_description!("[year]-[month]-[day]"))
                    .map_err(|e| Error::Generico{ error: e.to_string() })?;
            },
            _ => {}
        }
    };

    payload_crear.titulo.is_empty().then(|| { campos_vacios.push(format!("Titulo"))});
    payload_crear.descripcion.is_empty().then(|| { campos_vacios.push(format!("Descripción"))});

    if campos_vacios.len() > 0 {
        return Err(Error::CamposVacios { campos: campos_vacios });
    };

    let mut noticia_crear = NoticiaCrear {
        titulo: payload_crear.titulo,
        descripcion: payload_crear.descripcion,
        fecha: payload_crear.fecha,
        fotoURL: None
    };

    noticia_crear.fotoURL = match (payload_crear.nombre_archivo, payload_crear.archivo) {
        (Some(nombre), Some(archivo)) => Some(subir_archivo("noticias".to_string(), archivo, nombre).await?),
        _ => None
    };

    let noticia = ControladorNoticia::crear_noticia(cm, noticia_crear).await?;

    Ok(StatusCode::CREATED)
}

struct PayloadActualizar {
    id: u32,
    titulo: String,
    descripcion: String,
    fecha: Date,
    nombre_archivo: Option<String>,
    archivo: Option<Bytes>
}

async fn actualizar_noticia(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
    mut multipart: Multipart,
) -> Result<StatusCode> {
    let mut payload_actualizar = PayloadActualizar {
        id: 0,
        titulo: "".to_string(),
        descripcion: "".to_string(),
        fecha: OffsetDateTime::now_utc().date(),
        nombre_archivo: None,
        archivo: None
    };
    let mut campos_vacios = Vec::new();

    while let Some(campo) = multipart.next_field().await? {
        let nombre = campo.name().ok_or(Error::FormularioInvalido { error: format!("No se ha enviado el campo: {:?}", campo) })?;

        match nombre {
            "imagen" => {
                 payload_actualizar.nombre_archivo = campo
                    .file_name()
                    .map(|nombre| nombre.to_string());

                if payload_actualizar.nombre_archivo.is_some() {
                    payload_actualizar.archivo = Some(campo.bytes().await?);
                }
            },
            "titulo" => {
                payload_actualizar.titulo = campo.text().await.map_err(|e| {
                    Error::FormularioInvalido{error: format!("No se ha encontrado el campo titulo: {}", e)}
                })?;
            },
            "descripcion" => {
                payload_actualizar.descripcion = campo.text().await.map_err(|e| {
                    Error::FormularioInvalido{error: format!("No se ha encontrado el campo descripcion: {}", e)}
                })?;
            },
            "fecha" => {
                let fecha = campo.text().await.map_err(|e| {
                    Error::FormularioInvalido{error: format!("No se ha encontrado el campo fecha: {}", e)}
                })?;

                fecha.is_empty().then(|| { campos_vacios.push(format!("Fecha"))});

                payload_actualizar.fecha = Date::parse(&fecha, format_description!("[year]-[month]-[day]"))
                    .map_err(|e| Error::Generico{ error: e.to_string() })?;
            },
            "id" => {
                payload_actualizar.id = campo.text().await.map_err(|e| {
                    Error::FormularioInvalido{error: format!("No se ha encontrado el campo id: {}", e)}
                })?.parse().map_err(|e| {
                    Error::FormularioInvalido{error: format!("No se ha podido parsear el campo id: {}", e)}
                })?;
            },
            _ => {}
        }
    };

    payload_actualizar.titulo.is_empty().then(|| { campos_vacios.push(format!("Titulo"))});
    payload_actualizar.descripcion.is_empty().then(|| { campos_vacios.push(format!("Descripción"))});

    if campos_vacios.len() > 0 {
        return Err(Error::CamposVacios { campos: campos_vacios });
    };

    let mut noticia_actualizar = NoticiaActualizar {
        id: payload_actualizar.id,
        titulo: payload_actualizar.titulo,
        descripcion: payload_actualizar.descripcion,
        fecha: payload_actualizar.fecha,
        fotoURL: None
    };

    noticia_actualizar.fotoURL = match (payload_actualizar.nombre_archivo, payload_actualizar.archivo) {
        (Some(nombre), Some(archivo)) => Some(subir_archivo("noticias".to_string(), archivo, nombre).await?),
        _ => None
    };

    if noticia_actualizar.fotoURL.is_some() {
        let noticia = ControladorNoticia::noticia_id(cm.clone(), noticia_actualizar.id).await?.ok_or(Error::NoEncontradoPorId)?;
        if let Some(url) = noticia.fotoURL {
            eliminar_archivo(url).await?;
        }
    }

    let noticia = ControladorNoticia::actualizar_noticia(cm, noticia_actualizar).await?;

    Ok(StatusCode::CREATED)
}

async fn eliminar_noticia(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
    Path(id): Path<u32>
) -> Result<StatusCode> {
    let noticia = ControladorNoticia::eliminar_noticia(ctx, cm, id).await?;

    if let Some(url) = noticia.fotoURL {
        eliminar_archivo(url).await?;
    }

    Ok(StatusCode::OK)
}
