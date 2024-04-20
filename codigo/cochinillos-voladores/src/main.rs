use askama::Template;
use axum::{
    extract::State, routing::get, Extension, Router
};
use tokio::{net::TcpListener, sync::Mutex};
use sqlx::{
    Pool,
    MySql,
    mysql::MySqlPoolOptions
};
use dotenvy::dotenv;

pub use self::error::{Error, Result};

mod error;
mod tests;
mod web;

#[tokio::main]
async fn main() {
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

    let app = app()
        .layer(Extension(pool));

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

fn app() -> Router {
    Router::new()
        .merge(web::rutas_login::routes())
        .route("/", get(inicio))
}

#[derive(Template)]
#[template(path = "inicio.html")]
struct InicioTemplate;

async fn inicio() -> InicioTemplate {
    return InicioTemplate;
}
