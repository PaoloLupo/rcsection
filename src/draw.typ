#import "@preview/cetz:0.4.2"

#let parse-color(c) = {
  if c.starts-with("#") { rgb(c) } else if c == "black" { black } else if c == "blue" { blue } else if c == "red" {
    red
  } else if c == "orange" { orange } else if c == "purple" { purple } else if c == "green" { green } else { black }
}

#let draw(drawings, scale: 0.1, show_label: true, view: none) = {
  // Filter by view if specified
  let filtered = if view != none {
    drawings.filter(d => d.view == view or d.view == none or view == "all")
  } else {
    drawings
  }

  // Table Layout for multiple sections
  if filtered.len() > 0 {
    let cells = ()

    // Title Row
    // cells.push(table.cell(colspan: drawings.len(), align: center)[*Sections*])

    for drawing in filtered {
      // Title
      if show_label and drawing.id != none {
        cells.push(align(center)[*#drawing.id*])
      }

      // Figure
      let s = if "scale" in drawing and drawing.scale != none { drawing.scale } else { scale }
      cells.push(cetz.canvas(length: s * 1cm, {
        for primitive in drawing.primitives {
          if primitive.type == "Rect" {
            let s_val = primitive.stroke.width * s * 28.346
            let stroke = if primitive.stroke != none {
              (paint: parse-color(primitive.stroke.color), thickness: s_val * 1pt)
            } else {
              none
            }
            let fill = if primitive.fill != none { parse-color(primitive.fill) } else { none }

            cetz.draw.rect(
              (primitive.x, primitive.y),
              (primitive.x + primitive.width, primitive.y + primitive.height),
              stroke: stroke,
              fill: fill,
              name: primitive.group,
            )
          } else if primitive.type == "Circle" {
            let s_val = primitive.stroke.width * s * 28.346
            let stroke = if primitive.stroke != none {
              (paint: parse-color(primitive.stroke.color), thickness: s_val * 1pt)
            } else {
              none
            }
            let fill = if primitive.fill != none { parse-color(primitive.fill) } else { none }

            cetz.draw.circle(
              (primitive.x, primitive.y),
              radius: primitive.radius,
              stroke: stroke,
              fill: fill,
              name: primitive.group,
            )
          } else if primitive.type == "Path" {
            let s_val = primitive.stroke.width * s * 28.346
            let stroke = if primitive.stroke != none {
              (paint: parse-color(primitive.stroke.color), thickness: s_val * 1pt)
            } else {
              none
            }
            let fill = if primitive.fill != none { parse-color(primitive.fill) } else { none }

            cetz.draw.line(
              ..primitive.points,
              close: primitive.closed,
              stroke: stroke,
              fill: fill,
              name: primitive.group,
            )
          } else if primitive.type == "Text" {
            cetz.draw.content((primitive.x, primitive.y), [*#primitive.content*])
          } else if primitive.type == "LeaderLine" {
            // Leader line with arrow for rebar callouts
            let start = primitive.start
            let end = primitive.end

            // Draw line with arrow at start
            cetz.draw.line(
              start,
              end,
              stroke: (paint: black, thickness: 0.3pt),
              mark: (start: ">", fill: black, scale: 0.3),
            )

            // Text anchor based on side
            let anchor = if primitive.side == "top" or primitive.side == "top-right" {
              "south-west"
            } else if primitive.side == "bottom" {
              "north-west"
            } else if primitive.side == "right" {
              "west"
            } else {
              "west"
            }

            // Callout text
            cetz.draw.content(end, anchor: anchor, padding: 0.1, [#primitive.text])
          } else if primitive.type == "Dimension" {
            // Dimension line with arrows and text
            let x1 = primitive.x1
            let y1 = primitive.y1
            let x2 = primitive.x2
            let y2 = primitive.y2

            // Determine if horizontal or vertical
            let is_horizontal = calc.abs(y2 - y1) < 0.01

            // Draw dimension line with arrows at both ends
            cetz.draw.line(
              (x1, y1),
              (x2, y2),
              stroke: (paint: black, thickness: 0.3pt),
              mark: (start: "<", end: ">", fill: black, scale: 0.3),
            )

            // Extension lines - from section edge (with gap) to past dimension line
            let gap = 1.0 // Gap between section edge and extension line
            let beyond = 2.0 // How far past the dimension line
            if is_horizontal {
              // Bottom: vertical extension lines
              let section_y = y1 + 8.0 // Section edge (8 units offset)
              cetz.draw.line((x1, section_y - gap), (x1, y1 - beyond), stroke: 0.2pt)
              cetz.draw.line((x2, section_y - gap), (x2, y2 - beyond), stroke: 0.2pt)
            } else {
              // Left: horizontal extension lines
              let section_x = x1 + 8.0 // Section edge (8 units offset)
              cetz.draw.line((section_x - gap, y1), (x1 - beyond, y1), stroke: 0.2pt)
              cetz.draw.line((section_x - gap, y2), (x2 - beyond, y2), stroke: 0.2pt)
            }

            // Centered text
            let mid_x = (x1 + x2) / 2
            let mid_y = (y1 + y2) / 2
            let txt = if primitive.text != none { primitive.text } else { "" }
            if txt != "" {
              let anchor = if is_horizontal { "south" } else { "east" }
              cetz.draw.content((mid_x, mid_y), anchor: anchor, padding: 0.15, [#txt])
            }
          }
        }
      }))
    }

    table(
      columns: 1,
      align: center + horizon,
      stroke: none,
      ..cells
    )
  }
}
