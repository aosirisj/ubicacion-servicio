use actix_cors::Cors;

pub fn cors_config() -> Cors {
    let frontend_url =
        std::env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());

    // Configuración de CORS
    Cors::default()
        .allowed_origin(&frontend_url)
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![
            actix_web::http::header::AUTHORIZATION,
            actix_web::http::header::CONTENT_TYPE,
        ])
        .supports_credentials()
}
