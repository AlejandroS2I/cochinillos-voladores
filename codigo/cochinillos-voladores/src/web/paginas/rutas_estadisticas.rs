use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{delete, get, post};
use axum::{Form, Router};
use axum::response::{IntoResponse, Response};
use askama::Template;
use futures::future;
use time::{Date, Time};

use crate::ctx::Ctx;
use crate::modelo::categorias::{Categoria, ControladorCategoria};
use crate::modelo::competiciones::{Competicion, ControladorCompeticion};
use crate::modelo::equipos::{ControladorEquipo, Equipo};
use crate::modelo::eventos::{ControladorEvento, EstadisticasJugador, Evento};
use crate::modelo::jugadores::{ControladorJugador, Jugador};
use crate::modelo::partidos::{ControladorPartido, Partido};
use crate::modelo::tipos_evento::{ControladorTipoEvento, TipoEvento};
use crate::modelo::{ControladorModelo};
use crate::{Result, Error};

pub fn routes(cm: ControladorModelo) -> Router {
    Router::new()
        .route("/estadisticas", get(estadisticas))
        .route("/estadisticas/partido/:id", get(partido))
        .with_state(cm)
}

#[derive(Template)]
#[template(path = "estadisticas.html")]
struct EstadisticasTemplate {
   cochinillos: Equipo,
    jugador_cochinillos: Jugador,
    jugadores_cochinillos: usize,
    partidos_ganados: usize,
    partidos_perdidos: usize,
    estadisticas_jugador: EstadisticasJugador,
    competiciones: Vec<Competicion>,
    partidos: Vec<PartidoMostrar>
}

#[derive(Clone, Debug)]
pub struct PartidoMostrar {
    pub id: u32,
    pub fecha: Date,
    pub lugar: String,
    pub competicion: Competicion,
    pub equipoCasa: Equipo,
    pub equipoVisitante: Equipo,
    pub resultado: (i64, i64)
}

async fn estadisticas(
    State(cm): State<ControladorModelo>,
    ctx: Option<Ctx>,
) -> Result<EstadisticasTemplate> {
    let cochinillos = ControladorEquipo::equipo_id(cm.clone(), 1).await?.ok_or(Error::NoEncontradoPorId)?;
    let jugador_cochinillos = ControladorJugador::jugador_random_equipo(cm.clone(), 1).await?.ok_or(Error::NoEncontradoPorId)?;
    let jugadores_cochinillos = ControladorJugador::listar_jugadores_equipo(cm.clone(), 1).await?.len();
    let partidos_ganados = ControladorPartido::listar_partidos_ganados_equipo(cm.clone(), 1).await?.len();
    let partidos_perdidos = ControladorPartido::listar_partidos_perdidos_equipo(cm.clone(), 1).await?.len();
    let estadisticas_jugador = ControladorEvento::estadisticas_jugador_id(cm.clone(), jugador_cochinillos.id).await?;
    let competiciones = ControladorCompeticion::listar_competiciones(cm.clone()).await?;
    let mut partidos: Vec<PartidoMostrar> = future::try_join_all(ControladorPartido::listar_partidos_equipo(cm.clone(), 1).await?
        .iter().map(|partido| async {
        let competicion = ControladorCompeticion::competicion_id(cm.clone(), partido.idCompeticion).await?.ok_or(Error::NoEncontradoPorId)?;
        let equipoCasa = ControladorEquipo::equipo_id(cm.clone(), partido.idEquipoCasa).await?.ok_or(Error::NoEncontradoPorId)?;
        let equipoVisitante = ControladorEquipo::equipo_id(cm.clone(), partido.idEquipoVisitante).await?.ok_or(Error::NoEncontradoPorId)?;
        let resultado = ControladorEvento::resultado_partido_id(cm.clone(), partido.id).await?;
        Ok::<PartidoMostrar, Error>(PartidoMostrar {
            id: partido.id,
            fecha: partido.fecha,
            lugar: partido.lugar.clone(),
            competicion,
            equipoCasa,
            equipoVisitante,
            resultado
        })
    })).await?;

    partidos.sort_by(|a, b| b.fecha.cmp(&a.fecha));

    Ok(EstadisticasTemplate { 
        cochinillos, 
        jugador_cochinillos, 
        jugadores_cochinillos, 
        partidos_ganados,
        partidos_perdidos,
        estadisticas_jugador,
        competiciones,
        partidos
    })
}

