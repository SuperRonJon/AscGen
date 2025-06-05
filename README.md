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
  -h, --help                     Print help
  -V, --version                  Print version
```

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
