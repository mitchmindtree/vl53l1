# vl53l1 [![Actions Status](https://github.com/mitchmindtree/vl53l1/workflows/vl53l1/badge.svg)](https://github.com/mitchmindtree/vl53l1/actions) [![Crates.io](https://img.shields.io/crates/l/vl53l1.svg)](https://github.com/mitchmindtree/vl53l1/blob/master/LICENSE-MIT)

A pure-Rust port of the official ST VL53L1X ToF sensor C API (STSW-IMG007).

The `lib` directory contains the library crates. The `examples` directory
contains a single examples for the STM32F107, though the library itself should
be compatible with any device that Rust can target and that has an
implementation of the `embedded-hal` I2C traits.

The `lib/vl53l1-reg` crate contains a generated register map, register structs,
and some helper functions for writing to and reading from registers via I2C.

[![crates.io](https://img.shields.io/crates/v/vl53l1-reg.svg)](https://crates.io/crates/vl53l1-reg) [![docs.rs](https://docs.rs/vl53l1-reg/badge.svg)](https://docs.rs/vl53l1-reg/)

The `lib/vl53l1` crate depend on the `vl53l1-reg`, handles most of the
implementation and exposes the public API.

[![crates.io](https://img.shields.io/crates/v/vl53l1.svg)](https://crates.io/crates/vl53l1) [![docs.rs](https://docs.rs/vl53l1/badge.svg)](https://docs.rs/vl53l1/)

While much of the code has been Rust-ified, the function tree and overall
architecture are still a direct port of the original C code. Feel free to submit
PRs or issues related to rustifying the library further!
