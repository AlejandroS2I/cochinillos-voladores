use serde::Deserialize;
use crate::modelo::{Error, Result};
use crate::modelo::ControladorModelo;


#[derive(Clone, Debug, sqlx::FromRow)]
pub struct Usuario {
    pub id: u32,
    pub nombre: String,
    pub mail: String,
    pub password: String,
    pub esAdministrador: bool
}

#[derive(Deserialize)]
pub struct UsuarioCrear {
    pub nombre: String,
    pub mail: String,
    pub password: String
}

pub struct ControladorUsuario;


impl ControladorUsuario {
    pub async fn crear_usuario(cm: ControladorModelo, usuario: UsuarioCrear) -> Result<Usuario> {
        let pool = cm.conexion;
        let mut txn = pool.begin().await?;

        sqlx::query!("
            INSERT INTO tusuarios (nombre, mail, password) 
            VALUES (?, ?, ?); 
        ",
            usuario.nombre,
            usuario.mail,
            usuario.password
        )
        .execute(txn.as_mut())
        .await?;

        let usuario = sqlx::query_as!(
        Usuario,
        "
            SELECT id, nombre, mail, password, esAdministrador as `esAdministrador: _` FROM tusuarios
            WHERE id = LAST_INSERT_ID();
        ")
        .fetch_one(txn.as_mut())
        .await
        .map_err(|err| {
                Error::resolver_unico(
                    Error::Sqlx(err), 
                    Some(|tabla: &str, regla: &str| {
                        if tabla == "tusuarios" && regla.contains("mail") {
                            Some(Error::UsuarioExiste { mail: usuario.mail })
                        } else {
                            None
                        }
                    })
                )
        })?;

        txn.commit().await?;

        Ok(usuario)
    }

    pub async fn listar_usuarios(cm: ControladorModelo) -> Result<Vec<Usuario>> {
        let pool = cm.conexion;

        let usuarios: Vec<Usuario> = sqlx::query_as("
            SELECT id, nombre, mail, password, esAdministrador FROM tusuarios
        ")
        .fetch_all(&pool)
        .await?;

        Ok(usuarios)
    }

    pub async fn eliminar_usuario(cm: ControladorModelo, id: u32) -> Result<Usuario> {
        let pool = cm.conexion;
        let mut txn = pool.begin().await?;

        let usuario = sqlx::query_as!(
        Usuario,
        "
            SELECT id, nombre, mail, password, esAdministrador as `esAdministrador: _`
            FROM tusuarios WHERE id = ?;
        ",
            id
        )
        .fetch_optional(txn.as_mut())
        .await?
        .ok_or(Error::NoEncontrado { id })?;

        sqlx::query!(
        "
            DELETE FROM tusuarios WHERE id = ?
        ",
            id
        )
        .execute(txn.as_mut())
        .await?;

        txn.commit().await?;

        Ok(usuario)
    }

    pub async fn usuario_mail(cm: ControladorModelo, mail: String) -> Result<Option<Usuario>> {
        let pool = cm.conexion;

        let usuario = sqlx::query_as!(
        Usuario,
        "
            SELECT id, nombre, mail, password, esAdministrador as `esAdministrador: _` FROM tusuarios
            WHERE mail = ?
        ",
            mail
        )
        .fetch_optional(&pool)
        .await?;

        Ok(usuario)
    }
}
