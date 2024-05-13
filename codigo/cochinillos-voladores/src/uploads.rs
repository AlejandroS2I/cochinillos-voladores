use axum::body::Bytes;
use time::{macros::format_description, OffsetDateTime};
use tokio::{fs::{self, File}, io::AsyncWriteExt};

use crate::{Result, Error};

pub const RUTA_UPLOADS: &str = "./uploads";

pub async fn subir_archivo(
    carpeta: String,
    datos_archivo: Bytes,
    nombre_archivo: String
) -> Result<String> {
    if !fs::try_exists(format!("{}/{}", RUTA_UPLOADS, carpeta)).await.map_err(|_|Error::Generico { error: "Error comprobando carpeta".to_string() })? == true {
        fs::create_dir_all(format!("{}/{}", RUTA_UPLOADS, carpeta)).await.map_err(|_|Error::Generico { error: "Error creando carpeta".to_string() })?;
    }

    let timestamp = OffsetDateTime::now_utc()
        .format(format_description!("[year][month][day][hour][minute]"))
        .map_err(|e|Error::Generico { error: e.to_string() })?
        .to_string();
    let nombre = format!("{}/{}-{}", carpeta, timestamp, nombre_archivo);

    let mut archivo = File::create(format!("{}/{}", RUTA_UPLOADS, nombre.clone())).await.map_err(|err| Error::ErrorAlCrearArchivo{ error: err.to_string() })?;
    archivo.write(&datos_archivo).await.map_err(|err| Error::ErrorAlCrearArchivo{ error: err.to_string() })?;

    Ok(nombre)
}

pub async fn eliminar_archivo(
    url: String
) -> Result<()> {
    fs::remove_file(format!("{}/{}", RUTA_UPLOADS, url)).await.map_err(|err| Error::ErrorAlBorrarArchivo{ error: err.to_string() })?;

    Ok(())
}
