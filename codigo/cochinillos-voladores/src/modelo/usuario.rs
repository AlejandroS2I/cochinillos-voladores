use serde::Deserialize;
use crate::ctx::Ctx;
use crate::modelo::pwd::hash_password;
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

#[derive(Deserialize)]
pub struct UsuarioActualizar {
    pub id: u32,
    pub nombre: String,
    pub mail: String,
    pub esAdministrador: bool,
}

#[derive(Deserialize)]
pub struct UsuarioPassword {
    pub id: u32,
    pub password: String
}

pub struct ControladorUsuario;


impl ControladorUsuario {
    pub async fn crear_usuario(
        cm: ControladorModelo, 
        usuario: UsuarioCrear
    ) -> Result<Usuario> {
        let pool = cm.conexion;
        let mut txn = pool.begin().await?;

        sqlx::query!("
            INSERT INTO tusuarios (nombre, mail, password) 
            VALUES (?, ?, ?); 
        ",
            usuario.nombre,
            usuario.mail.to_lowercase(),
            hash_password(usuario.password)?
        )
        .execute(txn.as_mut())
        .await
        .map_err(|err| {
                Error::resolver_unico(
                    Error::Sqlx(err), 
                    Some(|| {
                        Some(Error::UsuarioExiste { mail: usuario.mail.to_lowercase() })
                    })
                )
        })?;

        let usuario = sqlx::query_as!(
        Usuario,
        "
            SELECT id, nombre, mail, password, esAdministrador as `esAdministrador: _` FROM tusuarios
            WHERE id = LAST_INSERT_ID();
        ")
        .fetch_one(txn.as_mut())
        .await?;

        txn.commit().await?;

        Ok(usuario)
    }

    pub async fn actualizar_usuario(
        cm: ControladorModelo, 
        usuario: UsuarioActualizar
    ) -> Result<Usuario> {
        let pool = cm.conexion;
        let mut txn = pool.begin().await?;

        sqlx::query!("
            UPDATE tusuarios SET nombre = ?, mail = ?, esAdministrador = ?
            WHERE id = ?
        ",
            usuario.nombre,
            usuario.mail.to_lowercase(),
            usuario.esAdministrador,
            usuario.id
        )
        .execute(txn.as_mut())
        .await
        .map_err(|err| {
                Error::resolver_unico(
                    Error::Sqlx(err), 
                    Some(|| {
                        Some(Error::UsuarioExiste { mail: usuario.mail.to_lowercase() })
                    })
                )
        })?;

        let usuario = sqlx::query_as!(
        Usuario,
        "
            SELECT id, nombre, mail, password, esAdministrador as `esAdministrador: _` FROM tusuarios
            WHERE id = ?;
        ",
            usuario.id
        )
        .fetch_one(txn.as_mut())
        .await?;

        txn.commit().await?;

        Ok(usuario)
    }

    pub async fn cambiar_password(
        cm: ControladorModelo, 
        usuario: UsuarioPassword
    ) -> Result<Usuario> {
        let pool = cm.conexion;
        let mut txn = pool.begin().await?;

        sqlx::query!("
            UPDATE tusuarios SET password = ?
            WHERE id = ?
        ",
            hash_password(usuario.password)?,
            usuario.id
        )
        .execute(txn.as_mut())
        .await?;

        let usuario = sqlx::query_as!(
        Usuario,
        "
            SELECT id, nombre, mail, password, esAdministrador as `esAdministrador: _` FROM tusuarios
            WHERE id = ?;
        ",
            usuario.id
        )
        .fetch_one(txn.as_mut())
        .await?;

        txn.commit().await?;

        Ok(usuario)
    }

    pub async fn listar_usuarios(
        ctx: Ctx,
        cm: ControladorModelo
    ) -> Result<Vec<Usuario>> {
        let pool = cm.conexion;

        let usuarios: Vec<Usuario> = sqlx::query_as("
            SELECT id, nombre, mail, password, esAdministrador FROM tusuarios
        ")
        .fetch_all(&pool)
        .await?;

        Ok(usuarios)
    }

    pub async fn eliminar_usuario(
        ctx: Ctx,
        cm: ControladorModelo, 
        id: u32
    ) -> Result<Usuario> {
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

    pub async fn usuario_id(cm: ControladorModelo, id: u32) -> Result<Option<Usuario>> {
        let pool = cm.conexion;

        let usuario = sqlx::query_as!(
        Usuario,
        "
            SELECT id, nombre, mail, password, esAdministrador as `esAdministrador: _` FROM tusuarios
            WHERE id = ?
        ",
            id
        )
        .fetch_optional(&pool)
        .await?;

        Ok(usuario)
    }
}
