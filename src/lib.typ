#import plugin("parser.wasm"): priv_parse

#let parse(expr) = {
  cbor(priv_parse(cbor.encode(expr)))
}

#show raw.where(lang: "rcs"): it => parse(it.text)

```rcs
Beam "V-101" {
    Shape: Rect(30cm, 40cm);
    Cover: 4cm;
    Concrete: 280;
    Rebar {
        Top: 2 x 1/2";
        Bottom: 3 x 1";
        Bottom: 2 x 3/4" layer 1;
    }
    Stirrups {
        Size: 3/8";
        Dist: 1@5cm, 5@10cm, Rest@20cm;
    }
}
```
