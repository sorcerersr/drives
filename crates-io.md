# drives

A rust library (crate) for listing mounted or mountable drives on linux (flash drives, sd-cards, etc.)

Uses the virtual sysfs filesystem (/sys) to gather information about the block devices known by the linux kernel.

## Example

For an simple example see [simple_main.rs](examples/simple_main.rs):

```
cargo run --example simple_main
```

## Documentation

Documentation can be found on [docs.rs](https://docs.rs/drives/latest/drives/).

## License


Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
