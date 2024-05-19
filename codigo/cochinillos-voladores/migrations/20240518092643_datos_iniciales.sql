-- Datos iniciales

-- Equipos
INSERT INTO tequipos (id, nombre, lugar) VALUES (1, "Cochinillos voladores", "Segovia");

-- Tipos jugador
INSERT INTO ltiposjugador (id, nombre) VALUES (1, "Jugador"), (2, "Portero");

-- Tipos evento
INSERT INTO ltiposevento (id, nombre) VALUES (1, "Partido jugado"), (2, "Gol"), 
    (3, "Tiros recibidos"), (4, "Asistencia"), (5, "Falta");
