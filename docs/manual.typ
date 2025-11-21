#import "../src/rcsection.typ": draw, init_rcsection, parse
#let package-meta = toml("/typst.toml").package
#let pkg-authors = package-meta.authors.first().split(" ")
#let removed = pkg-authors.remove(-1)
#let author = pkg-authors.join(" ")
#let date = datetime.today()

#set document(
  title: "Manual de RCSections",
  author: "Paolo Guillen Lupo",
  date: date,
)

#set page(
  margin: (top: 0.75in, rest: 0.5in),
)

#set text(
  size: 15pt,
  lang: "es",
)

#set heading(numbering: "1.")

#set par(leading: 0.5em, justify: true)

#set table(
  stroke: (_, y) => if y != -1 { (bottom: 0.5pt) },
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

#let example(path, caption: none, wide: false) = {
  figure(
    kind: "example",
    supplement: "Ejemplo",
    caption: caption,
    raw(
      block: true,
      lang: if wide { "rcs-example-wide" } else { "rcs-example" },
      read(path).trim("\n"),
    ),
  )
}

#show figure.where(kind: "example"): it => block(
  breakable: false,
  {
    set par(justify: false, spacing: 0.6em)
    set text(hyphenate: auto, overhang: false)
    {
      set align(left)
      if it.caption in (none, [], "") {
        strong[#it.supplement #it.counter.display()]
      } else [
        #strong[#it.supplement #it.counter.display(): ] #it.caption.body
      ]
    }
    box(
      stroke: 1pt,
      it.body,
    )
  },
)

#show raw.where(block: true): it => {
  if it.lang == none or not it.lang.starts-with("rcs") {
    block(
      width: 100%,
      fill: luma(85%),
      inset: 0.5em,
      stroke: 1pt,
      it.text,
    )
  } else if it.lang.starts-with("rcs-example") {
    let dir = ltr
    let inset = 0pt
    let width = 50%

    if it.lang.ends-with("-wide") {
      dir = ttb
      inset = (y: 1em)
      width = 100%
    }

    align(center, stack(
      dir: dir,
      {
        set text(size: 1em)
        block(
          width: width,
          inset: 1em,
          align(left, "```rcs\n" + it.text + "\n```"),
        )
      },

      align(horizon, block(inset: inset, width: width, raw(block: true, lang: "rcs", it.text))),
    ))
  } else {
    it
  }
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
#show: init_rcsection
```

= Sintaxis
Para representar un elemento estructural se define un encabezado seguido de dos puntos (`:`)
y un bloque indentado con las propiedades:

```
<tipo> <identificador>:
  <propiedad> <valor>
  <propiedad> <valor>
```

Podemos separar la sintaxis en dos partes:
- La primera parte es la definición de las propiedades globales que se aplican a todas las secciones.
- La segunda parte es la definición de cada sección.

== Vista
Las vistas soportadas son:

#table(
  columns: (1fr, 3fr),
  [`section`], [Vista de sección (corte)],
  [`long`], [Vista longitudinal],
)

== Propiedades globales
Se ubican al inicio del bloque y determina las propiedades que son aplicadas a todas las secciones definidas, si estas no son definidas, se toman los valores por defecto.

```
set:
  <propiedad global> <valor>
```

=== Escala de dibujo
Define la escala de dibujo para los tipos de vista. Se representa como una relación.

```
scale <vista> <valor>
```
_Valor por defecto: `1:20`_

=== Dimensiones
Habilita la gráfica de las cotas para las dimensiones de la sección.

```
dims <on | off>
```
_Valor por defecto: `off`_

=== Etiquetas
Habilita la gráfica de las etiquetas para los aceros de refuerzo.

```
labels <mode>
```

Los modos soportados son:

#table(
  columns: (1fr, 3fr),
  [`off`], [Deshabilita las etiquetas \ _valor por defecto_],
  [`callout`], [Cantidad y tamaño de acero con flechas],
  [`legend`], [Una leyenda debajo del gráfico con cantidad y tamaño de acero],
  [`both`], [Modos `callout` y `legend` activados],
)

== Tipos de secciones
Para la definición del tipo de sección, son soportados:

#table(
  columns: (1fr, 3fr),
  [`beam`], [Define una viga],
  [`column`], [Define una columna],
  [`wall`], [Define un muro/placa],
)

== Identificador
Se refiere al nombre único que se le asigna a cada sección. Ejm: `"V-101"`

== Propiedades geométricas
Para la definición de la geometría de una sección (por default en cm), se tiene:

#table(
  columns: (1fr, 3fr),
  [`ancho x alto`], [Define una sección Rectangular \ _ejemplo: `30 x 60`_],
  [`R ancho alto`], [Define una sección Rectangular \ _ejemplo: `R 30 60`_],
  [`D diámetro`], [Define una sección Circular \ _ejemplo: `D 50`_],
  [`T ancho_total alto_total espesor_ala espesor_alma `], [Define una sección en T \ _ejemplo: `T 60 60 20 30`_],
  [`L ancho_total alto_total espesor_ala espesor_alma `], [Define una sección en L \ _ejemplo: `L 50 50 15 25`_],
  [`cover valor`], [Valor del recubrimiento \ _ejemplo: `cover 2`_],
)

== Propiedades para el acero longitudinal
Para la ubicación de los aceros longitudinales, el lenguaje toma en cuenta el orden en las que
se declaren.

```
<zona> <cantidad> <tamaño> <cantidad> <tamaño> ...
```

=== Zonas
Las valores para definir zonas son:

#table(
  columns: (1fr, 3fr),
  [`top`], [Acero superior \ _Ejemplo: `top 1 3/4" 1 1/2" 1 3/4"`_],
  [`bot`], [Acero inferior \ _Ejemplo: `bot 2 #4`_],
  [`mid`], [Acero en la zona media \ _Ejemplo: `mid 2 3/4"`_],
  [`sides`], [Acero en los lados izquierdo y derecho \ _Ejemplo: `sides 2 #5`_],
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

#show: init_rcsection

#example("../examples/minimal.rcs", caption: "Viga peraltada")

#example("../examples/columna.rcs", caption: "Columna")

#example("../examples/circular.rcs", caption: "Muro")
