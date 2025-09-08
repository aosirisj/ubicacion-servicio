pub fn validar_token(token: String) -> Result<(), jsonwebtoken::errors::Error> {
    if token.trim().is_empty() {
        return Err(jsonwebtoken::errors::Error::from(
            jsonwebtoken::errors::ErrorKind::InvalidToken,
        ));
    }

    Ok(())
}
