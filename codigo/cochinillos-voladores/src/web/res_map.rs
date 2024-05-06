use std::sync::Arc;

use axum::{
    http::{Method, Uri},
    response::{Html, Response, IntoResponse}
};
use uuid::Uuid;
use crate::{ctx::Ctx};

use crate::{Result, Error};

pub async fn mapeador_respuestas_central(
    ctx: Option<Ctx>,
    uri: Uri,
    metodo_req: Method,
    res: Response
) -> Response {
    let uuid = Uuid::now_v7();

    let error_servicio = res.extensions().get::<Arc<Error>>().map(Arc::as_ref);
    // Posible log 
    println!("ERROR: {:?}", error_servicio);
    println!("RESPUESTA: {:?}", res);

    println!();
    res
}
