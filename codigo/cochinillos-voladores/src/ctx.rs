
#[derive(Clone, Debug)]
pub struct Ctx {
    idUsuario: u32,
    esAdministrador: bool
}

impl Ctx {
    pub fn new(idUsuario: u32, esAdministrador: bool) -> Self {
        Self { idUsuario, esAdministrador }
    }
}

impl Ctx {
    pub fn idUsuario(&self) -> u32 {
        self.idUsuario
    }

    pub fn esAdministrador(&self) -> bool {
        self.esAdministrador
    }
}
