#![allow(non_camel_case_types)]
#[allow(unused)]
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum cat_estados {
    Table,
    id,
    estado,
}

#[derive(DeriveIden)]
pub enum cat_municipios {
    Table,
    id,
    municipio,
    id_estado,
}

#[derive(DeriveIden)]
pub enum cat_codigos_postales {
    Table,
    codigo_postal,
    id_municipio,
    id_estado,
}

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
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
