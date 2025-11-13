//! # Funciones de servicio para la base de datos
//! En este módulo se incluyen estructuras y funciones con los siguientes fines:
//! - Estructuras para deserializar los CSV con información de localidades, municipios, etc. 
//! - Funciones para poblar las tablas (catálogos de estado, municipio, etc) en la base de datos
use crate::{entities::prelude::*, entities::*, utils::conversores::leer_catalogo};
use sea_orm::{DatabaseConnection, EntityTrait, Set};
use actix_web::{error, Error};
const BATCH_SIZE: usize = 5000;

/// Estructura para deserializar el csv de estados. 
/// Lee únicamente Id (que coincide con los ids políticos) y nombre del estado.
#[derive(Debug, serde::Deserialize)]
struct Estado {
    id_estado: i32,
    estado: String,
}
/// Estructura para deserializar el csv de municipios. 
/// Lee id, nombre del municipio y id del estado al que pertenece el municipio.
#[derive(Debug, serde::Deserialize)]
struct Municipio {
    id_municipio: i32,
    municipio: String,
    id_estado: i32,
}
/// Estructura para deserializar el csv de códigos postales. 
/// Lee código postal (que actúa como su propio id) y id del municipio y del estado al que pertenece.
#[derive(Debug, serde::Deserialize)]
struct CodigoPostal {
    cp: i32,
    id_municipio: i32,
    id_estado: i32,
}
/// Estructura para deserializar el csv de municipios. 
/// Lee id, nombre de la localidad y código postal y ids del estado y muninicipio al que pertenece.
#[derive(Debug, serde::Deserialize)]
struct Localidad {
    pub id_localidad: i32,
    pub localidad: String,
    pub cp: i32,
    pub id_municipio: i32,
    pub id_estado: i32,
}

/// Carga todas los estado y municipios desde un archivo CSV y los inserta en la base de datos.
/// - Si la tabla `cat_estados` ya contiene registros, no se realiza ninguna inserción.
/// - Si la tabla está vacía, se leen los registros desde `cat_estados.csv` y se insertan en lotes.
/// - Repite lo mismo para municipios.
///
/// ## Argumentos
/// * `db` - Conexión activa a la base de datos.
/// * `catalogos_path` - Ruta base donde se encuentran los archivos CSV de estados y municipios.
///
/// ## Errores
/// Retorna un InternalServerError ([`actix_web::Error`]) si ocurre algún problema al leer los archivos CSV o insertar en la base de datos.
///
/// ## Ejemplo
/// ```rust
/// llenar_catalogos_municipios(&db, "./catalogos").await?;
/// ```
pub async fn llenar_catalogos_estados_municipios(
    db: &DatabaseConnection,
    catalogos_path: &str,
) -> Result<(), Error> {
    // Verifica si ya existen registros en la tabla de estados
    let registros_edos = CatEstados::find().all(db).await.map_err(error::ErrorInternalServerError)?;
    if registros_edos.is_empty() {
        // Carga los registros desde CSV
        let mut estados = Vec::new();
        let mut rdr = leer_catalogo(catalogos_path, "cat_estados");
        for result in rdr.deserialize() {
            let record: Estado = result.map_err(error::ErrorInternalServerError)?;
            let modelo = cat_estados::ActiveModel {
                id: Set(record.id_estado),
                estado: Set(record.estado),
            };
            estados.push(modelo);
        }
        // Inserta los registros. Como son pocos, no es necesario insertalos por chunks.
        CatEstados::insert_many(estados).exec(db).await.map_err(error::ErrorInternalServerError)?;
    }

    // Verifica si ya existen registros en la tabla de municipios
    let registros_muns = CatMunicipios::find().all(db).await.map_err(error::ErrorInternalServerError)?;
    if registros_muns.is_empty() {
        // Carga los registros desde CSV
        let mut municipios = Vec::new();
        let mut rdr = leer_catalogo(catalogos_path, "cat_municipios");
        for result in rdr.deserialize() {
            let record: Municipio = result.map_err(error::ErrorInternalServerError)?;
            let modelo = cat_municipios::ActiveModel {
                id: Set(record.id_municipio),
                municipio: Set(record.municipio),
                id_estado: Set(record.id_estado),
            };
            municipios.push(modelo);
        }
        // Inserta los registros. Como son "pocos", no es necesario insertalos por chunks.
        CatMunicipios::insert_many(municipios).exec(db).await.map_err(error::ErrorInternalServerError)?;
    }
    Ok(())
}

