//! # Estructuras de catálogos para intercambio FE-BE
//!
//! Estructuras de datos utilizadas para el intercambio de información (con el frontend)
//!
//! Este módulo define los tipos usados para:
//! - Solicitar datos (estado y municipio) y catálogos (localidades) con CP.
//! - Representar la respuesta devuelta por los endpoints de ubicación.
//!
//! Los DTOs de la BD se encuentran en src/entities y son generados automaticamente por Sea ORM.
use crate::utils::conversores::CatalogoIdCadena;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Payload usado para solicitar información de un código postal específico.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CPPayload {
    /// Código postal a consultar
    pub cp: i32,
}

/// Respuesta que contiene los datos asociados a un código postal,
/// incluyendo estado, municipio y localidades correspondientes.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CPResponse {
    pub estado: CatalogoIdCadena,
    pub municipio: CatalogoIdCadena,
    pub localidades: Vec<CatalogoIdCadena>,
}

