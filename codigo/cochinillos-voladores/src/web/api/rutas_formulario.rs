use axum::body::Bytes;
use axum::extract::{Multipart, Path, State};
use axum::http::StatusCode;
use axum::routing::{delete, get, post, put};
use axum::{Form, Router};
use axum::response::{Html, IntoResponse, Response};
use axum::debug_handler;
use askama::Template;
use serde::Deserialize;
use time::macros::format_description;
use time::{Date, OffsetDateTime};

use crate::ctx::Ctx;
use crate::mail::enviar_mail;
use crate::modelo::pwd::verificar_password;
use crate::modelo::usuario::ControladorUsuario;
use crate::modelo::{ControladorModelo};
use crate::modelo::noticias::{ControladorNoticia, Noticia, NoticiaActualizar, NoticiaCrear};
use crate::uploads::{eliminar_archivo, subir_archivo};
use crate::{Result, Error};

pub fn routes(cm: ControladorModelo) -> Router {
    Router::new()
        .route("/apuntarse", post(apuntarse))
        .with_state(cm)
}

#[derive(Template)]
#[template(path = "componentes/mail_apuntado.html")]
struct MailTemplate {
    nombre: String,
    apellidos: String,
    edad: u32,
    email: String,
    telefono: String,
    url_fotografia: String
}

#[derive(Template)]
#[template(path = "componentes/formulario_completado.html")]
struct ExitoTemplate;

struct PayloadApuntarse {
    nombre: String,
    apellidos: String,
    edad: u32,
    email: String,
    telefono: String,
    nombre_fotografia: String,
    fotografia: Vec<u8>
}

async fn apuntarse(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
    mut multipart: Multipart,
) -> Result<impl IntoResponse> {
    let mut payload_apuntarse = PayloadApuntarse {
        nombre: "".to_string(),
        apellidos: "".to_string(),
        edad: 0,
        email: "".to_string(),
        telefono: "".to_string(),
        nombre_fotografia: "".to_string(),
        fotografia: Vec::new()
    };
    let mut campos_vacios = Vec::new();

    while let Some(campo) = multipart.next_field().await? {
        let nombre = campo.name().ok_or(Error::FormularioInvalido { error: format!("No se ha enviado el campo: {:?}", campo) })?;

        match nombre {
            "imagen" => {
                payload_apuntarse.nombre_fotografia = campo
                    .file_name().ok_or(Error::FormularioInvalido{error: format!("No se ha encontrado el campo nombre archivo")})?.to_string();

                payload_apuntarse.fotografia = campo.bytes().await?.to_vec();
            },
            "nombre" => {
                payload_apuntarse.nombre = campo.text().await.map_err(|e| {
                    Error::FormularioInvalido{error: format!("No se ha encontrado el campo nombre: {}", e)}
                })?;
            },
            "apellidos" => {
                payload_apuntarse.apellidos = campo.text().await.map_err(|e| {
                    Error::FormularioInvalido{error: format!("No se ha encontrado el campo apellidos: {}", e)}
                })?;
            },
            "edad" => {
                payload_apuntarse.edad = campo.text().await.map_err(|e| {
                    Error::FormularioInvalido{error: format!("No se ha encontrado el campo edad: {}", e)}
                })?.parse().map_err(|e| {
                    Error::FormularioInvalido{error: format!("El campo edad es incorrecto: {}", e)}
                })?;
            },
            "mail" => {
                payload_apuntarse.email = campo.text().await.map_err(|e| {
                    Error::FormularioInvalido{error: format!("No se ha encontrado el campo email: {}", e)}
                })?;
            },
            "telefono" => {
                payload_apuntarse.telefono = campo.text().await.map_err(|e| {
                    Error::FormularioInvalido{error: format!("No se ha encontrado el campo telefono: {}", e)}
                })?;
            },
            _ => {}
        }
    };

    payload_apuntarse.nombre.is_empty().then(|| { campos_vacios.push(format!("Nombre"))});
    payload_apuntarse.apellidos.is_empty().then(|| { campos_vacios.push(format!("Apellidos"))});
    payload_apuntarse.email.is_empty().then(|| { campos_vacios.push(format!("Email"))});
    payload_apuntarse.telefono.is_empty().then(|| { campos_vacios.push(format!("Telefono"))});
    if payload_apuntarse.edad <= 0 { campos_vacios.push(format!("Edad")) };

    if campos_vacios.len() > 0 {
        return Err(Error::CamposVacios { campos: campos_vacios });
    };

    let html = MailTemplate {
        nombre: payload_apuntarse.nombre,
        apellidos: payload_apuntarse.apellidos,
        email: payload_apuntarse.email,
        edad: payload_apuntarse.edad,
        telefono: payload_apuntarse.telefono,
        url_fotografia: payload_apuntarse.nombre_fotografia.clone()
    };

    let destinatarios = ControladorUsuario::listar_administradores(ctx, cm).await?
        .iter().map(|destinatario| format!("{} <{}>", destinatario.nombre, destinatario.mail)).collect();

    enviar_mail(destinatarios, "Nueva inscripción".to_string(), html.render().map_err(|_|Error::ErrorTemplate)?, Some((payload_apuntarse.nombre_fotografia, payload_apuntarse.fotografia))).await?;

    Ok((StatusCode::CREATED, ExitoTemplate))
}
