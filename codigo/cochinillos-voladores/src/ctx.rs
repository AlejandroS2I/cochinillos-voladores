use crate::modelo::usuario::Usuario;


#[derive(Clone, Debug)]
pub struct Ctx {
    usuario: Usuario
}

impl Ctx {
    pub fn new(usuario: Usuario) -> Self {
        Self { usuario }
    }
}

impl Ctx {
    pub fn usuario(&self) -> Usuario {
        self.usuario.clone()
    }
}
