use utoipa::{
    openapi::{
        security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
        Components,
    },
    Modify, OpenApi,
};

use crate::{routes, types};

// Documentación de la API
#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "Localidades API", description = "Mini servicio de ubicación")
    ),
    paths(
        routes::catalogos::busqueda_cp,
        ),
    components(
        schemas(
            types::catalogos::CPPayload,
            types::catalogos::CPResponse,
        )
    ),
    modifiers(&SecurityAddOn)
)]
pub struct ApiDoc;

// Implementación de los modificadores de seguridad
struct SecurityAddOn;
impl Modify for SecurityAddOn {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components: &mut Components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}
