CREATE TABLE IF NOT EXISTS tusuarios (
    id MEDIUMINT UNSIGNED AUTO_INCREMENT,
    nombre VARCHAR(255) NOT NULL,
    mail VARCHAR(255) NOT NULL,
    esAdministrador BOOLEAN NOT NULL DEFAULT 0,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS tlogins (
    uuid CHAR(36),
    idUsuario MEDIUMINT UNSIGNED,
    fechaCaducidad DATE,
    PRIMARY KEY (uuid, idUsuario),
    CONSTRAINT FK_tusuarios_tlogins
        FOREIGN KEY (idUsuario)
        REFERENCES tusuarios(id)
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS tblog (
    id MEDIUMINT UNSIGNED AUTO_INCREMENT,
    titulo VARCHAR(255) NOT NULL,
    descripcion TEXT,
    fecha DATE NOT NULL,
    fotoURL TEXT,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS tcategorias (
    id SMALLINT UNSIGNED AUTO_INCREMENT,
    nombre VARCHAR(255) NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS tcompeticiones (
    id MEDIUMINT UNSIGNED AUTO_INCREMENT,
    nombre VARCHAR(255) NOT NULL,
    fechaInicio DATE,
    fechaFin DATE,
    idCategoria SMALLINT UNSIGNED NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT FK_tcategoria_tcompeticion
        FOREIGN KEY (idCategoria)
        REFERENCES tcategorias(id)
);

CREATE TABLE IF NOT EXISTS tequipos (
    id MEDIUMINT UNSIGNED AUTO_INCREMENT,
    nombre VARCHAR(255) NOT NULL,
    lugar VARCHAR(255) NOT NULL,
);

CREATE TABLE IF NOT EXISTS rcompeticionesequipos (
    idCompeticion MEDIUMINT UNSIGNED,
    idEquipo MEDIUMINT UNSIGNED
    PRIMARY KEY (idCompeticion, idEquipo),
    CONSTRAINT FK_tcompeticiones_r
        FOREIGN KEY (idCompeticion)
        REFERENCES tcompeticiones(id),
    CONSTRAINT FK_tequipos_r
        FOREIGN KEY (idEquipo)
        REFERENCES tequipos(id)
);

CREATE TABLE IF NOT EXISTS ltiposjugador (
    id TINYINT UNSIGNED,
    nombre VARCHAR(255) NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS tjugadores (
    id MEDIUMINT UNSIGNED,
    numero TINYINT UNSIGNED NOT NULL,
    nombre VARCHAR(255) NOT NULL,
    apellido1 VARCHAR(255) NOT NULL,
    apellido2 VARCHAR(255) NOT NULL,
    nacimiento DATE NOT NULL,
    fotoURL TEXT,
    idTipoJugador TINYINT UNSIGNED NOT NULL,
    idEquipo MEDIUMINT UNSIGNED NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT FK_ltiposjugador_tjugadores
        FOREIGN KEY (idtipoJugador)
        REFERENCES ltiposjugador(id),
    CONSTRAINT FK_tequipos_tjugadores
        FOREIGN KEY (idEquipo)
        REFERENCES tequipos(id)
);

CREATE TABLE IF NOT EXISTS tpartidos (
    id MEDIUMINT UNSIGNED,
    fecha DATE NOT NULL,
    lugar VARCHAR(255) NOT NULL,
    idCompeticion MEDIUMINT UNSIGNED NOT NULL,
    idEquipoCasa MEDIUMINT UNSIGNED NOT NULL,
    idEquipoVisitante MEDIUMINT UNSIGNED NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT FK_tcompeticiones_tpartidos
        FOREIGN KEY (idCompeticion)
        REFERENCES tcompeticiones(id),
    CONSTRAINT FK_tequipocasa_tpartidos
        FOREIGN KEY (idEquipoCasa)
        REFERENCES tequipos(id),
    CONSTRAINT FK_tequipovisitante_tpartidos
        FOREIGN KEY (idEquipoVisitante)
        REFERENCES tequipos(id)
);

CREATE TABLE IF NOT EXISTS ltiposevento (
    id TINYINT UNSIGNED,
    nombre VARCHAR(255) NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS teventos (
    id INT UNSIGNED,
    valor SMALLINT UNSIGNED,
    minuto TIME,
    idTipoEvento TINYINT UNSIGNED NOT NULL,
    idJugador MEDIUMINT UNSIGNED NOT NULL,
    idPartido MEDIUMINT UNSIGNED NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT FK_ltiposevento_teventos
        FOREIGN KEY (idTipoEvento)
        REFERENCES ltiposevento(id),
    CONSTRAINT FK_tjugadores_teventos
        FOREIGN KEY (idJugador)
        REFERENCES tjugadores(id),
    CONSTRAINT FK_tpartidos_teventos
        FOREIGN KEY (idPartido)
        REFERENCES tpartidos(id)
);
