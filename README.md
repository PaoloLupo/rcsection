# RCSection - A plugin for drawing reinforced concrete sections in [Typst](https://typst.app)

RCSection is a plugin for [Typst](https://typst.app) that allows you to create sections for reinforced concrete beams, columns, and slabs.

## Installation

Import the package from the preview namespace (once published) or local file:

```typ
#import "@preview/rcsection:0.1.0": init_rcsection
```

Or if you are using it locally:

```typ
#import "src/rcsection.typ": init_rcsection
```

## Usage

To use `rcsection`, you need to initialize it with a show rule. Then you can write your section definitions in `rcs` code blocks.

```typ
#import "@preview/rcsection:0.1.0": init_rcsection

// Initialize the plugin
#show: init_rcsection

// Create a figure with a beam section
#figure(
  ```rcs
  beam "V-101":
      shape rect 30 50     // Dimensions: width x height
      length 400           // Span length (for longitudinal view)
      concrete:
          cover 4          // Concrete cover

      // Reinforcement
      top 2 1"             // Top bars: count size
      bot 3 1"             // Bottom bars: count size
      ties 3/8" 1@15       // Stirrups: size spacing
  ```,
  caption: "Reinforced Concrete Beam Detail",
)
```

## Features

- **Beams & Columns**: Define rectangular or circular sections with reinforcement.
- **Views**: Generate cross-sections, longitudinal views, or both.
- **Scaling**: Control the scale of the drawing (e.g., `1:50`, `1:20`).
- **Customization**: Configure concrete cover, stirrup spacing, bar sizes, and colors.
