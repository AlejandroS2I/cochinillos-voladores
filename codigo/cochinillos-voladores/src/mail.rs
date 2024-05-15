use dotenvy::dotenv;
use lettre::{address::AddressError, message::{header::{self, ContentType}, Attachment, Body, Mailbox, Mailboxes, MessageBuilder, MultiPart, SinglePart}, transport::smtp::{authentication::Credentials, AsyncSmtpTransportBuilder}, AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use tokio::fs;

use crate::{Error, Result};

pub trait ParseadorMultiplesDestinatarios {
    fn to_destinatarios(self, destinatarios: Vec<String>) -> std::result::Result<MessageBuilder, AddressError>;
}

impl ParseadorMultiplesDestinatarios for MessageBuilder {
    fn to_destinatarios(mut self, destinatarios: Vec<String>) -> std::result::Result<MessageBuilder, AddressError> {
        for destinatario in destinatarios {
            self = self.to(destinatario.parse()?);
        }
        Ok(self)
    }
}

pub async fn enviar_mail(
    destinatarios: Vec<String>,
    asunto: String,
    cuerpo: String,
    foto: Option<(String, Vec<u8>)>
) -> Result<()> {
    dotenv().ok();

    let mail = std::env::var("MAIL").expect("ERROR: Mail no especificado");
    let usuario_mail = std::env::var("USUARIO_MAIL").expect("ERROR: Usuario mail no especificado");
    let contra_mail = std::env::var("CONTRA_MAIL").expect("ERROR: Contra mail no especificada");

    let mail = match foto {
        Some((nombre_imagen, imagen)) => {
            Message::builder()
                .to_destinatarios(destinatarios)
                    .map_err(|_|Error::Generico { error: "Error parseando destinatarios".to_string() })?
                .from(
                    mail.parse()
                        .map_err(|_|Error::Generico { error: "Error parseando mail sender".to_string() })?
                )
                .subject(asunto)
                .multipart(
                    MultiPart::mixed()
                        .singlepart(
                            SinglePart::builder()
                                .header(header::ContentType::TEXT_HTML)
                                .body(cuerpo)
                        )
                        .singlepart(
                            Attachment::new(nombre_imagen.clone()).body(
                                Body::new(imagen), 
                                ContentType::parse(
                                    format!(
                                        "image/{}", 
                                        nombre_imagen.split(".").last().ok_or(Error::Generico { error: "No tipo de archivo".to_string() })?
                                    ).as_str()
                                ).unwrap()
                            )
                        )
                )?
        },
        None => {
            Message::builder()
                .to_destinatarios(destinatarios)
                    .map_err(|_|Error::Generico { error: "Error parseando destinatarios".to_string() })?
                .from(
                    mail.parse()
                        .map_err(|_|Error::Generico { error: "Error parseando mail sender".to_string() })?
                )
                .subject(asunto)
                .body(cuerpo)?
        }
    };
    let mailer: AsyncSmtpTransport<Tokio1Executor> = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.porkbun.com")
        .map_err(|err|Error::Generico { error: format!("Error obteniendo smtp: {}", err) })?
        .credentials(Credentials::new(usuario_mail.to_string(), contra_mail.to_string()))
        .build();

    let resultado = mailer.send(mail).await;
    println!("{:?}", resultado);

    Ok(())
}
