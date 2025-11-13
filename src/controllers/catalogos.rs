//! # Controladores para endpoints de ubicación
//! En este módulo se incluyen controladores de endpoints con las siguientes funcionalidades:
//! - Obtener estado, municipio y localidades a partir de un código postal (`busqueda_cp_controller`)

use crate::{
    entities::{prelude::*, *},
    types::catalogos::*,
    utils::conversores::*,
};
use actix_web::{error, web, Error};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

/// Dado un código postal, devuelve los ids y nombres del estado, municipio y localidades correspondientes
///
/// ## Parámetros
/// - `db`: Conexión a la base de datos
/// - `cp`: Código postal del que se quieren obtener los datos
///
/// ## Retorno
/// - [`CPResponse`]: Contiene estructuras para el estado y municipio correspondientes y un vector para las localidades
/// - `Err(BadRequest)`: El formato de CP no es válido
/// - `Err(NotFound)`: El CP introducido no fue encontrado
/// - `Err(InternalServerError)`: Si ocurre un error inesperado durante la consulta a la base de datos
///
/// ## Errores
/// Devuelve [`actix_web::Error`] en los casos antes mencionados.
pub async fn busqueda_cp_controller(
    db: web::Data<DatabaseConnection>,
    cp: i32,
) -> Result<CPResponse, Error> {
    // Valida el formato del CP
    if cp > 99999 || cp < 1000 {
        return Err(error::ErrorBadRequest("Formato de código postal inválido"));
    }

    // Busca el CP y sus relacionados en la BD
    let resultado: Vec<cat_localidades::Model> = CatLocalidades::find()
        .filter(cat_localidades::Column::CodigoPostal.eq(cp))
        .all(db.get_ref())
        .await
        .map_err(|e| error::ErrorInternalServerError(e))?;

    if resultado.is_empty() {
        return Err(error::ErrorNotFound("Código postal no encontrado"));
    }

    // Obtenemos estado y municipio de la primera localidad
    let primera_localidad = &resultado[0];
    let estado = registro_estructura(
        db.get_ref(),
        CatEstados::find_by_id(primera_localidad.id_estado),
        "Error en el catalogo de estados en la base de datos",
    )
    .await?;
    let municipio = registro_estructura(
        db.get_ref(),
        CatMunicipios::find_by_id(primera_localidad.id_municipio),
        "Error en el catalogo de municipios en la base de datos",
    )
    .await?;

    // Crea vector de localidades
    let localidades: Vec<CatalogoIdCadena> = resultado
        .into_iter()
        .map(|l| CatalogoIdCadena {
            id: l.id,
            value: l.localidad,
        })
        .collect();

    Ok(CPResponse {
        estado,
        municipio,
        localidades,
    })
}
