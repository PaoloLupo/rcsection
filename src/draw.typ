#import "@preview/cetz:0.4.2"

#let parse-color(c) = {
  if c == "black" { black } else if c == "blue" { blue } else if c == "red" { red } else { black }
}

#let draw(drawings, scale: 0.1) = {
  for drawing in drawings {
    cetz.canvas(length: scale * 1cm, {
      for primitive in drawing.primitives {
        if primitive.type == "Rect" {
          let stroke = if primitive.stroke != none {
            (paint: parse-color(primitive.stroke.color), thickness: primitive.stroke.width * 1pt)
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
          let stroke = if primitive.stroke != none {
            (paint: parse-color(primitive.stroke.color), thickness: primitive.stroke.width * 1pt)
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
        } else if primitive.type == "Text" {
          cetz.draw.content((primitive.x, primitive.y), [*#primitive.content*])
        }
      }
    })
  }
}
