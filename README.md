# RCSection — Gráficos para secciones de concreto armado en [Typst](https://typst.app)

RCSection es un plugin para [Typst](https://typst.app) que permite crear gráficos de secciones de concreto armado: vigas, columnas, muros y otros elementos estructurales.

## Instalación

Importa el paquete desde el namespace de preview (una vez publicado) o desde un archivo local:

```typ
#import "@preview/rcsection:0.1.0": init_rcsection
```

O si lo usas localmente:

```typ
#import "src/rcsection.typ": init_rcsection
```

## Uso

Para usar `rcsection`, inicialízalo con una regla `show`. Luego escribe las definiciones de los elementos en bloques de código `rcs`.

```typ
#import "@preview/rcsection:0.1.0": init_rcsection

// Inicializar el plugin
#show: init_rcsection

// Crear una figura con una viga
#figure(
  ```rcs
  beam "V-101":
      shape rect 30 50     // Dimensiones: ancho x alto
      length 400           // Luz (para vista longitudinal)
      concrete:
          cover 4          // Recubrimiento

      // Refuerzo longitudinal
      top 2 1"             // Barras superiores: cantidad tamaño
      bot 3 1"             // Barras inferiores: cantidad tamaño
      ties 3/8" 1@15       // Estribos: tamaño espaciamiento
  ```,
  caption: "Detalle de viga de concreto armado",
)
```

## Características

- **Vigas y columnas**: Define secciones rectangulares o circulares con refuerzo.
- **Vistas**: Genera cortes transversales, vistas longitudinales, vistas de elevación o ambas.
- **Escalas**: Controla la escala del dibujo globalmente o por tipo de vista (ej. `scale section 1:10`, `scale longitudinal 1:25`).
- **Personalización**: Configura recubrimiento, espaciamiento de estribos, tamaños de barra, estilos visuales y unidades.
- **Estilos**: Modo monocromático (SPD) con líneas negras y jerarquía técnica, o modo por defecto con colores por diámetro.

## Tipos de elemento

| Tipo | Descripción |
|------|-------------|
| `beam` | Viga — vista longitudinal horizontal |
| `column` | Columna — vista de elevación vertical |
| `wall` | Muro/placa — vista de elevación vertical |
| `section` | Genérico (compatibilidad) — vista longitudinal horizontal |

## Documentación

Consulta el [manual completo](docs/manual.pdf) para más detalles sobre la sintaxis, propiedades y ejemplos.

## Licencia

MIT
