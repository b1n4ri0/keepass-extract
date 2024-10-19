# KeePass Extractor

Este proyecto está diseñado para extraer datos de una base de datos KeePass (formato `.kdbx`) y exportarlos a un archivo CSV. Utiliza la biblioteca `keepass-ng` para interactuar con la base de datos de KeePass, y la librería `csv` para la exportación de los datos a un archivo CSV.

## Descripción

El extractor permite abrir una base de datos de KeePass, leer sus grupos y entradas, mostrar la información por consola y exportarla en un formato CSV para su uso posterior. Cada entrada exportada incluye el título, el nombre de usuario, la contraseña, la URL y las notas asociadas.

### Características:
- Abre bases de datos KeePass utilizando una contraseña maestra.
- Lee y muestra los grupos y entradas de la base de datos en la consola.
- Exporta las entradas a un archivo CSV.
- Utiliza la biblioteca `keepass-ng` para interactuar con el archivo `.kdbx`.

## Instalación
   ```bash
   git clone https://github.com/b1n4ri0/keepass-extract.git
   cd keepass-extract
   cargo build --release
   cd /target/release
```

## Uso
  ```bash
  ./keepass-extract --database database.kdbx --password '5uPerP4@@55!' --output out.csv 
