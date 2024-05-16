use axum::extract::{Path, State};
use axum::http::{StatusCode, Uri};
use axum::routing::{delete, get, post};
use axum::{Form, Router};
use axum::response::{IntoResponse, Redirect, Response};
use axum::debug_handler;
use askama::Template;
use axum_htmx::HxRedirect;
use futures::future;
use serde::Deserialize;

use crate::ctx::Ctx;
use crate::modelo::categorias::ControladorCategoria;
use crate::modelo::competiciones::ControladorCompeticion;
use crate::modelo::equipos::ControladorEquipo;
use crate::modelo::eventos::ControladorEvento;
use crate::modelo::jugadores::ControladorJugador;
use crate::modelo::noticias::ControladorNoticia;
use crate::modelo::partidos::ControladorPartido;
use crate::modelo::tipos_evento::ControladorTipoEvento;
use crate::modelo::tipos_jugador::ControladorTipoJugador;
use crate::modelo::usuario::{ControladorUsuario, Usuario};
use crate::modelo::ControladorModelo;
use crate::{Result, Error};

pub fn routes(cm: ControladorModelo) -> Router {
    let gestiones = Router::new()
        .route("/menu", get(menu))
        .route("/usuarios", get(lista_usuarios).post(crear_usuario))
        .route("/usuarios/:id", get(gestion_usuarios))
        .route("/noticias", get(lista_noticias).post(crear_noticia))
        .route("/noticias/:id", get(gestion_noticias))
        .route("/categorias", get(lista_categorias).post(crear_categoria))
        .route("/categorias/:id", get(gestion_categorias))
        .route("/competiciones", get(lista_competiciones).post(crear_competicion))
        .route("/competiciones/:id", get(gestion_competiciones))
        .route("/partidos", get(lista_partidos).post(crear_partido))
        .route("/partidos/:id", get(gestion_partidos))
        .route("/eventos", get(lista_eventos).post(crear_evento))
        .route("/eventos/:id", get(gestion_eventos))
        .route("/equipos", get(lista_equipos).post(crear_equipo))
        .route("/equipos/:id", get(gestion_equipos))
        .route("/jugadores", get(lista_jugadores).post(crear_jugador))
        .route("/jugadores/:id", get(gestion_jugadores))
        .route("/tiposJugador", get(lista_tipos_jugador).post(crear_tipo_jugador))
        .route("/tiposJugador/:id", get(gestion_tipo_jugador))
        .route("/tiposEvento", get(lista_tipos_evento).post(crear_tipo_evento))
        .route("/tiposEvento/:id", get(gestion_tipo_evento))
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
        url: "noticias",
        titulo: "Noticias"
    },
    &Seccion {
        url: "categorias",
        titulo: "Categorias"
    },
    &Seccion {
        url: "competiciones",
        titulo: "Competiciones"
    },
    &Seccion {
        url: "partidos",
        titulo: "Partidos"
    },
    &Seccion {
        url: "eventos",
        titulo: "Eventos"
    },
    &Seccion {
        url: "equipos",
        titulo: "Equipos"
    },
    &Seccion {
        url: "jugadores",
        titulo: "Jugadores"
    },
    &Seccion {
        url: "tiposJugador",
        titulo: "Tipos de jugador"
    },
    &Seccion {
        url: "tiposEvento",
        titulo: "Tipos de evento"
    },
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
pub enum TipoCampo {
    TEXT,
    TEXTAREA,
    PASSWORD,
    MAIL,
    CHECK,
    DATE,
    FILE,
    SELECT(Vec<(String, String)>)
}

// Listado
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

// Gestion 
#[derive(Debug, Deserialize)]
pub struct CampoGestion {
    titulo: String,
    nombre: String,
    tipo: TipoCampo,
    valor: Option<String>
}

#[derive(Template)]
#[template(path = "componentes/gestion.html")]
struct GestionTemplate {
    id: u32,
    url: String,
    encoding: String,
    campos: Vec<CampoGestion>
}

//Creacion
#[derive(Debug, Deserialize)]
pub struct CampoCreacion {
    titulo: String,
    nombre: String,
    tipo: TipoCampo,
}

#[derive(Template)]
#[template(path = "componentes/gestionCreacion.html")]
struct CreacionTemplate {
    url: String,
    encoding: String,
    campos: Vec<CampoCreacion>
}

#[derive(Template)]
#[template(path = "componentes/menuGestion.html")]
struct MenuGestionTemplate<'a> {
    secciones: Vec<&'a Seccion<'a>>
}

async fn menu<'a>(
    State(_cm): State<ControladorModelo>,
    ctx: Ctx,
) -> impl IntoResponse {
    MenuGestionTemplate { secciones: SECCIONES_CONTROL.to_vec() }
}

