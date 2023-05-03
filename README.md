[![Build and Tests](https://github.com/sorcerersr/drives/actions/workflows/build_and_test.yml/badge.svg)](https://github.com/sorcerersr/drives/actions/workflows/build_and_test.yml) [![crates.io](https://img.shields.io/crates/v/drives)](https://crates.io/crates/drives) [![docs.rs](https://img.shields.io/docsrs/drives/latest)](https://docs.rs/drives/latest/drives/)

# drives

A rust library (crate) for listing mounted or mountable drives on linux (flash drives, sd-cards, etc.)

Uses the virtual sysfs filesystem (/sys) to gather information about the block devices known by the linux kernel.

## Example

For an simple example see [simple_main.rs](examples/simple_main.rs):

```
cargo run --example simple_main
```


## License


Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
