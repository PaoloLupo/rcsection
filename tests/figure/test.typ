#import "/src/rcsection.typ": *

#show: init_rcsection
#set page(height: auto, width: auto, margin: 2pt)

#figure(
  ```rcs
  beam "Fig-Test":
      30 x 40
      span 400
      scale 1:50
      view both
      cover 4

      top 2 1"
      bot 2 1"
      ties 1/2" 1@10
  ```,
  caption: "A beam inside a figure",
)
