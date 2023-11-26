# dices_art_generator

The goal of this small project is to convert image pixels into dices.

It should give the illusion of shapes from the original image with dices.   

If you want to run it:
```shell
cargo run -- "original image path" factor "dices"
```
#### Examples:

To print with utf8 dices:
```shell
cargo run -- ./assets/crab.webp 12 "dices"
```
To print with dice numbers:
```shell
cargo run -- ./assets/crab.webp 12
```

After the first run you can see a generated file named `output.png` that represent the compressed original image.
It groups all pixels into a square of `factor` size and do the average gray-scaled colors of the group and then apply the new color to all pixels of the group. 