async fn lista_usuarios(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
) -> Result<ListaGestionTemplate> {
    let usuarios = ControladorUsuario::listar_usuarios(ctx, cm).await?;

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

#[debug_handler]
async fn gestion_usuarios(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
    Path(id): Path<u32>
) -> Result<GestionTemplate> {
    let usuario = ControladorUsuario::usuario_id(cm, id).await?.ok_or(Error::NoEncontradoPorId)?;

    Ok(GestionTemplate {
        id,
        url: format!("usuarios"),
        encoding: format!("application/x-www-form-urlencoded"),
        campos: vec![
            CampoGestion {
                titulo: format!("Nombre"),
                nombre: format!("nombre"),
                tipo: TipoCampo::TEXT,
                valor: Some(usuario.nombre)
            },
            CampoGestion {
                titulo: format!("Mail"),
                nombre: format!("mail"),
                tipo: TipoCampo::MAIL,
                valor: Some(usuario.mail)
            },
            CampoGestion {
                titulo: format!("Es administrador"),
                nombre: format!("esAdministrador"),
                tipo: TipoCampo::CHECK,
                valor: Some(usuario.esAdministrador.to_string())
            },
        ]
    })
}

async fn crear_usuario(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
) -> Result<CreacionTemplate> {
    Ok(CreacionTemplate {
        url: format!("usuarios"),
        encoding: format!("application/x-www-form-urlencoded"),
        campos: vec![
            CampoCreacion {
                titulo: format!("Nombre"),
                nombre: format!("nombre"),
                tipo: TipoCampo::TEXT,
            },
            CampoCreacion {
                titulo: format!("Mail"),
                nombre: format!("mail"),
                tipo: TipoCampo::MAIL,
            },
            CampoCreacion {
                titulo: format!("Contraseña"),
                nombre: format!("password"),
                tipo: TipoCampo::PASSWORD,
            },
        ]
    })
}

async fn lista_noticias(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
) -> Result<ListaGestionTemplate> {
    let noticias = ControladorNoticia::listar_noticias(cm, None).await?;

    Ok(ListaGestionTemplate {
        url: format!("noticias"),
        lista: noticias.iter().map(|noticia| {
            RegistroListaGestion {
                id: noticia.id.clone(),
                titulo: noticia.titulo.clone(),
                valores: vec![
                    (format!("Fecha"), noticia.fecha.to_string().clone()),
                    match noticia.fotoURL {
                        Some(_) => (format!("Foto"), format!("Si")),
                        None => (format!("Foto"), format!("No"))
                    },
                    (format!("Descripción"), noticia.descripcion.clone()),
                ]
            }
        }).collect()
    })
}

async fn gestion_noticias(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
    Path(id): Path<u32>
) -> Result<GestionTemplate> {
    let noticia = ControladorNoticia::noticia_id(cm, id).await?.ok_or(Error::NoEncontradoPorId)?;

    Ok(GestionTemplate {
        id,
        url: format!("noticias"),
        encoding: format!("multipart/form-data"),
        campos: vec![
            CampoGestion {
                titulo: format!("Titulo"),
                nombre: format!("titulo"),
                tipo: TipoCampo::TEXT,
                valor: Some(noticia.titulo)
            },
            CampoGestion {
                titulo: format!("Fecha"),
                nombre: format!("fecha"),
                tipo: TipoCampo::DATE,
                valor: Some(noticia.fecha.to_string())
            },
            CampoGestion {
                titulo: format!("Imagen"),
                nombre: format!("imagen"),
                tipo: TipoCampo::FILE,
                valor: noticia.fotoURL
            },
            CampoGestion {
                titulo: format!("Descripción"),
                nombre: format!("descripcion"),
                tipo: TipoCampo::TEXTAREA,
                valor: Some(noticia.descripcion)
            },
        ]
    })
}

async fn crear_noticia(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
) -> Result<CreacionTemplate> {
    Ok(CreacionTemplate {
        url: format!("noticias"),
        encoding: format!("multipart/form-data"),
        campos: vec![
            CampoCreacion {
                titulo: format!("Titulo"),
                nombre: format!("titulo"),
                tipo: TipoCampo::TEXT,
            },
            CampoCreacion {
                titulo: format!("Fecha"),
                nombre: format!("fecha"),
                tipo: TipoCampo::DATE,
            },
            CampoCreacion {
                titulo: format!("Imagen"),
                nombre: format!("imagen"),
                tipo: TipoCampo::FILE,
            },
            CampoCreacion {
                titulo: format!("Descripción"),
                nombre: format!("descripcion"),
                tipo: TipoCampo::TEXTAREA,
            },
        ]
    })
}

async fn lista_categorias(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
) -> Result<ListaGestionTemplate> {
    let categorias = ControladorCategoria::listar_categorias(ctx, cm).await?;

    Ok(ListaGestionTemplate {
        url: format!("categorias"),
        lista: categorias.iter().map(|categoria| {
            RegistroListaGestion {
                id: categoria.id.clone(),
                titulo: categoria.nombre.clone(),
                valores: vec![]
            }
        }).collect()
    })
}

async fn gestion_categorias(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
    Path(id): Path<u32>
) -> Result<GestionTemplate> {
    let categoria = ControladorCategoria::categoria_id(cm, id).await?.ok_or(Error::NoEncontradoPorId)?;

    Ok(GestionTemplate {
        id,
        url: format!("categorias"),
        encoding: format!("application/x-www-form-urlencoded"),
        campos: vec![
            CampoGestion {
                titulo: format!("Nombre"),
                nombre: format!("nombre"),
                tipo: TipoCampo::TEXT,
                valor: Some(categoria.nombre)
            },
        ]
    })
}

async fn crear_categoria(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
) -> Result<CreacionTemplate> {
    Ok(CreacionTemplate {
        url: format!("categorias"),
        encoding: format!("application/x-www-form-urlencoded"),
        campos: vec![
            CampoCreacion {
                titulo: format!("Nombre"),
                nombre: format!("nombre"),
                tipo: TipoCampo::TEXT,
            },
        ]
    })
}

async fn lista_competiciones(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
) -> Result<ListaGestionTemplate> {
    let competiciones = ControladorCompeticion::listar_competiciones(ctx, cm.clone()).await?;

    Ok(ListaGestionTemplate {
        url: format!("competiciones"),
        lista: future::try_join_all(competiciones.iter().map(|competicion| async {
            let nombre_categoria = ControladorCategoria::categoria_id(cm.clone(), competicion.idCategoria).await?.ok_or(Error::NoEncontradoPorId)?.nombre;

            Ok::<RegistroListaGestion, Error>(RegistroListaGestion {
                id: competicion.id.clone(),
                titulo: competicion.nombre.clone(),
                valores: vec![
                    (format!("Fecha inicio"), 
                        match competicion.fechaInicio {
                            Some(fecha) => {
                                fecha.to_string()
                            },
                            None => "No definida".to_string()
                        }
                    ),
                    (format!("Fecha fin"), 
                        match competicion.fechaFin {
                            Some(fecha) => {
                                fecha.to_string()
                            },
                            None => "No definida".to_string()
                        }
                    ),
                    (format!("Categoria"), nombre_categoria),
                ]
            })
        })).await?
    })
}

async fn gestion_competiciones(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
    Path(id): Path<u32>
) -> Result<GestionTemplate> {
    let competicion = ControladorCompeticion::competicion_id(cm.clone(), id).await?.ok_or(Error::NoEncontradoPorId)?;
    let categorias: Vec<(String, String)> = ControladorCategoria::listar_categorias(ctx, cm).await?.iter().map(|categoria| {
        (categoria.id.clone().to_string(), categoria.nombre.clone())
    }).collect();

    Ok(GestionTemplate {
        id,
        url: format!("competiciones"),
        encoding: format!("application/x-www-form-urlencoded"),
        campos: vec![
            CampoGestion {
                titulo: format!("Nombre"),
                nombre: format!("nombre"),
                tipo: TipoCampo::TEXT,
                valor: Some(competicion.nombre)
            },
            CampoGestion {
                titulo: format!("Fecha inicio"),
                nombre: format!("fechaInicio"),
                tipo: TipoCampo::DATE,
                valor: competicion.fechaInicio.map(|fecha| fecha.to_string())
            },
            CampoGestion {
                titulo: format!("Fecha fin"),
                nombre: format!("fechaFin"),
                tipo: TipoCampo::DATE,
                valor: competicion.fechaFin.map(|fecha| fecha.to_string())
            },
            CampoGestion {
                titulo: format!("Categoria"),
                nombre: format!("idCategoria"),
                tipo: TipoCampo::SELECT(categorias),
                valor: Some(competicion.idCategoria.to_string())
            },
        ]
    })
}

async fn crear_competicion(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
) -> Result<CreacionTemplate> {
    let categorias: Vec<(String, String)> = ControladorCategoria::listar_categorias(ctx, cm).await?.iter().map(|categoria| {
        (categoria.id.clone().to_string(), categoria.nombre.clone())
    }).collect();

    Ok(CreacionTemplate {
        url: format!("competiciones"),
        encoding: format!("application/x-www-form-urlencoded"),
        campos: vec![
            CampoCreacion {
                titulo: format!("Nombre"),
                nombre: format!("nombre"),
                tipo: TipoCampo::TEXT,
            },
            CampoCreacion {
                titulo: format!("Fecha inicio"),
                nombre: format!("fechaInicio"),
                tipo: TipoCampo::DATE,
            },
            CampoCreacion {
                titulo: format!("Fecha fin"),
                nombre: format!("fechaFin"),
                tipo: TipoCampo::DATE,
            },
            CampoCreacion {
                titulo: format!("Categoria"),
                nombre: format!("idCategoria"),
                tipo: TipoCampo::SELECT(categorias),
            },
        ]
    })
}

async fn lista_partidos(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
) -> Result<ListaGestionTemplate> {
    let partidos = ControladorPartido::listar_partidos(ctx, cm.clone()).await?;

    Ok(ListaGestionTemplate {
        url: format!("partidos"),
        lista: future::try_join_all(partidos.iter().map(|partido| async {
            let nombre_competicion = ControladorCompeticion::competicion_id(cm.clone(), partido.idCompeticion).await?.ok_or(Error::NoEncontradoPorId)?.nombre;
            let nombre_equipo_casa = ControladorEquipo::equipo_id(cm.clone(), partido.idEquipoCasa).await?.ok_or(Error::NoEncontradoPorId)?.nombre;
            let nombre_equipo_visitante = ControladorEquipo::equipo_id(cm.clone(), partido.idEquipoVisitante).await?.ok_or(Error::NoEncontradoPorId)?.nombre;

            Ok::<RegistroListaGestion, Error>(RegistroListaGestion {
                id: partido.id.clone(),
                titulo: partido.fecha.to_string().clone(),
                valores: vec![
                    (format!("Lugar"), partido.lugar.clone()),
                    (format!("Competicion"), nombre_competicion),
                    (format!("Equipo casa"), nombre_equipo_casa),
                    (format!("Equipo visitante"), nombre_equipo_visitante),
                ]
            })
        })).await?
    })
}

async fn gestion_partidos(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
    Path(id): Path<u32>
) -> Result<GestionTemplate> {
    let partido = ControladorPartido::partido_id(cm.clone(), id).await?.ok_or(Error::NoEncontradoPorId)?;
    let competiciones: Vec<(String, String)> = ControladorCompeticion::listar_competiciones(ctx.clone(), cm.clone()).await?.iter().map(|competicion| {
        (competicion.id.clone().to_string(), competicion.nombre.clone())
    }).collect();
    let equipos: Vec<(String, String)> = ControladorEquipo::listar_equipos(ctx.clone(), cm.clone()).await?.iter().map(|equipo| {
        (equipo.id.clone().to_string(), equipo.nombre.clone())
    }).collect();

    Ok(GestionTemplate {
        id,
        url: format!("partidos"),
        encoding: format!("application/x-www-form-urlencoded"),
        campos: vec![
            CampoGestion {
                titulo: format!("Fecha"),
                nombre: format!("fecha"),
                tipo: TipoCampo::DATE,
                valor: Some(partido.fecha.to_string())
            },
            CampoGestion {
                titulo: format!("Lugar"),
                nombre: format!("lugar"),
                tipo: TipoCampo::TEXT,
                valor: Some(partido.lugar)
            },
            CampoGestion {
                titulo: format!("Competicion"),
                nombre: format!("idCompeticion"),
                tipo: TipoCampo::SELECT(competiciones),
                valor: Some(partido.idCompeticion.to_string())
            },
            CampoGestion {
                titulo: format!("Equipo casa"),
                nombre: format!("idEquipoCasa"),
                tipo: TipoCampo::SELECT(equipos.clone()),
                valor: Some(partido.idEquipoCasa.to_string())
            },
            CampoGestion {
                titulo: format!("Equipo visitante"),
                nombre: format!("idEquipoVisitante"),
                tipo: TipoCampo::SELECT(equipos),
                valor: Some(partido.idEquipoVisitante.to_string())
            },
        ]
    })
}

async fn crear_partido(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
) -> Result<CreacionTemplate> {
    let competiciones: Vec<(String, String)> = ControladorCompeticion::listar_competiciones(ctx.clone(), cm.clone()).await?.iter().map(|competicion| {
        (competicion.id.clone().to_string(), competicion.nombre.clone())
    }).collect();
    let equipos: Vec<(String, String)> = ControladorEquipo::listar_equipos(ctx.clone(), cm.clone()).await?.iter().map(|equipo| {
        (equipo.id.clone().to_string(), equipo.nombre.clone())
    }).collect();

    Ok(CreacionTemplate {
        url: format!("partidos"),
        encoding: format!("application/x-www-form-urlencoded"),
        campos: vec![
            CampoCreacion {
                titulo: format!("Fecha"),
                nombre: format!("fecha"),
                tipo: TipoCampo::DATE,
            },
            CampoCreacion {
                titulo: format!("Lugar"),
                nombre: format!("lugar"),
                tipo: TipoCampo::TEXT,
            },
            CampoCreacion {
                titulo: format!("Competicion"),
                nombre: format!("idCompeticion"),
                tipo: TipoCampo::SELECT(competiciones),
            },
            CampoCreacion {
                titulo: format!("Equipo casa"),
                nombre: format!("idEquipoCasa"),
                tipo: TipoCampo::SELECT(equipos.clone()),
            },
            CampoCreacion {
                titulo: format!("Equipo visitante"),
                nombre: format!("idEquipoVisitante"),
                tipo: TipoCampo::SELECT(equipos),
            },
        ]
    })
}

async fn lista_eventos(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
) -> Result<ListaGestionTemplate> {
    let eventos = ControladorEvento::listar_eventos(ctx, cm.clone()).await?;

    Ok(ListaGestionTemplate {
        url: format!("eventos"),
        lista: future::try_join_all(eventos.iter().map(|evento| async {
            let nombre_tipo_evento = ControladorTipoEvento::tipo_evento_id(cm.clone(), evento.idTipoEvento).await?.ok_or(Error::NoEncontradoPorId)?.nombre;
            let jugador = ControladorJugador::jugador_id(cm.clone(), evento.idJugador).await?.ok_or(Error::NoEncontradoPorId)?;
            let partido = ControladorPartido::partido_id(cm.clone(), evento.idPartido).await?.ok_or(Error::NoEncontradoPorId)?;
            let nombre_equipo_casa = ControladorEquipo::equipo_id(cm.clone(), partido.idEquipoCasa).await?.ok_or(Error::NoEncontradoPorId)?.nombre;
            let nombre_equipo_visitante = ControladorEquipo::equipo_id(cm.clone(), partido.idEquipoVisitante).await?.ok_or(Error::NoEncontradoPorId)?.nombre;

            Ok::<RegistroListaGestion, Error>(RegistroListaGestion {
                id: evento.id.clone(),
                titulo: nombre_tipo_evento,
                valores: vec![
                    (format!("Minuto"), match evento.minuto {
                        Some(minuto) => minuto.to_string(),
                        None => "Sin valor".to_string()
                    }),
                    (format!("Valor"), match evento.valor {
                        Some(valor) => valor.to_string(),
                        None => "Sin valor".to_string()
                    }),
                    (format!("Jugador"), format!("{} {}, {}", jugador.apellido1, jugador.apellido2, jugador.nombre)),
                    (format!("Partido"), format!("{} - {} | {}", nombre_equipo_casa, nombre_equipo_visitante, partido.fecha.to_string())),
                ]
            })
        })).await?
    })
}

async fn gestion_eventos(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
    Path(id): Path<u32>
) -> Result<GestionTemplate> {
    let evento = ControladorEvento::evento_id(cm.clone(), id).await?.ok_or(Error::NoEncontradoPorId)?;
    let tipos_evento: Vec<(String, String)> = ControladorTipoEvento::listar_tipos_evento(ctx.clone(), cm.clone()).await?.iter().map(|tipo_evento| {
        (tipo_evento.id.clone().to_string(), tipo_evento.nombre.clone())
    }).collect();
    let jugadores: Vec<(String, String)> = ControladorJugador::listar_jugadores(cm.clone()).await?.iter().map(|jugador| {
        (jugador.id.clone().to_string(), format!("{} {}, {}", jugador.apellido1, jugador.apellido2, jugador.nombre))
    }).collect();
    let partidos: Vec<(String, String)> = future::try_join_all(ControladorPartido::listar_partidos(ctx.clone(), cm.clone()).await?.iter().map(|partido| async {
        let nombre_equipo_casa = ControladorEquipo::equipo_id(cm.clone(), partido.idEquipoCasa).await?.ok_or(Error::NoEncontradoPorId)?.nombre;
        let nombre_equipo_visitante = ControladorEquipo::equipo_id(cm.clone(), partido.idEquipoVisitante).await?.ok_or(Error::NoEncontradoPorId)?.nombre;

        Ok::<(String, String), Error>((partido.id.clone().to_string(), format!("{} - {} |{}", nombre_equipo_casa, nombre_equipo_visitante, partido.fecha.to_string())))
    })).await?;

    Ok(GestionTemplate {
        id,
        url: format!("eventos"),
        encoding: format!("application/x-www-form-urlencoded"),
        campos: vec![
            CampoGestion {
                titulo: format!("Valor"),
                nombre: format!("valor"),
                tipo: TipoCampo::TEXT,
                valor: evento.valor.map(|valor| valor.to_string())
            },
            CampoGestion {
                titulo: format!("Minuto"),
                nombre: format!("minuto"),
                tipo: TipoCampo::TEXT,
                valor: evento.minuto.map(|minuto| minuto.minute().to_string())
            },
            CampoGestion {
                titulo: format!("Segundo"),
                nombre: format!("segundo"),
                tipo: TipoCampo::TEXT,
                valor: evento.minuto.map(|minuto| minuto.second().to_string())
            },
            CampoGestion {
                titulo: format!("Tipo evento"),
                nombre: format!("idTipoEvento"),
                tipo: TipoCampo::SELECT(tipos_evento),
                valor: Some(evento.idTipoEvento.to_string())
            },
            CampoGestion {
                titulo: format!("Jugador"),
                nombre: format!("idJugador"),
                tipo: TipoCampo::SELECT(jugadores),
                valor: Some(evento.idJugador.to_string())
            },
            CampoGestion {
                titulo: format!("Partido"),
                nombre: format!("idPartido"),
                tipo: TipoCampo::SELECT(partidos),
                valor: Some(evento.idPartido.to_string())
            },
        ]
    })
}

async fn crear_evento(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
) -> Result<CreacionTemplate> {
    let tipos_evento: Vec<(String, String)> = ControladorTipoEvento::listar_tipos_evento(ctx.clone(), cm.clone()).await?.iter().map(|tipo_evento| {
        (tipo_evento.id.clone().to_string(), tipo_evento.nombre.clone())
    }).collect();
    let jugadores: Vec<(String, String)> = ControladorJugador::listar_jugadores(cm.clone()).await?.iter().map(|jugador| {
        (jugador.id.clone().to_string(), format!("{} {}, {}", jugador.apellido1, jugador.apellido2, jugador.nombre))
    }).collect();
    let partidos: Vec<(String, String)> = future::try_join_all(ControladorPartido::listar_partidos(ctx.clone(), cm.clone()).await?.iter().map(|partido| async {
        let nombre_equipo_casa = ControladorEquipo::equipo_id(cm.clone(), partido.idEquipoCasa).await?.ok_or(Error::NoEncontradoPorId)?.nombre;
        let nombre_equipo_visitante = ControladorEquipo::equipo_id(cm.clone(), partido.idEquipoVisitante).await?.ok_or(Error::NoEncontradoPorId)?.nombre;

        Ok::<(String, String), Error>((partido.id.clone().to_string(), format!("{} - {} |{}", nombre_equipo_casa, nombre_equipo_visitante, partido.fecha.to_string())))
    })).await?;

    Ok(CreacionTemplate {
        url: format!("eventos"),
        encoding: format!("application/x-www-form-urlencoded"),
        campos: vec![
            CampoCreacion {
                titulo: format!("Valor"),
                nombre: format!("valor"),
                tipo: TipoCampo::TEXT,
            },
            CampoCreacion {
                titulo: format!("Minuto"),
                nombre: format!("minuto"),
                tipo: TipoCampo::TEXT,
            },
            CampoCreacion {
                titulo: format!("Segundo"),
                nombre: format!("segundo"),
                tipo: TipoCampo::TEXT,
            },
            CampoCreacion {
                titulo: format!("Tipo evento"),
                nombre: format!("idTipoEvento"),
                tipo: TipoCampo::SELECT(tipos_evento),
            },
            CampoCreacion {
                titulo: format!("Jugador"),
                nombre: format!("idJugador"),
                tipo: TipoCampo::SELECT(jugadores),
            },
            CampoCreacion {
                titulo: format!("Partido"),
                nombre: format!("idPartido"),
                tipo: TipoCampo::SELECT(partidos),
            },
        ]
    })
}

async fn lista_equipos(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
) -> Result<ListaGestionTemplate> {
    let equipos = ControladorEquipo::listar_equipos(ctx, cm).await?;

    Ok(ListaGestionTemplate {
        url: format!("equipos"),
        lista: equipos.iter().map(|equipo| {
            RegistroListaGestion {
                id: equipo.id.clone(),
                titulo: equipo.nombre.clone(),
                valores: vec![
                    (format!("Lugar"), equipo.lugar.clone()),
                ]
            }
        }).collect()
    })
}

async fn gestion_equipos(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
    Path(id): Path<u32>
) -> Result<GestionTemplate> {
    let equipo = ControladorEquipo::equipo_id(cm, id).await?.ok_or(Error::NoEncontradoPorId)?;

    Ok(GestionTemplate {
        id,
        url: format!("equipos"),
        encoding: format!("application/x-www-form-urlencoded"),
        campos: vec![
            CampoGestion {
                titulo: format!("Nombre"),
                nombre: format!("nombre"),
                tipo: TipoCampo::TEXT,
                valor: Some(equipo.nombre)
            },
            CampoGestion {
                titulo: format!("Lugar"),
                nombre: format!("lugar"),
                tipo: TipoCampo::TEXT,
                valor: Some(equipo.lugar)
            },
        ]
    })
}

async fn crear_equipo(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
) -> Result<CreacionTemplate> {
    Ok(CreacionTemplate {
        url: format!("equipos"),
        encoding: format!("application/x-www-form-urlencoded"),
        campos: vec![
            CampoCreacion {
                titulo: format!("Nombre"),
                nombre: format!("nombre"),
                tipo: TipoCampo::TEXT,
            },
            CampoCreacion {
                titulo: format!("Lugar"),
                nombre: format!("lugar"),
                tipo: TipoCampo::TEXT,
            },
        ]
    })
}

async fn lista_jugadores(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
) -> Result<ListaGestionTemplate> {
    let jugadores = ControladorJugador::listar_jugadores(cm.clone()).await?;

    Ok(ListaGestionTemplate {
        url: format!("jugadores"),
        lista: future::try_join_all(jugadores.iter().map(|jugador| async {
            let nombre_tipo_jugador = ControladorTipoJugador::tipo_jugador_id(cm.clone(), jugador.idTipoJugador).await?.ok_or(Error::NoEncontradoPorId)?.nombre;
            let nombre_equipo = ControladorEquipo::equipo_id(cm.clone(), jugador.idEquipo).await?.ok_or(Error::NoEncontradoPorId)?.nombre;

            Ok::<RegistroListaGestion, Error>(RegistroListaGestion {
                id: jugador.id.clone(),
                titulo: format!("{} {}, {}", jugador.apellido1.clone(), jugador.apellido2.clone(), jugador.nombre.clone()),
                valores: vec![
                    (format!("Número"), jugador.numero.to_string().clone()),
                    (format!("Nacimiento"), jugador.nacimiento.to_string().clone()),
                    match jugador.fotoURL {
                        Some(_) => (format!("Foto"), format!("Si")),
                        None => (format!("Foto"), format!("No"))
                    },
                    (format!("Tipo de jugador"), nombre_tipo_jugador),
                    (format!("Equipo"), nombre_equipo),
                ]
            })
        })).await?
    })
}

async fn gestion_jugadores(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
    Path(id): Path<u32>
) -> Result<GestionTemplate> {
    let jugador = ControladorJugador::jugador_id(cm.clone(), id).await?.ok_or(Error::NoEncontradoPorId)?;
    let tipos_jugador: Vec<(String, String)> = ControladorTipoJugador::listar_tipos_jugador(ctx.clone(), cm.clone()).await?.iter().map(|tipo_jugador| {
        (tipo_jugador.id.clone().to_string(), tipo_jugador.nombre.clone())
    }).collect();
    let equipos: Vec<(String, String)> = ControladorEquipo::listar_equipos(ctx, cm).await?.iter().map(|equipo| {
        (equipo.id.clone().to_string(), equipo.nombre.clone())
    }).collect();

    Ok(GestionTemplate {
        id,
        url: format!("jugadores"),
        encoding: format!("multipart/form-data"),
        campos: vec![
            CampoGestion {
                titulo: format!("Número"),
                nombre: format!("numero"),
                tipo: TipoCampo::TEXT,
                valor: Some(jugador.numero.to_string())
            },
            CampoGestion {
                titulo: format!("Nombre"),
                nombre: format!("nombre"),
                tipo: TipoCampo::TEXT,
                valor: Some(jugador.nombre)
            },
            CampoGestion {
                titulo: format!("Apellido 1"),
                nombre: format!("apellido1"),
                tipo: TipoCampo::TEXT,
                valor: Some(jugador.apellido1)
            },
            CampoGestion {
                titulo: format!("Apellido 2"),
                nombre: format!("apellido2"),
                tipo: TipoCampo::TEXT,
                valor: Some(jugador.apellido2)
            },
            CampoGestion {
                titulo: format!("Nacimiento"),
                nombre: format!("nacimiento"),
                tipo: TipoCampo::DATE,
                valor: Some(jugador.nacimiento.to_string())
            },
            CampoGestion {
                titulo: format!("Imagen"),
                nombre: format!("imagen"),
                tipo: TipoCampo::FILE,
                valor: jugador.fotoURL
            },
            CampoGestion {
                titulo: format!("Tipo jugador"),
                nombre: format!("tipoJugador"),
                tipo: TipoCampo::SELECT(tipos_jugador),
                valor: Some(jugador.idTipoJugador.to_string())
            },
            CampoGestion {
                titulo: format!("Equipo"),
                nombre: format!("equipo"),
                tipo: TipoCampo::SELECT(equipos),
                valor: Some(jugador.idEquipo.to_string())
            },
        ]
    })
}

async fn crear_jugador(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
) -> Result<CreacionTemplate> {
    let tipos_jugador: Vec<(String, String)> = ControladorTipoJugador::listar_tipos_jugador(ctx.clone(), cm.clone()).await?.iter().map(|tipo_jugador| {
        (tipo_jugador.id.clone().to_string(), tipo_jugador.nombre.clone())
    }).collect();
    let equipos: Vec<(String, String)> = ControladorEquipo::listar_equipos(ctx, cm).await?.iter().map(|equipo| {
        (equipo.id.clone().to_string(), equipo.nombre.clone())
    }).collect();

    Ok(CreacionTemplate {
        url: format!("jugadores"),
        encoding: format!("multipart/form-data"),
        campos: vec![
            CampoCreacion {
                titulo: format!("Número"),
                nombre: format!("numero"),
                tipo: TipoCampo::TEXT,
            },
            CampoCreacion {
                titulo: format!("Nombre"),
                nombre: format!("nombre"),
                tipo: TipoCampo::TEXT,
            },
            CampoCreacion {
                titulo: format!("Apellido 1"),
                nombre: format!("apellido1"),
                tipo: TipoCampo::TEXT,
            },
            CampoCreacion {
                titulo: format!("Apellido 2"),
                nombre: format!("apellido2"),
                tipo: TipoCampo::TEXT,
            },
            CampoCreacion {
                titulo: format!("Nacimiento"),
                nombre: format!("nacimiento"),
                tipo: TipoCampo::DATE,
            },
            CampoCreacion {
                titulo: format!("Imagen"),
                nombre: format!("imagen"),
                tipo: TipoCampo::FILE,
            },
            CampoCreacion {
                titulo: format!("Tipo jugador"),
                nombre: format!("tipoJugador"),
                tipo: TipoCampo::SELECT(tipos_jugador),
            },
            CampoCreacion {
                titulo: format!("Equipo"),
                nombre: format!("equipo"),
                tipo: TipoCampo::SELECT(equipos),
            },
        ]
    })
}

async fn lista_tipos_jugador(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
) -> Result<ListaGestionTemplate> {
    let tipos_jugador = ControladorTipoJugador::listar_tipos_jugador(ctx, cm).await?;

    Ok(ListaGestionTemplate {
        url: format!("tiposJugador"),
        lista: tipos_jugador.iter().map(|tipo_jugador| {
            RegistroListaGestion {
                id: tipo_jugador.id.clone(),
                titulo: tipo_jugador.nombre.clone(),
                valores: vec![]
            }
        }).collect()
    })
}

async fn gestion_tipo_jugador(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
    Path(id): Path<u32>
) -> Result<GestionTemplate> {
    let tipo_jugador = ControladorTipoJugador::tipo_jugador_id(cm, id).await?.ok_or(Error::NoEncontradoPorId)?;

    Ok(GestionTemplate {
        id,
        url: format!("tiposJugador"),
        encoding: format!("application/x-www-form-urlencoded"),
        campos: vec![
            CampoGestion {
                titulo: format!("Nombre"),
                nombre: format!("nombre"),
                tipo: TipoCampo::TEXT,
                valor: Some(tipo_jugador.nombre)
            },
        ]
    })
}

async fn crear_tipo_jugador(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
) -> Result<CreacionTemplate> {
    Ok(CreacionTemplate {
        url: format!("tiposJugador"),
        encoding: format!("application/x-www-form-urlencoded"),
        campos: vec![
            CampoCreacion {
                titulo: format!("Nombre"),
                nombre: format!("nombre"),
                tipo: TipoCampo::TEXT,
            },
        ]
    })
}

async fn lista_tipos_evento(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
) -> Result<ListaGestionTemplate> {
    let tipos_evento = ControladorTipoEvento::listar_tipos_evento(ctx, cm).await?;

    Ok(ListaGestionTemplate {
        url: format!("tiposEvento"),
        lista: tipos_evento.iter().map(|tipo_evento| {
            RegistroListaGestion {
                id: tipo_evento.id.clone(),
                titulo: tipo_evento.nombre.clone(),
                valores: vec![]
            }
        }).collect()
    })
}

async fn gestion_tipo_evento(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
    Path(id): Path<u32>
) -> Result<GestionTemplate> {
    let tipo_evento = ControladorTipoEvento::tipo_evento_id(cm, id).await?.ok_or(Error::NoEncontradoPorId)?;

    Ok(GestionTemplate {
        id,
        url: format!("tiposEvento"),
        encoding: format!("application/x-www-form-urlencoded"),
        campos: vec![
            CampoGestion {
                titulo: format!("Nombre"),
                nombre: format!("nombre"),
                tipo: TipoCampo::TEXT,
                valor: Some(tipo_evento.nombre)
            },
        ]
    })
}

async fn crear_tipo_evento(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
) -> Result<CreacionTemplate> {
    Ok(CreacionTemplate {
        url: format!("tiposEvento"),
        encoding: format!("application/x-www-form-urlencoded"),
        campos: vec![
            CampoCreacion {
                titulo: format!("Nombre"),
                nombre: format!("nombre"),
                tipo: TipoCampo::TEXT,
            },
        ]
    })
}
