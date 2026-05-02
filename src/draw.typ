#import "@preview/cetz:0.4.2"

#let parse-color(c) = {
  if c.starts-with("#") { rgb(c) }
  else if c == "black" { black }
  else if c == "blue" { blue }
  else if c == "red" { red }
  else if c == "orange" { orange }
  else if c == "purple" { purple }
  else if c == "green" { green }
  else { black }
}

/// Extract stroke from primitive, scaling width appropriately
#let parse-stroke(primitive, scale) = {
  if primitive.stroke == none { return none }
  let s-val = primitive.stroke.width * scale * 28.346
  (paint: parse-color(primitive.stroke.color), thickness: s-val * 1pt)
}

/// Extract fill from primitive
#let parse-fill(primitive) = {
  if primitive.fill == none { none } else { parse-color(primitive.fill) }
}

/// Draw a dimension primitive with extension lines and centered text
#let draw-dimension(p) = {
  let x1 = p.x1
  let y1 = p.y1
  let x2 = p.x2
  let y2 = p.y2
  let is-horizontal = calc.abs(y2 - y1) < 0.01

  // Dimension line with arrows
  cetz.draw.line(
    (x1, y1),
    (x2, y2),
    stroke: (paint: black, thickness: 0.3pt),
    mark: (start: "<", end: ">", fill: black, scale: 0.3),
  )

  // Extension lines
  let gap = 1.0
  let beyond = 2.0
  if is-horizontal {
    let section-y = y1 + 8.0
    cetz.draw.line((x1, section-y - gap), (x1, y1 - beyond), stroke: 0.2pt)
    cetz.draw.line((x2, section-y - gap), (x2, y2 - beyond), stroke: 0.2pt)
  } else {
    let section-x = x1 + 8.0
    cetz.draw.line((section-x - gap, y1), (x1 - beyond, y1), stroke: 0.2pt)
    cetz.draw.line((section-x - gap, y2), (x2 - beyond, y2), stroke: 0.2pt)
  }

  // Centered text
  let txt = p.text
  if txt != none and txt != "" {
    let mid-x = (x1 + x2) / 2
    let mid-y = (y1 + y2) / 2
    let anchor = if is-horizontal { "south" } else { "east" }
    cetz.draw.content((mid-x, mid-y), anchor: anchor, padding: 0.15, [#txt])
  }
}

/// Draw a leader line with arrow and anchored text
#let draw-leader-line(p) = {
  let start = p.start
  let end = p.end

  cetz.draw.line(
    start,
    end,
    stroke: (paint: black, thickness: 0.3pt),
    mark: (start: ">", fill: black, scale: 0.3),
  )

  let anchor = if p.side == "top" or p.side == "top-right" {
    "south-west"
  } else if p.side == "bottom" {
    "north-west"
  } else if p.side == "right" {
    "west"
  } else if p.side == "left" {
    "east"
  } else {
    "west"
  }

  cetz.draw.content(end, anchor: anchor, padding: 0.1, [#p.text])
}

/// Draw a single primitive inside a CETZ canvas
#let draw-primitive(p, scale) = {
  let t = p.type
  if t == "Rect" {
    cetz.draw.rect(
      (p.x, p.y),
      (p.x + p.width, p.y + p.height),
      stroke: parse-stroke(p, scale),
      fill: parse-fill(p),
      name: p.group,
    )
  } else if t == "Circle" {
    cetz.draw.circle(
      (p.x, p.y),
      radius: p.radius,
      stroke: parse-stroke(p, scale),
      fill: parse-fill(p),
      name: p.group,
    )
  } else if t == "Path" {
    cetz.draw.line(
      ..p.points,
      close: p.closed,
      stroke: parse-stroke(p, scale),
      fill: parse-fill(p),
      name: p.group,
    )
  } else if t == "Text" {
    cetz.draw.content((p.x, p.y), [*#p.content*])
  } else if t == "LeaderLine" {
    draw-leader-line(p)
  } else if t == "Dimension" {
    draw-dimension(p)
  }
}

/// Main draw function: renders a list of drawings into a Typst table layout
#let draw(drawings, scale: 0.1, show-label: true, view: none) = {
  let filtered = if view != none {
    drawings.filter(d => d.view == view or d.view == none or view == "all" or view == "both")
  } else {
    drawings
  }

  if filtered.len() == 0 { return }

  let cells = ()

  for drawing in filtered {
    if show-label and drawing.id != none {
      cells.push(align(center)[*#drawing.id*])
    }

    let s = if "scale" in drawing and drawing.scale != none { drawing.scale } else { scale }
    cells.push(cetz.canvas(length: s * 1cm, {
      for primitive in drawing.primitives {
        draw-primitive(primitive, s)
      }
    }))
  }

  table(
    columns: 1,
    align: center + horizon,
    stroke: none,
    ..cells,
  )
}
