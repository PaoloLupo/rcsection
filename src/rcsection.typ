#import plugin("parser.wasm"): priv_parse_and_generate
#import "draw.typ": draw

/// Parse an expression into a data structure.
///  -> content
#let parse(
  /// The expression to parse.
  expr,
) = {
  let data = cbor(priv_parse_and_generate(cbor.encode(expr)))
  data
}

#let render_rcs(source) = {
  block(draw(parse(source)))
}

#let init_rcsection(body, ..options) = {
  show figure.where(kind: raw): it => {
    if it.body.has("lang") and it.body.lang == "rcs" {
      figure(
        it.body,
        kind: image,
        supplement: it.supplement,
        caption: it.caption,
        numbering: it.numbering,
        gap: it.gap,
        placement: it.placement,
        scope: it.scope,
      )
    } else {
      it
    }
  }
  show raw.where(lang: "rcs"): it => render_rcs(it.text)
  body
}
