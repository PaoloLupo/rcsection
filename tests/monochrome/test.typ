#import "/src/rcsection.typ": *

#set page(height: auto, width: auto, margin: 2pt)
#set text(lang: "es")

#rcs-define(
  "V-102-mono",
  ```rcs
  set:
      monochrome

  section "V-102":
      shape rect 30 40
      concrete:
          cover 4
      top 1 #8 1 3/8" 1 #8
      bot 3 #8
      ties #3 1@5 5@10 rto@20
  ```,
  show-view: "section",
)

#rcs-define(
  "V-102-color",
  ```rcs
  section "V-102":
      shape rect 30 40
      concrete:
          cover 4
      top 1 #8 1 3/8" 1 #8
      bot 3 #8
      ties #3 1@5 5@10 rto@20
  ```,
  show-view: "section",
)
