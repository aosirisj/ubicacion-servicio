use actix_web::{get, web, HttpResponse};
use sea_orm::DatabaseConnection;
use utoipa;

use crate::{controllers::catalogos::*, types::catalogos::*};

// Ruta para buscar estado, municipio y localidades por código postal
#[utoipa::path(
    path = "/api/busqueda-cp/{cp}",
    params(
        ("cp" = i32, Path, description = "Ruta para buscar estado, municipio y localidades por código postal", example = 14390),
    ),
    responses(
        (status = 200, description = "Se validó el CP y se encontraron datos vinculados a éste", body = CPResponse),
        (status = 400, description = "Error en la petición, formato incorrecto del CP", body = String, example = "Formato de código postal inválido"),
        (status = 404, description = "No se encontró el CP introducida", body = String, example = "Código postal no encontrado"),
        (status = 500, description = "Error interno del servidor", body = String, example = "Error en la base de datos")
    ),
    security(("bearer_auth"=[]))
)]
#[get("/busqueda-cp/{cp}")]
async fn busqueda_cp(
    db: web::Data<DatabaseConnection>,
    path_params: web::Path<CPPayload>,
) -> HttpResponse {
    match busqueda_cp_controller(db, path_params.cp).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) if e.to_string().contains("inválido") => {
            HttpResponse::BadRequest().body(e.to_string())
        }
        Err(e) if e.to_string().contains("no encontrado") => {
            HttpResponse::NotFound().body(e.to_string())
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
