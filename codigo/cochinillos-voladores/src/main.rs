use askama::Template;
use axum::{
    extract::State, middleware, response::Response, routing::get, Extension, Router
};
use modelo::ControladorModelo;
use tokio::{net::TcpListener, sync::Mutex};
use sqlx::{
    Pool,
    MySql,
    mysql::MySqlPoolOptions
};
use dotenvy::dotenv;
use tower_cookies::CookieManagerLayer;

pub use self::error::{Error, Result};

mod error;
mod tests;
mod modelo;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL tiene que estar definida");
    
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .unwrap_or_else(|_| panic!("No se ha podido conectar con la BBDD URL: {}", url));

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Escuchando en {:?}", listener.local_addr());

    let cm = ControladorModelo::new(pool).await?;

    let app = app(cm);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

fn app(cm: ControladorModelo) -> Router {
    let rutas_api = web::rutas_usuarios::routes(cm.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_requerir_auth));

    Router::new()
        .route("/", get(inicio))
        .nest("/api", web::rutas_login::routes(cm.clone()))
        .nest("/api", rutas_api)
        .layer(middleware::map_response(mapeador_respuestas_central))
        .layer(CookieManagerLayer::new())
}

async fn mapeador_respuestas_central(res: Response) -> Response {
    println!("->> {:<12} - mapeador_respuestas_central", "MAPEADOR_RES");
    println!();
    res
}

#[derive(Template)]
#[template(path = "inicio.html")]
struct InicioTemplate;

async fn inicio() -> InicioTemplate {
    return InicioTemplate;
}
