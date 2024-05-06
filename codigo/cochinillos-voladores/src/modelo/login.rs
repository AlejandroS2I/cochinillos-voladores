use serde::Deserialize;
use uuid::Uuid;
use time::{Date, OffsetDateTime, Duration};

use crate::modelo::{Error, Result};
use crate::modelo::ControladorModelo;


#[derive(Clone, Debug, sqlx::FromRow)]
pub struct Login {
    pub uuid: Uuid,
    pub idUsuario: u32,
    pub fechaCaducidad: Date
}

pub struct ControladorLogin;

impl ControladorLogin {
    pub async fn crear_login(cm: ControladorModelo, idUsuario: u32) -> Result<Login> {
        let pool = cm.conexion;
        let mut txn = pool.begin().await?;

        let caducidad = OffsetDateTime::now_utc().date() + Duration::days(30);
        let uuid = Uuid::now_v7();

        println!("CADUCIDAD: {}", caducidad.to_string());
        println!("HOY: {}", OffsetDateTime::now_utc().to_string());
        sqlx::query!("
            INSERT INTO tlogins (uuid, idUsuario, fechaCaducidad) 
            VALUES (?, ?, ?); 
        ",
            uuid,
            idUsuario,
            caducidad
        )
        .execute(txn.as_mut())
        .await?;

        let login = sqlx::query_as!(
        Login,
        "
            SELECT uuid as `uuid: uuid::Uuid`, idUsuario, fechaCaducidad FROM tlogins
            WHERE uuid = ?;
        ",
            uuid
        )
        .fetch_one(txn.as_mut())
        .await?;

        txn.commit().await?;

        Ok(login)
    }

    pub async fn actualizar_login(cm: ControladorModelo, login_anterior: Login) -> Result<Login> {
        let pool = cm.conexion;
        let mut txn = pool.begin().await?;

        let caducidad = OffsetDateTime::now_utc().date() + Duration::days(30);
        let uuid = Uuid::now_v7();

        sqlx::query!("
            UPDATE tlogins SET uuid = ?, fechaCaducidad = ?
            WHERE uuid = ?; 
        ",
            uuid,
            caducidad,
            login_anterior.uuid
        )
        .execute(txn.as_mut())
        .await?;

        let login = sqlx::query_as!(
        Login,
        "
            SELECT uuid as `uuid: uuid::Uuid`, idUsuario, fechaCaducidad FROM tlogins
            WHERE uuid = ?;
        ",
            uuid
        )
        .fetch_one(txn.as_mut())
        .await?;

        txn.commit().await?;

        Ok(login)
    }

    pub async fn listar_logins(cm: ControladorModelo) -> Result<Vec<Login>> {
        let pool = cm.conexion;

        let logins: Vec<Login> = sqlx::query_as("
            SELECT uuid, idUsuario, fechaCaducidad FROM tlogins
        ")
        .fetch_all(&pool)
        .await?;

        Ok(logins)
    }

    pub async fn eliminar_login(cm: ControladorModelo, uuid: Uuid) -> Result<Login> {
        let pool = cm.conexion;
        let mut txn = pool.begin().await?;

        let login = sqlx::query_as!(
        Login,
        "
            SELECT uuid as `uuid: uuid::Uuid` , idUsuario, fechaCaducidad
            FROM tlogins WHERE uuid = ?;
        ",
            uuid
        )
        .fetch_optional(txn.as_mut())
        .await?
        .ok_or(Error::NoEncontradoLogin { uuid })?;

        sqlx::query!(
        "
            DELETE FROM tlogins WHERE uuid = ?
        ",
            uuid
        )
        .execute(txn.as_mut())
        .await?;

        txn.commit().await?;

        Ok(login)
    }

    pub async fn login_uuid(cm: ControladorModelo, uuid: Uuid) -> Result<Option<Login>> {
        let pool = cm.conexion;

        let login = sqlx::query_as!(
        Login,
        "
            SELECT uuid as `uuid: uuid::Uuid`, idUsuario, fechaCaducidad FROM tlogins
            WHERE uuid = ?
        ",
            uuid
        )
        .fetch_optional(&pool)
        .await?;

        Ok(login)
    }
}