#[derive(Template)]
#[template(path = "partido.html")]
struct PartidoTemplate {
    partido: PartidoMostrar,
    jugadores_casa: Vec<JugadorMostrar>,
    jugadores_visitante: Vec<JugadorMostrar>,
    eventos: Vec<EventoMostrar>
}

#[derive(Clone, Debug)]
pub struct JugadorMostrar {
    pub jugador: Jugador,
    pub estadisticas_jugador: EstadisticasJugador
}

#[derive(Clone, Debug)]
pub struct EventoMostrar {
    pub id: u32,
    pub valor: Option<u16>,
    pub minuto: Option<Time>,
    pub tipoEvento: TipoEvento,
    pub jugador: Jugador
}

async fn partido(
    State(cm): State<ControladorModelo>,
    Path(id): Path<u32>
) -> Result<PartidoTemplate> {
    let partido_temp = ControladorPartido::partido_id(cm.clone(), id).await?.ok_or(Error::NoEncontradoPorId)?;
    let competicion = ControladorCompeticion::competicion_id(cm.clone(), partido_temp.idCompeticion).await?.ok_or(Error::NoEncontradoPorId)?;
    let equipoCasa = ControladorEquipo::equipo_id(cm.clone(), partido_temp.idEquipoCasa).await?.ok_or(Error::NoEncontradoPorId)?;
    let equipoVisitante = ControladorEquipo::equipo_id(cm.clone(), partido_temp.idEquipoVisitante).await?.ok_or(Error::NoEncontradoPorId)?;
    let resultado = ControladorEvento::resultado_partido_id(cm.clone(), partido_temp.id).await?;
    let partido = PartidoMostrar {
        id: partido_temp.id,
        fecha: partido_temp.fecha,
        lugar: partido_temp.lugar,
        competicion,
        equipoCasa,
        equipoVisitante,
        resultado
    };
    let jugadores = future::try_join_all(ControladorJugador::listar_jugadores_partido(cm.clone(), partido.id).await?
        .iter().map(|jugador| async {
            let estadisticas_jugador = ControladorEvento::estadisticas_jugador_partido_id(cm.clone(), jugador.id, partido.id).await?;
            Ok::<JugadorMostrar, Error>(JugadorMostrar {
                jugador: jugador.clone(),
                estadisticas_jugador
            })
        })).await?;
    let mut eventos = future::try_join_all(ControladorEvento::listar_eventos_partido(cm.clone(), partido.id).await?
        .iter().filter(|evento| evento.idTipoEvento != 1 && evento.idTipoEvento != 3).map(|evento| async {
            let tipoEvento = ControladorTipoEvento::tipo_evento_id(cm.clone(), evento.idTipoEvento).await?.ok_or(Error::NoEncontradoPorId)?;
            let jugador = ControladorJugador::jugador_id(cm.clone(), evento.idJugador).await?.ok_or(Error::NoEncontradoPorId)?;
            Ok::<EventoMostrar, Error>(EventoMostrar {
                id: evento.id,
                valor: evento.valor,
                minuto: evento.minuto,
                tipoEvento,
                jugador
            })
        })).await?;

    let jugadores_casa = jugadores.iter().filter(|jugador| jugador.jugador.idEquipo == partido.equipoCasa.id).map(|jugador| jugador.to_owned()).collect();
    let jugadores_visitante = jugadores.iter().filter(|jugador| jugador.jugador.idEquipo == partido.equipoVisitante.id).map(|jugador| jugador.to_owned()).collect();

    eventos.sort_by(|a, b| a.minuto.cmp(&b.minuto));

    Ok(PartidoTemplate { partido, jugadores_casa, jugadores_visitante, eventos })
}
