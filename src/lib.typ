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

#let init_rcs(body, ..options) = {
  show raw.where(lang: "rcs"): it => draw(parse(it.text))
  body
}
