//! # Migración de catálogos de ubicación geográfica
//!
//! Esta migración crea las tablas necesarias para almacenar la información
//! jerárquica de estados, municipios, códigos postales y localidades,
//! que sirven de base para el endpoint que consulta ubicaciones por código postal.
//!
//! ## Tablas creadas
//!
//! - cat_estados  
//!   Contiene los nombres de los estados.
//!
//! - cat_municipios  
//!   Depende de `cat_estados`. Almacena los municipios con su respectivo estado.
//!
//! - cat_codigos_postales  
//!   Relaciona un código postal con su municipio y estado.
//!
//! - cat_localidades  
//!   Contiene las localidades asociadas a un código postal, municipio y estado.
//!
//! Cada tabla utiliza claves primarias enteras autoincrementales y define
//! claves foráneas explícitas para mantener la integridad referencial.
#![allow(non_camel_case_types)]
use sea_orm_migration::prelude::*;

/// Migración que crea las tablas de catálogos geográficos:
/// estados, municipios, códigos postales y localidades.
#[derive(DeriveMigrationName)]
pub struct Migration;

/// Estructura de la tabla `cat_estados`
#[derive(DeriveIden)]
pub enum cat_estados {
    Table,
    id,
    estado,
}

/// Estructura de la tabla `cat_municipios`
#[derive(DeriveIden)]
pub enum cat_municipios {
    Table,
    id,
    municipio,
    id_estado,
}

/// Estructura de la tabla `cat_codigos_postales`
#[derive(DeriveIden)]
pub enum cat_codigos_postales {
    Table,
    codigo_postal,
    id_municipio,
    id_estado,
}

/// Estructura de la tabla `cat_localidades`
#[derive(DeriveIden)]
pub enum cat_localidades {
    Table,
    id,
    localidad,
    codigo_postal,
    id_municipio,
    id_estado,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Crea las tablas con sus respectivas claves primarias y foráneas.
    /// El orden de creación sigue la jerarquía natural:
    /// 1. Estados  
    /// 2. Municipios  
    /// 3. Códigos postales  
    /// 4. Localidades
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Tabla de estados
        manager
            .create_table(
                Table::create()
                    .table(cat_estados::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(cat_estados::id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(cat_estados::estado)
                            .string_len(50)
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        // Tabla de municipios
        manager
            .create_table(
                Table::create()
                    .table(cat_municipios::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(cat_municipios::id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(cat_municipios::municipio)
                            .string_len(50)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(cat_municipios::id_estado)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_id_estado")
                            .to(cat_estados::Table, cat_estados::id)
                            .from(cat_municipios::Table, cat_municipios::id_estado),
                    )
                    .to_owned(),
            )
            .await?;
        // Tabla de códigos postales
        manager
            .create_table(
                Table::create()
                    .table(cat_codigos_postales::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(cat_codigos_postales::codigo_postal)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(cat_codigos_postales::id_municipio)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_id_municipio")
                            .to(cat_municipios::Table, cat_municipios::id)
                            .from(
                                cat_codigos_postales::Table,
                                cat_codigos_postales::id_municipio,
                            ),
                    )
                    .col(
                        ColumnDef::new(cat_codigos_postales::id_estado)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_id_estado")
                            .to(cat_estados::Table, cat_estados::id)
                            .from(cat_codigos_postales::Table, cat_codigos_postales::id_estado),
                    )
                    .to_owned(),
            )
            .await?;
        // Tabla de localidades
        manager
            .create_table(
                Table::create()
                    .table(cat_localidades::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(cat_localidades::id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(cat_localidades::localidad)
                            .string_len(100)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(cat_localidades::codigo_postal)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_codigo_postal")
                            .to(
                                cat_codigos_postales::Table,
                                cat_codigos_postales::codigo_postal,
                            )
                            .from(cat_localidades::Table, cat_localidades::codigo_postal),
                    )
                    .col(
                        ColumnDef::new(cat_localidades::id_municipio)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_id_municipio")
                            .to(cat_municipios::Table, cat_municipios::id)
                            .from(cat_localidades::Table, cat_localidades::id_municipio),
                    )
                    .col(
                        ColumnDef::new(cat_localidades::id_estado)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_id_estado")
                            .to(cat_estados::Table, cat_estados::id)
                            .from(cat_localidades::Table, cat_localidades::id_estado),
                    )
                    .to_owned(),
            )
            .await
    }

    /// Elimina las tablas creadas por `up` en el orden inverso para respetar dependencias.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(cat_localidades::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(cat_codigos_postales::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(cat_municipios::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(cat_estados::Table).to_owned())
            .await
    }
}
