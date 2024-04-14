# Segunda versión

La aplicación web contará con un inicio en el que se mostrará información
sobre el equipo, un apartado de estadísticas de este, un formulario
en el que se puedan apuntar los visitantes de la página al equipo y
un blog donde publicar noticias.

## Pantallas

- [Inicio](#inicio)
  - [Portada](#portada)
  - [Sobre nosotros](#sobre-nosotros)
  - [Dónde estamos?](#donde-estamos)
  - [Noticias](#noticias)
  - [Conócenos](#conocenos)
- [Estadísticas](#estadisticas)
  - [Resúmen](#resumen)
  - [Partidos](#partidos)
    - [Listado](#listadoPartidos)
    - [Detalles](#detallesPartidos)
  - [Plantilla](#plantilla)
    - [Listado](#listadoPlantilla)
    - [Detalles](#detallesDetalles)
- [Formulario](#formulario)
- [Blog](#blog)

## Inicio

Esta será la sección inicial de la aplicación, donde mostraremos toda
la información que podamos a los visitantes de forma resumida.

### Portada

Se mostrará el nombre y logotipo del equipo además de una sencilla pero
atractiva animación de inicio.

### Sobre nosotros

Se hablará sobre el equipo en un texto resumido.

### Dónde estamos?

Se mostrará un mapa fijo con un punto de interés en el centro señalando
el polideportivo en el que se llevan a cabo los entrenamientos y si
se hace click en este se redirigirá a su localización en Google Maps.

### Noticias

Se podrá iterar por las últimas noticias en forma de periódicos uno encima
de otro, mostrando el título de la noticia, la descripción, la fecha
de publicación y la imagen relacionada si la tiene. En caso de hacer
click en este periódico se redirigirá a la noticia en el apartado de
[Blog](#blog)

### Conócenos

Se mostrarán los trabajadores y organizadores del equipo en unas tarjetas
que se deslizarán horizontalmente. Estas tarjetas contarán con una
fotografía del trabajador, su nombre, títulos relacionados y posiblemente
un teléfono de contacto.

## Estadísticas

Esta será la sección de la aplicacion en la que se podrá observar el
rendimiento del equipo de forma conjunta e individual.

### Resúmen

En este resúmen se mostrarán los datos más importantes sobre el equipo,
siendo estos, el logo, el nombre, partidos ganados, partidos perdidos,
partidos empatados, jugadores destacados, listado de últimos partidos
,con la categoría y el resultado, y listado de categorías con sus
posiciones.

### Partidos

- **Listado.**<a name="listadoPartidos"></a> En este se mostrará un listado de todos los partidos en
  los que ha participado el equipo y datos importantes de estos: fecha y hora, lugar,
  equipos enfrentados y resultado. Estos estarán filtrados por la categoría seleccionada en
  la pantalla de resúmen.
- **Detalles.**<a name="detallesPartidos"></a> En este se mostrarán los detalles del partido seleccionado,
  fecha y hora, lugar, equipos enfrentados, resultado, listado de eventos,
  con tipo, jugador y minuto, y listado de jugadores, con su número, apellidos, nombre y estadísticas.

### Plantilla

- **Listado.**<a name="listadoPlantilla"></a> En este se mostrará un listado de todos los jugadores del
    equipo con su imagen, número, apellidos, nombre y estadísticas.
- **Detalles.**<a name="detallesPlantilla"></a> En este se mostrará la imagen del jugador, su número
    sus apellidos, su nombre, y sus estadísticas dependiendo de si es jugador o portero.

Posibles estadísticas:

- General
  - Partidos jugados
- Jugadores:
  - Goles
  - Asistencias
  - Faltas
  - Puntos (Goles + Asistencias)
  - Puntos por partido
  - Minutos de sanción (Sumario faltas)
  - Goles por partido
  - Asistencias por partido
  - Totales por temporada
  - Totales histórico
- Porteros:
  - Goles en contra
  - Tiros recibidos
  - Porcentaje de parada

## Formulario

Este será un simple formulario para poder inscribirse al equipo, los
campos pedidos serán:

- Nombre
- Apellidos
- Edad
- Fotografía
- Email
- Teléfono

## Blog

El blog consistirá en una plantilla en la que se listen las entradas,
estas podrán ser gestionadas por los administradores.
