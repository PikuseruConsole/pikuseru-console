<!-- PROJECT BADGES -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->
[![Github Link][github badge]][github link]
[![Travis Link][travis badge]][travis link]

<!-- PROJECT LOGO -->
<p align="center">
  <img src="pikuseru.png">
</p>

<!-- ABOUT THE PROJECT -->
# Pikuseru Console 

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/PikuseruConsole/pikuseru-console/blob/master/LICENSE.md)

 
## Examples

Some classic 128x128 game:

<img src="docs/dino.gif" width="320">
<img src="docs/ski.gif" width="320">


A 128x128 [demoscene](https://github.com/PikuseruConsole/pikuseru-examples/tree/master/pik/demoscene.pik) cartridge:

<img src="docs/demoscene.gif" width="320">


A 256x256 [ghostmark python](https://github.com/PikuseruConsole/pikuseru-examples/tree/master/pik/ghostmark_py.pik) or a [ghostmark wasm/rust](https://github.com/PikuseruConsole/pikuseru-examples/tree/master/wasm/ghostmark/rust) cartridge to do some benchmark:

<img src="docs/ghostmark.gif" width="320">

A sand game with a custom window (286x286) in WASM (RUST):

<img src="docs/sable.gif" width="320">

## Build

Cargo feature:
  * cpython: enable python support
  * rlua: enable lua support

You can build the console directly the main UI to play games:
```
cd pikuseru-console
cargo build --release --features=pikuseru/cpython,pikuseru/rlua,pikuseru/image
```

[github badge]: https://img.shields.io/badge/github-pikuseruconsole/pikuseruconsole-8da0cb?style=for-the-badge&logo=github
[github link]: https://github.com/PikuseruConsole/pikuseru-console
[travis badge]: https://app.travis-ci.com/PikuseruConsole/pikuseru-console.svg?branch=master
[travis link]: https://travis-ci.com/PikuseruConsole/pikuseru-console