use axum::{
    http::Request,
    middleware::Next,
    response::Response,
    body::Body
};
use lazy_regex::regex_captures;
use time::macros::format_description;
use time::Date;
use tower_cookies::Cookies;
use uuid::Uuid;

use crate::modelo::login::ControladorLogin;
use crate::modelo::ControladorModelo;
use crate::web::AUTH_TOKEN;
use crate::{Error, Result};

pub async fn mw_requerir_auth(
    cookies: Cookies,
    req: Request<Body>, 
    next: Next
) -> Result<Response> {
    println!("->> {:<12} - mw_requerir_auth", "MIDDLEWARE");

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    let (idUsuario, fechaCaducidad, uuid) = auth_token
        .ok_or(Error::NoAuthTokenCookie)
        .and_then(parsear_token)?;

    // let login = ControladorLogin::login_uuid(cm, uuid).await?
    //     .ok_or(Error::ErrorLogin)?;

    Ok(next.run(req).await)
}

fn parsear_token(token: String) -> Result<(u32, Date, Uuid)> {
    let (_whole, idUsuario, fechaCaducidad, uuid) = regex_captures!(
        r#"^(\d+)\.(.+)\.(.+)"#, // a literal regex
        &token
    )
        .ok_or(Error::TokenFormatoIncorrecto)?;

    println!("id: {}, fechaCaducidad: {}, uuid: {}", idUsuario, fechaCaducidad, uuid);
    let idUsuario: u32 = idUsuario
        .parse()
        .map_err(|_| Error::TokenFormatoIncorrecto)?;
    println!("{}", idUsuario);

    let uuid: Uuid = Uuid::parse_str(uuid)?;
    println!("{}", uuid);

    let fechaCaducidad = Date::parse(fechaCaducidad, &format_description!("[year]-[month]-[day]"))
        .map_err(|_| Error::TokenFormatoIncorrecto)?;

    Ok((idUsuario, fechaCaducidad, uuid))
}