/// Carga todos los códigos postales desde un archivo CSV y los inserta en la base de datos.
/// - Si la tabla `cat_codigos_postales` ya contiene registros, no se realiza ninguna inserción.
/// - Si la tabla está vacía, se leen los registros desde `cat_codigos_postales.csv` y se insertan en lotes.
///
/// ## Argumentos
/// * `db` - Conexión activa a la base de datos.
/// * `catalogos_path` - Ruta base donde se encuentra el archivo CSV de códigos postales.
///
/// ## Errores
/// Retorna un InternalServerError ([`actix_web::Error`]) si ocurre algún problema al leer el archivo CSV o insertar en la base de datos.
///
/// ## Ejemplo
/// ```rust
/// llenar_catalogos_cps(&db, "./catalogos").await?;
/// ```
pub async fn llenar_catalogos_cps(
    db: &DatabaseConnection,
    catalogos_path: &str,
) -> Result<(), Error> {
    // Verifica si ya existen registros en la tabla
    let registros = CatCodigosPostales::find().all(db).await.map_err(error::ErrorInternalServerError)?;
    if registros.is_empty() {
        // Carga los registros desde CSV
        let mut rdr = leer_catalogo(catalogos_path, "cat_codigos_postales");
        let mut codigos_postales = Vec::new();
        for result in rdr.deserialize() {
            let record: CodigoPostal = result.map_err(error::ErrorInternalServerError)?;
            let modelo = cat_codigos_postales::ActiveModel {
                codigo_postal: Set(record.cp),
                id_municipio: Set(record.id_municipio),
                id_estado: Set(record.id_estado),
            };
            codigos_postales.push(modelo);
        }
        // Inserta los registros en lotes
        for chunk in codigos_postales.chunks(BATCH_SIZE) {
            CatCodigosPostales::insert_many(chunk.to_vec())
                .exec(db)
                .await.map_err(error::ErrorInternalServerError)?;
        }
    }
    Ok(())
}

/// Carga todas las localidades desde un archivo CSV y las inserta en la base de datos.
/// - Si la tabla `cat_localidades` ya contiene registros, no se realiza ninguna inserción.
/// - Si la tabla está vacía, se leen los registros desde `cat_localidades.csv` y se insertan en lotes.
///
/// ## Argumentos
/// * `db` - Conexión activa a la base de datos.
/// * `catalogos_path` - Ruta base donde se encuentra el archivo CSV de localidades.
///
/// ## Errores
/// Retorna un InternalServerError ([`actix_web::Error`]) si ocurre algún problema al leer el archivo CSV o insertar en la base de datos.
///
/// ## Ejemplo
/// ```rust
/// llenar_catalogos_localidades(&db, "./catalogos").await?;
/// ```
pub async fn llenar_catalogos_localidades(
    db: &DatabaseConnection,
    catalogos_path: &str,
) -> Result<(), Error> {
    // Verifica si ya existen registros en la tabla
    let registros = CatLocalidades::find().all(db).await.map_err(error::ErrorInternalServerError)?;
    if registros.is_empty() {
        // Carga los registros desde CSV
        let mut rdr = leer_catalogo(catalogos_path, "cat_localidades");
        let mut localidades = Vec::new();
        for result in rdr.deserialize() {
            let record: Localidad = result.map_err(error::ErrorInternalServerError)?;
            let modelo = cat_localidades::ActiveModel {
                id: Set(record.id_localidad),
                localidad: Set(record.localidad),
                codigo_postal: Set(record.cp),
                id_municipio: Set(record.id_municipio),
                id_estado: Set(record.id_estado),
            };
            localidades.push(modelo);
        }
        // Inserta los registros en lotes
        for chunk in localidades.chunks(BATCH_SIZE) {
            CatLocalidades::insert_many(chunk.to_vec()).exec(db).await.map_err(error::ErrorInternalServerError)?;
        }
    }
    Ok(())
}