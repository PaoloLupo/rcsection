#import "/src/rcsection.typ": *
#set page(height: auto, width: auto, margin: 2pt)
#set text(lang: "es")

#rcs-define(
  "C-Rect",
  ```rcs
  section "C-Rect":
      shape rect 40 40
      concrete:
          cover 4
      perim 8 #6
      view both
      ties #3 1@5 5@10 rto@20
  ```,
  show-view: "both",
)


