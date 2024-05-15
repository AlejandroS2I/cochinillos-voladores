use askama::{Template};
use axum::{
    middleware,
    routing::get,
    Router
};
use modelo::ControladorModelo;
use tokio::{net::TcpListener};
use sqlx::{
    mysql::MySqlPoolOptions
};
use dotenvy::dotenv;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use web::res_map::mapeador_respuestas_central;

use crate::{
    web::auth,
    web::api,
    web::paginas
};

pub use self::error::{Error, Result};

mod ctx;
mod error;
mod modelo;
mod web;
mod uploads;
mod mail;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL tiene que estar definida");
    let ruta_uploads = std::env::var("RUTA_UPLOADS").expect("ERROR: Ruta uploads no especificada");
    
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .unwrap_or_else(|_| panic!("No se ha podido conectar con la BBDD URL: {}", url));

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Escuchando en {:?}", listener.local_addr());

    let cm = ControladorModelo::new(pool).await?;

    let app = app(cm, ruta_uploads);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

fn app(cm: ControladorModelo, ruta_uploads: String) -> Router {
    let rutas_auth = Router::new()
        .merge(api::rutas_usuarios::routes(cm.clone()))
        .merge(api::rutas_formulario::routes(cm.clone()))
        .route_layer(middleware::from_fn(auth::mw_auth::mw_requerir_auth));

    let rutas_admin = Router::new()
        .merge(paginas::rutas_gestion::routes(cm.clone()))
        .nest("/api", api::rutas_categorias::routes(cm.clone()))
        .nest("/api", api::rutas_noticias::routes(cm.clone()))
        .route_layer(middleware::from_fn(auth::mw_auth::mw_requerir_admin));

    Router::new()
        .merge(paginas::rutas_inicio::routes(cm.clone()))
        .merge(paginas::rutas_login::routes(cm.clone()))
        .merge(paginas::rutas_noticias::routes(cm.clone()))
        .merge(paginas::rutas_formulario::routes(cm.clone()))
        .merge(rutas_admin)
        .nest("/api", api::rutas_login::routes(cm.clone()))
        .nest("/api", rutas_auth)
        .layer(middleware::map_response(mapeador_respuestas_central))
        .layer(middleware::from_fn_with_state(
            cm.clone(),
            auth::mw_auth::mw_resolvedor_ctx
        ))
        .layer(CookieManagerLayer::new())
        .nest_service("/assets", ServeDir::new("assets"))
        .nest_service("/uploads", ServeDir::new(ruta_uploads))
}

#[cfg(test)]
mod tests;
