use crate::{ctx::Ctx, error::ClientError, Error, Result};
use axum::http::{Method, Uri};
use serde::Serialize;
use serde_json::{json, Value};
use serde_with::skip_serializing_none;
use uuid::Uuid;

pub async fn log_request(
    uuid: Uuid,
    metodo_req: Method,
    uri: Uri,
    ctx: Option<Ctx>,
    error_servicio: Option<&Error>,
    error_cliente: Option<ClientError>
) -> Result<()> {
    let timestamp = time::OffsetDateTime::now_utc();
    
    let tipo_error = error_servicio.map(|se| se.as_ref().to_string());
    let datos_error = serde_json::to_value(error_servicio)
        .ok()
        .and_then(|mut v| v.get_mut("data").map(|v| v.take()));

    let linea = LineaLogRequest {
        uuid: uuid.to_string(),
        timestamp: timestamp.to_string(),

        ruta_req: uri.to_string(),
        metodo_req: metodo_req.to_string(),

        id_usuario: ctx.map(|c| c.idUsuario()),

        tipo_error_cliente: error_cliente.map(|e| e.as_ref().to_string()),

        tipo_error,
        datos_error
    };

    println!("      ->> log_peticion: \n{}", json!(linea));

    Ok(())
}

#[skip_serializing_none]
#[derive(Serialize)]
struct LineaLogRequest {
    uuid: String,
    timestamp: String,

    id_usuario: Option<u32>,

    ruta_req: String,
    metodo_req: String,

    tipo_error_cliente: Option<String>,
    tipo_error: Option<String>,
    datos_error: Option<Value>
}
