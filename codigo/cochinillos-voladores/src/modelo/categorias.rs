use serde::Deserialize;
use crate::ctx::Ctx;
use crate::modelo::pwd::hash_password;
use crate::modelo::{Error, Result};
use crate::modelo::ControladorModelo;


#[derive(Clone, Debug, sqlx::FromRow)]
pub struct Categoria {
    pub id: u32,
    pub nombre: String,
}

#[derive(Deserialize)]
pub struct CategoriaCrear {
    pub nombre: String,
}

#[derive(Deserialize)]
pub struct CategoriaActualizar {
    pub id: u32,
    pub nombre: String,
}

pub struct ControladorCategoria;


impl ControladorCategoria {
    pub async fn crear_categoria(
        cm: ControladorModelo, 
        categoria: CategoriaCrear
    ) -> Result<Categoria> {
        let pool = cm.conexion;
        let mut txn = pool.begin().await?;

        sqlx::query!("
            INSERT INTO tcategorias (nombre) 
            VALUES (?); 
        ",
            categoria.nombre,
        )
        .execute(txn.as_mut())
        .await?;

        let categoria = sqlx::query_as!(
        Categoria,
        "
            SELECT id, nombre FROM tcategorias
            WHERE id = LAST_INSERT_ID();
        ")
        .fetch_one(txn.as_mut())
        .await?;

        txn.commit().await?;

        Ok(categoria)
    }

    pub async fn actualizar_categoria(
        cm: ControladorModelo, 
        categoria: CategoriaActualizar
    ) -> Result<Categoria> {
        let pool = cm.conexion;
        let mut txn = pool.begin().await?;

        sqlx::query!("
            UPDATE tcategorias SET nombre = ?
            WHERE id = ?
        ",
            categoria.nombre,
            categoria.id
        )
        .execute(txn.as_mut())
        .await?;

        let categoria = sqlx::query_as!(
        Categoria,
        "
            SELECT id, nombre FROM tcategorias
            WHERE id = ?;
        ",
            categoria.id
        )
        .fetch_one(txn.as_mut())
        .await?;

        txn.commit().await?;

        Ok(categoria)
    }

    pub async fn listar_categorias(
        cm: ControladorModelo
    ) -> Result<Vec<Categoria>> {
        let pool = cm.conexion;

        let categorias: Vec<Categoria> = sqlx::query_as("
            SELECT id, nombre FROM tcategorias
        ")
        .fetch_all(&pool)
        .await?;

        Ok(categorias)
    }

    pub async fn eliminar_categoria(
        ctx: Ctx,
        cm: ControladorModelo, 
        id: u32
    ) -> Result<Categoria> {
        let pool = cm.conexion;
        let mut txn = pool.begin().await?;

        let categoria = sqlx::query_as!(
        Categoria,
        "
            SELECT id, nombre
            FROM tcategorias WHERE id = ?;
        ",
            id
        )
        .fetch_optional(txn.as_mut())
        .await?
        .ok_or(Error::NoEncontrado { id })?;

        sqlx::query!(
        "
            DELETE FROM tcategorias WHERE id = ?
        ",
            id
        )
        .execute(txn.as_mut())
        .await?;

        txn.commit().await?;

        Ok(categoria)
    }

    pub async fn categoria_id(cm: ControladorModelo, id: u32) -> Result<Option<Categoria>> {
        let pool = cm.conexion;

        let categoria = sqlx::query_as!(
        Categoria,
        "
            SELECT id, nombre FROM tcategorias
            WHERE id = ?
        ",
            id
        )
        .fetch_optional(&pool)
        .await?;

        Ok(categoria)
    }
}
