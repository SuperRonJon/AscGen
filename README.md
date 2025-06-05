# AscGen
CLI Ascii Art Generator. Creates ascii art from image files.

## Installation
Pre-built binaries available for Windows & Linux x86_64 [from the latest release here](https://github.com/SuperRonJon/AscGen/releases/latest), or build from source with `cargo build --release`.

## Options/Usage
```
Usage: ascgen [OPTIONS] <FILENAME>

Arguments:
  <FILENAME>  Path to the image file

Options:
  -i, --invert                   Invert the image color brightnesses
  -w, --width <WIDTH_SCALING>    Width Scaling Factor [default: -1]
  -t, --height <HEIGHT_SCALING>  Height Scaling Factor [default: -1]
  -s, --scaling <SCALING>        Even scaling factor for both height & width [default: 1]
  -f, --file <OUT_FILE>          Output file name. If supplied, outputs to file rather than to console [default: ]
  -h, --help                     Print help
  -V, --version                  Print version
```

Scaling factor values are floating point values that indicate the amount to scale the original image by.

Ex. `ascgen --width 0.2 --height 0.1 image.png` will scale the ascii-art output's width down to 20% and height to 10% the original image.png's pixel resolution. This is equivalent to `ascgen -w 0.2 -t 0.1 image.png` using short flags

Ex. `ascgen --invert --scaling 0.015 high-res-image.png` will scale both the height and width of high-res-image.png to 1.5% the original image's pixel resolution, as well as invert the color scale, so that bright colors to use the densest characters rather than vice-versa. This is equivalent to `ascgen -is 0.015 high-res-image.png`.

## Example
```
-> $ ascgen -iw 0.055 -t 0.035 saturn.jpg
-> $ ascgen --invert --width 0.055 --height 0.035 saturn.jpg


                                                        **
                                            :.::::#******###%@@@::::::
                                      :::::. =---::---==++**##%%@@--- .::::-
                                  :::: -------       ..:-==++*##%%@@---=--- ::::
                               :::: --=--::             .::-=++**#%@@ .::----- ::::
                             :::: ----::                   .:-=++*#%%@  . ::----..:::
                            ::: ----::      .                .:-=+*#%%    . ::--:- ::::
                            :: ----::      .                  .:-=+*#%@      ::---- ::.
                          :::: -=--:.                           .:-=*#%      .:----:.:::
                          :::: ----::                            .:-+*#      .:--=- .:::
                           ::: -----                               :=*      .:----= :::
                             ::  --                                 --    .::----:.::.:
                             ::::..                                 -   ::------ ::::
                               ::::                                :::------- .::::
                                  :::.:        -----::::::::::-----------  :::::
                                      :.:::::   :-----------------   :::::::
                                            ::::::::::::::::::::::::::



```
