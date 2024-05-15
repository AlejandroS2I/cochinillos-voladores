use axum::body::Bytes;
use dotenvy::dotenv;
use time::{macros::format_description, OffsetDateTime};
use tokio::{fs::{self, File}, io::AsyncWriteExt};

use crate::{Result, Error};

pub async fn subir_archivo(
    carpeta: String,
    datos_archivo: Bytes,
    nombre_archivo: String
) -> Result<String> {
    dotenv().ok();

    let ruta_uploads = std::env::var("RUTA_UPLOADS").expect("ERROR: Ruta uploads no especificada");

    if !fs::try_exists(format!("{}/{}", ruta_uploads, carpeta)).await.map_err(|_|Error::Generico { error: "Error comprobando carpeta".to_string() })? == true {
        fs::create_dir_all(format!("{}/{}", ruta_uploads, carpeta)).await.map_err(|_|Error::Generico { error: "Error creando carpeta".to_string() })?;
    }

    let timestamp = OffsetDateTime::now_utc()
        .format(format_description!("[year][month][day][hour][minute]"))
        .map_err(|e|Error::Generico { error: e.to_string() })?
        .to_string();
    let nombre = format!("{}/{}-{}", carpeta, timestamp, nombre_archivo);

    let mut archivo = File::create(format!("{}/{}", ruta_uploads, nombre.clone())).await.map_err(|err| Error::ErrorAlCrearArchivo{ error: err.to_string() })?;
    archivo.write(&datos_archivo).await.map_err(|err| Error::ErrorAlCrearArchivo{ error: err.to_string() })?;

    Ok(nombre)
}

pub async fn eliminar_archivo(
    url: String
) -> Result<()> {
    dotenv().ok();

    let ruta_uploads = std::env::var("RUTA_UPLOADS").expect("ERROR: Ruta uploads no especificada");

    fs::remove_file(format!("{}/{}", ruta_uploads, url)).await.map_err(|err| Error::ErrorAlBorrarArchivo{ error: err.to_string() })?;

    Ok(())
}
