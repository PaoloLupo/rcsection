# RCSections - A plugin for drawing reinforced concrete sections in [Typst](https://typst.app)

RCSections is a plugin for [Typst](https://typst.app) that allows you to create sections for reinforced concrete beams, columns, and slabs.

## Installation

Import the package from the preview namespace (once published) or local file:

```typ
#import "@preview/rcsections:0.0.1": init_rcs
```

Or if you are using it locally:

```typ
#import "src/lib.typ": init_rcs
```

## Usage

To use `rcsections`, you need to initialize it with a show rule. Then you can write your section definitions in `rcs` code blocks.

```typ
#import "@preview/rcsections:0.0.1": init_rcs

// Initialize the plugin
#show: init_rcs

// Create a figure with a beam section
#figure(
  ```rcs
  beam "B-101":
      30 x 50            // Dimensions: width x height
      span 400           // Span length (for longitudinal view)
      scale 1:50         // Drawing scale
      view both          // View: section, longitudinal, or both
      cover 4            // Concrete cover

      // Reinforcement
      top 2 1"           // Top bars: count size
      bot 3 1"           // Bottom bars: count size
      ties 3/8" 1@15     // Stirrups: size spacing
  ```,
  caption: "Reinforced Concrete Beam Detail",
)
```

## Features

- **Beams**: Define rectangular beams with top and bottom reinforcement.
- **Views**: Generate cross-sections, longitudinal views, or both.
- **Scaling**: Control the scale of the drawing (e.g., `1:50`, `1:20`).
- **Customization**: Configure concrete cover, stirrup spacing, and bar sizes.
