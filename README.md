[![Build and Tests](https://github.com/sorcerersr/drives/actions/workflows/build_and_test.yml/badge.svg)](https://github.com/sorcerersr/drives/actions/workflows/build_and_test.yml)
[![codecov](https://codecov.io/gh/sorcerersr/drives/branch/main/graph/badge.svg?token=4ATZX63FP6)](https://codecov.io/gh/sorcerersr/drives)
[![crates.io](https://img.shields.io/crates/v/drives)](https://crates.io/crates/drives)
[![docs.rs](https://img.shields.io/docsrs/drives/latest)](https://docs.rs/drives/latest/drives/)

# drives

A rust library (crate) for listing mounted or mountable drives on linux (flash drives, sd-cards, etc.)

Uses the virtual kernel filesystems (/sys, /proc and /dev) to gather information about the block devices known by the linux kernel.
Optionally reads the GUID Partition Table (GPT) to enrich gathered data with informations from the partition table.

## Data

* devices
  * name
  * model, serial
  * size
  * partitions
  * is removable
  * uuid (optionally from GPT)
* partition
  * name
  * size
  * mountpoint (path, filesystem)
  * part_uuid (optionally from GPT)

## Example

For an simple example see [simple_main.rs](examples/simple_main.rs):

```
cargo run --example simple_main
```

## Optional Data from GUID Partition Table (GPT)

Currently only the UUID for a device and the PART_UUID of partitions are retreived using the GPT. This needs the feature "gpt" to be enabled.

```
cargo build --features gpt
```

## License


Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
