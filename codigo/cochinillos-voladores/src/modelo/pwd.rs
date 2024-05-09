use bcrypt::{DEFAULT_COST, hash, verify};
use super::{Error, Result};

pub fn hash_password(password: String) -> Result<String> {
    hash(password, DEFAULT_COST).map_err(|_| Error::ErrorPasswordHashing)
}

pub fn verificar_password(password: String, hash: &str) -> Result<bool> {
    verify(password, hash).map_err(|err| Error::ErrorVerificandoPassword{ error: err.to_string() })
}
