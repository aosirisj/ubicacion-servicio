//! # Utilidades para conversión y carga de catálogos
//!
//! Funciones y traits genéricos para trabajar con catálogos
//! en la aplicación, incluyendo conversión de modelos, consultas genéricas y carga
//! automática desde archivos CSV.
//!
//! Se utiliza principalmente desde los módulos de servicio y controladores de catálogos
//! para:
//! - Evitar duplicación de código al convertir modelos (`Model`) en estructuras simples.
//! - Simplificar consultas genéricas (`find`, `find_by_id`) sobre tablas de catálogos.
//! - Permitir inicializar catálogos básicos desde archivos CSV cuando la base de datos está vacía.
//!
//! ## Componentes
//!
//! - Estructura id y etiqueta ([`CatalogoIdCadena`])
//! - Consulta un registro, devolviendo el modelo ([`registro`])
//! - Consulta un único registro, devolviendo la estructura `{ id, value }` ([`registro_estructura`])
use crate::entities::*;
use actix_web::error;
use sea_orm::{
    DatabaseConnection, FromQueryResult, Select
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use std::fs::File;
use csv::Reader;

// -----------------------------------------------------------------------------
// Conversiones de modelos a estructuras genéricas
// -----------------------------------------------------------------------------

/// Define la conversión genérica de un modelo de catálogo de base de datos
/// (`Model`) hacia una estructura común con identificador y valor ([`CatalogoIdCadena`]).
///
/// Este trait es implementado por cada modelo de catálogo que contiene
/// únicamente un campo de texto descriptivo.
///
/// ## Ejemplo
/// ```rust
/// let banco_model = cat_bancos::Model { id: 1, banco: "BBVA".to_string() };
/// let entrada = banco_model.to_id_value();
/// ```
pub trait CatalogoCombo {
    fn to_id_value(&self) -> CatalogoIdCadena;
}

/// Representa un registro leído desde un archivo CSV que contiene un id y un campo de texto.
#[derive(Debug, Deserialize, Serialize, ToSchema, PartialEq, FromQueryResult)]
pub struct CatalogoIdCadena {
    pub id: i32,
    pub value: String,
}

// Implementaciones del trait para cada catálogo con estructura simple
// (id, cadena descriptiva)
impl CatalogoCombo for cat_estados::Model {
    fn to_id_value(&self) -> CatalogoIdCadena {
        CatalogoIdCadena {
            id: self.id,
            value: self.estado.clone(),
        }
    }
}
impl CatalogoCombo for cat_municipios::Model {
    fn to_id_value(&self) -> CatalogoIdCadena {
        CatalogoIdCadena {
            id: self.id,
            value: self.municipio.clone(),
        }
    }
}

/// Ejecuta una consulta para obtener **un solo registro** de una entidad.
///
/// ## Parámetros
/// - `db`: Conexión a la base de datos
/// - `selector`: Estructura de selector que devuelve modelos `M`
/// - `mensaje`: mensaje de error para NotFound
///
/// ## Retorno
/// - `Ok(modelo)` donde modelo es el resultado de `selector`
/// - `Err(NotFound)` si el selector no arrojó resultados
/// - `Err(InternalServerError)` si ocurre un error inesperado durante la operación con la base de datos
///
/// ## Errores
/// Devuelve [`actix_web::Error`] en los casos antes mencionados
///
/// ## Ejemplo de uso
/// ```rust
/// let ciclo = registro(db, CatCiclos::find_by_id(1), "Ciclo no encontrado".to_owned()).await?;
/// ```
pub async fn registro<M>(
    db: &DatabaseConnection,
    selector: Select<M>,
    mensaje: &'static str,
) -> Result<M::Model, actix_web::Error>
where
    M: sea_orm::EntityTrait,
    M::Model: Send,
{
    match selector
        .one(db)
        .await
        .map_err(|e| error::ErrorInternalServerError(e))?
    {
        Some(l) => Ok(l),
        None => return Err(error::ErrorNotFound(mensaje)),
    }
}

/// Variante de [`registro`] que devuelve la estructura [`CatalogoIdCadena`]
/// en lugar del modelo.
///
/// ## Parámetros
/// - `db`: Conexión a la base de datos
/// - `selector`: Estructura de selector que devuelve modelos `M`
/// - `mensaje`: mensaje de error para NotFound
///
/// ## Retorno
/// - `Ok(estructura)` donde estructura es el modelo obtenido de selector convertido a [`CatalogoIdCadena`]
/// - `Err(NotFound)` si el selector no arrojó resultados
/// - `Err(InternalServerError)` si ocurre un error inesperado durante la operación con la base de datos
///
/// ## Errores
/// Devuelve [`actix_web::Error`] en los casos antes mencionados
///
/// ## Ejemplo de uso
/// ```rust
/// let programa = registro_estructura(db, CatProgramas::find_by_id(programa_ciclo.id_programa), "Error en el catalogo de programas en la base de datos".to_owned()).await?;
/// ```
pub async fn registro_estructura<M>(
    db: &DatabaseConnection,
    selector: Select<M>,
    mensaje: &'static str,
) -> Result<CatalogoIdCadena, actix_web::Error>
where
    M: sea_orm::EntityTrait,
    M::Model: CatalogoCombo + Send,
{
    match registro(db, selector, mensaje).await {
        Ok(l) => Ok(l.to_id_value()),
        Err(e) => Err(e),
    }
}

/// Lee un archivo CSV de catálogos y devuelve su [`csv::Reader`].
///
/// ## Parámetros
/// - `ruta`: directorio en donde se encuentra el catálogo (sin "/" final)
/// - `catalogo`: nombre del catálogo sin extensión
///
/// ## Retorno
/// - `Reader<File>` si el csv pudo leerse correctamente
///
/// ## Panics
/// Si el archivo no puede abrirse o el nombre es inválido.
///
/// ## Ejemplo
/// ```rust
/// leer_catalogo("./catalogos", "cat_estados");
/// ```
pub fn leer_catalogo(ruta: &str, catalogo: &str) -> Reader<File> {
    let catalogo_path = format!("{}/{}.csv", ruta, catalogo);
    let archivo = File::open(catalogo_path)
        .unwrap_or_else(|err| panic!("No se pudo leer el catálogo de {}: {}", catalogo, err));
    csv::Reader::from_reader(archivo)
}