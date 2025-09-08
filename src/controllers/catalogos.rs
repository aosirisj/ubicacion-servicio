use crate::{
    entities::{prelude::*, *},
    types::catalogos::*,
};
use actix_web::{error, web, Error};
use core::{
    result::Result::{Err, Ok},
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

// Función para buscar CP
pub async fn busqueda_cp_controller(
    db: web::Data<DatabaseConnection>,
    cp: i32,
) -> Result<CPResponse, Error> {
    // Valida el formato de la CURP
    if cp > 99999 {
        return Err(error::ErrorBadRequest("Formato de código postal inválido"));
    }

    // Busca el CP y sus relacionados en la BD
    let resultado: Vec<cat_localidades::Model> = CatLocalidades::find()
        .filter(cat_localidades::Column::CodigoPostal.eq(cp))
        .all(db.get_ref())
        .await
        .map_err(|e| error::ErrorNotFound(e))?;

    // Crea vector de localidades
    if let Some(primera_localidad) = resultado.clone().first() {
        let mut localidades: Vec<Entradas> = Vec::new();
        for localidad in resultado {
            localidades.push(Entradas {
                id: localidad.id,
                value: localidad.localidad,
            });
        }

        let estado = match CatEstados::find_by_id(primera_localidad.id_estado)
            .one(db.get_ref())
            .await
            .map_err(|e| error::ErrorNotFound(e))?
        {
            Some(l) => Entradas {
                id: l.id,
                value: l.estado,
            },
            None => {
                return Err(error::ErrorInternalServerError(
                    "Error en el catalogo de estados en la base de datos",
                ))
            }
        };
        let municipio = match CatMunicipios::find_by_id(primera_localidad.id_municipio)
            .one(db.get_ref())
            .await
            .map_err(|e| error::ErrorNotFound(e))?
        {
            Some(l) => Entradas {
                id: l.id,
                value: l.municipio,
            },
            None => {
                return Err(error::ErrorInternalServerError(
                    "Error en el catalogo de municipios en la base de datos",
                ))
            }
        };

        Ok(CPResponse {
            estado,
            municipio,
            localidades,
        })
    } else {
        Err(error::ErrorNotFound("Código postal no encontrado"))
    }
}

