mod config;
mod controllers;
mod middleware;
mod routes;
mod entities;
mod services;
mod types;
mod utils;
use crate::services::catalogos_ubicacion::{
    llenar_catalogos_cps, llenar_catalogos_estados_municipios, llenar_catalogos_localidades,
};

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use actix_web_httpauth::extractors::bearer::Config as BearerConfig;
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenv::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use std::env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Carga variables de entorno desde .env
    dotenv().ok();
    let ip = env::var("IP").expect("Variable IP debe ser fijada");
    let port: u16 = env::var("PORT")
        .expect("Variable PORT debe ser fijada")
        .parse()
        .expect("Variable PORT debe ser de tipo u16");

    // Crea pool de conexiones a la nueva base de datos de acopio
    let database_url = env::var("DATABASE_URL").expect("Variable DATABASE_URL debe ser fijada");
    let db = Database::connect(&database_url)
        .await
        .expect("Error al conectarse a la base de datos");
    Migrator::up(&db, None)
        .await
        .expect("Error al correr las migraciones");

    //Poblar catálogos
    llenar_catalogos_estados_municipios(&db, "./catalogos")
        .await
        .expect("Error al llenar los catalogos de estados o municipios");
    llenar_catalogos_cps(&db, "./catalogos")
        .await
        .expect("Error al llenar los catalogos de código postal");
    llenar_catalogos_localidades(&db, "./catalogos")
        .await
        .expect("Error al llenar los catalogos de localidades");

    // Inicializa Swagger
    let openapi = config::swagger::ApiDoc::openapi();

    // Inicializa el servidor HTTP
    HttpServer::new(move || {
        let auth = HttpAuthentication::with_fn(middleware::jwt::validador_jwt);

        App::new()
            .wrap(config::cors::cors_config()) // CORS
            .wrap(Logger::default()) // Logging
            .app_data(BearerConfig::default().realm("Area privada")) // Configuración de Extractor
            .app_data(web::Data::new(db.clone())) // Pool de conexiones a la bd
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            ) // UI de Swagger
            // Servicios privados
            .service(
                web::scope("/api")
                    .wrap(auth)
                    .service(routes::catalogos::busqueda_cp)
            )
    })
    .bind((ip, port))?
    .run()
    .await
}
