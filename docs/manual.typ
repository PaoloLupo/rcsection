#import "../src/rcsection.typ": init_rcsection
#let package-meta = toml("/typst.toml").package
#let pkg-authors = package-meta.authors.first().split(" ")
#let removed = pkg-authors.remove(-1)
#let author = pkg-authors.join(" ")
#let date = datetime.today()
#show: init_rcsection

#set document(
  title: "Manual de RCSections",
  author: "Paolo Guillen Lupo",
  date: date
)

#set page(
  margin: (top: 0.75in, rest: 0.5in)
)

#set text(
  size: 15pt,
  lang: "es",
)

#set heading(numbering: "1.")

#set par(leading: 0.5em, justify: true)

#set table(
  stroke: (_, y) => if y != -1 {(bottom: 0.5pt)},
)

#show raw.where(block: false): it => {
  box(
    fill: luma(80%),
    inset: (x: 0.1em),
    outset: (y: 0.3em),
    radius: 0.3em,
    it.text,
  )
}

#{
  v(1fr)
  strong[
    #set align(center)
    #set text(size: 2em)
    RCSections: Gráficos \ para secciones de concreto armado \ en Typst
  ]
  [
    #set align(center)
    #set text(size: 1.5em)
    Version #package-meta.version \
    #author \
    #date.display("[year]")
  ]
  v(1fr)
  pagebreak(weak: true)
}

#outline()
#pagebreak()

= Introducción
RCSections es un pequeño lenguaje para Typst que permite representar secciones de concreto
armado en Typst.

= Uso
+ Es necesario la instalación de Typst con la versión 0.14 o superior.
+ Agregar el siguiente código al inicio de tu archivo `.typ`:

```typ
#import "@preview/rcsection:0.1.0"
#show: init_rcsections
```

Para representar un elemento estructural se define un encabezado seguido de dos puntos (`:`)
y un bloque indentado con las propiedades:

```
<propiedad global>
<tipo> <identificador>:
  <propiedad> <valor>
  <propiedad> <valor>
```

Podemos separar la sintaxis en dos partes:
- La primera parte es la definición de las propiedades globales que se aplican a todas las secciones.
- La segunda parte es la definición de cada sección.

= Sintaxis
== Tipos de secciones
Para la definición del tipo de sección, son soportados:

#table(
  columns: (1fr,3fr),
  [`beam`], [Define una viga],
  [`column`], [Define una columna],
  [`wall`], [Define un muro/placa],
)

== Identificador
Se refiere al nombre único que se le asigna a cada sección. Ejm: `V-101`

== Propiedades geométricas
Para la definición de la geometría de una sección, se tiene:

#table(
  columns: (1fr,3fr),
  [`ancho x alto`], [Define una sección Rectangular \ _ejemplo: `30 x 60`_],
  [`R ancho alto`], [Define una sección Rectangular \ _ejemplo: `R 30 60`_],
  [`D diámetro`], [Define una sección Circular \ _ejemplo: `D 50`_],
  [`cover valor`], [Valor del recubrimiento \ _ejemplo: `cover 2`_],
)

== Propiedades para el acero longitudinal
Para la ubicación de los aceros longitudinales, el lenguaje toma en cuenta el orden en las que
se declaren.

=== Sintaxis
```
<zona> <cantidad> <tamaño>
```

=== Zonas
Las valores para definir zonas son:

#table(
  columns: (1fr, 3fr),
  [`top`], [Acero superior \ _Ejemplo: `top 2 1"`_],
  [`bot`], [Acero inferior \ _Ejemplo: `bot 2 #4`_],
  [`sides`], [Acero en las caras laterales \ _Ejemplo: `sides 2 3/4"`_],
  [`perim`], [Distribución perimetral equitativa (Para columnas) \ _Ejemplo: `perim 7 1"`_],
)

=== Cantidad
Número de aceros

=== Tamaño
Soporta la notación por pulgadas y estándar.

_Ejemplo: `#3`, `1/2"`_

== Propiedades para el acero transversal
Define el confinamiento de la sección y su espaciamiento.

=== Sintaxis
```
ties <tamaño> <distribución>
```

=== Tamaño
Soporta la notación por pulgadas y estándar.

_Ejemplo: `#3`, `1/2"`_

=== Distribución
Es una secuencia separada por espacios
```
<cantidad>@<espaciado> <rto>@<espaciado>
```

_Ejemplo: `1@5 4@10 rto@20`_

== Ejemplos

=== Ejemplo 1: Viga peraltada
```
beam "V-101":
  30 x 60
  cover 4
  top 2 1/2"
  bot 2 1/2"
  bot 3 3/4"
  ties 3/8" 1@5 5@10 rto@25
```

```rcs
beam "V-101":
  30 x 40
  cover 4
  top 2 1/2"
  bot 2 1/2"
  bot 3 1"
  ties 3/8" 1@5 5@10 rto@25
```
