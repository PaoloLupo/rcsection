#import plugin("parser.wasm"): priv_parse_and_generate
#import "draw.typ": draw

// Global cache for section definitions
#let rcs-cache = state("rcs-cache", (:))

/// Parse RCS code and return drawing data
#let parse(expr) = {
  cbor(priv_parse_and_generate(cbor.encode(expr)))
}

/// Define a section and cache it. Optionally render a specific view.
/// - id: Unique identifier for caching
/// - code: The RCS code (raw block)
/// - show-view: Which view to display ("section", "longitudinal", "both", none)
#let rcs-define(id, code, show-view: "section") = {
  let data = parse(code.text)

  // Store in cache
  rcs-cache.update(cache => {
    cache.insert(id, data)
    cache
  })

  // Render if requested
  if show-view != none {
    draw(data, view: show-view)
  }
}

/// Render a cached section with a specific view
/// - id: The section ID to retrieve
/// - view: View type ("section", "longitudinal", "both")
#let rcs-view(id, view: "section") = {
  context {
    let cache = rcs-cache.get()
    if id in cache {
      draw(cache.at(id), view: view)
    } else {
      text(fill: red)[Error: Sección "#id" no definida. Use rcs-define primero.]
    }
  }
}

/// Legacy: Auto-render RCS blocks (no caching)
#let init_rcsection(body, ..options) = {
  context {
    let font = text.font
    show raw.where(lang: "rcs"): it => {
      set text(font: font)
      draw(parse(it.text))
    }
    body
  }
}
