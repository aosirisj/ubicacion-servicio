use crate::{entities::prelude::*, entities::*};
use sea_orm::{DatabaseConnection, EntityTrait, Set};
use std::{error::Error, fs::File};
const BATCH_SIZE: usize = 5000;

#[derive(Debug, serde::Deserialize)]
struct Estado {
    id_estado: i32,
    estado: String,
}
#[derive(Debug, serde::Deserialize)]
struct Municipio {
    id_municipio: i32,
    municipio: String,
    id_estado: i32,
}
#[derive(Debug, serde::Deserialize)]
struct CodigoPostal {
    cp: i32,
    id_municipio: i32,
    id_estado: i32,
}
#[derive(Debug, serde::Deserialize)]
struct Localidad {
    pub id_localidad: i32,
    pub localidad: String,
    pub cp: i32,
    pub id_municipio: i32,
    pub id_estado: i32,
}

pub async fn llenar_catalogos_estados_municipios(
    db: &DatabaseConnection,
    catalogos_path: &str,
) -> Result<(), Box<dyn Error>> {
    let estados = CatEstados::find().all(db).await?;
    if estados.is_empty() {
        let mut estados = Vec::new();
        let estados_path = format!("{}/cat_estados.csv", catalogos_path);
        let archivo = File::open(estados_path)?;
        let mut rdr = csv::Reader::from_reader(archivo);
        for result in rdr.deserialize() {
            let record: Estado = result?;
            let modelo = cat_estados::ActiveModel {
                id: Set(record.id_estado),
                estado: Set(record.estado),
            };
            estados.push(modelo);
        }
        CatEstados::insert_many(estados).exec(db).await?;
    }

    let municipios = CatMunicipios::find().all(db).await?;
    if municipios.is_empty() {
        let mut municipios = Vec::new();
        let municipios_path = format!("{}/cat_municipios.csv", catalogos_path);
        let archivo = File::open(municipios_path)?;
        let mut rdr = csv::Reader::from_reader(archivo);
        for result in rdr.deserialize() {
            let record: Municipio = result?;
            let modelo = cat_municipios::ActiveModel {
                id: Set(record.id_municipio),
                municipio: Set(record.municipio),
                id_estado: Set(record.id_estado),
            };
            municipios.push(modelo);
        }
        CatMunicipios::insert_many(municipios).exec(db).await?;
    }

    Ok(())
}

pub async fn llenar_catalogos_cps(
    db: &DatabaseConnection,
    catalogos_path: &str,
) -> Result<(), Box<dyn Error>> {
    let codigos_postales = CatCodigosPostales::find().all(db).await?;
    if codigos_postales.is_empty() {
        let mut codigos_postales = Vec::new();
        let codigos_postales_path = format!("{}/cat_codigos_postales.csv", catalogos_path);
        let archivo = File::open(codigos_postales_path)?;
        let mut rdr = csv::Reader::from_reader(archivo);
        for result in rdr.deserialize() {
            let record: CodigoPostal = result?;
            let modelo = cat_codigos_postales::ActiveModel {
                codigo_postal: Set(record.cp),
                id_municipio: Set(record.id_municipio),
                id_estado: Set(record.id_estado),
            };
            codigos_postales.push(modelo);
        }

        for chunk in codigos_postales.chunks(BATCH_SIZE) {
            CatCodigosPostales::insert_many(chunk.to_vec())
                .exec(db)
                .await?;
        }
    }

    Ok(())
}

pub async fn llenar_catalogos_localidades(
    db: &DatabaseConnection,
    catalogos_path: &str,
) -> Result<(), Box<dyn Error>> {
    let localidades = CatLocalidades::find().all(db).await?;
    if localidades.is_empty() {
        let mut localidades = Vec::new();
        let localidades_path = format!("{}/cat_localidades.csv", catalogos_path);
        let archivo = File::open(localidades_path)?;
        let mut rdr = csv::Reader::from_reader(archivo);
        for result in rdr.deserialize() {
            let record: Localidad = result?;
            let modelo = cat_localidades::ActiveModel {
                id: Set(record.id_localidad),
                localidad: Set(record.localidad),
                codigo_postal: Set(record.cp),
                id_municipio: Set(record.id_municipio),
                id_estado: Set(record.id_estado),
            };
            localidades.push(modelo);
        }
        for chunk in localidades.chunks(BATCH_SIZE) {
            CatLocalidades::insert_many(chunk.to_vec()).exec(db).await?;
        }
    }

    Ok(())
}
