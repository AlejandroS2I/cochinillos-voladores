use serde::Deserialize;
use time::Date;
use crate::ctx::Ctx;
use crate::modelo::{Error, Result};
use crate::modelo::ControladorModelo;


#[derive(Clone, Debug, sqlx::FromRow)]
pub struct Noticia {
    pub id: u32,
    pub titulo: String,
    pub descripcion: String,
    pub fecha: Date,
    pub fotoURL: Option<String>
}

#[derive(Deserialize)]
pub struct NoticiaCrear {
    pub titulo: String,
    pub descripcion: String,
    pub fecha: Date,
    pub fotoURL: Option<String>
}

#[derive(Deserialize)]
pub struct NoticiaActualizar {
    pub id: u32,
    pub titulo: String,
    pub descripcion: String,
    pub fecha: Date,
    pub fotoURL: Option<String>
}

pub struct ControladorNoticia;


impl ControladorNoticia {
    pub async fn crear_noticia(
        cm: ControladorModelo, 
        noticia: NoticiaCrear
    ) -> Result<Noticia> {
        let pool = cm.conexion;
        let mut txn = pool.begin().await?;

        sqlx::query!("
            INSERT INTO tnoticias (titulo, descripcion, fecha, fotoURL) 
            VALUES (?, ?, ?, ?); 
        ",
            noticia.titulo,
            noticia.descripcion,
            noticia.fecha,
            noticia.fotoURL,
        )
        .execute(txn.as_mut())
        .await?;

        let noticia = sqlx::query_as!(
        Noticia,
        "
            SELECT id, titulo, descripcion, fecha, fotoURL FROM tnoticias
            WHERE id = LAST_INSERT_ID();
        ")
        .fetch_one(txn.as_mut())
        .await?;

        txn.commit().await?;

        Ok(noticia)
    }

    pub async fn actualizar_noticia(
        cm: ControladorModelo, 
        noticia: NoticiaActualizar
    ) -> Result<Noticia> {
        let pool = cm.conexion;
        let mut txn = pool.begin().await?;

        match noticia.fotoURL {
            Some(url) => {
                sqlx::query!("
                    UPDATE tnoticias SET titulo = ?, descripcion = ?, fecha = ?, fotoURL = ?
                    WHERE id = ?
                ",
                    noticia.titulo,
                    noticia.descripcion,
                    noticia.fecha,
                    url,
                    noticia.id
                )
                .execute(txn.as_mut())
                .await?;
            },
            None => {
                sqlx::query!("
                    UPDATE tnoticias SET titulo = ?, descripcion = ?, fecha = ?
                    WHERE id = ?
                ",
                    noticia.titulo,
                    noticia.descripcion,
                    noticia.fecha,
                    noticia.id
                )
                .execute(txn.as_mut())
                .await?;
            }
        }

        let noticia = sqlx::query_as!(
        Noticia,
        "
            SELECT id, titulo, descripcion, fecha, fotoURL FROM tnoticias
            WHERE id = ?;
        ",
            noticia.id
        )
        .fetch_one(txn.as_mut())
        .await?;

        txn.commit().await?;

        Ok(noticia)
    }

    pub async fn listar_noticias(
        cm: ControladorModelo,
        limite: Option<u32>
    ) -> Result<Vec<Noticia>> {
        let pool = cm.conexion;

        let noticia = match limite {
            Some(limite) => {
                sqlx::query_as!(
                    Noticia,
                    "
                        SELECT id, titulo, descripcion, fecha, fotoURL FROM tnoticias
                        ORDER BY fecha DESC
                        LIMIT ?
                    ",
                        limite
                )
                .fetch_all(&pool)
                .await?
            },
            None => {
                sqlx::query_as!(
                    Noticia,
                    "
                        SELECT id, titulo, descripcion, fecha, fotoURL FROM tnoticias
                        ORDER BY fecha DESC
                    ",
                )
                .fetch_all(&pool)
                .await?
            }
        };

        Ok(noticia)
    }

    pub async fn eliminar_noticia(
        ctx: Ctx,
        cm: ControladorModelo, 
        id: u32
    ) -> Result<Noticia> {
        let pool = cm.conexion;
        let mut txn = pool.begin().await?;

        let noticia = sqlx::query_as!(
        Noticia,
        "
            SELECT id, titulo, descripcion, fecha, fotoURL
            FROM tnoticias WHERE id = ?;
        ",
            id
        )
        .fetch_optional(txn.as_mut())
        .await?
        .ok_or(Error::NoEncontrado { id })?;

        sqlx::query!(
        "
            DELETE FROM tnoticias WHERE id = ?
        ",
            id
        )
        .execute(txn.as_mut())
        .await?;

        txn.commit().await?;

        Ok(noticia)
    }

    pub async fn noticia_id(cm: ControladorModelo, id: u32) -> Result<Option<Noticia>> {
        let pool = cm.conexion;

        let noticia = sqlx::query_as!(
        Noticia,
        "
            SELECT id, titulo, descripcion, fecha, fotoURL FROM tnoticias
            WHERE id = ?
        ",
            id
        )
        .fetch_optional(&pool)
        .await?;

        Ok(noticia)
    }
}
