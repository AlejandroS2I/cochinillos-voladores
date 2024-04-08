# Base de datos
Se ha desarrollado una base de datos teniendo en cuenta los requisitos de la aplicación con
     el primer funcional desarrollado después de hablar con el cliente. Esto incluye un [Modelo E/R](#modelo-er)
     para poder determinar las relaciones entre todas las entidades, y un [Modelo relacional](#modelo-relacional)
     en el que se determina el diseño final de la base de datos traduciendo el modelo E/R a tablas y
     añadiendo los campos a estas.

## Modelos

### [Modelo E/R](./design/DiagramaERcochinillos.drawio)
Este modelo se ha desarrollado para observar las relaciones de manera sencilla entre todas las entidades.
     Empezando con las más sencillas, la entidad **"Entrada"** no tiene relación ninguna ya que esta solo representa
     las noticias que aparecerán en el blog de la aplicación, así mismo, la entidad **"Usuario"** tampoco tiene relaciones
     ya que solo se utilizará para que los visitantes de la web se subscriban a un servicio de notificación mediante
     correos electrónicos de las noticias y futuros eventos.

<p align="center" width="100%">
    <img src="../img/DiagramaERcochinillos.png" alt="Modelo E/R" title="Modelo E/R" />
</p>

### [Modelo relacional](https://dbdesigner.page.link/EctTiUCBiYPVZTbQA)
Este es el [modelo inicial](https://dbdesigner.page.link/EctTiUCBiYPVZTbQA) de la base de datos:

<p align="center" width="100%">
    <img src="../img/cochinillosvoladores-modeloBD.png" alt="Modelo relacional" title="Modelo relacional" />
</p>
