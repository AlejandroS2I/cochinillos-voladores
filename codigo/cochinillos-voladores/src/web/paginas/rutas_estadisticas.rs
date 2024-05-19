use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{delete, get, post};
use axum::{Form, Router};
use axum::response::{IntoResponse, Response};
use askama::Template;
use futures::future;
use time::Date;

use crate::ctx::Ctx;
use crate::modelo::categorias::{Categoria, ControladorCategoria};
use crate::modelo::competiciones::{Competicion, ControladorCompeticion};
use crate::modelo::equipos::{ControladorEquipo, Equipo};
use crate::modelo::eventos::{ControladorEvento, EstadisticasJugador};
use crate::modelo::jugadores::{ControladorJugador, Jugador};
use crate::modelo::partidos::ControladorPartido;
use crate::modelo::{ControladorModelo};
use crate::{Result, Error};

pub fn routes(cm: ControladorModelo) -> Router {
    Router::new()
        .route("/estadisticas", get(estadisticas))
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
    partidos: Vec<ListaPartidos>
}

#[derive(Clone, Debug)]
pub struct ListaPartidos {
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
    let mut partidos: Vec<ListaPartidos> = future::try_join_all(ControladorPartido::listar_partidos_equipo(cm.clone(), 1).await?
        .iter().map(|partido| async {
        let competicion = ControladorCompeticion::competicion_id(cm.clone(), partido.idCompeticion).await?.ok_or(Error::NoEncontradoPorId)?;
        let equipoCasa = ControladorEquipo::equipo_id(cm.clone(), partido.idEquipoCasa).await?.ok_or(Error::NoEncontradoPorId)?;
        let equipoVisitante = ControladorEquipo::equipo_id(cm.clone(), partido.idEquipoVisitante).await?.ok_or(Error::NoEncontradoPorId)?;
        let resultado = ControladorEvento::resultado_partido_id(cm.clone(), partido.id).await?;
        Ok::<ListaPartidos, Error>(ListaPartidos {
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
