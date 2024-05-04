use std::sync::Arc;

use axum::{
    http::{Method, Uri},
    response::{Html, Response, IntoResponse}
};
use uuid::Uuid;
use crate::{ctx::Ctx, log::log_request};

use crate::{Result, Error};

pub async fn mapeador_respuestas_central(
    ctx: Option<Ctx>,
    uri: Uri,
    metodo_req: Method,
    res: Response
) -> Response {
    println!("->> {:<12} - mapeador_respuestas_central", "MAPEADOR_RES");
    let uuid = Uuid::now_v7();

    let error_servicio = res.extensions().get::<Arc<Error>>().map(Arc::as_ref);
    let error_estado_cliente = error_servicio.map(|se| se.estado_error_cliente());

    let error_response = error_estado_cliente
        .as_ref()
        .map(|(status_code, error_cliente)| {
            let body = format!("
                <p>Tipo: {}</p>
                <p>Id: {}</p>
            ", error_cliente.as_ref(), uuid.to_string());

            (*status_code, Html(body)).into_response()
        });

    // Logear al servidor
    let error_cliente = error_estado_cliente.unzip().1;
    log_request(uuid, metodo_req, uri, ctx, error_servicio, error_cliente).await;

    println!();
    error_response.unwrap_or(res)
}
