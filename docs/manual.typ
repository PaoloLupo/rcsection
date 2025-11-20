#import "@preview/tidy:0.4.3"

#let package-meta = toml("/typst.toml").package
#let date = datetime.today().display()

= Hola mundo
#package-meta.name
#date

#let docs = tidy.parse-module(read("../src/lib.typ"), name: "Lib")
#tidy.show-module(docs)
