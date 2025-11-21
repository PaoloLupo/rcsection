#import "/src/rcsection.typ": *

#set page(height: auto, width: auto, margin: 2pt)
#set text(lang: "es")
#show: init_rcsection

#raw(
  block: true,
  lang: "rcs",
  read("../../examples/circular.rcs").trim("\n"),
)
