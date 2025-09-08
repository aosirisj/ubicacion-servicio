use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Entradas {
    pub id: i32,
    pub value: String,
}

// Payload para buscar CP
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CPPayload {
    pub cp: i32,
}

// Response para buscar CP
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CPResponse {
    pub estado: Entradas,
    pub municipio: Entradas,
    pub localidades: Vec<Entradas>,
}

