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
