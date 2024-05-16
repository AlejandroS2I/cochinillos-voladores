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
use crate::modelo::jugadores::{ControladorJugador, Jugador, JugadorActualizar, JugadorCrear};
use crate::uploads::{eliminar_archivo, subir_archivo};
use crate::{Result, Error};

pub fn routes(cm: ControladorModelo) -> Router {
    Router::new()
        .route("/jugadores", post(crear_jugador).put(actualizar_jugador))
        .route("/jugadores/:id", delete(eliminar_jugador))
        .with_state(cm)
}

struct PayloadCrear {
    numero: u32,
    nombre: String,
    apellido1: String,
    apellido2: String,
    nacimiento: Date,
    nombre_archivo: Option<String>,
    archivo: Option<Bytes>,
    idTipoJugador: u32,
    idEquipo: u32,
}

async fn crear_jugador(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
    mut multipart: Multipart,
) -> Result<StatusCode> {
    let mut payload_crear = PayloadCrear {
        numero: 0,
        nombre: "".to_string(),
        apellido1: "".to_string(),
        apellido2: "".to_string(),
        nacimiento: OffsetDateTime::now_utc().date(),
        nombre_archivo: None,
        archivo: None,
        idTipoJugador: 0,
        idEquipo: 0
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
            "numero" => {
                payload_crear.numero = campo.text().await.map_err(|e| {
                    Error::FormularioInvalido{error: format!("No se ha encontrado el campo nombre: {}", e)}
                })?.parse().map_err(|e|Error::FormularioInvalido { error: format!("Error parseando el campo numero: {}", e) })?;
            },
            "tipoJugador" => {
                payload_crear.idTipoJugador = campo.text().await.map_err(|e| {
                    Error::FormularioInvalido{error: format!("No se ha encontrado el campo tipo jugador: {}", e)}
                })?.parse().map_err(|e|Error::FormularioInvalido { error: format!("Error parseando el campo tipo jugador: {}", e) })?;
            },
            "equipo" => {
                payload_crear.idEquipo = campo.text().await.map_err(|e| {
                    Error::FormularioInvalido{error: format!("No se ha encontrado el campo equipo: {}", e)}
                })?.parse().map_err(|e|Error::FormularioInvalido { error: format!("Error parseando el campo equipo: {}", e) })?;
            },
            "nombre" => {
                payload_crear.nombre = campo.text().await.map_err(|e| {
                    Error::FormularioInvalido{error: format!("No se ha encontrado el campo nombre: {}", e)}
                })?;
            },
            "apellido1" => {
                payload_crear.apellido1 = campo.text().await.map_err(|e| {
                    Error::FormularioInvalido{error: format!("No se ha encontrado el campo apellido1: {}", e)}
                })?;
            },
            "apellido2" => {
                payload_crear.apellido2 = campo.text().await.map_err(|e| {
                    Error::FormularioInvalido{error: format!("No se ha encontrado el campo apellido2: {}", e)}
                })?;
            },
            "nacimiento" => {
                let nacimiento = campo.text().await.map_err(|e| {
                    Error::FormularioInvalido{error: format!("No se ha encontrado el campo nacimiento: {}", e)}
                })?;

                nacimiento.is_empty().then(|| { campos_vacios.push(format!("Nacimiento"))});

                payload_crear.nacimiento = Date::parse(&nacimiento, format_description!("[year]-[month]-[day]"))
                    .map_err(|e| Error::Generico{ error: e.to_string() })?;
            },
            _ => {}
        }
    };

    payload_crear.nombre.is_empty().then(|| { campos_vacios.push(format!("Nombre"))});
    payload_crear.apellido1.is_empty().then(|| { campos_vacios.push(format!("Apellido 1"))});
    payload_crear.apellido2.is_empty().then(|| { campos_vacios.push(format!("Apellido 2"))});

    if campos_vacios.len() > 0 {
        return Err(Error::CamposVacios { campos: campos_vacios });
    };

    let mut jugador_crear = JugadorCrear {
        numero: payload_crear.numero,
        nombre: payload_crear.nombre,
        apellido1: payload_crear.apellido1,
        apellido2: payload_crear.apellido2,
        nacimiento: payload_crear.nacimiento,
        fotoURL: None,
        idTipoJugador: payload_crear.idTipoJugador,
        idEquipo: payload_crear.idEquipo
    };

    jugador_crear.fotoURL = match (payload_crear.nombre_archivo, payload_crear.archivo) {
        (Some(nombre), Some(archivo)) => Some(subir_archivo("jugadores".to_string(), archivo, nombre).await?),
        _ => None
    };

    let jugador = ControladorJugador::crear_jugador(cm, jugador_crear).await?;

    Ok(StatusCode::CREATED)
}

