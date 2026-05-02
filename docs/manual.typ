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
#import "@preview/rcsection:0.1.0": init_rcsection

// Initialize the plugin
#show: init_rcsection

// Create a figure with a beam section
#figure(
  ````rcs
  beam "V-101":
      shape rect 30 50     // Dimensions: width x height
      length 400           // Span length (for longitudinal view)
      concrete:
          cover 4          // Concrete cover

      // Reinforcement
      top 2 1"             // Top bars: count size
      bot 3 1"             // Bottom bars: count size
      ties 3/8" 1@15       // Stirrups: size spacing
  ````,
  caption: "Reinforced Concrete Beam Detail",
)
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

== Tipos de elemento

El tipo de elemento determina la orientación de la vista longitudinal por defecto:

#table(
  columns: (1fr, 3fr),
  [`beam`], [Viga — vista longitudinal horizontal],
  [`column`], [Columna — vista de elevación vertical],
  [`wall`], [Muro/placa — vista de elevación vertical],
  [`section`], [Genérico (compatibilidad) — vista longitudinal horizontal],
)

== Vistas

Las vistas soportadas son:

#table(
  columns: (1fr, 3fr),
  [`section`], [Solo vista de sección (corte transversal)],
  [`longitudinal`], [Solo vista longitudinal (para beams)],
  [`elevation`], [Solo vista de elevación (para columns/walls)],
  [`both`], [Ambas vistas: corte + longitudinal/elevación según el tipo],
)

Si no se especifica `view`, el tipo de elemento determina la vista secundaria:
- `beam` → genera vista longitudinal horizontal
- `column` / `wall` → genera vista de elevación vertical
- `section` → genera vista longitudinal horizontal

```rcs
beam "V-101":
    shape rect 30 60
    top 3 #6
    // Sin view: muestra sección + longitudinal

column "C-101":
    shape rect 40 40
    perim 8 #6
    // Sin view: muestra sección + elevación
```

== Propiedades globales
Se ubican al inicio del bloque y determina las propiedades que son aplicadas a todas las secciones definidas, si estas no son definidas, se toman los valores por defecto.

```
set:
  <propiedad global> <valor>
```

=== Escala de dibujo
Define la escala de dibujo. Puede aplicarse globalmente (todas las vistas) o por tipo de vista.

```
scale <valor>            // todas las vistas
scale <vista> <valor>    // solo una vista
```

Vistas disponibles:

#table(
  columns: (1fr, 3fr),
  [`section`], [Vista de sección transversal],
  [`longitudinal`], [Vista longitudinal (vigas)],
  [`elevation`], [Vista de elevación (columnas/muros)],
)

_Valor por defecto: `1:20`_

Esto es útil cuando una vista longitudinal o de elevación es muy alargada y desbordaría la página: se puede asignar una escala más pequeña solo a esa vista.

*Ejemplo:*
```rcs
set:
    unit "mm"
    scale section 1:10
    scale longitudinal 1:25

beam "V-101":
    shape rect 300 600   // 30 cm x 60 cm
    cover 40             // 4 cm
    top 3 #6
    length 6000
    longitudinal:
        bottom 4 #6
        stirrup #3 @150
```

=== Unidades
Define la unidad de longitud para todas las dimensiones de la sección. El motor interno trabaja en centímetros, por lo que los valores son convertidos automáticamente.

```
unit <unidad>
```

Unidades soportadas:

#table(
  columns: (1fr, 3fr),
  [`cm`], [Centímetros \ _valor por defecto_],
  [`mm`], [Milímetros],
  [`m`], [Metros],
  [`in`], [Pulgadas],
  [`ft`], [Pies],
)

*Ejemplo:*
```rcs
set:
    unit "mm"
    scale 1:50

beam "V-101":
    shape rect 300 600   // 30 cm x 60 cm
    cover 40             // 4 cm
    top 3 #6
```

=== Estilo de dibujo
Permite escoger un preset visual para controlar jerarquía de líneas, rellenos y apariencia general.

```
style <preset>
```

Presets soportados:

#table(
  columns: (1fr, 3fr),
  [`default`], [Estilo actual con colores por diámetro y presentación expresiva],
  [`spd`], [Estilo técnico monocromático inspirado en documentación estructural tipo SPD/ISO: líneas negras, jerarquía sobria y aceros sin relleno],
)

`style "spd"` activa una salida más profesional para memorias y planos:
- concreto en negro con contorno principal más fuerte,
- estribos como anillo hueco con doble contorno,
- aceros longitudinales en negro sin relleno,
- linework más sobrio para impresión.

*Ejemplo:*
```rcs
set:
    style "spd"
    scale 1:25

section "V-SPD":
    shape rect 30 50
    concrete:
        cover 4
    top 2 #8 1 #6
    bot 3 #8
    ties #3 1@5 4@10 rto@20
    view both
```

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
  [`length valor`], [Longitud del tramo (para vista longitudinal) \ _ejemplo: `length 200`_],
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

== Ejemplos por tipo de elemento

#show: init_rcsection

=== Vigas

#example("../examples/minimal.rcs", caption: "Viga peraltada (sección + longitudinal)")

#example("../examples/beam_section.rcs", caption: "Viga — solo sección")

#example("../examples/beam_longitudinal.rcs", caption: "Viga — solo longitudinal", wide: true)

=== Columnas

#example("../examples/columna.rcs", caption: "Columna rectangular y circular (sección + elevación)")

#example("../examples/column_section.rcs", caption: "Columna — solo sección")

#example("../examples/column_elevation.rcs", caption: "Columna — solo elevación", wide: true)

=== Otros ejemplos

#example("../examples/circular.rcs", caption: "Columna circular")

#example("../examples/longitudinal.rcs", caption: "Viga con longitud definida", wide: true)

#example("../examples/spd.rcs", caption: "Preset técnico profesional SPD", wide: true)

= Propuestas de evolución del lenguaje

Las siguientes características están propuestas para futuras versiones. Su sintaxis puede cambiar.

== Templates (reusabilidad)

Permite definir plantillas reutilizables para evitar repetición en proyectos con muchas secciones idénticas:

```rcs-future
set:
    unit "cm"

template "viga-std":
    shape rect 30 50
    cover 4
    ties #3 rto@15

section "V-101" from "viga-std":
    top 3 #6
    bot 2 #8

section "V-102" from "viga-std":
    top 2 #8
    bot 3 #8
```

== Control de capas (z-order)

Controla el orden de superposición de los elementos gráficos:

```rcs-future
section "V-101":
    shape rect 30 50
    layer "concrete" z 0
    layer "stirrup" z 10
    layer "rebar" z 20
    top 3 #6
```

== Control de callouts

Permite personalizar la posición y orientación de las flechas de acero:

```rcs-future
section "V-101":
    shape rect 30 50
    top 3 #6:
        callout right offset 12
    bot 2 #8:
        callout left offset 10
```

== Vistas nombradas y escalas múltiples

Genera distintas vistas con escalas independientes en la misma figura:

```rcs-future
section "V-101":
    shape rect 30 50
    view "section" scale 1:20
    view "detail" scale 1:5 region (10,10,20,30)
    top 3 #6
    bot 2 #8
```

== Secciones compuestas (T, L, I)

Define secciones compuestas a partir de formas simples:

```rcs-future
section "VT-101":
    shape t-beam:
        web rect 30 50
        flange rect 60 10 offset (0, 25)
    top 3 #6
    bot 2 #8
```
