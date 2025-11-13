use actix_web::{dev::ServiceRequest, error, Error};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::utils::jwt::validar_token;

// Middleware para validar el token JWT
// Sólo valida la ecuación del token, pero no valida el contenido
pub async fn validador_jwt(
    req: ServiceRequest,
    auth: Option<BearerAuth>,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let Some(auth) = auth else {
        return Err((error::ErrorBadRequest("Token is required"), req));
    };

    let token = auth.token();
    let claims = validar_token(token.to_string());

    match claims {
        Ok(()) => Ok(req),
        Err(_e) => Err((error::ErrorForbidden("Invalid token"), req)),
    }
}