struct PayloadActualizar {
    id: u32,
    numero: u32,
    nombre: String,
    apellido1: String,
    apellido2: String,
    nacimiento: Date,
    nombre_archivo: Option<String>,
    archivo: Option<Bytes>,
    idTipoJugador: u32,
    idEquipo: u32,
}

async fn actualizar_jugador(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
    mut multipart: Multipart,
) -> Result<StatusCode> {
    let mut payload_actualizar = PayloadActualizar {
        id: 0,
        numero: 0,
        nombre: "".to_string(),
        apellido1: "".to_string(),
        apellido2: "".to_string(),
        nacimiento: OffsetDateTime::now_utc().date(),
        nombre_archivo: None,
        archivo: None,
        idTipoJugador: 0,
        idEquipo: 0
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
            "numero" => {
                payload_actualizar.numero = campo.text().await.map_err(|e| {
                    Error::FormularioInvalido{error: format!("No se ha encontrado el campo nombre: {}", e)}
                })?.parse().map_err(|e|Error::FormularioInvalido { error: format!("Error parseando el campo numero: {}", e) })?;
            },
            "tipoJugador" => {
                payload_actualizar.idTipoJugador = campo.text().await.map_err(|e| {
                    Error::FormularioInvalido{error: format!("No se ha encontrado el campo tipo jugador: {}", e)}
                })?.parse().map_err(|e|Error::FormularioInvalido { error: format!("Error parseando el campo tipo jugador: {}", e) })?;
            },
            "equipo" => {
                payload_actualizar.idEquipo = campo.text().await.map_err(|e| {
                    Error::FormularioInvalido{error: format!("No se ha encontrado el campo equipo: {}", e)}
                })?.parse().map_err(|e|Error::FormularioInvalido { error: format!("Error parseando el campo equipo: {}", e) })?;
            },
            "nombre" => {
                payload_actualizar.nombre = campo.text().await.map_err(|e| {
                    Error::FormularioInvalido{error: format!("No se ha encontrado el campo nombre: {}", e)}
                })?;
            },
            "apellido1" => {
                payload_actualizar.apellido1 = campo.text().await.map_err(|e| {
                    Error::FormularioInvalido{error: format!("No se ha encontrado el campo apellido1: {}", e)}
                })?;
            },
            "apellido2" => {
                payload_actualizar.apellido2 = campo.text().await.map_err(|e| {
                    Error::FormularioInvalido{error: format!("No se ha encontrado el campo apellido2: {}", e)}
                })?;
            },
            "nacimiento" => {
                let nacimiento = campo.text().await.map_err(|e| {
                    Error::FormularioInvalido{error: format!("No se ha encontrado el campo nacimiento: {}", e)}
                })?;

                nacimiento.is_empty().then(|| { campos_vacios.push(format!("Nacimiento"))});

                payload_actualizar.nacimiento = Date::parse(&nacimiento, format_description!("[year]-[month]-[day]"))
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

    payload_actualizar.nombre.is_empty().then(|| { campos_vacios.push(format!("Nombre"))});
    payload_actualizar.apellido1.is_empty().then(|| { campos_vacios.push(format!("Apellido 1"))});
    payload_actualizar.apellido2.is_empty().then(|| { campos_vacios.push(format!("Apellido 2"))});

    if campos_vacios.len() > 0 {
        return Err(Error::CamposVacios { campos: campos_vacios });
    };

    let mut jugador_actualizar = JugadorActualizar {
        id: payload_actualizar.id,
        numero: payload_actualizar.numero,
        nombre: payload_actualizar.nombre,
        apellido1: payload_actualizar.apellido1,
        apellido2: payload_actualizar.apellido2,
        nacimiento: payload_actualizar.nacimiento,
        fotoURL: None,
        idTipoJugador: payload_actualizar.idTipoJugador,
        idEquipo: payload_actualizar.idEquipo
    };

    jugador_actualizar.fotoURL = match (payload_actualizar.nombre_archivo, payload_actualizar.archivo) {
        (Some(nombre), Some(archivo)) => Some(subir_archivo("jugadores".to_string(), archivo, nombre).await?),
        _ => None
    };

    if jugador_actualizar.fotoURL.is_some() {
        let jugador = ControladorJugador::jugador_id(cm.clone(), jugador_actualizar.id).await?.ok_or(Error::NoEncontradoPorId)?;
        if let Some(url) = jugador.fotoURL {
            eliminar_archivo(url).await?;
        }
    }

    let jugador = ControladorJugador::actualizar_jugador(cm, jugador_actualizar).await?;

    Ok(StatusCode::CREATED)
}

async fn eliminar_jugador(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
    Path(id): Path<u32>
) -> Result<StatusCode> {
    let jugador = ControladorJugador::eliminar_jugador(ctx, cm, id).await?;

    if let Some(url) = jugador.fotoURL {
        eliminar_archivo(url).await?;
    }

    Ok(StatusCode::OK)
}
