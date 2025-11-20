#import plugin("parser.wasm"): priv_parse_and_generate
#import "draw.typ": draw

#let parse(expr) = {
  let data = cbor(priv_parse_and_generate(cbor.encode(expr)))
  data
}

#show raw.where(lang: "rcs"): it => draw(parse(it.text))

```rcs
beam "V-101":
    30 x 60
    cover 5
    fc 280

    top 2 1"
    bot 3 1"
    bot 2 3/4"

    ties 3/8" 1@5 5@10 rto@20

beam "V-103":
    40 x 60
    cover 4
    fc 280

    top 2 1"
    bot 2 2"

    ties 3/8" 1@5 5@10 rto@20
```
